use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use leptos::html;
use leptos::portal::Portal;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, KeyboardEvent, MouseEvent};

use super::button::DtButton;
use super::icon::DtIconClose;
use super::list_nav::{event_inside, join_class, popover_padding_class};
use super::popper::{
    compute_position, ComputedPosition, Options as PopperOptions, Placement, Rect,
};
use super::unique_id;

/// Vue `offset: [0, 4]`. The skid stays 0.
const DEFAULT_OFFSET: f64 = 4.0;
const VIEWPORT_PADDING: f64 = 8.0;

fn element_rect(element: &HtmlElement) -> Rect {
    let rect = element.get_bounding_client_rect();
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
            .and_then(|value| value.as_f64())
            .unwrap_or(0.0),
        height: window
            .inner_height()
            .ok()
            .and_then(|value| value.as_f64())
            .unwrap_or(0.0),
    }
}

fn reference_element(anchor: &HtmlElement) -> HtmlElement {
    let mut current: HtmlElement = anchor
        .first_element_child()
        .and_then(|element| element.dyn_into().ok())
        .unwrap_or_else(|| anchor.clone());
    // `display: contents` wrappers have an empty box. Walk to the button.
    for _ in 0..6 {
        let rect = current.get_bounding_client_rect();
        if rect.width() > 0.0 || rect.height() > 0.0 {
            return current;
        }
        match current
            .first_element_child()
            .and_then(|element| element.dyn_into().ok())
        {
            Some(child) => current = child,
            None => return current,
        }
    }
    current
}

fn parse_fallbacks(raw: &str) -> Vec<Placement> {
    raw.split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty() && *part != "auto")
        .map(Placement::parse)
        .collect()
}

/// Zero-size anchor at a viewport point. Context menus use the pointer.
pub(crate) fn pointer_rect(x: f64, y: f64) -> Rect {
    Rect {
        x,
        y,
        width: 0.0,
        height: 0.0,
    }
}

fn popover_motion_style(transition: &str, shown: bool) -> &'static str {
    if transition != "fade" {
        return "";
    }
    if shown {
        "opacity:1;transition:opacity 120ms ease;"
    } else {
        "opacity:0;transition:opacity 120ms ease;"
    }
}

fn trap_tab(event: &KeyboardEvent, root: &HtmlElement) {
    if event.key() != "Tab" {
        return;
    }
    let Ok(nodes) = root.query_selector_all(
        "button, [href], input, select, textarea, [tabindex]:not([tabindex='-1'])",
    ) else {
        return;
    };
    let mut elements = Vec::new();
    for index in 0..nodes.length() {
        let Some(node) = nodes.item(index) else {
            continue;
        };
        let Ok(element) = node.dyn_into::<web_sys::Element>() else {
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
    if elements.is_empty() {
        event.prevent_default();
        return;
    }
    let first = &elements[0];
    let last = elements.last().unwrap_or(first);
    let active = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.active_element());
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
        let target = if event.shift_key() { last } else { first };
        if let Some(element) = target.dyn_ref::<HtmlElement>() {
            let _ = element.focus();
        }
        event.prevent_default();
    }
}

fn label_anchor(anchor: &HtmlElement, expanded: bool, controls: &str, popup: &str) {
    let target = reference_element(anchor);
    let _ = target.set_attribute("aria-expanded", if expanded { "true" } else { "false" });
    if !controls.is_empty() {
        let _ = target.set_attribute("aria-controls", controls);
    }
    if !popup.is_empty() {
        let _ = target.set_attribute("aria-haspopup", popup);
    }
}

