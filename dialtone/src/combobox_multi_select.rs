use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, HtmlInputElement, KeyboardEvent};

use super::chip::DtChip;
use super::combobox_with_popover::DtComboboxWithPopover;
use super::list_nav::join_class;
use super::validation::{validation_messages, DtValidationMessage};

/// Chip size for a multi-select `size` (`CHIP_SIZES` in `dialtone/v9.190.0`).
pub fn multi_select_chip_size(size: &str) -> &'static str {
    match size {
        "md" => "sm",
        _ => "xs",
    }
}

fn chip_top_adjust(size: &str) -> f64 {
    match size {
        "xs" => 1.4,
        "sm" => -0.1,
        _ => -0.3,
    }
}

fn collapsed_chip_class(size: &str) -> &'static str {
    match size {
        "xs" => "d-recipe-combobox-multi-select__chip-wrapper-xs--collapsed",
        "sm" => "d-recipe-combobox-multi-select__chip-wrapper-sm--collapsed",
        _ => "d-recipe-combobox-multi-select__chip-wrapper-md--collapsed",
    }
}

fn input_size_class(size: &str) -> &'static str {
    match size {
        "xs" => "d-input--xs",
        "sm" => "d-input--sm",
        _ => "d-input--md",
    }
}

/// Dialtone Vue `DtComboboxMultiSelect`.
///
/// `DtRecipeComboboxMultiSelect` is the same component. `selected_items` is the
/// chip labels. `on_select` receives the highlighted list index; this component
/// clears the input and does not push the item. `on_remove` receives the chip
/// label. `max_selected` of 0 is unlimited. `show_list` unset uses the
/// suggestion-list behavior of [`DtComboboxWithPopover`]. Incrementing is not
/// required: chip arrow keys call `close_request` on that popover.
///
/// Left/right move between chips. Left or backspace at the start of the input
/// focuses the last chip. Backspace or delete on a chip removes it.
#[component]
pub fn DtComboboxMultiSelect(
    label: String,
    #[prop(optional, default = true)] label_visible: bool,
    #[prop(optional, into)] description: String,
    /// Empty while a chip is selected. Default `Select one or start typing`.
    #[prop(optional, default = "Select one or start typing")]
    placeholder: &'static str,
    /// `xs`, `sm`, or `md` (default).
    #[prop(optional, default = "md")]
    size: &'static str,
    #[prop(into)] selected_items: Signal<Vec<String>>,
    #[prop(optional)] show_list: Option<Signal<bool>>,
    #[prop(optional, into)] on_open_change: Option<Callback<bool>>,
    #[prop(optional, default = true)] has_suggestion_list: bool,
    #[prop(optional, default = 0)] max_selected: usize,
    #[prop(optional, into)] max_selected_message: Vec<DtValidationMessage>,
    #[prop(optional)] loading: bool,
    #[prop(optional, default = "loading...")] loading_message: &'static str,
    /// CSS max-height of the list (`listMaxHeight`). Default `300px`.
    #[prop(optional, default = "300px")]
    list_max_height: &'static str,
    #[prop(optional, into)] list_max_width: String,
    #[prop(optional, default = "body")] append_to: &'static str,
    #[prop(optional, into)] dialog_class: String,
    /// Collapse chips to one row after blur (`collapseOnFocusOut`).
    #[prop(optional)]
    collapse_on_focus_out: bool,
    /// CSS max-width of one chip (`chipMaxWidth`). Truncates when set.
    #[prop(optional, into)]
    chip_max_width: String,
    #[prop(optional, into)] input_class: String,
    #[prop(optional, into)] input_wrapper_class: String,
    #[prop(optional)] disabled: bool,
    /// Px kept free on the right before the caret wraps (`reservedRightSpace`).
    #[prop(optional, default = 64.0)]
    reserved_right_space: f64,
    #[prop(optional, into)] on_input: Option<Callback<String>>,
    #[prop(optional, into)] on_select: Option<Callback<usize>>,
    #[prop(optional, into)] on_remove: Option<Callback<String>>,
    #[prop(optional, into)] on_max_selected: Option<Callback<()>>,
    #[prop(optional, into)] on_highlight: Option<Callback<i32>>,
    #[prop(optional, into)] on_escape: Option<Callback<()>>,
    #[prop(optional, into)] on_enter: Option<Callback<()>>,
    #[prop(optional)] header: Option<ViewFn>,
    #[prop(optional)] footer: Option<ViewFn>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let value = RwSignal::new(String::new());
    let input_focused = RwSignal::new(false);
    let close_request = RwSignal::new(0u32);
    let chip_style = RwSignal::new(String::new());
    let input_style = RwSignal::new(String::new());
    let root_ref = NodeRef::<leptos::html::Span>::new();
    let input_ref = NodeRef::<leptos::html::Input>::new();
    let user_open = StoredValue::new(on_open_change);
    let on_input = StoredValue::new(on_input);
    let on_remove = StoredValue::new(on_remove);
    let on_max_selected = StoredValue::new(on_max_selected);
    let on_enter = StoredValue::new(on_enter);
    let chip_max_width = StoredValue::new(chip_max_width);
    let input_class_extra = StoredValue::new(input_class);
    let at_max = StoredValue::new(false);

    let request_close = move || {
        if show_list.is_none() {
            close_request.set(close_request.get_untracked().wrapping_add(1));
        } else if let Some(callback) = user_open.get_value() {
            callback.run(false);
        }
    };
    let request_open = move || {
        if show_list.is_none() {
            return;
        }
        if let Some(callback) = user_open.get_value() {
            callback.run(true);
        }
    };

    Effect::new(move |_| {
        let count = selected_items.get().len();
        let reached = max_selected > 0 && count >= max_selected;
        if reached && !at_max.get_value() {
            if let Some(callback) = on_max_selected.get_value() {
                callback.run(());
            }
        }
        at_max.set_value(reached);
    });

    Effect::new(move |_| {
        let _items = selected_items.get();
        let focused = input_focused.get();
        request_animation_frame(move || {
            let Some(root) = root_ref.get_untracked() else {
                return;
            };
            let root: HtmlElement = root.unchecked_into();
            let Ok(Some(input)) = root.query_selector("input") else {
                return;
            };
            let Ok(input) = input.dyn_into::<HtmlElement>() else {
                return;
            };
            let Ok(Some(chips)) =
                root.query_selector(".d-recipe-combobox-multi-select__chip-wrapper")
            else {
                return;
            };
            let Ok(chips) = chips.dyn_into::<HtmlElement>() else {
                return;
            };
            let top =
                input.get_bounding_client_rect().top() - root.get_bounding_client_rect().top();
            chip_style.set(format!("top:{}px;", top - chip_top_adjust(size)));
            let Ok(nodes) = chips.query_selector_all(".d-recipe-combobox-multi-select__chip")
            else {
                return;
            };
            if nodes.length() == 0 || (collapse_on_focus_out && !focused) {
                input_style.set(String::new());
                return;
            }
            let Some(last) = nodes.item(nodes.length() - 1) else {
                return;
            };
            let Ok(last) = last.dyn_into::<HtmlElement>() else {
                return;
            };
            let left = f64::from(last.offset_left() + last.offset_width());
            let space = input.get_bounding_client_rect().width() - left;
            if space > reserved_right_space {
                input_style.set(format!("padding-left:{left}px;"));
            } else {
                let pad_top = last.offset_top() + 2;
                input_style.set(format!("padding-left:4px;padding-top:{pad_top}px;"));
            }
        });
    });

    let remove_at = move |index: usize| {
        if disabled {
            return;
        }
        let items = selected_items.get_untracked();
        if let Some(item) = items.get(index) {
            if let Some(callback) = on_remove.get_value() {
                callback.run(item.clone());
            }
        }
        if let Some(input) = input_ref.get() {
            let input: HtmlElement = input.unchecked_into();
            let _ = input.focus();
        }
    };

    let focus_chip = move |index: usize| {
        let Some(root) = root_ref.get() else {
            return;
        };
        let root: HtmlElement = root.unchecked_into();
        let Ok(nodes) = root.query_selector_all("[data-chip-index] .d-chip__label") else {
            return;
        };
        if let Some(node) = nodes.item(index as u32) {
            if let Ok(element) = node.dyn_into::<HtmlElement>() {
                let _ = element.focus();
            }
        }
    };

    let label_for_input = label.clone();
    let description_for_input = description.clone();
    let input_slot = ViewFn::from(move || {
        let items = selected_items.get();
        let focused = input_focused.get();
        let collapsed = collapse_on_focus_out && !focused;
        let wrapper_class = join_class([
            "d-recipe-combobox-multi-select__chip-wrapper",
            collapsed
                .then_some(collapsed_chip_class(size))
                .unwrap_or(""),
        ]);
        let hide_text = collapsed;
        let field_class = join_class([
            "d-input__input",
            "d-input",
            input_size_class(size),
            "d-recipe-combobox-multi-select__input",
            input_class_extra.get_value().as_str(),
            hide_text
                .then_some("d-recipe-combobox-multi-select__input--hidden")
                .unwrap_or(""),
        ]);
        let wrapper = join_class(["d-input__wrapper", input_wrapper_class.as_str()]);
        let placeholder = if items.is_empty() { placeholder } else { "" };
        let max_width = chip_max_width.get_value();
        view! {
            <span
                node_ref=root_ref
                class="d-recipe-combobox-multi-select__input-wrapper"
                on:focusin=move |_| input_focused.set(true)
                on:focusout=move |_| input_focused.set(false)
                on:keydown=move |event: KeyboardEvent| {
                    let Some(target) = event.target() else {
                        return;
                    };
                    let Ok(target) = target.dyn_into::<Element>() else {
                        return;
                    };
                    let Ok(Some(chip)) = target.closest("[data-chip-index]") else {
                        return;
                    };
                    let Ok(index) = chip
                        .get_attribute("data-chip-index")
                        .unwrap_or_default()
                        .parse::<usize>()
                    else {
                        return;
                    };
                    let count = selected_items.get_untracked().len();
                    match event.key().as_str() {
                        "ArrowLeft" if index > 0 => {
                            event.prevent_default();
                            focus_chip(index - 1);
                            request_close();
                        }
                        "ArrowRight" if index + 1 >= count => {
                            event.prevent_default();
                            if let Some(input) = input_ref.get() {
                                let input: HtmlElement = input.unchecked_into();
                                let _ = input.focus();
                            }
                            request_open();
                        }
                        "ArrowRight" => {
                            event.prevent_default();
                            focus_chip(index + 1);
                            request_close();
                        }
                        "Backspace" | "Delete" => {
                            event.prevent_default();
                            remove_at(index);
                        }
                        _ => {}
                    }
                }
            >
                <span class=wrapper_class style=move || chip_style.get()>
                    {items.into_iter().enumerate().map(|(index, item)| {
                        let chip_class = join_class([
                            "d-recipe-combobox-multi-select__chip",
                            (!max_width.is_empty()).then_some("d-recipe-combobox-multi-select__chip--truncate").unwrap_or(""),
                        ]);
                        let chip_width = if max_width.is_empty() {
                            String::new()
                        } else {
                            format!("max-width:{max_width};")
                        };
                        let item_for_remove = item.clone();
                        view! {
                            <span data-chip-index=index.to_string() style=chip_width>
                                <DtChip
                                    class=chip_class
                                    size=multi_select_chip_size(size)
                                    interactive=true
                                    show_close=!disabled
                                    nostrip:on_close=(!disabled).then(|| Callback::new(move |_| {
                                        if let Some(callback) = on_remove.get_value() {
                                            callback.run(item_for_remove.clone());
                                        }
                                    }))
                                >
                                    {item}
                                </DtChip>
                            </span>
                        }
                    }).collect_view()}
                </span>
                <div class="d-input__root w-full">
                    <label class="d-input__label">
                        {(label_visible && !label_for_input.is_empty()).then(|| view! {
                            <div class="d-input__label-text d-label" data-qa="dt-input-label">
                                {label_for_input.clone()}
                            </div>
                        })}
                        {(!description_for_input.is_empty()).then(|| view! {
                            <div class="d-input__description d-description" data-qa="dt-input-description">
                                {description_for_input.clone()}
                            </div>
                        })}
                        <div class=wrapper>
                            <input
                                node_ref=input_ref
                                class=field_class
                                style=move || input_style.get()
                                type="text"
                                placeholder=placeholder
                                prop:disabled=disabled
                                prop:value=move || value.get()
                                aria-label=label_for_input.clone()
                                data-qa="dt-input-input"
                                on:input=move |event| {
                                    let text = event_target_value(&event);
                                    value.set(text.clone());
                                    if let Some(callback) = on_input.get_value() {
                                        callback.run(text);
                                    }
                                }
                                on:keydown=move |event: KeyboardEvent| {
                                    let key = event.key();
                                    if key == "Escape" {
                                        return;
                                    }
                                    if key == "Enter" {
                                        if let Some(callback) = on_enter.get_value() {
                                            callback.run(());
                                        }
                                    }
                                    if selected_items.get_untracked().is_empty() {
                                        return;
                                    }
                                    let Ok(input) = event.target().unwrap().dyn_into::<HtmlInputElement>() else {
                                        return;
                                    };
                                    let start = input.selection_start().ok().flatten().unwrap_or(0);
                                    let end = input.selection_end().ok().flatten().unwrap_or(0);
                                    if start != 0 || start != end {
                                        return;
                                    }
                                    if key == "Backspace" || key == "ArrowLeft" {
                                        event.prevent_default();
                                        let last = selected_items.get_untracked().len().saturating_sub(1);
                                        focus_chip(last);
                                        request_close();
                                    }
                                }
                            />
                        </div>
                    </label>
                </div>
                {validation_messages(
                    max_selected_message.clone(),
                    max_selected > 0 && selected_items.get().len() >= max_selected,
                    "",
                    "",
                )}
            </span>
        }
    });

    let list_body = ViewFn::from(move || {
        let children = children.clone();
        view! {
            <div class="d-recipe-combobox-multi-select__list" on:mousedown=|event| event.prevent_default()>
                {if loading {
                    view! {
                        <div class="d-recipe-combobox-multi-select__list--loading">{loading_message}</div>
                    }
                    .into_any()
                } else {
                    children.map(|children| children()).into_any()
                }}
            </div>
        }
    });

    view! {
        <DtComboboxWithPopover
            label=label
            label_visible=false
            value=value
            on_input=Callback::new(move |text| value.set(text))
            nostrip:show_list=show_list
            nostrip:on_open_change=user_open.get_value()
            has_suggestion_list=has_suggestion_list
            offset=4.0
            append_to=append_to
            content_width="anchor"
            max_height=list_max_height
            max_width=list_max_width
            dialog_class=dialog_class
            close_request=Signal::from(close_request)
            on_select=Callback::new(move |index| {
                if loading {
                    return;
                }
                value.set(String::new());
                if let Some(on_select) = on_select {
                    on_select.run(index);
                }
            })
            nostrip:on_highlight=on_highlight
            nostrip:on_escape=on_escape
            nostrip:header=header
            nostrip:footer=footer
            input=input_slot
        >
            {list_body.run()}
        </DtComboboxWithPopover>
    }
}

/// Dialtone export alias `DtRecipeComboboxMultiSelect`.
pub use DtComboboxMultiSelect as DtRecipeComboboxMultiSelect;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chip_sizes_follow_the_vue_map() {
        assert_eq!(multi_select_chip_size("xs"), "xs");
        assert_eq!(multi_select_chip_size("sm"), "xs");
        assert_eq!(multi_select_chip_size("md"), "sm");
    }

    #[test]
    fn chip_top_offsets_match_dlt_3409() {
        assert_eq!(chip_top_adjust("xs"), 1.4);
        assert_eq!(chip_top_adjust("sm"), -0.1);
        assert_eq!(chip_top_adjust("md"), -0.3);
    }
}
