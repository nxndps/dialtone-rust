use std::time::Duration;

use leptos::html;
use leptos::portal::Portal;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, KeyboardEvent, MouseEvent};

use super::popper::{
    compute_position, ComputedPosition, Options as PopperOptions, Placement, Rect,
};
use super::unique_id;

/// Matches Dialtone Vue `TOOLTIP_DELAY_MS`.
const TOOLTIP_DELAY_MS: u64 = 300;
/// Matches Dialtone Vue `offset: [0, 12]`.
const DEFAULT_OFFSET: f64 = 12.0;
const VIEWPORT_PADDING: f64 = 8.0;

fn has_visible_focus(anchor: Option<HtmlElement>) -> bool {
    anchor
        .and_then(|el| el.matches(":focus-visible").ok())
        .unwrap_or(false)
}

fn tooltip_z_index(anchor: Option<HtmlElement>) -> i32 {
    let in_overlay = anchor
        .and_then(|el| {
            el.closest(".d-modal[aria-hidden=\"false\"], .d-modal--transparent[aria-hidden=\"false\"], .d-modal[open], .d-modal--transparent[open], .d-zi-drawer, [aria-modal=\"true\"]")
                .ok()
                .flatten()
        })
        .is_some();
    if in_overlay {
        651
    } else {
        400
    }
}

fn element_rect(el: &HtmlElement) -> Rect {
    let rect = el.get_bounding_client_rect();
    Rect {
        x: rect.x(),
        y: rect.y(),
        width: rect.width(),
        height: rect.height(),
    }
}

fn viewport_rect() -> Rect {
    let window = web_sys::window().expect("window");
    Rect {
        x: 0.0,
        y: 0.0,
        width: window
            .inner_width()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0),
        height: window
            .inner_height()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0),
    }
}

fn reference_element(anchor: &HtmlElement) -> HtmlElement {
    let mut current: HtmlElement = anchor
        .first_element_child()
        .and_then(|el| el.dyn_into().ok())
        .unwrap_or_else(|| anchor.clone());
    // `display: contents` wrappers have an empty box. Walk to the button.
    for _ in 0..6 {
        let rect = current.get_bounding_client_rect();
        if rect.width() > 0.0 || rect.height() > 0.0 {
            return current;
        }
        match current
            .first_element_child()
            .and_then(|el| el.dyn_into().ok())
        {
            Some(child) => current = child,
            None => return current,
        }
    }
    current
}

fn pointer_stays_inside(event: &MouseEvent, anchor: &HtmlElement) -> bool {
    event
        .related_target()
        .and_then(|target| target.dyn_into::<web_sys::Node>().ok())
        .is_some_and(|node| anchor.contains(Some(&node)))
}

fn parse_fallbacks(raw: &str) -> Vec<Placement> {
    raw.split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty() && *part != "auto")
        .map(Placement::parse)
        .collect()
}

fn tooltip_class(arrow: &str, extra: &str, inverted: bool, shown: bool, show_arrow: bool) -> String {
    let mut parts = vec!["d-tooltip".to_string()];
    if show_arrow && !arrow.is_empty() {
        parts.push(arrow.to_string());
    }
    if !extra.trim().is_empty() {
        parts.push(extra.trim().to_string());
    }
    if inverted {
        parts.push("d-tooltip--inverted".to_string());
    }
    parts.push(if shown {
        "d-tooltip--show".to_string()
    } else {
        "d-tooltip--hide".to_string()
    });
    parts.join(" ")
}