fn popover_z_index(anchor: Option<HtmlElement>) -> i32 {
    // A portaled menu defaults to 400. The schedule dialog is not a `.d-modal`,
    // but it is `aria-modal`, and its overlay is 500. Treat that as an overlay
    // so the menu paints above it.
    let in_overlay = anchor
        .and_then(|element| {
            element
                .closest(
                    ".d-modal[aria-hidden=\"false\"], .d-modal:not([aria-hidden]), .d-zi-drawer, [aria-modal=\"true\"]",
                )
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

/// `data-dt-mode` for a popover `contentMode`. `invert` is the opposite of `root_mode`.
pub(crate) fn resolved_content_mode(mode: &str, root_mode: &str) -> Option<&'static str> {
    match mode {
        "" => None,
        "light" => Some("light"),
        "dark" => Some("dark"),
        "invert" => Some(if root_mode == "dark" { "light" } else { "dark" }),
        _ => {
            leptos::logging::warn!("DtPopover content_mode `{mode}` is not light, dark, or invert");
            None
        }
    }
}

fn root_mode() -> String {
    web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.document_element())
        .and_then(|element| element.get_attribute("data-dt-mode"))
        .unwrap_or_else(|| "light".to_string())
}

fn root_contrast() -> String {
    web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.document_element())
        .and_then(|element| element.get_attribute("data-dt-contrast"))
        .unwrap_or_else(|| "default".to_string())
}

fn focus_first(dialog: &HtmlElement) {
    let selector = "button, [href], input, select, textarea, [tabindex]:not([tabindex='-1'])";
    if let Ok(Some(element)) = dialog.query_selector(selector) {
        if let Ok(element) = element.dyn_into::<HtmlElement>() {
            let _ = element.focus();
        }
    }
}

/// `POPOVER_HEADER_FOOTER_PADDING_CLASSES` at `dialtone/v10.7.0`.
fn popover_edge_padding(padding: &str) -> &'static str {
    match padding {
        "small" => "d-pis-50",
        "medium" => "d-pis-100",
        "large" => "d-pis-200",
        _ => "",
    }
}

fn known_popover_role(role: &str) -> bool {
    matches!(role, "dialog" | "menu" | "listbox" | "tree" | "grid")
}

