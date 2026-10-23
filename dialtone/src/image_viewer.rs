use std::time::Duration;

use leptos::html;
use leptos::leptos_dom::helpers::set_timeout;
use leptos::portal::Portal;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, KeyboardEvent, MouseEvent, Node};

use super::button::DtButton;
use super::icon::DtIconClose;

/// Close control label. Dialtone `DIALTONE_CLOSE_BUTTON` in en-US.
const CLOSE_LABEL: &str = "Click to close";

/// Visible close icon size. Vue sets `size="400"` on the icon and the button.
const CLOSE_ICON_SIZE: &str = "400";

const FOCUSABLE: &str = "button,[href],input,select,textarea,details,[tabindex]";

/// `appendTo` other than `body` is an explicit CSS selector.
/// An empty value uses the `body` default.
pub(crate) fn uses_explicit_append_to(append_to: &str) -> bool {
    let append_to = append_to.trim();
    !append_to.is_empty() && append_to != "body"
}

fn document() -> Option<web_sys::Document> {
    web_sys::window().and_then(|window| window.document())
}

fn resolve_append_target(append_to: &str, root: Option<Element>) -> Option<Element> {
    let Some(document) = document() else {
        return None;
    };
    if uses_explicit_append_to(append_to) {
        let selector = append_to.trim();
        match document.query_selector(selector) {
            Ok(Some(element)) => return Some(element),
            Ok(None) => {
                leptos::logging::warn!(
                    "DtImageViewer append_to {selector:?} matched nothing. Using body."
                );
            }
            Err(_) => {
                leptos::logging::warn!(
                    "DtImageViewer append_to {selector:?} is not a CSS selector. Using body."
                );
            }
        }
    } else if let Some(root) = root {
        if let Ok(Some(dialog)) = root.closest("dialog") {
            return Some(dialog);
        }
    }
    document.body().map(|body| body.unchecked_into())
}

fn active_html_element() -> Option<HtmlElement> {
    document()
        .and_then(|document| document.active_element())
        .and_then(|element| element.dyn_into().ok())
}

fn clicked_backdrop(event: &MouseEvent) -> bool {
    let Some(target) = event.target() else {
        return false;
    };
    let Some(current) = event.current_target() else {
        return false;
    };
    let Some(target) = target.dyn_ref::<Node>() else {
        return false;
    };
    let Some(current) = current.dyn_ref::<Node>() else {
        return false;
    };
    target.is_same_node(Some(current))
}

fn is_escape(event: &KeyboardEvent) -> bool {
    matches!(event.code().as_str(), "Escape" | "Esc")
}

fn focusable_elements(root: &Element) -> Vec<Element> {
    let Ok(list) = root.query_selector_all(FOCUSABLE) else {
        return Vec::new();
    };
    let mut elements = Vec::new();
    for index in 0..list.length() {
        let Some(node) = list.item(index) else {
            continue;
        };
        let Ok(element) = node.dyn_into::<Element>() else {
            continue;
        };
        if element.has_attribute("disabled") {
            continue;
        }
        if element.get_attribute("aria-disabled").as_deref() == Some("true") {
            continue;
        }
        if element.get_attribute("tabindex").as_deref() == Some("-1") {
            continue;
        }
        elements.push(element);
    }
    elements
}

fn focus_element(element: &Element) {
    if let Some(html) = element.dyn_ref::<HtmlElement>() {
        let _ = html.focus();
    }
}

/// Tab cycle from Dialtone `v-dt-focustrap` (`initialFocus: false`).
fn trap_tab(event: &KeyboardEvent, root: &Element) {
    if event.key() != "Tab" {
        return;
    }
    let elements = focusable_elements(root);
    if elements.is_empty() {
        event.prevent_default();
        return;
    }
    let first = &elements[0];
    let last = elements.last().unwrap_or(first);
    let active = document().and_then(|document| document.active_element());
    let on_edge = active.as_ref().is_some_and(|current| {
        if event.shift_key() {
            current.is_same_node(Some(first))
        } else {
            current.is_same_node(Some(last))
        }
    });
    let outside = active
        .as_ref()
        .is_none_or(|current| !root.contains(Some(current)));
    if on_edge || outside {
        focus_element(if event.shift_key() { last } else { first });
        event.prevent_default();
    }
}

