use std::time::Duration;

use leptos::leptos_dom::helpers::set_timeout;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Element, KeyboardEvent, MouseEvent};

use super::list_nav::{ensure_ids, join_class, paint_highlight, role_items};
use super::skeleton::{DtSkeleton, DtSkeletonTextOption};
use super::unique_id;

/// `xs`–`xl` and `100`–`500`. Empty is Vue `null`.
pub(crate) fn combobox_size_known(size: &str) -> bool {
    matches!(
        size,
        "" | "xs" | "sm" | "md" | "lg" | "xl" | "100" | "200" | "300" | "400" | "500"
    )
}

/// ArrowDown. `None` means the selection passed the end (`onEndOfList`).
pub(crate) fn combobox_move_down(len: usize, index: i32) -> Option<i32> {
    let last = len as i32 - 1;
    if index < last {
        Some(index + 1)
    } else {
        None
    }
}

/// ArrowUp. `None` means the selection passed the start (`onBeginningOfList`).
pub(crate) fn combobox_move_up(index: i32) -> Option<i32> {
    if index > 0 {
        Some(index - 1)
    } else {
        None
    }
}

pub(crate) fn combobox_jump_start() -> i32 {
    0
}

pub(crate) fn combobox_jump_end(len: usize) -> i32 {
    len as i32 - 1
}

fn empty_option_class(extra: &str) -> String {
    let marker = join_class(["d-combobox__empty-list-item", extra]);
    join_class(["d-list-item", "d-list-item--static", marker.as_str()])
}

fn validate_field(label: &str, show_label: bool, size: &str, description: &str) {
    if label.is_empty() {
        leptos::logging::warn!("DtCombobox requires a label");
    }
    if !combobox_size_known(size) {
        leptos::logging::warn!("DtCombobox size `{size}` is not xs–xl or 100–500");
    }
    // `showLabel`, `size`, and `description` are Vue `inputProps`. The caller
    // copies them onto the input slot. Leptos slots cannot receive that object.
    let _ = (show_label, description.len());
}

fn find_list(list_id: &str) -> Option<Element> {
    web_sys::window()?.document()?.get_element_by_id(list_id)
}

fn stamp_input(input: &Element, expanded: bool, list_id: &str, label: &str, active: Option<&str>) {
    let _ = input.set_attribute("role", "combobox");
    let _ = input.set_attribute("aria-label", label);
    let _ = input.set_attribute("aria-expanded", if expanded { "true" } else { "false" });
    let _ = input.set_attribute("aria-owns", list_id);
    let _ = input.set_attribute("aria-haspopup", "listbox");
    let _ = input.set_attribute("aria-controls", list_id);
    if let Some(active) = active.filter(|id| !id.is_empty()) {
        let _ = input.set_attribute("aria-activedescendant", active);
    } else {
        let _ = input.remove_attribute("aria-activedescendant");
    }
}

fn stamp_list_element(element: &Element, list_id: &str, label: &str) {
    let _ = element.set_attribute("role", "listbox");
    let _ = element.set_attribute("id", list_id);
    let _ = element.set_attribute("aria-label", label);
    let _ = element.class_list().add_1("d-ps-relative");
}

fn active_descendant(list_id: &str, shown: bool, loading: bool, index: i32) -> Option<String> {
    if !shown || loading || index < 0 {
        return None;
    }
    let list = find_list(list_id)?;
    let items = role_items(&list, "[role=option]");
    let id = items.get(usize::try_from(index).ok()?)?.id();
    if id.is_empty() {
        None
    } else {
        Some(id)
    }
}

fn set_highlight_index(
    highlight: RwSignal<i32>,
    list_id: &str,
    index: i32,
    scroll: bool,
    loading: bool,
    on_highlight: &Option<Callback<i32>>,
) {
    highlight.set(index);
    let Some(list) = find_list(list_id) else {
        return;
    };
    let items = role_items(&list, "[role=option]");
    ensure_ids(&items, &format!("{list_id}-option"));
    paint_highlight(&items, index);
    for (item_index, element) in items.iter().enumerate() {
        let selected = item_index as i32 == index;
        let _ = element.set_attribute("aria-selected", if selected { "true" } else { "false" });
    }
    if scroll {
        if let Some(element) = usize::try_from(index)
            .ok()
            .and_then(|index| items.get(index))
        {
            element.scroll_into_view_with_bool(false);
        }
    }
    if !items.is_empty() && !loading {
        if let Some(callback) = on_highlight {
            callback.run(index);
        }
    }
}

