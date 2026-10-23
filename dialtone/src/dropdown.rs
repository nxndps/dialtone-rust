use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, EventTarget, HtmlElement, KeyboardEvent, MouseEvent};

use super::list_nav::{
    ensure_ids, focus_item, item_index_at, join_class, move_index, paint_highlight, role_items,
};
use super::popover::DtPopover;
use super::unique_id;

/// Vue `DtPopover` default offset `[0, 4]`.
const DROPDOWN_OFFSET: f64 = 4.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HighlightStep {
    Move(i32),
    Beginning,
    End,
}

/// `arrow-keys`, `tab`, or `none`. Anything else is `arrow-keys`.
fn navigation_kind(value: &str) -> &'static str {
    match value {
        "tab" => "tab",
        "none" => "none",
        _ => "arrow-keys",
    }
}

/// Dialtone `DROPDOWN_PADDING_CLASSES` at `dialtone/v10.7.0`.
fn list_padding_class(padding: &str) -> &'static str {
    match padding {
        "none" => "",
        "large" => "d-py-50",
        _ => "d-py-0",
    }
}

fn list_classes(padding: &str, list_class: &str, context: bool) -> String {
    join_class([
        "d-dropdown-list",
        list_padding_class(padding),
        list_class,
        context.then_some("d-context-menu-list").unwrap_or(""),
    ])
}

fn highlight_step(len: usize, index: i32, delta: i32) -> HighlightStep {
    if len == 0 {
        return HighlightStep::Move(-1);
    }
    let last = len as i32 - 1;
    if index < 0 {
        return HighlightStep::Move(if delta < 0 { last } else { 0 });
    }
    let next = index as i64 + delta as i64;
    if next < 0 {
        HighlightStep::Beginning
    } else if next > last as i64 {
        HighlightStep::End
    } else {
        HighlightStep::Move(move_index(len, index, delta, false))
    }
}

fn enabled_menuitem(target: Option<EventTarget>) -> Option<Element> {
    let element = target?.dyn_into::<Element>().ok()?;
    let item = element.closest("[role=menuitem]").ok().flatten()?;
    if item.get_attribute("aria-disabled").as_deref() == Some("true") {
        return None;
    }
    Some(item)
}

/// Dialtone Vue `DtDropdownSeparator`.
#[component]
pub fn DtDropdownSeparator() -> impl IntoView {
    view! { <li aria-hidden="true" class="d-list-item-separator"></li> }
}

