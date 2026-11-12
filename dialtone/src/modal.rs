use std::time::Duration;

use leptos::html;
use leptos::leptos_dom::helpers::{request_animation_frame, set_timeout};
use leptos::portal::Portal;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{
    Element, Event, FocusOptions, HtmlDialogElement, HtmlElement, HtmlInputElement, KeyboardEvent,
    MouseEvent, Node,
};

use super::button::{button_icon_size, DtButton};
use super::icon::DtIconClose;
use super::text::{text_classes, TextStyle};
use super::unique_id;

/// Close control label. Dialtone `DIALTONE_CLOSE_BUTTON` in en-US.
const CLOSE_LABEL: &str = "Click to close";

const FOCUSABLE: &str = concat!(
    "button:not(:disabled),[href],input:not(:disabled),select:not(:disabled),",
    "textarea:not(:disabled),details,[tabindex]:not([tabindex=\"-1\"]):not(:disabled):not([aria-disabled=\"true\"])"
);

const MISSING_FOCUS: &str = "Could not find the element specified in dt-modal prop \"initialFocusElement\". Defaulting to focusing the first element.";

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn modal_kind_class(kind: &str) -> &'static str {
    match kind {
        "default" => "",
        "critical" => "d-modal--critical",
        _ => {
            leptos::logging::warn!("DtModal kind `{kind}` is not default or critical");
            ""
        }
    }
}

fn banner_kind_class(kind: &str) -> &'static str {
    match kind {
        "critical" => "d-modal__banner--critical",
        "info" => "d-modal__banner--info",
        "positive" => "d-modal__banner--positive",
        "warning" => "d-modal__banner--warning",
        "base" => "d-modal__banner--general",
        _ => {
            leptos::logging::warn!(
                "DtModal banner_kind `{kind}` is not base, critical, info, positive, or warning"
            );
            ""
        }
    }
}

fn modal_root_class(
    kind: &str,
    fullscreen: bool,
    transparent_backdrop: bool,
    extra: &str,
) -> String {
    join_class([
        "d-modal",
        modal_kind_class(kind),
        if fullscreen { "d-modal--full" } else { "" },
        if transparent_backdrop {
            "d-modal--transparent-backdrop"
        } else {
            ""
        },
        extra,
    ])
}

fn modal_dialog_class(fixed_header_footer: bool, extra: &str) -> String {
    join_class([
        "d-modal__dialog",
        if fixed_header_footer {
            "d-modal__dialog--scrollable"
        } else {
            ""
        },
        extra,
    ])
}

fn modal_banner_class(kind: &str, extra: &str) -> String {
    join_class(["d-modal__banner", extra, banner_kind_class(kind)])
}

fn modal_part_class(base: &str, extra: &str) -> String {
    join_class([base, extra])
}

fn modal_header_class(extra: &str) -> String {
    let text = text_classes(&TextStyle {
        kind: "headline",
        variant: "",
        size: "600",
        family: "",
        italic: false,
        tone: "",
        align: "",
        truncate: false,
        max_lines: 0,
        numeric: false,
        wrap: "",
        text_box_trim: "start",
        strength: "medium",
        density: "100",
    });
    join_class([text.as_str(), "d-modal__header", extra])
}

fn document() -> Option<web_sys::Document> {
    web_sys::window().and_then(|window| window.document())
}

fn document_element() -> Option<Element> {
    document().and_then(|document| document.document_element())
}

fn read_root_attr(name: &str, fallback: &str) -> String {
    document_element()
        .and_then(|element| element.get_attribute(name))
        .unwrap_or_else(|| fallback.to_string())
}

fn is_document_element(element: &Element) -> bool {
    document_element().is_some_and(|root| element.is_same_node(Some(root.unchecked_ref())))
}

fn find_parent_mode(element: &Element) -> String {
    let mut parent = element.parent_element();
    while let Some(node) = parent {
        if is_document_element(&node) {
            break;
        }
        if node.has_attribute("data-dt-mode") {
            return node.get_attribute("data-dt-mode").unwrap_or_default();
        }
        parent = node.parent_element();
    }
    read_root_attr("data-dt-mode", "light")
}

fn opposite_mode(current: &str) -> &'static str {
    if current == "light" {
        "dark"
    } else {
        "light"
    }
}