/// Dialtone Vue `DtPopover`. Follows `dialtone/v10.7.0`.
///
/// Positioning uses [`super::popper`], not Tippy. While open, scroll and resize
/// recompute that position. `sticky` true also polls the rects. `open` unset
/// toggles from the anchor click. A set `open` signal is controlled through
/// `on_open_change`. `open_on_context` (`openOnContext`) opens from the pointer
/// and ignores the left click. The anchor wrapper gets `tabindex="0"` only then.
/// `tether` defaults to false here so existing popovers keep shifting into the
/// viewport. Vue's default is true. `DtDropdown` passes true. `initial_focus`
/// stays `none`. Vue's default is `first`, which would focus the visually
/// hidden close button inside a modal `DtDropdown`. `transition` stays empty.
/// Vue's default is `fade`. `toggle_on_click` defaults to true. `data_qa` empty
/// keeps the fragment DOM. Vue always wraps `.d-popover`. Those classes are not
/// in the vendored CSS. `focus_trap` stays independent of `modal`. Vue also
/// traps when `modal` is true. `appendTo` `root` and an `HTMLElement` warn and
/// use `body`. The modal scrim is not ported.
#[component]
pub fn DtPopover(
    /// Popover id. Generated when empty.
    #[prop(optional, into)]
    id: String,
    /// Controlled visibility (`open`). Unset toggles on anchor click.
    #[prop(optional)]
    open: Option<Signal<bool>>,
    /// `update:open`.
    #[prop(optional, into)]
    on_open_change: Option<Callback<bool>>,
    /// Fired after open or close (`opened`).
    #[prop(optional, into)]
    on_opened: Option<Callback<bool>>,
    #[prop(optional, default = "bottom-end")] placement: &'static str,
    /// Comma-separated fallbacks. Default `auto`.
    #[prop(optional, default = "auto")]
    fallback_placements: &'static str,
    /// Skidding is 0. This is the distance in px (`offset` `[0, distance]`).
    #[prop(optional, default = DEFAULT_OFFSET)]
    offset: f64,
    /// `none`, `small`, `medium`, or `large` (default).
    #[prop(optional, default = "large")]
    padding: &'static str,
    /// Dialog role. Default `dialog`.
    #[prop(optional, default = "dialog")]
    role: &'static str,
    /// `body` (default) portals the dialog. `parent` renders it inline.
    #[prop(optional, default = "body")]
    append_to: &'static str,
    #[prop(optional, default = true)] modal: bool,
    /// Close on outside `mousedown` (`hideOnClick`).
    #[prop(optional, default = true)]
    close_on_outside: bool,
    /// `none` (default), `dialog`, `first`, or `#id` (`initialFocusElement`).
    #[prop(optional, default = "none")]
    initial_focus: &'static str,
    /// Dialog tabindex (`contentTabindex`). Default `-1`. `None` omits it.
    #[prop(default = Some(-1))]
    content_tabindex: Option<i32>,
    /// Arrow up or down on the anchor opens an uncontrolled popover (`openWithArrowKeys`).
    #[prop(optional)]
    open_with_arrow_keys: bool,
    /// Poll the anchor rect while open (`sticky`). Scroll and resize always do.
    #[prop(optional)]
    sticky: bool,
    /// `anchor` matches the dialog width to the anchor (`contentWidth`).
    #[prop(optional, default = "")]
    content_width: &'static str,
    /// CSS max-height (`maxHeight`).
    #[prop(optional, into)]
    max_height: String,
    /// CSS max-width (`maxWidth`).
    #[prop(optional, into)]
    max_width: String,
    /// Extra class on `.d-popover__dialog` (`dialogClass`).
    #[prop(optional, into)]
    dialog_class: String,
    /// Extra class on `.d-popover__content` (`contentClass`).
    #[prop(optional, into)]
    content_class: String,
    /// Extra class on `.d-popover__header` (`headerWrapperClass`).
    #[prop(optional, into)]
    header_class: String,
    /// Extra class on `.d-popover__footer` (`footerWrapperClass`).
    #[prop(optional, into)]
    footer_class: String,
    /// `light`, `dark`, or `invert` (`contentMode`). Empty omits `data-dt-mode`.
    #[prop(optional, default = "")]
    content_mode: &'static str,
    /// Element id to position against when there is no anchor slot.
    #[prop(optional, into)]
    external_anchor_id: String,
    /// Anchor slot. Ignored when `external_anchor_id` is set.
    #[prop(optional)]
    anchor: Option<ViewFn>,
    /// `display: contents` on the anchor wrapper so a parent flex row sees the child.
    #[prop(optional)]
    flatten_anchor: bool,
    /// Right-click opens at the pointer. The left click does not toggle (`openOnContext`).
    #[prop(optional)]
    open_on_context: bool,
    /// Keep the placement when it overflows (`tether`). False shifts into view.
    #[prop(optional)]
    tether: bool,
    /// `fade` fades the dialog in. Anything else, including empty, does not animate.
    #[prop(optional, default = "")]
    transition: &'static str,
    /// Anchor click toggles. `DtHovercard` passes false.
    #[prop(optional, default = true)]
    toggle_on_click: bool,
    /// `data-qa` prefix. Empty keeps the popover DOM unchanged.
    /// `dt-hovercard` sets the anchor, dialog, and content suffixes from Vue.
    #[prop(optional, into)]
    data_qa: String,
    /// Extra class on the anchor wrapper (`anchorClass`).
    #[prop(optional, into)]
    anchor_class: String,
    /// Anchor and dialog `mouseenter`.
    #[prop(optional, into)]
    on_pointer_enter: Option<Callback<()>>,
    /// Anchor and dialog `mouseleave`.
    #[prop(optional, into)]
    on_pointer_leave: Option<Callback<()>>,
    /// Trap Tab while open and restore focus on close. Default false.
    #[prop(optional)]
    focus_trap: bool,
    /// Keydown on the dialog, after it bubbles from the content.
    #[prop(optional, into)]
    on_keydown: Option<Callback<KeyboardEvent>>,
    #[prop(optional)] header: Option<ViewFn>,
    #[prop(optional)] footer: Option<ViewFn>,
    children: ChildrenFn,
) -> impl IntoView {
    let controlled = open.is_some();
    let internal = RwSignal::new(false);
    let shown = move || {
        open.map(|open| open.get())
            .unwrap_or_else(|| internal.get())
    };
    let shown_untracked = move || {
        open.map(|open| open.get_untracked())
            .unwrap_or_else(|| internal.get_untracked())
    };
    let restore_focus = StoredValue::new(None::<HtmlElement>);
    let set_shown = move |next: bool| {
        let was = shown_untracked();
        if next && !was && focus_trap {
            if let Some(active) = web_sys::window()
                .and_then(|window| window.document())
                .and_then(|document| document.active_element())
                .and_then(|element| element.dyn_into::<HtmlElement>().ok())
            {
                restore_focus.set_value(Some(active));
            }
        }
        if !controlled {
            internal.set(next);
        }
        if let Some(on_open_change) = on_open_change {
            on_open_change.run(next);
        }
        if !next && was && focus_trap {
            if let Some(element) = restore_focus.get_value() {
                let _ = element.focus();
            }
        }
    };
    let was_open = StoredValue::new(false);
    Effect::new(move |_| {
        let open_now = shown();
        let previous = was_open.get_value();
        if open_now == previous {
            return;
        }
        was_open.set_value(open_now);
        if let Some(on_opened) = on_opened {
            on_opened.run(open_now);
        }
    });

    let dialog_id = if id.is_empty() {
        unique_id("dt-popover")
    } else {
        id
    };
    let dialog_id_label = dialog_id.clone();
    let anchor_ref = NodeRef::<html::Span>::new();
    let dialog_ref = NodeRef::<html::Div>::new();
    let coords = RwSignal::new(None::<ComputedPosition>);
    let anchor_width = RwSignal::new(None::<f64>);
    let preferred = Placement::parse(placement);
    if !known_popover_role(role) {
        leptos::logging::warn!(
            "DtPopover role `{role}` is not dialog, menu, listbox, tree, or grid"
        );
    }
    if append_to != "body" && append_to != "parent" {
        leptos::logging::warn!(
            "DtPopover append_to `{append_to}` is not body or parent; using body"
        );
    }
    let edge_padding = popover_edge_padding(padding);
    let content_class = join_class([
        "d-popover__content",
        popover_padding_class(padding),
        content_class.as_str(),
    ]);
    let header_class = join_class(["d-popover__header", edge_padding, header_class.as_str()]);
    let footer_class = join_class(["d-popover__footer", edge_padding, footer_class.as_str()]);
    let dialog_class = join_class([
        "d-popover__dialog",
        modal.then_some("d-popover__dialog--modal").unwrap_or(""),
        dialog_class.as_str(),
    ]);
    let external_anchor_id = StoredValue::new(external_anchor_id);
    let max_height = StoredValue::new(max_height);
    let max_width = StoredValue::new(max_width);
    let context_point = RwSignal::new(None::<(f64, f64)>);
    let layout_epoch = RwSignal::new(0u32);
    let fade_in = RwSignal::new(false);
    let close_id = StoredValue::new(unique_id("dt-sr-only-close"));
    let on_keydown = StoredValue::new(on_keydown);
    let on_pointer_enter = StoredValue::new(on_pointer_enter);
    let on_pointer_leave = StoredValue::new(on_pointer_leave);
    let qa_anchor = (!data_qa.is_empty()).then(|| format!("{data_qa}-anchor"));
    let qa_dialog = (!data_qa.is_empty()).then(|| format!("{data_qa}__dialog"));
    let qa_content = (!data_qa.is_empty()).then(|| format!("{data_qa}-content"));
    let anchor_class = (!anchor_class.is_empty()).then_some(anchor_class);

    let anchor_element = move || -> Option<HtmlElement> {
        let external = external_anchor_id.get_value();
        if !external.is_empty() {
            return web_sys::window()?
                .document()?
                .get_element_by_id(&external)?
                .dyn_into()
                .ok();
        }
        anchor_ref
            .get_untracked()
            .map(|element| element.unchecked_into())
    };

    let reposition = move || {
        let Some(anchor) = anchor_element() else {
            return;
        };
        let Some(dialog) = dialog_ref.get_untracked() else {
            return;
        };
        let dialog: HtmlElement = dialog.unchecked_into();
        let reference = match context_point.get_untracked() {
            Some((x, y)) => pointer_rect(x, y),
            None => element_rect(&reference_element(&anchor)),
        };
        anchor_width.set(Some(reference.width));
        let floating = element_rect(&dialog);
        if floating.width <= 0.0 || floating.height <= 0.0 {
            return;
        }
        let placement_now = if context_point.get_untracked().is_some() {
            Placement::parse("right-start")
        } else {
            preferred
        };
        coords.set(Some(compute_position(
            reference,
            floating,
            placement_now,
            &PopperOptions {
                offset,
                padding: VIEWPORT_PADDING,
                viewport: viewport_rect(),
                fallbacks: parse_fallbacks(fallback_placements),
                auto: Placement::is_auto(placement) && context_point.get_untracked().is_none(),
                tether,
            },
        )));
    };

    Effect::new(move |_| {
        let _ = layout_epoch.get();
        let open_now = shown();
        let Some(anchor) = anchor_element() else {
            return;
        };
        label_anchor(&anchor, open_now, &dialog_id_label, role);
        if !open_now {
            coords.set(None);
            context_point.set(None);
            fade_in.set(false);
            return;
        }
        fade_in.set(false);
        request_animation_frame(move || fade_in.set(true));
        if dialog_ref.get().is_none() {
            return;
        }
        reposition();
        request_animation_frame(move || reposition());

        let queued = Rc::new(Cell::new(false));
        let follow = Closure::<dyn FnMut()>::new(move || {
            if queued.get() {
                return;
            }
            queued.set(true);
            let queued = Rc::clone(&queued);
            request_animation_frame(move || {
                queued.set(false);
                reposition();
            });
        });
        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(document) = window.document() else {
            return;
        };
        let function = follow.as_ref().unchecked_ref::<js_sys::Function>().clone();
        if document
            .add_event_listener_with_callback_and_bool("scroll", function.unchecked_ref(), true)
            .is_err()
        {
            return;
        }
        if window
            .add_event_listener_with_callback("resize", function.unchecked_ref())
            .is_err()
        {
            let _ = document.remove_event_listener_with_callback_and_bool(
                "scroll",
                function.unchecked_ref(),
                true,
            );
            return;
        }
        follow.forget();
        let remove_scroll = function.clone();
        let remove_resize = function;
        on_cleanup(move || {
            if let Some(window) = web_sys::window() {
                let _ = window
                    .remove_event_listener_with_callback("resize", remove_resize.unchecked_ref());
                if let Some(document) = window.document() {
                    let _ = document.remove_event_listener_with_callback_and_bool(
                        "scroll",
                        remove_scroll.unchecked_ref(),
                        true,
                    );
                }
            }
        });
    });

    Effect::new(move |_| {
        if !shown() || !sticky {
            return;
        }
        if let Ok(handle) =
            set_interval_with_handle(move || reposition(), Duration::from_millis(50))
        {
            on_cleanup(move || handle.clear());
        }
    });

    Effect::new(move |_| {
        if !shown() {
            return;
        }
        let mode = initial_focus;
        request_animation_frame(move || {
            let Some(dialog) = dialog_ref.get_untracked() else {
                return;
            };
            let dialog: HtmlElement = dialog.unchecked_into();
            match mode {
                "dialog" => {
                    let _ = dialog.focus();
                }
                "first" => focus_first(&dialog),
                _ if mode.starts_with('#') => {
                    let selector = mode.to_string();
                    let found = dialog
                        .query_selector(&selector)
                        .ok()
                        .flatten()
                        .and_then(|element| element.dyn_into::<HtmlElement>().ok());
                    if let Some(element) = found {
                        let _ = element.focus();
                    } else {
                        leptos::logging::warn!(
                            "Could not find the element specified in dt-popover prop \"initialFocusElement\". Defaulting to focusing the dialog."
                        );
                        let _ = dialog.focus();
                    }
                }
                _ => {}
            }
        });
    });

    Effect::new(move |_| {
        if !shown() || !close_on_outside {
            return;
        }
        let closure = Closure::<dyn FnMut(MouseEvent)>::new(move |event: MouseEvent| {
            if let Some(anchor) = anchor_element() {
                if event_inside(&event, anchor.as_ref()) {
                    return;
                }
            }
            if let Some(dialog) = dialog_ref.get_untracked() {
                let dialog: HtmlElement = dialog.unchecked_into();
                if event_inside(&event, dialog.as_ref()) {
                    return;
                }
            }
            set_shown(false);
        });
        let Some(document) = web_sys::window().and_then(|window| window.document()) else {
            return;
        };
        let function = closure.as_ref().unchecked_ref::<js_sys::Function>().clone();
        if document
            .add_event_listener_with_callback("mousedown", function.unchecked_ref())
            .is_err()
        {
            return;
        }
        // The document listener keeps this closure. Cleanup only removes the listener.
        closure.forget();
        on_cleanup(move || {
            if let Some(document) = web_sys::window().and_then(|window| window.document()) {
                let _ = document
                    .remove_event_listener_with_callback("mousedown", function.unchecked_ref());
            }
        });
    });

    let dialog = ViewFn::from(move || {
        let header = header.clone();
        let footer = footer.clone();
        let children = children.clone();
        let dialog_id = dialog_id.clone();
        let dialog_class = dialog_class.clone();
        let content_class = content_class.clone();
        let header_class = header_class.clone();
        let footer_class = footer_class.clone();
        let qa_dialog = qa_dialog.clone();
        let qa_content = qa_content.clone();
        view! {
            <Show when=move || shown()>
                <div
                    node_ref=dialog_ref
                    id=dialog_id.clone()
                    class=dialog_class.clone()
                    data-qa=qa_dialog.clone()
                    role=role
                    tabindex=content_tabindex.map(|index| index.to_string())
                    aria-modal=modal.then_some("true")
                    data-dt-mode=resolved_content_mode(content_mode, &root_mode()).map(str::to_string)
                    data-dt-contrast=resolved_content_mode(content_mode, &root_mode()).map(|_| root_contrast())
                    style=move || {
                        let anchor = anchor_element();
                        let z = popover_z_index(anchor);
                        let width = if content_width == "anchor" {
                            anchor_width
                                .get()
                                .map(|width| format!("width:{width}px;"))
                                .unwrap_or_default()
                        } else {
                            String::new()
                        };
                        let motion = popover_motion_style(transition, fade_in.get());
                        match coords.get() {
                            Some(position) => format!(
                                "position:fixed;top:{}px;left:{}px;bottom:auto;right:auto;transform:none;z-index:{z};{width}{motion}",
                                position.y, position.x
                            ),
                            None => format!(
                                "position:fixed;top:0;left:0;visibility:hidden;pointer-events:none;z-index:{z};{width}{motion}"
                            ),
                        }
                    }
                    on:mouseenter=move |_| {
                        if let Some(callback) = on_pointer_enter.get_value() {
                            callback.run(());
                        }
                    }
                    on:mouseleave=move |_| {
                        if let Some(callback) = on_pointer_leave.get_value() {
                            callback.run(());
                        }
                    }
                    on:keydown=move |event: KeyboardEvent| {
                        if let Some(callback) = on_keydown.get_value() {
                            callback.run(event.clone());
                        }
                        if focus_trap && event.key() == "Tab" {
                            if let Some(dialog) = dialog_ref.get_untracked() {
                                let dialog: HtmlElement = dialog.unchecked_into();
                                trap_tab(&event, &dialog);
                            }
                        }
                        if event.key() == "Escape" {
                            event.stop_propagation();
                            set_shown(false);
                        }
                    }
                >
                    {header.as_ref().map(|slot| view! {
                        <div class=header_class.clone()>{slot.run()}</div>
                    })}
                    <div
                        class=content_class.clone()
                        data-qa=qa_content.clone()
                        style=move || {
                            let mut style = String::new();
                            let max_height = max_height.get_value();
                            let max_width = max_width.get_value();
                            if !max_height.is_empty() {
                                style.push_str(&format!("max-height:{max_height};overflow:auto;"));
                            }
                            if !max_width.is_empty() {
                                style.push_str(&format!("max-width:{max_width};"));
                            }
                            style
                        }
                    >
                        {children()}
                    </div>
                    {footer.as_ref().map(|slot| view! {
                        <div class=footer_class.clone()>{slot.run()}</div>
                    })}
                    <DtButton
                        id=close_id.get_value()
                        data_qa="dt-sr-only-close-button"
                        class="d-vi-visible-sr"
                        aria_label="Click to close"
                        title="Click to close"
                        on_click=Callback::new(move |_| set_shown(false))
                        icon=ViewFn::from(|| view! { <DtIconClose /> })
                    />
                </div>
            </Show>
        }
    });

    let floating = if append_to == "parent" {
        dialog.run().into_any()
    } else {
        let dialog = dialog.clone();
        view! { <Portal>{dialog.run()}</Portal> }.into_any()
    };

    let shell = view! {
        {anchor.map(|slot| view! {
            <span
                node_ref=anchor_ref
                class=anchor_class.clone()
                data-qa=qa_anchor.clone()
                style=flatten_anchor.then_some("display: contents")
                tabindex=open_on_context.then_some("0")
                on:mouseenter=move |_| {
                    if let Some(callback) = on_pointer_enter.get_value() {
                        callback.run(());
                    }
                }
                on:mouseleave=move |_| {
                    if let Some(callback) = on_pointer_leave.get_value() {
                        callback.run(());
                    }
                }
                on:click=move |_| {
                    if open_on_context || !toggle_on_click {
                        return;
                    }
                    set_shown(!shown_untracked());
                }
                on:contextmenu=move |event: MouseEvent| {
                    if !open_on_context {
                        return;
                    }
                    event.prevent_default();
                    context_point.set(Some((event.client_x() as f64, event.client_y() as f64)));
                    layout_epoch.update(|epoch| *epoch = epoch.wrapping_add(1));
                    set_shown(true);
                }
                on:keydown=move |event: KeyboardEvent| {
                    let key = event.key();
                    if key == "Escape" {
                        if shown_untracked() {
                            set_shown(false);
                        }
                        return;
                    }
                    if key != "ArrowUp" && key != "ArrowDown" {
                        return;
                    }
                    if !open_with_arrow_keys || controlled || shown_untracked() {
                        return;
                    }
                    event.prevent_default();
                    set_shown(true);
                }
            >
                {slot.run()}
            </span>
        })}
        {floating}
    };
    if data_qa.is_empty() {
        shell.into_any()
    } else {
        view! {
            <div
                class=move || {
                    if shown() {
                        "d-popover d-popover__anchor--opened"
                    } else {
                        "d-popover"
                    }
                }
                data-qa=data_qa
            >
                {shell}
            </div>
        }
        .into_any()
    }
}