fn loading_list(list_id: String, label: String) -> impl IntoView {
    view! {
        <ol
            id=list_id
            class="d-combobox__loading-list d-ps-relative"
            role="listbox"
            aria-label=label
            aria-busy="true"
        >
            {(0..7).map(|_| view! {
                <li class="d-list-item d-list-item--static" role="option">
                    <DtSkeleton text_option=DtSkeletonTextOption::default() />
                </li>
            }).collect_view()}
        </ol>
    }
}

fn empty_list_view(
    list_id: String,
    label: String,
    message: String,
    item_class: String,
    slot: Option<ViewFn>,
) -> impl IntoView {
    let option_class = empty_option_class(&item_class);
    view! {
        <ol
            id=list_id
            class="d-combobox__empty-list d-ps-relative"
            role="listbox"
            aria-label=label
            data-qa="dt-combobox-empty-list"
        >
            {if let Some(slot) = slot {
                slot.run().into_any()
            } else {
                view! {
                    <li class=option_class role="option">
                        <span>{message}</span>
                    </li>
                }
                .into_any()
            }}
        </ol>
    }
}

/// Dialtone Vue `DtCombobox` at `dialtone/v10.7.0`.
///
/// The component does not draw an input or a list. The `input` slot supplies
/// the field and the `list` slot supplies the options. Leptos slots cannot
/// receive Vue's `inputProps` and `listProps`. This port writes the combobox
/// ARIA attributes onto the first `input` in the input wrapper, and writes
/// `role="listbox"`, `list_id`, `d-ps-relative`, and `aria-label` onto the
/// list element. Copy `label`, `show_label` (`showLabel`), `size`, and
/// `description` onto [`super::DtInput`].
///
/// `show_list` is `showList` and defaults to false. It does not open or close
/// the list. `on_opened` is `opened` and runs when `show_list` changes after
/// the first value. `on_select` is `select` and receives the highlighted
/// index. `on_escape` is `escape`. `on_highlight` is `highlight`.
/// `on_beginning_of_list` and `on_end_of_list` replace the wrap
/// (`onBeginningOfList`, `onEndOfList`). `empty_state_message` is
/// `emptyStateMessage`. `empty_state_class` is `emptyStateClass`.
/// `list_rendered_outside` is `listRenderedOutside`: loading and empty rows
/// stay with the parent, and highlight reads the element with `list_id`.
#[component]
pub fn DtCombobox(
    label: String,
    /// `showLabel`. Default true.
    #[prop(optional, default = true)]
    show_label: bool,
    /// `xs`–`xl` or `100`–`500`. Empty is Vue `null` (`size`).
    #[prop(optional, default = "")]
    size: &'static str,
    #[prop(optional, into)] description: String,
    /// `listId`. Generated when empty.
    #[prop(optional, into)]
    list_id: String,
    /// `onBeginningOfList`.
    #[prop(optional, into)]
    on_beginning_of_list: Option<Callback<()>>,
    /// `onEndOfList`.
    #[prop(optional, into)]
    on_end_of_list: Option<Callback<()>>,
    /// `showList`. Default false.
    #[prop(optional, into)]
    show_list: Signal<bool>,
    /// `listRenderedOutside`.
    #[prop(optional)]
    list_rendered_outside: bool,
    /// `loading`.
    #[prop(optional, into)]
    loading: Signal<bool>,
    /// `emptyList`.
    #[prop(optional, into)]
    empty_list: Signal<bool>,
    /// `emptyStateMessage`.
    #[prop(optional, into)]
    empty_state_message: String,
    /// Extra class on the empty option (`emptyStateClass`).
    #[prop(optional, into)]
    empty_state_class: String,
    /// `clickOnSelect`.
    #[prop(optional, into)]
    click_on_select: Signal<bool>,
    /// `select`.
    #[prop(optional, into)]
    on_select: Option<Callback<usize>>,
    /// `escape`.
    #[prop(optional, into)]
    on_escape: Option<Callback<()>>,
    /// `highlight`.
    #[prop(optional, into)]
    on_highlight: Option<Callback<i32>>,
    /// `opened`.
    #[prop(optional, into)]
    on_opened: Option<Callback<bool>>,
    /// Input slot.
    #[prop(optional)]
    input: Option<ViewFn>,
    /// List slot.
    #[prop(optional)]
    list: Option<ViewFn>,
    /// Empty list slot (`emptyListItem`).
    #[prop(optional)]
    empty_list_item: Option<ViewFn>,
) -> impl IntoView {
    validate_field(&label, show_label, size, &description);
    if input.is_none() {
        leptos::logging::warn!("DtCombobox requires an input slot");
    }
    let list_id = if list_id.is_empty() {
        unique_id("dt-combobox-list")
    } else {
        list_id
    };
    let has_empty_slot = empty_list_item.is_some();
    let list_id_stored = StoredValue::new(list_id.clone());
    let label_stored = StoredValue::new(label.clone());
    let message_stored = StoredValue::new(empty_state_message.clone());
    let empty_class_stored = StoredValue::new(empty_state_class.clone());
    let on_select = StoredValue::new(on_select);
    let on_escape = StoredValue::new(on_escape);
    let on_highlight = StoredValue::new(on_highlight);
    let on_opened = StoredValue::new(on_opened);
    let on_beginning = StoredValue::new(on_beginning_of_list);
    let on_end = StoredValue::new(on_end_of_list);
    let highlight = RwSignal::new(-1);
    let root_ref = NodeRef::<leptos::html::Div>::new();
    let list_wrapper_ref = NodeRef::<leptos::html::Div>::new();
    let saw_show = StoredValue::new(false);
    let saw_loading = StoredValue::new(false);
    let warned_empty = StoredValue::new(false);
    let generation = StoredValue::new(0_u32);
    let listen_generation = StoredValue::new(0_u32);
    let outside_listeners: StoredValue<
        Option<(String, js_sys::Function, js_sys::Function, js_sys::Function)>,
    > = StoredValue::new(None);

    let detach_outside = move || {
        let Some((id, on_move, on_leave, on_focus)) = outside_listeners.get_value() else {
            return;
        };
        if let Some(list) = find_list(&id) {
            let _ = list.remove_event_listener_with_callback("mousemove", &on_move);
            let _ = list.remove_event_listener_with_callback("mouseleave", &on_leave);
            let _ = list.remove_event_listener_with_callback("focusout", &on_focus);
        }
        outside_listeners.set_value(None);
    };

    let apply = move |index: i32, scroll: bool| {
        set_highlight_index(
            highlight,
            &list_id_stored.get_value(),
            index,
            scroll,
            loading.get_untracked(),
            &on_highlight.get_value(),
        );
    };

    let schedule_initial = move || {
        let token = generation.get_value().wrapping_add(1);
        generation.set_value(token);
        let loading_now = loading.get_untracked();
        set_timeout(
            move || {
                if generation.get_value() != token || !show_list.get_untracked() {
                    return;
                }
                if find_list(&list_id_stored.get_value()).is_none() {
                    return;
                }
                apply(if loading_now { -1 } else { 0 }, false);
            },
            Duration::ZERO,
        );
    };

    Effect::new(move |_| {
        let invalid = empty_list.get() && message_stored.get_value().is_empty() && !has_empty_slot;
        if invalid && !warned_empty.get_value() {
            warned_empty.set_value(true);
            leptos::logging::warn!(
                "DtCombobox empty_list needs empty_state_message or the empty_list_item slot"
            );
        }
        if !invalid {
            warned_empty.set_value(false);
        }
    });

    Effect::new(move |_| {
        let shown = show_list.get();
        let index = highlight.get();
        let loading_now = loading.get();
        let Some(root) = root_ref.get() else {
            return;
        };
        let label = label_stored.get_value();
        let list_id = list_id_stored.get_value();
        if let Ok(Some(input)) = root.query_selector("input") {
            let active = active_descendant(&list_id, shown, loading_now, index);
            stamp_input(&input, shown, &list_id, &label, active.as_deref());
        }
        if !shown {
            return;
        }
        if let Some(list) = find_list(&list_id) {
            stamp_list_element(&list, &list_id, &label);
            return;
        }
        let Some(wrapper) = list_wrapper_ref.get() else {
            return;
        };
        let Some(child) = wrapper.first_element_child() else {
            return;
        };
        stamp_list_element(&child, &list_id, &label);
    });

    Effect::new(move |_| {
        let shown = show_list.get();
        if !saw_show.get_value() {
            saw_show.set_value(true);
            return;
        }
        if let Some(callback) = on_opened.get_value() {
            callback.run(shown);
        }
        if shown {
            schedule_initial();
        }
    });

    Effect::new(move |_| {
        let _loading_now = loading.get();
        if !saw_loading.get_value() {
            saw_loading.set_value(true);
            return;
        }
        if show_list.get_untracked() {
            schedule_initial();
        }
    });

    Effect::new(move |_| {
        let shown = show_list.get();
        detach_outside();
        if !list_rendered_outside || !shown {
            return;
        }
        let token = listen_generation.get_value().wrapping_add(1);
        listen_generation.set_value(token);
        on_cleanup(move || detach_outside());
        set_timeout(
            move || {
                if listen_generation.get_value() != token || !show_list.get_untracked() {
                    return;
                }
                let list_id = list_id_stored.get_value();
                let Some(list) = find_list(&list_id) else {
                    return;
                };
                let on_move = Closure::<dyn FnMut(MouseEvent)>::new(move |event: MouseEvent| {
                    if loading.get_untracked() {
                        return;
                    }
                    let Some(target) = event.target() else {
                        return;
                    };
                    let Some(element) = target.dyn_ref::<Element>() else {
                        return;
                    };
                    let Some(item) = element.closest("li").ok().flatten() else {
                        return;
                    };
                    let id = item.id();
                    if id.is_empty() {
                        return;
                    }
                    let current = highlight.get_untracked();
                    if let Some(list) = find_list(&list_id_stored.get_value()) {
                        let items = role_items(&list, "[role=option]");
                        if let Some(active) = usize::try_from(current)
                            .ok()
                            .and_then(|index| items.get(index))
                        {
                            if active.id() == id {
                                return;
                            }
                        }
                        if let Some(index) = items.iter().position(|element| element.id() == id) {
                            apply(index as i32, false);
                        }
                    }
                });
                let on_leave = Closure::<dyn FnMut(MouseEvent)>::new(move |_: MouseEvent| {
                    if find_list(&list_id_stored.get_value()).is_some() {
                        apply(-1, false);
                    }
                });
                let on_focus = Closure::<dyn FnMut(web_sys::FocusEvent)>::new(
                    move |_: web_sys::FocusEvent| {
                        if find_list(&list_id_stored.get_value()).is_some() {
                            apply(-1, false);
                        }
                    },
                );
                let move_fn = on_move.as_ref().unchecked_ref::<js_sys::Function>().clone();
                let leave_fn = on_leave
                    .as_ref()
                    .unchecked_ref::<js_sys::Function>()
                    .clone();
                let focus_fn = on_focus
                    .as_ref()
                    .unchecked_ref::<js_sys::Function>()
                    .clone();
                if list
                    .add_event_listener_with_callback("mousemove", move_fn.unchecked_ref())
                    .is_err()
                {
                    return;
                }
                let _ =
                    list.add_event_listener_with_callback("mouseleave", leave_fn.unchecked_ref());
                let _ = list.add_event_listener_with_callback("focusout", focus_fn.unchecked_ref());
                on_move.forget();
                on_leave.forget();
                on_focus.forget();
                outside_listeners.set_value(Some((list_id, move_fn, leave_fn, focus_fn)));
            },
            Duration::ZERO,
        );
    });

    let on_key = move |event: KeyboardEvent| {
        let key = event.key();
        let shown = show_list.get_untracked() && find_list(&list_id_stored.get_value()).is_some();
        match key.as_str() {
            "Escape" => {
                event.stop_propagation();
                if !shown {
                    return;
                }
                if let Some(callback) = on_escape.get_value() {
                    callback.run(());
                }
            }
            "ArrowDown" | "ArrowUp" | "Home" | "End" => {
                event.prevent_default();
                event.stop_propagation();
                if !shown {
                    return;
                }
                let list_id = list_id_stored.get_value();
                let len = find_list(&list_id)
                    .map(|list| role_items(&list, "[role=option]").len())
                    .unwrap_or(0);
                let index = highlight.get_untracked();
                match key.as_str() {
                    "ArrowDown" => match combobox_move_down(len, index) {
                        Some(next) => apply(next, true),
                        None => {
                            if let Some(callback) = on_end.get_value() {
                                callback.run(());
                            } else {
                                apply(combobox_jump_start(), true);
                            }
                        }
                    },
                    "ArrowUp" => match combobox_move_up(index) {
                        Some(next) => apply(next, true),
                        None => {
                            if let Some(callback) = on_beginning.get_value() {
                                callback.run(());
                            } else {
                                apply(combobox_jump_end(len), true);
                            }
                        }
                    },
                    "Home" => apply(combobox_jump_start(), true),
                    _ => apply(combobox_jump_end(len), true),
                }
            }
            "Enter" => {
                if event.shift_key() || event.ctrl_key() || event.alt_key() || event.meta_key() {
                    return;
                }
                if !shown || loading.get_untracked() || empty_list.get_untracked() {
                    return;
                }
                let index = highlight.get_untracked();
                if index < 0 {
                    return;
                }
                if let Some(callback) = on_select.get_value() {
                    callback.run(index as usize);
                }
                if click_on_select.get_untracked() {
                    if let Some(list) = find_list(&list_id_stored.get_value()) {
                        let items = role_items(&list, "[role=option]");
                        if let Some(element) = usize::try_from(index)
                            .ok()
                            .and_then(|index| items.get(index))
                        {
                            if let Ok(element) = element.clone().dyn_into::<web_sys::HtmlElement>()
                            {
                                element.click();
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    };

    let on_mouse = move |event: MouseEvent| {
        if loading.get_untracked() || !show_list.get_untracked() {
            return;
        }
        let Some(target) = event.target() else {
            return;
        };
        let Some(element) = target.dyn_ref::<Element>() else {
            return;
        };
        let Some(item) = element.closest("li").ok().flatten() else {
            return;
        };
        let id = item.id();
        if id.is_empty() {
            return;
        }
        let list_id = list_id_stored.get_value();
        let Some(list) = find_list(&list_id) else {
            return;
        };
        let items = role_items(&list, "[role=option]");
        let current = highlight.get_untracked();
        if let Some(active) = usize::try_from(current)
            .ok()
            .and_then(|index| items.get(index))
        {
            if active.id() == id {
                return;
            }
        }
        if let Some(index) = items.iter().position(|element| element.id() == id) {
            apply(index as i32, false);
        }
    };

    let clear_highlight = move || {
        if show_list.get_untracked() && find_list(&list_id_stored.get_value()).is_some() {
            apply(-1, false);
        }
    };

    let list_slot = StoredValue::new(list);
    let empty_slot = StoredValue::new(empty_list_item);
    let input_slot = input;

    view! {
        <div node_ref=root_ref on:keydown=on_key>
            <div data-qa="dt-combobox-input-wrapper">
                {input_slot.map(|slot| slot.run())}
            </div>
            <Show when=move || show_list.get()>
                <div
                    node_ref=list_wrapper_ref
                    data-qa="dt-combobox-list-wrapper"
                    on:mousemove=on_mouse
                    on:mouseleave=move |_| clear_highlight()
                    on:focusout=move |_| clear_highlight()
                >
                    {move || {
                        let list_id = list_id_stored.get_value();
                        let label = label_stored.get_value();
                        if list_rendered_outside {
                            return list_slot.get_value().map(|slot| slot.run()).into_any();
                        }
                        if loading.get() {
                            return loading_list(list_id, label).into_any();
                        }
                        let message = message_stored.get_value();
                        let show_empty =
                            empty_list.get() && (!message.is_empty() || has_empty_slot);
                        if show_empty {
                            return empty_list_view(
                                list_id,
                                label,
                                message,
                                empty_class_stored.get_value(),
                                empty_slot.get_value(),
                            )
                            .into_any();
                        }
                        list_slot.get_value().map(|slot| slot.run()).into_any()
                    }}
                </div>
            </Show>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_tokens_match_vue() {
        for size in [
            "", "xs", "sm", "md", "lg", "xl", "100", "200", "300", "400", "500",
        ] {
            assert!(combobox_size_known(size), "{size}");
        }
        assert!(!combobox_size_known("success"));
        assert!(!combobox_size_known("600"));
    }

    #[test]
    fn arrow_keys_match_the_list_mixin() {
        assert_eq!(combobox_move_down(3, -1), Some(0));
        assert_eq!(combobox_move_down(3, 1), Some(2));
        assert_eq!(combobox_move_down(3, 2), None);
        assert_eq!(combobox_move_down(0, -1), None);
        assert_eq!(combobox_move_up(2), Some(1));
        assert_eq!(combobox_move_up(0), None);
        assert_eq!(combobox_move_up(-1), None);
        assert_eq!(combobox_jump_start(), 0);
        assert_eq!(combobox_jump_end(3), 2);
        assert_eq!(combobox_jump_end(0), -1);
    }

    #[test]
    fn empty_option_class_matches_vue_order() {
        assert_eq!(
            empty_option_class("extra"),
            "d-list-item d-list-item--static d-combobox__empty-list-item extra"
        );
        assert_eq!(
            empty_option_class(""),
            "d-list-item d-list-item--static d-combobox__empty-list-item"
        );
    }
}