fn attribute_options(name: &str) -> web_sys::MutationObserverInit {
    let options = web_sys::MutationObserverInit::new();
    options.set_attributes(true);
    let filter = js_sys::Array::of1(&wasm_bindgen::JsValue::from_str(name));
    options.set_attribute_filter(&filter);
    options
}

fn watch_root_contrast(slot: RwSignal<Option<String>>) -> Option<web_sys::MutationObserver> {
    let root = document_element()?;
    let closure = Closure::wrap(Box::new(
        move |_records: js_sys::Array, _observer: web_sys::MutationObserver| {
            slot.set(Some(read_root_attr("data-dt-contrast", "default")));
        },
    )
        as Box<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>);
    let Ok(observer) = web_sys::MutationObserver::new(closure.as_ref().unchecked_ref()) else {
        return None;
    };
    if observer
        .observe_with_options(&root, &attribute_options("data-dt-contrast"))
        .is_err()
    {
        observer.disconnect();
        return None;
    }
    closure.forget();
    Some(observer)
}

fn watch_ancestor_mode(
    element: Element,
    mode_attr: RwSignal<Option<String>>,
) -> Option<web_sys::MutationObserver> {
    let watched = element.clone();
    let closure = Closure::wrap(Box::new(
        move |_records: js_sys::Array, _observer: web_sys::MutationObserver| {
            let resolved = opposite_mode(&find_parent_mode(&watched));
            mode_attr.set(Some(resolved.to_string()));
        },
    )
        as Box<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>);
    let Ok(observer) = web_sys::MutationObserver::new(closure.as_ref().unchecked_ref()) else {
        return None;
    };
    let options = attribute_options("data-dt-mode");
    let Some(root) = document_element() else {
        return None;
    };
    if observer.observe_with_options(&root, &options).is_err() {
        observer.disconnect();
        return None;
    }
    let mut parent = element.parent_element();
    while let Some(node) = parent {
        if is_document_element(&node) {
            break;
        }
        let _ = observer.observe_with_options(&node, &options);
        parent = node.parent_element();
    }
    closure.forget();
    Some(observer)
}

fn mount_content_mode(
    dialog: &HtmlDialogElement,
    content_mode: &'static str,
    mode_attr: RwSignal<Option<String>>,
    contrast_attr: RwSignal<Option<String>>,
) {
    let element: Element = dialog.clone().unchecked_into();
    if content_mode == "invert" {
        mode_attr.set(Some(opposite_mode(&find_parent_mode(&element)).to_string()));
    }
    let mut observers = Vec::new();
    if let Some(observer) = watch_root_contrast(contrast_attr) {
        observers.push(observer);
    }
    if content_mode == "invert" {
        if let Some(observer) = watch_ancestor_mode(element, mode_attr) {
            observers.push(observer);
        }
    }
    on_cleanup(move || {
        for observer in observers {
            observer.disconnect();
        }
    });
}

fn resolve_append_to(append_to: &str) -> Option<Element> {
    let selector = append_to.trim();
    if selector.is_empty() {
        return None;
    }
    let Some(document) = document() else {
        return None;
    };
    match document.query_selector(selector) {
        Ok(Some(element)) => Some(element),
        Ok(None) => {
            leptos::logging::warn!(
                "DtModal append_to {selector:?} matched nothing. Rendering in place."
            );
            None
        }
        Err(_) => {
            leptos::logging::warn!(
                "DtModal append_to {selector:?} is not a CSS selector. Rendering in place."
            );
            None
        }
    }
}

fn shadow_host(dialog: &Element) -> Option<Element> {
    let root = dialog.get_root_node();
    let host = js_sys::Reflect::get(&root, &wasm_bindgen::JsValue::from_str("host")).ok()?;
    host.dyn_into::<Element>().ok()
}

fn scroll_root(dialog: &Element) -> Option<Element> {
    shadow_host(dialog).or_else(document_element)
}

fn lock_scroll(dialog: &Element, locked: bool) {
    let Some(root) = scroll_root(dialog) else {
        return;
    };
    let class_name = if root.has_attribute("data-overlayscrollbars") {
        "d-scrollbar-disabled"
    } else {
        "d-of-hidden"
    };
    if locked {
        let _ = root.class_list().add_1(class_name);
    } else {
        let _ = root.class_list().remove_1(class_name);
    }
}

fn active_html_element() -> Option<HtmlElement> {
    document()
        .and_then(|document| document.active_element())
        .and_then(|element| element.dyn_into().ok())
}