#[cfg(test)]
mod tests {
    use super::resolved_content_mode;

    #[test]
    fn content_mode_inverts_against_the_root() {
        assert_eq!(resolved_content_mode("", "dark"), None);
        assert_eq!(resolved_content_mode("light", "dark"), Some("light"));
        assert_eq!(resolved_content_mode("dark", "light"), Some("dark"));
        assert_eq!(resolved_content_mode("invert", "dark"), Some("light"));
        assert_eq!(resolved_content_mode("invert", "light"), Some("dark"));
        assert_eq!(resolved_content_mode("invert", ""), Some("dark"));
        assert_eq!(resolved_content_mode("nope", "light"), None);
    }

    #[test]
    fn pointer_rect_is_a_zero_box_at_the_click() {
        let rect = super::pointer_rect(12.0, 40.0);
        assert_eq!(rect.x, 12.0);
        assert_eq!(rect.y, 40.0);
        assert_eq!(rect.width, 0.0);
        assert_eq!(rect.height, 0.0);
    }

    #[test]
    fn fade_hides_until_the_frame_after_open() {
        assert_eq!(
            super::popover_motion_style("fade", false),
            "opacity:0;transition:opacity 120ms ease;"
        );
        assert_eq!(
            super::popover_motion_style("fade", true),
            "opacity:1;transition:opacity 120ms ease;"
        );
        assert_eq!(super::popover_motion_style("none", true), "");
        assert_eq!(super::popover_motion_style("", true), "");
    }

    #[test]
    fn header_and_footer_padding_follows_the_content_size() {
        assert_eq!(super::popover_edge_padding("none"), "");
        assert_eq!(super::popover_edge_padding("small"), "d-pis-50");
        assert_eq!(super::popover_edge_padding("medium"), "d-pis-100");
        assert_eq!(super::popover_edge_padding("large"), "d-pis-200");
        assert_eq!(super::popover_edge_padding("xl"), "");
    }

    #[test]
    fn roles_include_tree_and_grid() {
        assert!(super::known_popover_role("dialog"));
        assert!(super::known_popover_role("menu"));
        assert!(super::known_popover_role("listbox"));
        assert!(super::known_popover_role("tree"));
        assert!(super::known_popover_role("grid"));
        assert!(!super::known_popover_role("tooltip"));
    }
}