/// Dialtone Vue `DtTooltip`. Follows `dialtone/v10.7.0`.
///
/// The tooltip is portaled to `document.body` (`appendTo: 'body'`) unless
/// `append_to` is `parent`. `root` and an `HTMLElement` warn and use `body`.
/// A `body` value does not look for an ancestor `dialog`. Placement uses
/// [`super::popper`], not Tippy. `show` is Vue `open`. `inverted` is deprecated
/// and still adds `d-tooltip--inverted`. `sticky` `reference` and `popper` are
/// not separate; true polls. `externalAnchor` is not ported.
#[component]
pub fn DtTooltip(
    /// Tooltip id. Auto-generated when omitted.
    #[prop(optional, into)]
    id: String,
    /// Tooltip text. Ignored when `content` is set.
    #[prop(optional, into)]
    message: String,
    /// Extra classes on the tooltip element (`contentClass` in Vue).
    #[prop(optional, into)]
    content_class: String,
    /// Extra class on the anchor wrapper (`anchorClass`).
    #[prop(optional, into)]
    anchor_class: String,
    /// Draw the placement arrow (`showArrow`). Default true.
    #[prop(optional, default = true)]
    show_arrow: bool,
    /// `body` (default) or `parent` (`appendTo`).
    #[prop(optional, default = "body")]
    append_to: &'static str,
    /// Inverted (light) tooltip surface.
    #[prop(optional)]
    inverted: bool,
    /// Preferred placement. Popper flips when it does not fit.
    #[prop(optional, default = "top")]
    placement: &'static str,
    /// Comma-separated fallbacks, or `auto` (Dialtone default).
    #[prop(optional, default = "auto")]
    fallback_placements: &'static str,
    /// Distance from the anchor in px. Vue `offset: [0, 12]`.
    #[prop(optional, default = DEFAULT_OFFSET)]
    offset: f64,
    /// Reposition on scroll/resize while open. Vue `sticky` default true.
    #[prop(optional, default = true)]
    sticky: bool,
    /// When false, hover/focus never open the tooltip.
    #[prop(optional, default = true)]
    enabled: bool,
    /// Controlled visibility. Unset = show on hover / `:focus-visible`.
    #[prop(optional, into)]
    show: MaybeProp<bool>,
    /// Delay before showing on hover/focus. Default true (300ms), matching Vue.
    #[prop(optional, default = true)]
    delay: bool,
    /// Fade transition via Dialtone show/hide classes. Default true.
    #[prop(optional, default = true)]
    transition: bool,
    /// Extra inline style on the tooltip.
    #[prop(optional, into)]
    style: MaybeProp<String>,
    /// `display: contents` on the anchor wrappers so a parent flex row sees the
    /// child. Hover uses `mouseover` because `mouseenter` needs a box.
    #[prop(optional)]
    flatten: bool,
    /// Emitted when visibility changes (`shown` in Vue).
    #[prop(optional, into)]
    on_shown: Option<Callback<bool>>,
    /// Anchor element (`#anchor` slot in Vue).
    children: Children,
    /// Rich tooltip body (default slot in Vue). Overrides `message`.
    #[prop(optional)]
    content: Option<ViewFn>,
) -> impl IntoView {
    let id = if id.is_empty() {
        unique_id("dt-tooltip")
    } else {
        id
    };
    let anchor_ref = NodeRef::<html::Span>::new();
    let tooltip_ref = NodeRef::<html::Div>::new();
    let internal_show = RwSignal::new(false);
    let coords = RwSignal::new(Option::<ComputedPosition>::None);
    let in_timer = StoredValue::new(None::<TimeoutHandle>);
    let has_body = content.is_some() || !message.trim().is_empty();
    let preferred = Placement::parse(placement);
    let tooltip_body = StoredValue::new(content.unwrap_or_else(|| {
        let message = message.clone();
        ViewFn::from(move || view! { {message.clone()} })
    }));
    let content_class = StoredValue::new(content_class);
    let tooltip_id = StoredValue::new(id);
    let warned_enabled = StoredValue::new(false);
    if append_to != "body" && append_to != "parent" {
        leptos::logging::warn!("DtTooltip append_to `{append_to}` is not body or parent; using body");
    }
    let append_inline = append_to == "parent";

    let clear_timer = move || {
        in_timer.try_update_value(|slot| {
            if let Some(handle) = slot.take() {
                handle.clear();
            }
        });
    };

    let set_shown = move |value: bool| {
        let Some(current) = internal_show.try_get_untracked() else {
            return;
        };
        if !enabled || (value && !has_body) {
            if current {
                internal_show.try_set(false);
                coords.try_set(None);
                if let Some(on_shown) = on_shown {
                    on_shown.run(false);
                }
            }
            return;
        }
        if current == value {
            return;
        }
        if internal_show.try_set(value).is_none() {
            return;
        }
        if !value {
            coords.try_set(None);
        }
        if let Some(on_shown) = on_shown {
            on_shown.run(value);
        }
    };

    Effect::new(move |_| {
        if !enabled {
            if show.get().is_some()
                && warned_enabled
                    .try_update_value(|warned| {
                        if *warned {
                            false
                        } else {
                            *warned = true;
                            true
                        }
                    })
                    .unwrap_or(false)
            {
                leptos::logging::warn!(
                    "Tooltip: You cannot use both the enabled and open props at the same time."
                );
                leptos::logging::warn!("The open prop will be ignored.");
            }
            set_shown(false);
            return;
        }
        if let Some(show) = show.get() {
            set_shown(show);
        }
    });

    let trigger_show = move |kind: &'static str| {
        if show.try_get_untracked().is_none_or(|show| show.is_some()) {
            return;
        }
        if kind == "focusin" {
            let el = anchor_ref
                .get_untracked()
                .map(|el| el.unchecked_into::<HtmlElement>());
            if has_visible_focus(el) {
                set_shown(true);
            }
        } else {
            set_shown(true);
        }
    };

    let on_enter_anchor = move |kind: &'static str| {
        if !enabled {
            return;
        }
        if show.try_get_untracked().is_none_or(|show| show.is_some()) {
            return;
        }
        if delay {
            if in_timer
                .try_get_value()
                .is_none_or(|handle| handle.is_some())
            {
                return;
            }
            match set_timeout_with_handle(
                move || {
                    if in_timer.try_update_value(|slot| *slot = None).is_none() {
                        return;
                    }
                    trigger_show(kind);
                },
                Duration::from_millis(TOOLTIP_DELAY_MS),
            ) {
                Ok(handle) => {
                    in_timer.try_set_value(Some(handle));
                }
                Err(_) => trigger_show(kind),
            }
        } else {
            trigger_show(kind);
        }
    };

    let on_leave_anchor = move || {
        clear_timer();
        if matches!(show.try_get_untracked(), Some(None)) {
            set_shown(false);
        }
    };

    // Animation frames and the sticky interval are not a tracking context.
    let reposition = move || {
        let Some(anchor) = anchor_ref.get_untracked() else {
            return;
        };
        let Some(tooltip) = tooltip_ref.get_untracked() else {
            return;
        };
        let anchor: HtmlElement = anchor.unchecked_into();
        let tooltip: HtmlElement = tooltip.unchecked_into();
        let reference = element_rect(&reference_element(&anchor));
        let floating = element_rect(&tooltip);
        if floating.width <= 0.0 || floating.height <= 0.0 {
            return;
        }
        let computed = compute_position(
            reference,
            floating,
            preferred,
            &PopperOptions {
                offset,
                padding: VIEWPORT_PADDING,
                viewport: viewport_rect(),
                fallbacks: parse_fallbacks(fallback_placements),
                auto: Placement::is_auto(placement),
                tether: false,
            },
        );
        coords.set(Some(computed));
    };

    Effect::new(move |_| {
        if !internal_show.get() {
            coords.set(None);
            return;
        }
        if tooltip_ref.get().is_none() {
            return;
        }
        reposition();
        request_animation_frame(move || {
            reposition();
        });
        if !sticky {
            return;
        }
        if let Ok(handle) =
            set_interval_with_handle(move || reposition(), Duration::from_millis(50))
        {
            on_cleanup(move || handle.clear());
        }
    });

    let flatten_style = flatten.then_some("display: contents");
    let bubble = StoredValue::new(ViewFn::from(move || {
        view! {
            <div
                node_ref=tooltip_ref
                id=tooltip_id.get_value()
                class=move || {
                    let computed = coords.get();
                    let arrow = computed
                        .map(|c| c.placement.arrow_class())
                        .unwrap_or(preferred.arrow_class());
                    let extra = content_class.get_value();
                    let visible = computed.is_some_and(|c| !c.hidden);
                    tooltip_class(&arrow, &extra, inverted, visible, show_arrow)
                }
                role="tooltip"
                data-qa="dt-tooltip"
                attr:data-placement=move || {
                    coords
                        .get()
                        .map(|c| c.placement.as_str())
                        .unwrap_or(placement)
                }
                style=move || {
                    let z = tooltip_z_index(
                        anchor_ref.get().map(|el| el.unchecked_into::<HtmlElement>()),
                    );
                    let extra = style.get().unwrap_or_default();
                    let fade = if transition {
                        String::new()
                    } else {
                        "transition:none;".to_string()
                    };
                    match coords.get() {
                        Some(c) => format!(
                            "position:fixed;top:{}px;left:{}px;bottom:auto;right:auto;transform:none;z-index:{z};--dt-tooltip-arrow:{}px;{fade}{extra}",
                            c.y, c.x, c.arrow_offset
                        ),
                        None => format!(
                            "position:fixed;top:0;left:0;visibility:hidden;pointer-events:none;z-index:{z};{fade}{extra}"
                        ),
                    }
                }
            >
                {move || tooltip_body.with_value(|body| body.run())}
            </div>
        }
    }));
    view! {
        <div
            data-qa="dt-tooltip-container"
            class="d-tooltip-container"
            style=flatten_style
        >
            <span
                node_ref=anchor_ref
                class=(!anchor_class.is_empty()).then_some(anchor_class)
                data-qa="dt-tooltip-anchor"
                style=flatten_style
                on:mouseenter=move |_| {
                    if !flatten {
                        on_enter_anchor("mouseenter");
                    }
                }
                on:mouseleave=move |_| {
                    if !flatten {
                        on_leave_anchor();
                    }
                }
                on:mouseover=move |event: MouseEvent| {
                    if !flatten {
                        return;
                    }
                    let Some(anchor) = anchor_ref.get_untracked() else {
                        return;
                    };
                    let anchor: HtmlElement = anchor.unchecked_into();
                    if pointer_stays_inside(&event, &anchor) {
                        return;
                    }
                    on_enter_anchor("mouseenter");
                }
                on:mouseout=move |event: MouseEvent| {
                    if !flatten {
                        return;
                    }
                    let Some(anchor) = anchor_ref.get_untracked() else {
                        return;
                    };
                    let anchor: HtmlElement = anchor.unchecked_into();
                    if pointer_stays_inside(&event, &anchor) {
                        return;
                    }
                    on_leave_anchor();
                }
                on:focusin=move |_| on_enter_anchor("focusin")
                on:focusout=move |_| on_leave_anchor()
                on:keydown=move |ev: KeyboardEvent| {
                    if ev.code() == "Escape" || ev.key() == "Escape" {
                        on_leave_anchor();
                    }
                }
            >
                {children()}
            </span>
        </div>
        <Show when=move || internal_show.get()>
            {move || {
                let bubble = bubble.get_value();
                if append_inline {
                    bubble.run().into_any()
                } else {
                    view! { <Portal>{bubble.run()}</Portal> }.into_any()
                }
            }}
        </Show>
    }
}

#[cfg(test)]
mod tests {
    use super::tooltip_class;

    #[test]
    fn tooltip_class_keeps_the_arrow_until_show_arrow_is_false() {
        assert_eq!(
            tooltip_class("d-tooltip__arrow-tippy--top", "extra", true, true, true),
            "d-tooltip d-tooltip__arrow-tippy--top extra d-tooltip--inverted d-tooltip--show"
        );
        assert_eq!(
            tooltip_class("d-tooltip__arrow-tippy--top", "", false, false, false),
            "d-tooltip d-tooltip--hide"
        );
    }
}
