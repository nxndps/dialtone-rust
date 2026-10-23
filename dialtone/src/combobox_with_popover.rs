use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent};

use super::combobox::DtCombobox;
use super::input::DtInput;
use super::list_nav::{dropdown_padding_class, event_inside, join_class};
use super::popover::DtPopover;
use super::unique_id;

/// Dialtone Vue `DtComboboxWithPopover`.
///
/// `DtRecipeComboboxWithPopover` is the same component. The list renders in a
/// [`DtPopover`] (`placement` `bottom-start`, offset 4, `padding` `none`,
/// `modal` false). `show_list` unset opens on input focus when
/// `has_suggestion_list` is true (the default) and stays open after a select.
/// Set it false to open only while `value` is non-empty and to close after select.
/// A set `show_list` signal is controlled through `on_open_change`.
///
/// The `input` slot replaces the default [`DtInput`]. [`DtCombobox`] writes the
/// combobox ARIA attributes onto the first `input` in the anchor.
#[component]
pub fn DtComboboxWithPopover(
    label: String,
    #[prop(optional, default = true)] label_visible: bool,
    #[prop(optional, into)] description: String,
    #[prop(optional, default = "")] size: &'static str,
    #[prop(optional, into)] placeholder: String,
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_input: Callback<String>,
    #[prop(optional)] show_list: Option<Signal<bool>>,
    #[prop(optional, into)] on_open_change: Option<Callback<bool>>,
    #[prop(optional, default = true)] has_suggestion_list: bool,
    #[prop(optional, default = true)] open_with_arrow_keys: bool,
    /// Popover distance in px. Vue offset `[0, 4]`.
    #[prop(optional, default = 4.0)]
    offset: f64,
    /// `body` (default) or `parent` (`appendTo`).
    #[prop(optional, default = "body")]
    append_to: &'static str,
    /// `anchor` matches the popover to the input (`contentWidth`).
    #[prop(optional, default = "")]
    content_width: &'static str,
    #[prop(optional, into)] max_height: String,
    #[prop(optional, into)] max_width: String,
    #[prop(optional, into)] dialog_class: String,
    /// `none`, `small` (default), or `large`. Same map as `DtDropdown`.
    #[prop(optional, default = "small")]
    list_padding: &'static str,
    /// Extra class on the list (`listClass`).
    #[prop(optional, into)]
    list_class: String,
    #[prop(optional)] loading: bool,
    #[prop(optional)] empty_list: bool,
    #[prop(optional, into)] empty_message: String,
    #[prop(optional, into)] empty_state_class: String,
    #[prop(optional)] click_on_select: bool,
    #[prop(optional, into)] on_select: Option<Callback<usize>>,
    #[prop(optional, into)] on_escape: Option<Callback<()>>,
    #[prop(optional, into)] on_highlight: Option<Callback<i32>>,
    #[prop(optional, into)] on_opened: Option<Callback<bool>>,
    /// Increment to close an uncontrolled list (`closeComboboxList`).
    #[prop(optional)]
    close_request: Option<Signal<u32>>,
    #[prop(optional)] input: Option<ViewFn>,
    #[prop(optional)] header: Option<ViewFn>,
    #[prop(optional)] footer: Option<ViewFn>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let controlled = show_list.is_some();
    let internal = RwSignal::new(false);
    let open_signal = show_list.unwrap_or_else(|| Signal::from(internal));
    let user_change = StoredValue::new(on_open_change);
    let set_open = move |next: bool| {
        if !controlled {
            internal.set(next);
        }
        if let Some(callback) = user_change.get_value() {
            callback.run(next);
        }
    };
    let anchor_id = unique_id("dt-combobox-anchor");
    let list_id = unique_id("dt-combobox-list");
    let list_id_stored = StoredValue::new(list_id.clone());
    let anchor_id_stored = StoredValue::new(anchor_id.clone());
    let list_class = join_class([
        "d-recipe-combobox-with-popover__list",
        dropdown_padding_class(list_padding),
        list_class.as_str(),
    ]);

    Effect::new(move |_| {
        if controlled || has_suggestion_list {
            return;
        }
        set_open(!value.get().is_empty());
    });

    let close_seen = StoredValue::new(false);
    Effect::new(move |_| {
        let Some(close_request) = close_request else {
            return;
        };
        let _tick = close_request.get();
        if !close_seen.get_value() {
            close_seen.set_value(true);
            return;
        }
        set_open(false);
    });

    Effect::new(move |_| {
        if !open_signal.get() || controlled {
            return;
        }
        let closure = Closure::<dyn FnMut(MouseEvent)>::new(move |event: MouseEvent| {
            let document = web_sys::window().and_then(|window| window.document());
            let Some(document) = document else {
                return;
            };
            if let Some(anchor) = document.get_element_by_id(&anchor_id_stored.get_value()) {
                if event_inside(&event, anchor.as_ref()) {
                    return;
                }
            }
            if let Some(list) = document.get_element_by_id(&list_id_stored.get_value()) {
                if event_inside(&event, list.as_ref()) {
                    return;
                }
            }
            set_open(false);
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
        closure.forget();
        on_cleanup(move || {
            if let Some(document) = web_sys::window().and_then(|window| window.document()) {
                let _ = document
                    .remove_event_listener_with_callback("mousedown", function.unchecked_ref());
            }
        });
    });

    let on_highlight_stored = StoredValue::new(on_highlight);
    let on_escape_stored = StoredValue::new(on_escape);
    let on_select_stored = StoredValue::new(on_select);
    let on_input_stored = StoredValue::new(on_input);
    let user_input = StoredValue::new(input);
    let label_stored = StoredValue::new(label.clone());
    let description_stored = StoredValue::new(description.clone());
    let placeholder_stored = StoredValue::new(placeholder.clone());
    let anchor_for_input = anchor_id.clone();
    let label_for_list = label.clone();
    let combo_input = ViewFn::from(move || {
        let content = if let Some(slot) = user_input.get_value() {
            slot.run().into_any()
        } else {
            let on_input = on_input_stored.get_value();
            let field_label = label_stored.get_value();
            let field_description = description_stored.get_value();
            let field_placeholder = placeholder_stored.get_value();
            view! {
                <DtInput
                    label=field_label
                    show_label=label_visible
                    size=size
                    description=field_description
                    placeholder=field_placeholder
                    value=value
                    on_input=move |next| on_input.run(next)
                />
            }
            .into_any()
        };
        let anchor_id = anchor_for_input.clone();
        view! {
            <div
                id=anchor_id
                class="w-full"
                on:focusin=move |event: FocusEvent| {
                    if controlled || !has_suggestion_list {
                        return;
                    }
                    let Some(target) = event.target() else {
                        return;
                    };
                    if target.dyn_ref::<web_sys::HtmlInputElement>().is_some() {
                        set_open(true);
                    }
                }
                on:keydown=move |event: KeyboardEvent| {
                    let key = event.key();
                    if key != "ArrowUp" && key != "ArrowDown" {
                        return;
                    }
                    if controlled || open_signal.get_untracked() || !open_with_arrow_keys {
                        return;
                    }
                    set_open(true);
                }
            >
                {content}
            </div>
        }
    });

    view! {
        <DtCombobox
            label=label
            show_label=label_visible
            description=description
            size=size
            show_list=open_signal
            loading=loading
            empty_list=empty_list
            empty_state_message=empty_message
            empty_state_class=empty_state_class
            click_on_select=click_on_select
            list_rendered_outside=true
            list_id=list_id
            on_select=Callback::new(move |index| {
                if let Some(callback) = on_select_stored.get_value() {
                    callback.run(index);
                }
                if !has_suggestion_list {
                    set_open(false);
                }
            })
            on_escape=Callback::new(move |_| {
                if let Some(callback) = on_escape_stored.get_value() {
                    callback.run(());
                }
                set_open(false);
            })
            on_highlight=Callback::new(move |index| {
                if let Some(callback) = on_highlight_stored.get_value() {
                    callback.run(index);
                }
            })
            nostrip:on_opened=on_opened
            input=combo_input
        />
        <DtPopover
            open=open_signal
            on_open_change=Callback::new(move |next| set_open(next))
            placement="bottom-start"
            offset=offset
            padding="none"
            role="listbox"
            append_to=append_to
            modal=false
            close_on_outside=false
            initial_focus="none"
            content_width=content_width
            max_height=max_height
            max_width=max_width
            dialog_class=dialog_class
            external_anchor_id=anchor_id_stored.get_value()
        >
            <div
                id=list_id_stored.get_value()
                role="listbox"
                class=list_class.clone()
                aria-label=label_for_list.clone()
            >
                {header.as_ref().map(|slot| view! { <div>{slot.run()}</div> })}
                {children.clone().map(|children| children())}
                {footer.as_ref().map(|slot| view! { <div>{slot.run()}</div> })}
            </div>
        </DtPopover>
    }
}

/// Dialtone export alias `DtRecipeComboboxWithPopover`.
pub use DtComboboxWithPopover as DtRecipeComboboxWithPopover;
