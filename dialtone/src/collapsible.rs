use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use super::button::DtButton;
use super::icon::{DtIconChevronDown, DtIconChevronRight};
use super::list_nav::join_class;
use super::unique_id;

/// Root and content tags. Anything other than `div` warns and still renders `div`.
pub(crate) fn collapsible_element_tag(prop: &str, element_type: &str) -> &'static str {
    if element_type != "div" {
        leptos::logging::warn!("DtCollapsible {prop} `{element_type}` is not div; rendering div");
    }
    "div"
}

/// Dialtone Vue `DtCollapsible` (`dialtone/v10.7.0`).
///
/// `open` unset is uncontrolled and starts expanded. The default anchor toggles
/// that local state. A set `open` is controlled: the default anchor does
/// nothing, and the parent drives visibility. `on_open_change` (`update:open`)
/// and `on_opened` (`opened`) run after the height transition, and only while
/// the component is still mounted. `on_open_change` runs only when `open` is
/// set. Vue's `opened` also passes the content element. This callback is
/// `bool` only.
///
/// `element_type` (`elementType`) and `content_element_type`
/// (`contentElementType`) other than `div` warn and still render a `div`.
/// The chevron is the button `start_icon`, with `d-collapsible__icon` on the
/// svg. A custom `anchor` slot does not receive Vue's `aria-controls`,
/// `aria-expanded`, or `role`. Put those on the slotted control yourself.
/// Fallthrough listeners are not ported. Children stay unmounted until the
/// first open, then stay mounted.
#[component]
pub fn DtCollapsible(
    /// Content element id. Generated when empty.
    #[prop(optional, into)]
    id: String,
    /// `aria-label` on the content.
    #[prop(optional, into)]
    aria_label: String,
    /// `aria-labelledby` (`ariaLabelledBy`). Overrides the generated anchor id.
    #[prop(optional, into)]
    aria_labelled_by: String,
    /// Default anchor label (`anchorText`).
    #[prop(optional, into)]
    anchor_text: String,
    /// Extra class on the anchor wrapper (`anchorClass`).
    #[prop(optional, into)]
    anchor_class: String,
    /// Extra class on the content (`contentClass`).
    #[prop(optional, into)]
    content_class: String,
    /// CSS max-width on the anchor and the content (`maxWidth`).
    #[prop(optional, into)]
    max_width: String,
    /// CSS max-height on the content (`maxHeight`).
    #[prop(optional, into)]
    max_height: String,
    /// Root tag (`elementType`). Anything other than `div` warns and still renders `div`.
    #[prop(optional, default = "div")]
    element_type: &'static str,
    /// Content tag (`contentElementType`). Anything other than `div` warns and still renders `div`.
    #[prop(optional, default = "div")]
    content_element_type: &'static str,
    /// Controlled open state (`open`). Unset starts open and toggles locally.
    #[prop(optional)]
    open: Option<Signal<bool>>,
    /// `update:open`. Fired after the height transition when `open` is set.
    #[prop(optional, into)]
    on_open_change: Option<Callback<bool>>,
    /// `opened`. Fired with the next open state after the height transition.
    #[prop(optional, into)]
    on_opened: Option<Callback<bool>>,
    /// Replaces the default anchor button.
    #[prop(optional)]
    anchor: Option<ViewFn>,
    children: ChildrenFn,
) -> impl IntoView {
    collapsible_element_tag("elementType", element_type);
    collapsible_element_tag("contentElementType", content_element_type);

    let content_id = if id.is_empty() {
        unique_id("dt-collapsible")
    } else {
        id
    };
    let generated_anchor = unique_id("DtCollapsible__anchor");
    let labelled_by = if !aria_labelled_by.is_empty() {
        aria_labelled_by.clone()
    } else if aria_label.is_empty() {
        generated_anchor
    } else {
        String::new()
    };
    let anchor_id = if aria_labelled_by.is_empty() {
        labelled_by.clone()
    } else {
        String::new()
    };
    if anchor.is_none() && anchor_text.is_empty() {
        leptos::logging::warn!("anchor text and anchor slot content cannot both be falsy");
    }

    let controlled = open.is_some();
    // A controlled section that starts closed must not paint open. The effect
    // below only copies later changes.
    let initial_open = open.map(|signal| signal.get_untracked()).unwrap_or(true);
    let is_open = RwSignal::new(initial_open);
    Effect::new(move |_| {
        if let Some(open) = open {
            let next = open.get();
            // Same-value writes still notify. Skip them so the transition
            // callback can echo the value the parent already set.
            if is_open.get_untracked() != next {
                is_open.set(next);
            }
        }
    });

    let initialized = RwSignal::new(initial_open);
    let phase = RwSignal::new("idle");
    let height_style = RwSignal::new(if initial_open {
        String::new()
    } else {
        "height:0px;overflow:hidden;".to_string()
    });
    let content_ref = NodeRef::<html::Div>::new();
    // The height frames are queued on the window. Cleanup has to ignore them
    // or a disposed node ref panics after the collapsible unmounts.
    let alive = Arc::new(AtomicBool::new(true));
    on_cleanup({
        let alive = Arc::clone(&alive);
        move || alive.store(false, Ordering::Relaxed)
    });
    let alive_for_effect = Arc::clone(&alive);

    let finish = {
        let alive = Arc::clone(&alive);
        move |open_now: bool| {
            if !alive.load(Ordering::Relaxed) {
                return;
            }
            phase.set("idle");
            if open_now {
                height_style.set(String::new());
            } else {
                height_style.set("height:0px;overflow:hidden;".to_string());
            }
            if let Some(on_opened) = on_opened.as_ref() {
                on_opened.run(open_now);
            }
            if controlled {
                if let Some(on_open_change) = on_open_change.as_ref() {
                    on_open_change.run(open_now);
                }
            }
        }
    };

    Effect::new(move |_| {
        let open_now = is_open.get();
        if open_now {
            initialized.set(true);
        }
        // A section that has never opened has no content to measure.
        if !initialized.get_untracked() {
            return;
        }
        let alive = Arc::clone(&alive_for_effect);
        request_animation_frame(move || {
            if !alive.load(Ordering::Relaxed) {
                return;
            }
            let Some(element) = content_ref.get_untracked() else {
                return;
            };
            let element: HtmlElement = element.unchecked_into();
            if !alive.load(Ordering::Relaxed) {
                return;
            }
            if open_now {
                phase.set("enter");
                height_style.set("height:0px;overflow:hidden;".to_string());
                let alive = Arc::clone(&alive);
                request_animation_frame(move || {
                    if !alive.load(Ordering::Relaxed) {
                        return;
                    }
                    let height = element.scroll_height();
                    height_style.set(format!("height:{height}px;overflow:hidden;"));
                });
            } else {
                let height = element.offset_height();
                phase.set("leave");
                height_style.set(format!("height:{height}px;overflow:hidden;"));
                let alive = Arc::clone(&alive);
                request_animation_frame(move || {
                    if !alive.load(Ordering::Relaxed) {
                        return;
                    }
                    height_style.set("height:0px;overflow:hidden;".to_string());
                });
            }
        });
    });

    let content_class_signal = StoredValue::new(content_class);
    let max_height = StoredValue::new(max_height);
    let max_width_content = StoredValue::new(max_width.clone());
    let labelled_by_attr = (!labelled_by.is_empty()).then_some(labelled_by);
    let aria_label_attr = (!aria_label.is_empty()).then_some(aria_label);
    let anchor_id_attr = (!anchor_id.is_empty()).then_some(anchor_id);

    let anchor_controls = content_id.clone();
    let anchor_view = if let Some(anchor) = anchor {
        anchor.run().into_any()
    } else {
        let anchor_text_title = anchor_text.clone();
        view! {
            {move || {
                let open_now = is_open.get();
                let start_icon = if open_now {
                    ViewFn::from(|| {
                        view! { <DtIconChevronDown class="d-collapsible__icon" size="300" /> }
                    })
                } else {
                    ViewFn::from(|| {
                        view! { <DtIconChevronRight class="d-collapsible__icon" size="300" /> }
                    })
                };
                let text = anchor_text.clone();
                let title = anchor_text_title.clone();
                view! {
                    <DtButton
                        importance="clear"
                        kind="muted"
                        width=max_width.clone()
                        aria_controls=anchor_controls.clone()
                        aria_expanded=open_now
                        start_icon=Some(start_icon)
                        on_click=Callback::new(move |_| {
                            if controlled {
                                return;
                            }
                            is_open.set(!is_open.get_untracked());
                        })
                    >
                        <span class="d-collapsible__anchor-text" title=title>{text}</span>
                    </DtButton>
                }
            }}
        }
        .into_any()
    };

    view! {
        <div>
            <div id=anchor_id_attr class=anchor_class>
                {anchor_view}
            </div>
            <div
                node_ref=content_ref
                id=content_id.clone()
                class=move || {
                    let phase = phase.get();
                    let extra = content_class_signal.get_value();
                    let motion = match phase {
                        "enter" => "d-collapsible__enter-active",
                        "leave" => "d-collapsible__leave-active",
                        _ => "",
                    };
                    join_class([extra.as_str(), motion])
                }
                style=move || {
                    let mut style = height_style.get();
                    let max_height = max_height.get_value();
                    let max_width = max_width_content.get_value();
                    if !max_height.is_empty() {
                        style.push_str(&format!("max-height:{max_height};"));
                    }
                    if !max_width.is_empty() {
                        style.push_str(&format!("max-width:{max_width};"));
                    }
                    style
                }
                aria-hidden=move || if is_open.get() { "false" } else { "true" }
                aria-labelledby=labelled_by_attr.clone()
                aria-label=aria_label_attr.clone()
                data-qa="dt-collapsible--content"
                tabindex="-1"
                on:transitionend=move |_| finish(is_open.get_untracked())
            >
                <Show when=move || initialized.get()>
                    {children()}
                </Show>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn element_type_other_than_div_still_renders_div() {
        assert_eq!(collapsible_element_tag("elementType", "div"), "div");
        assert_eq!(collapsible_element_tag("elementType", "section"), "div");
        assert_eq!(
            collapsible_element_tag("contentElementType", "article"),
            "div"
        );
    }
}