fn focus_close_button(modal: NodeRef<html::Div>) {
    set_timeout(
        move || {
            request_animation_frame(move || {
                let Some(root) = modal.get_untracked() else {
                    return;
                };
                let Ok(Some(button)) = root.query_selector("[data-qa='dt-image-viewer-close-btn']")
                else {
                    return;
                };
                focus_element(&button);
            });
        },
        Duration::ZERO,
    );
}

/// Dialtone Vue `DtImageViewer` (`dialtone/v10.7.0`).
///
/// The preview is `DtButton` importance `clear`, `data-qa="dt-image-viewer-preview"`,
/// class `d-image-viewer__preview-button`. A click opens the viewer when `open`
/// is unset (`null`). A set `open` is controlled (`v-model:open`): the click
/// does nothing, and closing emits `on_open` (`update:open`) with `false`.
/// `on_opened` is `opened`.
///
/// The overlay is `div.d-modal` (`data-qa="dt-modal"`, `aria-hidden="false"`)
/// portaled to `append_to`. `body` uses the nearest ancestor `dialog`, then
/// `document.body`. Any other value is a CSS selector. An `HTMLElement`
/// `appendTo` is not ported. Leptos `Portal` wraps the overlay in a `div`.
///
/// Escape and a click on the overlay close it. Tab stays inside the overlay.
/// The visible close button is `DtButton` circle, size `400`, importance
/// `clear`, kind `inverted`, `data-qa="dt-image-viewer-close-btn"`. The label
/// is "Click to close". Pointer leave and focus out swap it for the visually
/// hidden close button. The Vue `fade` on that swap is not animated. `v-dt-mode`
/// is not ported.
#[component]
pub fn DtImageViewer(
    /// Image URL (`imageSrc`).
    #[prop(into)]
    image_src: String,
    /// Image alt text (`imageAlt`).
    #[prop(into)]
    image_alt: String,
    /// Accessible name of the preview button (`ariaLabel`).
    #[prop(into)]
    aria_label: String,
    /// Class on the preview image (`imageButtonClass`).
    #[prop(optional, into)]
    image_button_class: String,
    /// Portal target (`appendTo`). Empty and `body` use the dialog-or-body rule.
    #[prop(optional, into)]
    append_to: String,
    /// Controlled visibility (`open`). Unset means a click opens the viewer.
    #[prop(optional, into)]
    open: MaybeProp<bool>,
    /// `opened`. `true` when a click opens the viewer, `false` when it closes.
    #[prop(optional, into)]
    on_opened: Option<Callback<bool>>,
    /// `update:open`. Fired with `false` when a controlled viewer closes.
    #[prop(optional, into)]
    on_open: Option<Callback<bool>>,
    /// Extra class on the root (`$attrs.class`).
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    if image_src.is_empty() || image_alt.is_empty() || aria_label.is_empty() {
        leptos::logging::warn!("DtImageViewer requires image_src, image_alt, and aria_label.");
    }

    let initial = open.get_untracked();
    let is_open = RwSignal::new(initial.unwrap_or(false));
    let controlled = StoredValue::new(initial.is_some());
    let show_close = RwSignal::new(true);
    let previous = StoredValue::new(None::<HtmlElement>);
    let restore = StoredValue::new(false);
    let root_ref = NodeRef::<html::Div>::new();
    let modal_ref = NodeRef::<html::Div>::new();
    let append_to = StoredValue::new(append_to);
    let on_opened = StoredValue::new(on_opened);
    let on_open = StoredValue::new(on_open);
    let mount_target = RwSignal::new(None::<Element>);

    Effect::new(move |_| {
        if let Some(value) = open.get() {
            controlled.set_value(true);
            is_open.set(value);
        } else {
            controlled.set_value(false);
        }
    });

    Effect::new(move |_| {
        let _ = is_open.get();
        let selector = append_to.get_value();
        let root = root_ref.get().map(Element::from);
        mount_target.set(resolve_append_target(&selector, root));
    });

    Effect::new(move |_| {
        let showing = is_open.get();
        if showing {
            previous.set_value(active_html_element());
            restore.set_value(true);
            return;
        }
        if !restore.get_value() {
            return;
        }
        restore.set_value(false);
        if let Some(element) = previous.get_value() {
            let _ = element.focus();
        }
        previous.set_value(None);
    });

    let close = move || {
        is_open.set(false);
        if let Some(on_opened) = on_opened.get_value() {
            on_opened.run(false);
        }
        if controlled.get_value() {
            if let Some(on_open) = on_open.get_value() {
                on_open.run(false);
            }
        }
    };
    let open_from_click = move || {
        if controlled.get_value() {
            return;
        }
        is_open.set(true);
        show_close.set(true);
        if let Some(on_opened) = on_opened.get_value() {
            on_opened.run(true);
        }
        focus_close_button(modal_ref);
    };

    let image_src = RwSignal::new(image_src);
    let image_alt = RwSignal::new(image_alt);
    let aria_label = RwSignal::new(aria_label);
    let image_button_class = RwSignal::new(image_button_class);
    let class_name = RwSignal::new(class);

    view! {
        <div node_ref=root_ref class=move || class_name.get()>
            <DtButton
                data_qa="dt-image-viewer-preview"
                class="d-image-viewer__preview-button"
                importance="clear"
                aria_label=aria_label.get()
                on_click=move || open_from_click()
            >
                <img
                    src=image_src
                    alt=image_alt
                    class=move || {
                        let name = image_button_class.get();
                        (!name.is_empty()).then_some(name)
                    }
                />
            </DtButton>
            <Show when=move || is_open.get()>
                {move || {
                    let Some(mount) = mount_target.get() else {
                        return ().into_any();
                    };
                    let dismiss = close;
                    view! {
                        <Portal mount=mount>
                            <div
                                class="d-modal"
                                data-qa="dt-modal"
                                aria-hidden="false"
                                node_ref=modal_ref
                                on:click=move |event: MouseEvent| {
                                    if clicked_backdrop(&event) {
                                        dismiss();
                                    }
                                }
                                on:keydown=move |event: KeyboardEvent| {
                                    if is_escape(&event) {
                                        dismiss();
                                        return;
                                    }
                                    if let Some(root) = modal_ref.get() {
                                        trap_tab(&event, &root);
                                    }
                                }
                                on:mouseover=move |_| show_close.set(true)
                                on:mouseleave=move |_| show_close.set(false)
                                on:focusin=move |_| show_close.set(true)
                                on:focusout=move |_| show_close.set(false)
                            >
                                <div
                                    data-qa="dt-image-viewer-full"
                                    class="d-image-viewer__full"
                                    role="dialog"
                                    aria-modal="true"
                                >
                                    <img class="d-image-viewer__full__image" src=image_src alt=image_alt />
                                </div>
                                {move || {
                                    if show_close.get() {
                                        view! {
                                            <DtButton
                                                data_qa="dt-image-viewer-close-btn"
                                                class="d-modal__close"
                                                circle=true
                                                size="400"
                                                importance="clear"
                                                kind="inverted"
                                                aria_label=CLOSE_LABEL
                                                title=CLOSE_LABEL
                                                icon=ViewFn::from(|| view! {
                                                    <DtIconClose
                                                        class="d-image-viewer__close-button"
                                                        size=CLOSE_ICON_SIZE
                                                    />
                                                })
                                                on_click=move || dismiss()
                                            />
                                        }
                                        .into_any()
                                    } else {
                                        view! {
                                            <DtButton
                                                id="sr-only-close-button"
                                                class="d-vi-visible-sr"
                                                data_qa="dt-sr-only-close-button"
                                                aria_label=CLOSE_LABEL
                                                title=CLOSE_LABEL
                                                on_click=move || dismiss()
                                            >
                                                <DtIconClose />
                                            </DtButton>
                                        }
                                        .into_any()
                                    }
                                }}
                            </div>
                        </Portal>
                    }
                    .into_any()
                }}
            </Show>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn body_is_not_an_explicit_append_target() {
        assert!(!uses_explicit_append_to(""));
        assert!(!uses_explicit_append_to("  "));
        assert!(!uses_explicit_append_to("body"));
        assert!(!uses_explicit_append_to(" body "));
    }

    #[test]
    fn a_selector_is_an_explicit_append_target() {
        assert!(uses_explicit_append_to("#viewer"));
        assert!(uses_explicit_append_to("dialog"));
        assert!(uses_explicit_append_to(" #custom "));
    }
}