fn same_target(event: &MouseEvent) -> bool {
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
    event.key() == "Escape"
}

fn is_space(event: &KeyboardEvent) -> bool {
    event.key() == " " || event.code() == "Space"
}

fn space_on_sr_only_close(event: &KeyboardEvent) -> bool {
    if !is_space(event) {
        return false;
    }
    let Some(target) = event.target() else {
        return false;
    };
    let Ok(element) = target.dyn_into::<Element>() else {
        return false;
    };
    element
        .closest("#sr-only-close-button")
        .ok()
        .flatten()
        .is_some()
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
        elements.push(element);
    }
    elements
}

fn preferred_radio(elements: &[Element]) -> Element {
    let first = &elements[0];
    let Some(input) = first.dyn_ref::<HtmlInputElement>() else {
        return first.clone();
    };
    if input.type_() != "radio" || input.checked() {
        return first.clone();
    }
    let name = input.name();
    for candidate in elements {
        let Some(other) = candidate.dyn_ref::<HtmlInputElement>() else {
            continue;
        };
        if other.checked() && other.name() == name {
            return candidate.clone();
        }
    }
    first.clone()
}

fn focus_element(element: &Element, prevent_scroll: bool) {
    let Some(html) = element.dyn_ref::<HtmlElement>() else {
        return;
    };
    if prevent_scroll {
        let options = FocusOptions::new();
        options.set_prevent_scroll(true);
        let _ = html.focus_with_options(&options);
    } else {
        let _ = html.focus();
    }
}

fn focus_first_tabbable(dialog: &HtmlDialogElement) {
    let elements = focusable_elements(dialog);
    if elements.is_empty() {
        return;
    }
    let target = preferred_radio(&elements);
    focus_element(&target, true);
}

fn focus_after_open(dialog: &HtmlDialogElement, initial: &str) {
    if initial == "first" {
        focus_first_tabbable(dialog);
        return;
    }
    if initial.starts_with('#') {
        match dialog.query_selector(initial) {
            Ok(Some(element)) => focus_element(&element, false),
            _ => {
                leptos::logging::warn!("{MISSING_FOCUS}");
                focus_first_tabbable(dialog);
            }
        }
        return;
    }
    leptos::logging::warn!(
        "DtModal initial_focus_element `{initial}` is not \"first\" or an id selector. Focusing the first element."
    );
    focus_first_tabbable(dialog);
}

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
        focus_element(if event.shift_key() { last } else { first }, true);
        event.prevent_default();
    }
}

fn present_dialog(dialog: &HtmlDialogElement, modal: bool) {
    if !dialog.is_connected() || dialog.open() {
        return;
    }
    if modal {
        if let Err(error) = dialog.show_modal() {
            leptos::logging::warn!("DtModal showModal failed: {error:?}");
        }
    } else {
        dialog.show();
    }
}

fn bump(generation: RwSignal<u32>) -> u32 {
    generation.update(|value| *value = value.wrapping_add(1));
    generation.get_untracked()
}

/// Vue's `d-modal__dialog-enter-*` classes are not in the vendored CSS, so the
/// wait is one frame rather than the `.d-modal--animate-*` durations.
fn after_frame(job: impl FnOnce() + 'static) {
    if web_sys::window().is_none() {
        return;
    }
    set_timeout(move || request_animation_frame(job), Duration::ZERO);
}