/// Dialtone Vue `DtDropdown`.
///
/// Renders [`DtPopover`] with `padding` `none` and `role` `menu`. List rows stay
/// in the default slot (`role="menuitem"`). A click on an enabled menuitem
/// closes the menu. Arrow keys move focus when `navigation_type` is
/// `arrow-keys`. Home and End jump. A typed letter focuses the first item whose
/// text starts with that prefix. `open` unset toggles from the anchor. A set
/// signal is controlled through `on_open_change`. `on_highlight` receives the
/// focused index.
///
/// `open_on_context` (`openOnContext`) adds `d-context-menu-list`, opens from
/// the pointer on `contextmenu`, and does not open from a left click. `tether`
/// true keeps the menu on its placement. The menu follows the anchor on scroll.
/// `on_beginning_of_list` and `on_end_of_list` replace the wrap.
#[component]
pub fn DtDropdown(
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional, into)] on_open_change: Option<Callback<bool>>,
    #[prop(optional, into)] on_opened: Option<Callback<bool>>,
    #[prop(optional, into)] on_highlight: Option<Callback<i32>>,
    /// `arrow-keys` (default), `tab`, or `none` (`navigationType`).
    #[prop(optional, default = "arrow-keys")]
    navigation_type: &'static str,
    #[prop(optional, default = "bottom")] placement: &'static str,
    #[prop(optional, default = "auto")] fallback_placements: &'static str,
    /// `body` (default) or `parent` (`appendTo`).
    #[prop(optional, default = "body")]
    append_to: &'static str,
    #[prop(optional, default = true)] modal: bool,
    #[prop(optional, into)] max_height: String,
    #[prop(optional, into)] max_width: String,
    /// `anchor` sizes the dialog to the trigger (`contentWidth`).
    #[prop(optional, default = "")]
    content_width: &'static str,
    /// `none`, `small` (default), or `large`.
    #[prop(optional, default = "small")]
    padding: &'static str,
    /// Extra class on the list (`listClass`).
    #[prop(optional, into)]
    list_class: String,
    /// Id on the list. Generated when empty (`listId`).
    #[prop(optional, into)]
    list_id: String,
    /// Extra class on the footer wrapper (`footerClass`).
    #[prop(optional, into)]
    footer_class: String,
    /// Context menu (`openOnContext`).
    #[prop(optional)]
    open_on_context: bool,
    /// Clip instead of shifting over the anchor (`tether`). Default true.
    #[prop(optional, default = true)]
    tether: bool,
    /// `fade` (default) or `none` (`transition`). Other names do not animate.
    #[prop(optional, default = "fade")]
    transition: &'static str,
    /// Called instead of wrapping to the last item (`onBeginningOfList`).
    #[prop(optional, into)]
    on_beginning_of_list: Option<Callback<()>>,
    /// Called instead of wrapping to the first item (`onEndOfList`).
    #[prop(optional, into)]
    on_end_of_list: Option<Callback<()>>,
    /// `light`, `dark`, or `invert` (`contentMode`).
    #[prop(optional, default = "")]
    content_mode: &'static str,
    /// `display: contents` on the anchor wrappers. Split button uses this so the
    /// trigger is a flex item of `.d-split-btn`.
    #[prop(optional)]
    flatten_anchor: bool,
    /// Anchor slot.
    anchor: ViewFn,
    #[prop(optional)] footer: Option<ViewFn>,
    children: ChildrenFn,
) -> impl IntoView {
    let kind = navigation_kind(navigation_type);
    if kind == "arrow-keys" && navigation_type != "arrow-keys" {
        leptos::logging::warn!(
            "DtDropdown navigation_type `{navigation_type}` is not arrow-keys, tab, or none"
        );
    }
    if !matches!(padding, "none" | "small" | "large") {
        leptos::logging::warn!("DtDropdown padding `{padding}` is not none, small, or large");
    }
    if !matches!(transition, "fade" | "none") {
        leptos::logging::warn!("DtDropdown transition `{transition}` is not fade or none");
    }

    let controlled = open.is_some();
    let internal = RwSignal::new(false);
    let open_signal = open.unwrap_or_else(|| Signal::from(internal));
    let user_change = StoredValue::new(on_open_change);
    let set_open = move |next: bool| {
        if !controlled {
            internal.set(next);
        }
        if let Some(callback) = user_change.get_value() {
            callback.run(next);
        }
    };
    let list_id = if list_id.is_empty() {
        unique_id("dt-dropdown-list")
    } else {
        list_id
    };
    let list_id = StoredValue::new(list_id);
    let list_ref = NodeRef::<leptos::html::Ul>::new();
    let highlight = RwSignal::new(-1i32);
    let opened_with_keyboard = RwSignal::new(false);
    let list_class = list_classes(padding, &list_class, open_on_context);
    let typed = StoredValue::new(String::new());
    let on_highlight = StoredValue::new(on_highlight);
    let on_beginning = StoredValue::new(on_beginning_of_list);
    let on_end = StoredValue::new(on_end_of_list);

    let items_in = move |list: &HtmlElement| {
        let items = role_items(list, "[role=menuitem]");
        ensure_ids(&items, &list_id.get_value());
        items
    };

    let apply_highlight = move |index: i32| {
        let Some(list) = list_ref.get() else {
            return;
        };
        let list: HtmlElement = list.unchecked_into();
        let items = items_in(&list);
        let index = if items.is_empty() { -1 } else { index };
        highlight.set(index);
        paint_highlight(&items, index);
        if index >= 0 {
            focus_item(&items, index);
        }
        if let Some(callback) = on_highlight.get_value() {
            callback.run(index);
        }
    };

    let move_highlight = move |delta: i32| {
        let Some(list) = list_ref.get() else {
            return;
        };
        let list: HtmlElement = list.unchecked_into();
        let items = items_in(&list);
        if items.is_empty() {
            return;
        }
        match highlight_step(items.len(), highlight.get_untracked(), delta) {
            HighlightStep::Move(index) => apply_highlight(index),
            HighlightStep::Beginning => {
                if let Some(callback) = on_beginning.get_value() {
                    callback.run(());
                } else {
                    apply_highlight((items.len() as i32) - 1);
                }
            }
            HighlightStep::End => {
                if let Some(callback) = on_end.get_value() {
                    callback.run(());
                } else {
                    apply_highlight(0);
                }
            }
        }
    };

    let activate_item = move |event: &KeyboardEvent| {
        let Some(list) = list_ref.get() else {
            return;
        };
        let list: HtmlElement = list.unchecked_into();
        let items = items_in(&list);
        let indexed = items
            .get(highlight.get_untracked() as usize)
            .cloned()
            .filter(|item| item.get_attribute("aria-disabled").as_deref() != Some("true"));
        let item = indexed.or_else(|| enabled_menuitem(event.target()));
        let Some(item) = item else {
            return;
        };
        event.prevent_default();
        if let Ok(item) = item.dyn_into::<HtmlElement>() {
            let _ = item.click();
        }
        set_open(false);
    };

    Effect::new(move |_| {
        let open_now = open_signal.get();
        if !open_now {
            highlight.set(-1);
            opened_with_keyboard.set(false);
            typed.set_value(String::new());
            return;
        }
        if opened_with_keyboard.get_untracked() && kind == "arrow-keys" {
            request_animation_frame(move || apply_highlight(0));
        }
    });

    let handle_key = Callback::new(move |event: KeyboardEvent| {
        let key = event.key();
        let open_now = open_signal.get_untracked();
        if !open_now {
            if open_on_context || kind == "none" {
                return;
            }
            if matches!(key.as_str(), "ArrowDown" | "ArrowUp" | "Enter" | " ") {
                event.prevent_default();
                opened_with_keyboard.set(true);
                set_open(true);
            }
            return;
        }
        if key == "Escape" {
            set_open(false);
            return;
        }
        if kind == "none" {
            return;
        }
        if matches!(key.as_str(), "Enter" | " ") {
            activate_item(&event);
            return;
        }
        if kind != "arrow-keys" {
            return;
        }
        if matches!(key.as_str(), "ArrowDown" | "ArrowUp" | "Home" | "End") {
            event.prevent_default();
            let delta = match key.as_str() {
                "ArrowDown" => 1,
                "ArrowUp" => -1,
                "Home" => {
                    apply_highlight(0);
                    return;
                }
                _ => {
                    let Some(list) = list_ref.get() else {
                        return;
                    };
                    let list: HtmlElement = list.unchecked_into();
                    let items = items_in(&list);
                    if !items.is_empty() {
                        apply_highlight((items.len() as i32) - 1);
                    }
                    return;
                }
            };
            move_highlight(delta);
            return;
        }
        if key.len() == 1 && !event.ctrl_key() && !event.meta_key() && !event.alt_key() {
            event.prevent_default();
            let mut buffer = typed.get_value();
            buffer.push_str(&key.to_lowercase());
            typed.set_value(buffer.clone());
            let Some(list) = list_ref.get() else {
                return;
            };
            let list: HtmlElement = list.unchecked_into();
            let items = items_in(&list);
            let index = items.iter().position(|element| {
                element
                    .text_content()
                    .unwrap_or_default()
                    .trim()
                    .to_lowercase()
                    .starts_with(&buffer)
            });
            if let Some(index) = index {
                apply_highlight(index as i32);
            }
        }
    });
    let handle_key_anchor = handle_key.clone();
    let handle_key_dialog = handle_key;

    let anchor = StoredValue::new(anchor);
    let anchor_slot = ViewFn::from(move || {
        let anchor = anchor.get_value();
        view! {
            <span
                style=flatten_anchor.then_some("display: contents")
                on:keydown=move |event: KeyboardEvent| handle_key_anchor.run(event)
            >
                {anchor.run()}
            </span>
        }
    });

    let menu_transition = if transition == "fade" { "fade" } else { "" };

    view! {
        <DtPopover
            open=open_signal
            on_open_change=Callback::new(move |next| set_open(next))
            nostrip:on_opened=on_opened
            on_keydown=handle_key_dialog
            placement=placement
            fallback_placements=fallback_placements
            offset=DROPDOWN_OFFSET
            padding="none"
            role="menu"
            append_to=append_to
            modal=modal
            max_height=max_height
            max_width=max_width
            content_width=content_width
            content_mode=content_mode
            footer_class=footer_class
            flatten_anchor=flatten_anchor
            open_on_context=open_on_context
            tether=tether
            transition=menu_transition
            anchor=anchor_slot
            nostrip:footer=footer
        >
            <ul
                node_ref=list_ref
                id=list_id.get_value()
                class=list_class.clone()
                data-qa="dt-dropdown-list-wrapper"
                on:click=move |event: MouseEvent| {
                    if enabled_menuitem(event.target()).is_some() {
                        set_open(false);
                    }
                }
                on:mouseleave=move |_| {
                    if let Some(list) = list_ref.get() {
                        let list: HtmlElement = list.unchecked_into();
                        paint_highlight(&role_items(&list, "[role=menuitem]"), -1);
                    }
                    highlight.set(-1);
                }
                on:mousemove=move |event: MouseEvent| {
                    let Some(list) = list_ref.get() else {
                        return;
                    };
                    let list: HtmlElement = list.unchecked_into();
                    let Some(index) = item_index_at(&list, &event, "li[role=menuitem]") else {
                        return;
                    };
                    if highlight.get_untracked() == index as i32 {
                        return;
                    }
                    apply_highlight(index as i32);
                }
            >
                {children()}
            </ul>
        </DtPopover>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_class_uses_the_v10_padding_tokens() {
        assert_eq!(list_classes("small", "", false), "d-dropdown-list d-py-0");
        assert_eq!(
            list_classes("large", "extra", true),
            "d-dropdown-list d-py-50 extra d-context-menu-list"
        );
        assert_eq!(list_classes("none", "", false), "d-dropdown-list");
    }

    #[test]
    fn navigation_kind_rejects_unknown_tokens() {
        assert_eq!(navigation_kind("arrow-keys"), "arrow-keys");
        assert_eq!(navigation_kind("tab"), "tab");
        assert_eq!(navigation_kind("none"), "none");
        assert_eq!(navigation_kind("grid"), "arrow-keys");
    }

    #[test]
    fn highlight_step_wraps_only_when_the_caller_asks() {
        assert_eq!(highlight_step(3, 0, -1), HighlightStep::Beginning);
        assert_eq!(highlight_step(3, 2, 1), HighlightStep::End);
        assert_eq!(highlight_step(3, 1, 1), HighlightStep::Move(2));
        assert_eq!(highlight_step(3, -1, 1), HighlightStep::Move(0));
        assert_eq!(highlight_step(3, -1, -1), HighlightStep::Move(2));
        assert_eq!(highlight_step(0, 0, 1), HighlightStep::Move(-1));
    }
}