/// Dialtone Vue `DtModal` (`dialtone/v10.7.0`).
///
/// The root is `<dialog class="d-modal" data-qa="dt-modal">`. It stays mounted.
/// `open` is controlled and defaults to false. `on_open` is `update:open`.
/// This component does not write that signal. `modal` false (default) calls
/// `show()` and omits `aria-modal`. `modal` true calls `showModal()` and sets
/// `aria-modal="true"`. A repeat `true` does not call `show` or `showModal`
/// again. Close emits `false` immediately. After the open frame this port
/// emits `true` and focuses. After the close frame it calls `dialog.close()`
/// and restores the previously active element. Vue's
/// `<transition name="d-modal__dialog">` classes are not in the vendored CSS.
/// `.d-modal--animate*` is, and this template does not add it.
///
/// `kind` is `default` or `critical`. `fullscreen` adds `d-modal--full`.
/// `transparent_backdrop` (`transparentBackdrop`) adds
/// `d-modal--transparent-backdrop`. The dialog width is the CSS
/// `max-inline-size`. `dialog_class` (`dialogClass`) is extra on
/// `d-modal__dialog`. `fixed_header_footer` (`fixedHeaderFooter`) defaults to
/// true and adds `d-modal__dialog--scrollable`. The banner renders only when
/// `open` and the `banner` slot or `banner_header_text` (`bannerHeaderText`)
/// is set. Banner kinds are `critical`, `info`, `positive`, `warning`
/// (default), and `base` (`d-modal__banner--general`). An unknown kind warns
/// and omits the modifier. The header slot replaces `header_text`
/// (`headerText`). The text header is an `h2` with the headline `600` classes.
/// `labelled_by_id` (`labelledById`) is generated when empty.
/// `described_by_id` (`describedById`) sets `aria-describedby` only when set.
/// Children replace `copy`. An empty `copy` still renders the paragraph.
///
/// `show_close` (`showClose`) defaults to true. The visible close button is
/// size `300`, kind `muted`, importance `clear`, label "Click to close".
/// `show_close` false uses the visually hidden button. Space on that button
/// closes and does not emit `keydown`. `close_on_click` (`closeOnClick`)
/// defaults to true and closes only when press and release are both on the
/// dialog. Escape closes when `modal` is false and the event is not already
/// cancelled. Native `cancel` is prevented and closes. Tab stays inside.
/// `initial_focus_element` (`initialFocusElement`) is `first` or `#id`.
/// `dialog` is not implemented. An `HTMLElement` value is not ported.
///
/// `append_to` (`appendTo`) empty renders in place. A CSS selector that
/// matches uses `Portal`, which wraps the dialog in a `div`. A miss warns and
/// stays in place. An `HTMLElement` target is not ported. Vue also reparents
/// into a shadow root. This port stays where it was mounted. Scroll lock is
/// `d-of-hidden`, or `d-scrollbar-disabled` when the root has
/// `data-overlayscrollbars`. `content_mode` (`contentMode`) is `light`,
/// `dark`, or `invert`. `invert` uses the dialog's ancestor `data-dt-mode`,
/// not the document-root invert `DtPopover` uses. `v-dt-mode` is not ported.
#[component]
pub fn DtModal(
    /// Body text (`copy`). Used when the default slot is absent.
    #[prop(optional, into)]
    copy: String,
    /// `aria-describedby` (`describedById`). Empty omits it.
    #[prop(optional, into)]
    described_by_id: String,
    /// `aria-labelledby` (`labelledById`). Empty generates an id.
    #[prop(optional, into)]
    labelled_by_id: String,
    /// Whether the dialog is open (`open`). Unset stays closed.
    #[prop(optional, into)]
    open: Option<Signal<bool>>,
    /// Header text (`headerText`).
    #[prop(optional, into)]
    header_text: String,
    /// Banner text (`bannerHeaderText`).
    #[prop(optional, into)]
    banner_header_text: String,
    /// `default` or `critical` (`kind`).
    #[prop(optional, default = "default")]
    kind: &'static str,
    /// Fill the viewport (`fullscreen`).
    #[prop(optional)]
    fullscreen: bool,
    /// Extra class on the dialog element (`modalClass`).
    #[prop(optional, into)]
    modal_class: String,
    /// Extra class on `.d-modal__dialog` (`dialogClass`).
    #[prop(optional, into)]
    dialog_class: String,
    /// Extra class on the body (`contentClass`).
    #[prop(optional, into)]
    content_class: String,
    /// Banner color (`bannerKind`). Default `warning`.
    #[prop(optional, default = "warning")]
    banner_kind: &'static str,
    /// Extra class on the banner (`bannerClass`).
    #[prop(optional, into)]
    banner_class: String,
    /// Extra class on the header (`headerClass`).
    #[prop(optional, into)]
    header_class: String,
    /// Extra class on the footer (`footerClass`).
    #[prop(optional, into)]
    footer_class: String,
    /// Show the visible close button (`showClose`). Default true.
    #[prop(optional, default = true)]
    show_close: bool,
    /// Close when the backdrop press and release are both outside (`closeOnClick`).
    #[prop(optional, default = true)]
    close_on_click: bool,
    /// Keep the header and footer fixed (`fixedHeaderFooter`). Default true.
    #[prop(optional, default = true)]
    fixed_header_footer: bool,
    /// `first` or an `#id` inside the dialog (`initialFocusElement`).
    #[prop(optional, default = "first")]
    initial_focus_element: &'static str,
    /// CSS selector for `Portal` (`appendTo`). Empty renders in place.
    #[prop(optional, into)]
    append_to: String,
    /// Transparent backdrop (`transparentBackdrop`).
    #[prop(optional)]
    transparent_backdrop: bool,
    /// `showModal()` and `aria-modal` (`modal`). Default false.
    #[prop(optional)]
    modal: bool,
    /// `light`, `dark`, or `invert` (`contentMode`). Empty omits the attributes.
    #[prop(optional, default = "")]
    content_mode: &'static str,
    /// `click`.
    #[prop(optional, into)]
    on_click: Option<Callback<MouseEvent>>,
    /// `keydown`.
    #[prop(optional, into)]
    on_keydown: Option<Callback<KeyboardEvent>>,
    /// `update:open`.
    #[prop(optional, into)]
    on_open: Option<Callback<bool>>,
    /// Header slot (`header`). Replaces `header_text`.
    #[prop(optional)]
    header: Option<ViewFn>,
    /// Banner slot (`banner`). Replaces `banner_header_text`.
    #[prop(optional)]
    banner: Option<ViewFn>,
    /// Footer slot (`footer`). Omitted when unset.
    #[prop(optional)]
    footer: Option<ViewFn>,
    /// Body slot. Replaces `copy`.
    #[prop(optional)]
    children: Option<ChildrenFn>,
) -> impl IntoView {
    if !matches!(content_mode, "" | "light" | "dark" | "invert") {
        leptos::logging::warn!(
            "DtModal content_mode `{content_mode}` is not light, dark, or invert"
        );
    }
    let labelled_by_id = if labelled_by_id.trim().is_empty() {
        unique_id("dt-modal")
    } else {
        labelled_by_id
    };
    let described_by = (!described_by_id.trim().is_empty()).then_some(described_by_id);
    let root_class = modal_root_class(kind, fullscreen, transparent_backdrop, &modal_class);
    let panel_class = modal_dialog_class(fixed_header_footer, &dialog_class);
    let banner_class_name = modal_banner_class(banner_kind, &banner_class);
    let header_text_class = modal_header_class(&header_class);
    let header_slot_class = modal_part_class("d-modal__header", &header_class);
    let content_class_name = modal_part_class("d-modal__content", &content_class);
    let footer_class_name = modal_part_class("d-modal__footer", &footer_class);
    let mode_attr = RwSignal::new(match content_mode {
        "light" | "dark" => Some(content_mode.to_string()),
        _ => None,
    });
    let contrast_attr = RwSignal::new(match content_mode {
        "light" | "dark" | "invert" => Some(read_root_attr("data-dt-contrast", "default")),
        _ => None,
    });
    let dialog_ref = NodeRef::<html::Dialog>::new();
    let ready = RwSignal::new(false);
    let generation = RwSignal::new(0u32);
    let alive = StoredValue::new(true);
    let previous = StoredValue::new(None::<HtmlElement>);
    let down_on_backdrop = StoredValue::new(false);
    let up_on_backdrop = StoredValue::new(false);
    let request_close = Callback::new(move |_: ()| {
        if let Some(on_open) = on_open {
            on_open.run(false);
        }
    });
    let mount = resolve_append_to(&append_to);
    let watches_mode = matches!(content_mode, "light" | "dark" | "invert");

    dialog_ref.on_load(move |dialog| {
        ready.set(true);
        if watches_mode {
            mount_content_mode(&dialog, content_mode, mode_attr, contrast_attr);
        }
    });

    Effect::new(move |_| {
        if !ready.get() {
            return;
        }
        let showing = open.is_some_and(|signal| signal.get());
        let Some(dialog) = dialog_ref.get_untracked() else {
            return;
        };
        if showing {
            if dialog.open() {
                return;
            }
            previous.set_value(active_html_element());
            present_dialog(&dialog, modal);
            lock_scroll(&dialog, true);
            let ticket = bump(generation);
            let dialog_later = dialog.clone();
            after_frame(move || {
                if !alive.get_value() || generation.get_untracked() != ticket {
                    return;
                }
                if let Some(on_open) = on_open {
                    on_open.run(true);
                }
                if !alive.get_value() || generation.get_untracked() != ticket {
                    return;
                }
                focus_after_open(&dialog_later, initial_focus_element);
            });
        } else if dialog.open() {
            lock_scroll(&dialog, false);
            let ticket = bump(generation);
            let dialog_later = dialog.clone();
            after_frame(move || {
                if !alive.get_value() || generation.get_untracked() != ticket {
                    return;
                }
                if dialog_later.open() {
                    dialog_later.close();
                }
                let restore = previous.try_update_value(|slot| slot.take()).flatten();
                if let Some(element) = restore {
                    let _ = element.focus();
                }
            });
        }
    });

    on_cleanup(move || {
        alive.set_value(false);
        generation.update(|value| *value = value.wrapping_add(1));
        if let Some(dialog) = dialog_ref.get_untracked() {
            if dialog.open() {
                dialog.close();
                lock_scroll(&dialog, false);
            }
        }
        previous.set_value(None);
    });

    let labelled_for_aria = labelled_by_id.clone();
    let labelled_for_header = labelled_by_id;
    let close_icon_size = button_icon_size("300");

    // Portal children are `Fn + Sync`. A built `dialog` view is neither, so
    // rebuild it from a `ViewFn` and call `run` (see `DtPopover`).
    let dialog = ViewFn::from(move || {
        let banner = banner.clone();
        let banner_header_text = banner_header_text.clone();
        let header = header.clone();
        let children = children.clone();
        let footer = footer.clone();
        let copy = copy.clone();
        let header_text = header_text.clone();
        let labelled_for_header = labelled_for_header.clone();
        let labelled_for_aria = labelled_for_aria.clone();
        let described_by = described_by.clone();
        let root_class = root_class.clone();
        let panel_class = panel_class.clone();
        let banner_class_name = banner_class_name.clone();
        let header_slot_class = header_slot_class.clone();
        let header_text_class = header_text_class.clone();
        let content_class_name = content_class_name.clone();
        let footer_class_name = footer_class_name.clone();
        let show_banner = {
            let banner = banner.clone();
            let banner_text = banner_header_text.clone();
            move || {
                let showing = open.is_some_and(|signal| signal.get());
                showing && (banner.is_some() || !banner_text.is_empty())
            }
        };
        view! {
            <dialog
                node_ref=dialog_ref
                class=root_class
                data-qa="dt-modal"
                aria-modal=modal.then_some("true")
                aria-describedby=described_by
                aria-labelledby=labelled_for_aria
                data-dt-mode=move || mode_attr.get()
                data-dt-contrast=move || contrast_attr.get()
                on:cancel=move |event: Event| {
                    event.prevent_default();
                    request_close.run(());
                }
                on:mousedown=move |event| {
                    down_on_backdrop.set_value(same_target(&event));
                }
                on:mouseup=move |event| {
                    up_on_backdrop.set_value(same_target(&event));
                }
                on:click=move |event| {
                    let pressed = down_on_backdrop.get_value() && up_on_backdrop.get_value();
                    down_on_backdrop.set_value(false);
                    up_on_backdrop.set_value(false);
                    if close_on_click && same_target(&event) && pressed {
                        request_close.run(());
                    }
                    if let Some(on_click) = on_click {
                        on_click.run(event);
                    }
                }
                on:keydown=move |event| {
                    if !show_close && space_on_sr_only_close(&event) {
                        event.prevent_default();
                        event.stop_propagation();
                        request_close.run(());
                        return;
                    }
                    if open.is_some_and(|signal| signal.get_untracked()) {
                        if let Some(dialog) = dialog_ref.get_untracked() {
                            trap_tab(&event, &dialog);
                        }
                    }
                    if !modal && is_escape(&event) && !event.default_prevented() {
                        event.prevent_default();
                        request_close.run(());
                    }
                    if let Some(on_keydown) = on_keydown {
                        on_keydown.run(event);
                    }
                }
            >
                <Show when=show_banner>
                    <div class=banner_class_name.clone() data-qa="dt-modal-banner">
                        {banner.clone().as_ref().map(|slot| slot.run().into_any()).unwrap_or_else(|| {
                            let text = banner_header_text.clone();
                            view! { {text} }.into_any()
                        })}
                    </div>
                </Show>
                <div
                    class=panel_class
                    style=move || {
                        let showing = open.is_some_and(|signal| signal.get());
                        (!showing).then_some("display: none")
                    }
                >
                    {if let Some(header) = header {
                        view! {
                            <div id=labelled_for_header.clone() class=header_slot_class data-qa="dt-modal-title">
                                {header.run()}
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <h2 id=labelled_for_header class=header_text_class data-qa="dt-modal-title">
                                {header_text}
                            </h2>
                        }.into_any()
                    }}
                    {if let Some(children) = children {
                        view! {
                            <div class=content_class_name.clone() data-qa="dt-modal-copy">
                                {children()}
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <p class=content_class_name data-qa="dt-modal-copy">{copy}</p>
                        }.into_any()
                    }}
                    {footer.map(|slot| {
                        view! {
                            <footer class=footer_class_name>{slot.run()}</footer>
                        }
                    })}
                    {if show_close {
                        let request_close = request_close;
                        view! {
                            <DtButton
                                class="d-modal__close"
                                data_qa="dt-modal-close-button"
                                size="300"
                                kind="muted"
                                importance="clear"
                                aria_label=CLOSE_LABEL
                                title=CLOSE_LABEL
                                icon=ViewFn::from(move || view! {
                                    <DtIconClose size=close_icon_size />
                                })
                                on_click=move || request_close.run(())
                            />
                        }.into_any()
                    } else {
                        let request_close = request_close;
                        view! {
                            <DtButton
                                id="sr-only-close-button"
                                class="d-vi-visible-sr"
                                data_qa="dt-sr-only-close-button"
                                aria_label=CLOSE_LABEL
                                title=CLOSE_LABEL
                                on_click=move || request_close.run(())
                            >
                                <DtIconClose />
                            </DtButton>
                        }.into_any()
                    }}
                </div>
            </dialog>
        }
    });

    if let Some(mount) = mount {
        let dialog = dialog.clone();
        view! { <Portal mount=mount>{dialog.run()}</Portal> }.into_any()
    } else {
        dialog.run().into_any()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_class_matches_vue_kind_and_flags() {
        assert_eq!(modal_root_class("default", false, false, ""), "d-modal");
        assert_eq!(
            modal_root_class("critical", false, false, ""),
            "d-modal d-modal--critical"
        );
        assert_eq!(
            modal_root_class("default", true, false, ""),
            "d-modal d-modal--full"
        );
        assert_eq!(
            modal_root_class("default", false, true, "extra"),
            "d-modal d-modal--transparent-backdrop extra"
        );
        assert_eq!(modal_root_class("nope", false, false, ""), "d-modal");
    }

    #[test]
    fn dialog_class_scrolls_without_editor_widths() {
        assert_eq!(
            modal_dialog_class(true, ""),
            "d-modal__dialog d-modal__dialog--scrollable"
        );
        assert_eq!(modal_dialog_class(false, "extra"), "d-modal__dialog extra");
        let class_name = modal_dialog_class(true, "");
        assert!(!class_name.contains("d-hmx764"));
        assert!(!class_name.contains("min-w-"));
        assert!(!class_name.contains("max-w-"));
        assert!(!class_name.contains("w-max"));
    }

    #[test]
    fn banner_defaults_to_warning_and_unknown_is_omitted() {
        assert_eq!(
            modal_banner_class("warning", ""),
            "d-modal__banner d-modal__banner--warning"
        );
        assert_eq!(
            modal_banner_class("base", "extra"),
            "d-modal__banner extra d-modal__banner--general"
        );
        assert_eq!(
            modal_banner_class("critical", ""),
            "d-modal__banner d-modal__banner--critical"
        );
        assert_eq!(
            modal_banner_class("info", ""),
            "d-modal__banner d-modal__banner--info"
        );
        assert_eq!(
            modal_banner_class("positive", ""),
            "d-modal__banner d-modal__banner--positive"
        );
        assert_eq!(modal_banner_class("success", ""), "d-modal__banner");
    }

    #[test]
    fn part_class_omits_an_empty_extra() {
        assert_eq!(modal_part_class("d-modal__content", ""), "d-modal__content");
        assert_eq!(
            modal_part_class("d-modal__footer", "extra"),
            "d-modal__footer extra"
        );
    }

    #[test]
    fn header_text_uses_the_headline_classes() {
        assert_eq!(
            modal_header_class(""),
            "d-text d-text-headline--2xl d-text--trim-start d-text--fw-medium d-text--lh-100 d-modal__header"
        );
        assert_eq!(
            modal_header_class("extra"),
            "d-text d-text-headline--2xl d-text--trim-start d-text--fw-medium d-text--lh-100 d-modal__header extra"
        );
    }
}
