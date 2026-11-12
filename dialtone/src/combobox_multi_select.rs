use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, HtmlInputElement, KeyboardEvent};

use super::chip::DtChip;
use super::combobox_with_popover::DtComboboxWithPopover;
use super::input::{input_description_classes, input_label_classes, input_size_class};
use super::list_nav::join_class;
use super::radio::{radio_message_state, radio_messages_view};
use super::unique_id;
use super::validation::DtValidationMessage;

/// Chip size for a multi-select `size` (`CHIP_SIZES` at `dialtone/v10.7.0`).
///
/// `100` and `200` are `xs`. `300` is `sm`. `xs` and `sm` stay `xs`. `md` is
/// `sm`. Anything else warns and uses `xs`.
pub fn multi_select_chip_size(size: &str) -> &'static str {
    match size {
        "100" | "200" | "xs" | "sm" => "xs",
        "300" | "md" => "sm",
        other => {
            leptos::logging::warn!(
                "DtComboboxMultiSelect size `{other}` is not 100, 200, 300, xs, sm, or md"
            );
            "xs"
        }
    }
}

/// `COMPONENT_SIZES[size] || size`. Numeric tokens become the t-shirt name.
/// `xs`, `sm`, and `md` stay literal.
pub fn multi_select_wrapper_size(size: &str) -> String {
    match size {
        "100" => "xs".to_string(),
        "200" => "sm".to_string(),
        "300" => "md".to_string(),
        "400" => "lg".to_string(),
        "500" => "xl".to_string(),
        other => other.to_string(),
    }
}

pub(crate) fn chip_wrapper_class(size: &str, collapsed: bool) -> String {
    let token = multi_select_wrapper_size(size);
    let sized = format!("d-combobox-multi-select__chip-wrapper--{token}");
    let collapsed_class = if collapsed {
        format!("d-combobox-multi-select__chip-wrapper-{token}--collapsed")
    } else {
        String::new()
    };
    join_class([
        "d-combobox-multi-select__chip-wrapper",
        sized.as_str(),
        collapsed_class.as_str(),
    ])
}

/// Centers the first chip row on the input's first line.
///
/// `top = inputTop + (inputRowHeight - chipHeight) / 2 - chipOffsetInWrapper`.
pub fn chip_row_top(
    input_top: f64,
    input_row_height: f64,
    chip_height: f64,
    chip_offset_in_wrapper: f64,
) -> f64 {
    input_top + (input_row_height - chip_height) / 2.0 - chip_offset_in_wrapper
}

/// Distance from the first chip row to the caret row.
///
/// `None` keeps the caret on the first row and clears `padding-top`.
pub fn caret_row_top(
    row_top: f64,
    chip_height: f64,
    row_gap: f64,
    has_space: bool,
    is_wrapped: bool,
) -> Option<f64> {
    if has_space && !is_wrapped {
        return None;
    }
    if has_space {
        return Some(row_top);
    }
    Some(row_top + chip_height + row_gap)
}

fn parse_px(value: &str) -> f64 {
    value
        .trim()
        .trim_end_matches("px")
        .parse::<f64>()
        .unwrap_or(0.0)
}

fn computed_px(element: &HtmlElement, property: &str) -> f64 {
    web_sys::window()
        .and_then(|window| window.get_computed_style(element).ok().flatten())
        .and_then(|style| style.get_property_value(property).ok())
        .map(|value| parse_px(&value))
        .unwrap_or(0.0)
}

fn full_width(element: &HtmlElement) -> f64 {
    f64::from(element.offset_width())
        + computed_px(element, "margin-left")
        + computed_px(element, "margin-right")
}

fn clear_prop(style: &web_sys::CssStyleDeclaration, name: &str) {
    let _ = style.set_property(name, "");
}

struct LaidOut {
    chip_style: String,
    input_style: String,
    measured_height: Option<f64>,
}

fn input_style_string(
    inline_start: Option<f64>,
    padding_top: Option<f64>,
    min_height: Option<f64>,
    height_auto: bool,
    min_width: Option<f64>,
) -> String {
    let mut style = String::new();
    if let Some(value) = inline_start {
        style.push_str(&format!("padding-inline-start:{value}px;"));
    }
    if let Some(value) = padding_top {
        style.push_str(&format!("padding-top:{value}px;"));
    }
    if let Some(value) = min_height {
        style.push_str(&format!("min-height:{value}px;"));
    }
    if height_auto {
        style.push_str("height:auto;");
    }
    if let Some(value) = min_width {
        style.push_str(&format!("min-width:{value}px;"));
    }
    style
}

/// Vue `setInitialInputHeight`, `setInputPadding`, `setChipsTopPosition`, and
/// `setInputMinWidth`. `xs` skips the min-height floor. Other sizes keep it.
fn layout_chips(
    root: &HtmlElement,
    size: &str,
    collapsed: bool,
    reserved_right_space: f64,
    initial_height: f64,
) -> LaidOut {
    let empty = LaidOut {
        chip_style: String::new(),
        input_style: String::new(),
        measured_height: None,
    };
    let Ok(Some(input_node)) = root.query_selector("input") else {
        return empty;
    };
    let Ok(input) = input_node.dyn_into::<HtmlElement>() else {
        return empty;
    };
    let Ok(Some(chips_node)) = root.query_selector(".d-combobox-multi-select__chip-wrapper") else {
        return empty;
    };
    let Ok(chips) = chips_node.dyn_into::<HtmlElement>() else {
        return empty;
    };

    let css = input.style();
    let mut height = initial_height;
    let mut measured_height = None;
    if height <= 0.0 {
        clear_prop(&css, "min-height");
        clear_prop(&css, "height");
        let next = input.get_bounding_client_rect().height();
        if next > 0.0 {
            height = next;
            measured_height = Some(next);
        }
    }
    let floor = (height > 0.0 && size != "xs").then_some(height);
    clear_prop(&css, "padding-inline-start");
    clear_prop(&css, "padding-top");
    if let Some(floor) = floor {
        let _ = css.set_property("min-height", &format!("{floor}px"));
        let _ = css.set_property("height", "auto");
    } else {
        clear_prop(&css, "min-height");
        clear_prop(&css, "height");
    }

    let Ok(nodes) = chips.query_selector_all(".d-combobox-multi-select__chip") else {
        return LaidOut {
            chip_style: String::new(),
            input_style: input_style_string(None, None, floor, floor.is_some(), None),
            measured_height,
        };
    };
    if nodes.length() == 0 {
        clear_prop(&css, "min-width");
        return LaidOut {
            chip_style: String::new(),
            input_style: input_style_string(None, None, floor, floor.is_some(), None),
            measured_height,
        };
    }
    let (Some(first_node), Some(last_node)) = (nodes.item(0), nodes.item(nodes.length() - 1))
    else {
        return empty;
    };
    let (Ok(first), Ok(last)) = (
        first_node.dyn_into::<HtmlElement>(),
        last_node.dyn_into::<HtmlElement>(),
    ) else {
        return empty;
    };

    let mut inline_start = None;
    let mut padding_top = None;
    let mut min_height = floor;
    if !collapsed {
        let input_padding_top = computed_px(&input, "padding-top");
        let left = f64::from(last.offset_left()) + full_width(&last);
        let space_left = input.get_bounding_client_rect().width() - left;
        let has_space = space_left > reserved_right_space;
        let is_wrapped = last.offset_top() > first.offset_top();
        let inline = if has_space { left } else { 4.0 };
        inline_start = Some(inline);
        let _ = css.set_property("padding-inline-start", &format!("{inline}px"));
        let row_top = last.get_bounding_client_rect().top() - first.get_bounding_client_rect().top();
        let row_gap = computed_px(&chips, "row-gap");
        let chip_height = last.get_bounding_client_rect().height();
        if let Some(caret) = caret_row_top(row_top, chip_height, row_gap, has_space, is_wrapped) {
            let top = caret + input_padding_top;
            padding_top = Some(top);
            let _ = css.set_property("padding-top", &format!("{top}px"));
            if height > 0.0 {
                let grown = caret + height;
                min_height = Some(grown);
                let _ = css.set_property("min-height", &format!("{grown}px"));
            }
        }
    }

    let input_top = input.get_bounding_client_rect().top() - root.get_bounding_client_rect().top();
    let row_height = if height > 0.0 {
        height
    } else {
        input.get_bounding_client_rect().height()
    };
    let chip_rect = first.get_bounding_client_rect();
    let chip_offset = chip_rect.top() - chips.get_bounding_client_rect().top();
    let top = chip_row_top(input_top, row_height, chip_rect.height(), chip_offset);
    let min_width = full_width(&first) + 4.0;
    let _ = css.set_property("min-width", &format!("{min_width}px"));

    LaidOut {
        chip_style: format!("top:{top}px;"),
        input_style: input_style_string(
            inline_start,
            padding_top,
            min_height,
            size != "xs" && min_height.is_some(),
            Some(min_width),
        ),
        measured_height,
    }
}

/// Dialtone Vue `DtComboboxMultiSelect` (`dialtone/v10.7.0`).
///
/// `DtRecipeComboboxMultiSelect` is the same component. `selected_items` is the
/// chip labels. `on_select` receives the highlighted list index; this component
/// clears the input and does not push the item. `on_remove` receives the chip
/// label. `max_selected` of 0 is unlimited. Validation shows when the selection
/// is longer than `max_selected`. `aria-invalid` is `"true"` only when that
/// message state is `critical`. `show_list` unset uses the suggestion-list
/// behavior of [`DtComboboxWithPopover`]. Incrementing is not required: chip
/// arrow keys call `close_request` on that popover.
///
/// The input is inlined so chip padding can be written on the element. Its
/// size class follows v10 `DtInput` (`300` and `md` add no size class).
/// Left/right move between chips. Left or backspace at the start of the input
/// focuses the last chip. Backspace or delete on a chip removes it. Enter uses
/// `event.key`. Chip arrows use `event.code`.
#[component]
pub fn DtComboboxMultiSelect(
    label: String,
    #[prop(optional, default = true)] label_visible: bool,
    #[prop(optional, into)] description: String,
    /// Empty while a chip is selected. Default `Select one or start typing`.
    #[prop(optional, default = "Select one or start typing")]
    placeholder: &'static str,
    /// `100`, `200`, or `300` (default). `xs`, `sm`, and `md` are aliases.
    #[prop(optional, default = "300")]
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
    let layout_tick = RwSignal::new(0u32);
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
    let initial_height = StoredValue::new(0.0_f64);
    let resize_observer = StoredValue::new(None::<web_sys::ResizeObserver>);
    let visibility_observer = StoredValue::new(None::<web_sys::ResizeObserver>);
    let messages_id = unique_id("dt-combobox-multi-select-messages");

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
        let reached = max_selected > 0 && count > max_selected;
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
        let _ = layout_tick.get();
        let collapsed = collapse_on_focus_out && !focused;
        request_animation_frame(move || {
            let Some(root) = root_ref.get_untracked() else {
                return;
            };
            let root: HtmlElement = root.unchecked_into();
            let laid = layout_chips(
                &root,
                size,
                collapsed,
                reserved_right_space,
                initial_height.get_value(),
            );
            if let Some(height) = laid.measured_height {
                initial_height.set_value(height);
            }
            chip_style.set(laid.chip_style);
            input_style.set(laid.input_style);
            if initial_height.get_value() > 0.0 {
                if let Some(observer) = visibility_observer.get_value() {
                    observer.disconnect();
                    visibility_observer.set_value(None);
                }
            } else if visibility_observer.get_value().is_none() {
                let Ok(Some(input)) = root.query_selector("input") else {
                    return;
                };
                let callback = Closure::wrap(Box::new(
                    move |_entries: js_sys::Array, _observer: web_sys::ResizeObserver| {
                        layout_tick.update(|tick| *tick = tick.wrapping_add(1));
                    },
                )
                    as Box<dyn FnMut(js_sys::Array, web_sys::ResizeObserver)>);
                if let Ok(observer) =
                    web_sys::ResizeObserver::new(callback.as_ref().unchecked_ref())
                {
                    observer.observe(&input);
                    callback.forget();
                    visibility_observer.set_value(Some(observer));
                }
            }
        });
    });

    Effect::new(move |_| {
        if resize_observer.get_value().is_some() {
            return;
        }
        let Some(body) = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.body())
        else {
            return;
        };
        let callback = Closure::wrap(Box::new(
            move |_entries: js_sys::Array, _observer: web_sys::ResizeObserver| {
                layout_tick.update(|tick| *tick = tick.wrapping_add(1));
            },
        )
            as Box<dyn FnMut(js_sys::Array, web_sys::ResizeObserver)>);
        if let Ok(observer) = web_sys::ResizeObserver::new(callback.as_ref().unchecked_ref()) {
            observer.observe(&body);
            callback.forget();
            resize_observer.set_value(Some(observer));
        }
    });

    on_cleanup(move || {
        if let Some(observer) = resize_observer.get_value() {
            observer.disconnect();
        }
        if let Some(observer) = visibility_observer.get_value() {
            observer.disconnect();
        }
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
    let messages_for_input = max_selected_message.clone();
    let messages_id_for_input = messages_id.clone();
    let input_slot = ViewFn::from(move || {
        let items = selected_items.get();
        let focused = input_focused.get();
        let collapsed = collapse_on_focus_out && !focused;
        let wrapper_class = chip_wrapper_class(size, collapsed);
        let hide_text = collapsed;
        let field_class = join_class([
            "d-input__input",
            "d-input",
            input_size_class(size),
            "d-combobox-multi-select__input",
            input_class_extra.get_value().as_str(),
            hide_text
                .then_some("d-combobox-multi-select__input--hidden")
                .unwrap_or(""),
        ]);
        let wrapper = join_class(["d-input__wrapper", input_wrapper_class.as_str()]);
        let label_class_name = input_label_classes(size, "", "", "");
        let description_class_name = input_description_classes(size, "");
        let placeholder = if items.is_empty() { placeholder } else { "" };
        let max_width = chip_max_width.get_value();
        let show_max = max_selected > 0 && items.len() > max_selected;
        let messages = messages_for_input.clone();
        let described_by =
            (show_max && !messages.is_empty()).then(|| messages_id_for_input.clone());
        let aria_invalid = (radio_message_state(&messages) == Some("critical")).then_some("true");
        let messages_view = radio_messages_view(
            &messages,
            show_max,
            "",
            "validation-messages-container",
            &messages_id_for_input,
        );
        view! {
            <span
                node_ref=root_ref
                class="d-combobox-multi-select__input-wrapper"
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
                    match event.code().to_lowercase().as_str() {
                        "arrowleft" if index > 0 => {
                            event.prevent_default();
                            focus_chip(index - 1);
                            request_close();
                        }
                        "arrowright" if index + 1 >= count => {
                            event.prevent_default();
                            if let Some(input) = input_ref.get() {
                                let input: HtmlElement = input.unchecked_into();
                                let _ = input.focus();
                            }
                            request_open();
                        }
                        "arrowright" => {
                            event.prevent_default();
                            focus_chip(index + 1);
                            request_close();
                        }
                        "backspace" | "delete" => {
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
                            "d-combobox-multi-select__chip",
                            (!max_width.is_empty()).then_some("d-combobox-multi-select__chip--truncate").unwrap_or(""),
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
                                    label_class="d-chip__label--focus-always"
                                    size=multi_select_chip_size(size)
                                    interactive=true
                                    disabled=disabled
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
                            <div class=label_class_name.clone() data-qa="dt-input-label">
                                {label_for_input.clone()}
                            </div>
                        })}
                        {(!description_for_input.is_empty()).then(|| view! {
                            <div class=description_class_name.clone() data-qa="dt-input-description">
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
                                aria-invalid=aria_invalid
                                aria-describedby=described_by
                                data-qa="dt-input-input"
                                on:input=move |event| {
                                    let text = event_target_value(&event);
                                    value.set(text.clone());
                                    if let Some(callback) = on_input.get_value() {
                                        callback.run(text);
                                    }
                                }
                                on:keydown=move |event: KeyboardEvent| {
                                    let key = event.key().to_lowercase();
                                    if key == "escape" {
                                        return;
                                    }
                                    if key == "enter" {
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
                                    if key == "backspace" || key == "arrowleft" {
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
                {messages_view}
            </span>
        }
    });

    let list_body = ViewFn::from(move || {
        let children = children.clone();
        view! {
            <div class="d-combobox-multi-select__list" on:mousedown=|event| event.prevent_default()>
                {if loading {
                    view! {
                        <div class="d-combobox-multi-select__list--loading">{loading_message}</div>
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
        assert_eq!(multi_select_chip_size("100"), "xs");
        assert_eq!(multi_select_chip_size("200"), "xs");
        assert_eq!(multi_select_chip_size("300"), "sm");
        assert_eq!(multi_select_chip_size("xs"), "xs");
        assert_eq!(multi_select_chip_size("sm"), "xs");
        assert_eq!(multi_select_chip_size("md"), "sm");
    }

    #[test]
    fn wrapper_size_uses_component_sizes() {
        assert_eq!(multi_select_wrapper_size("100"), "xs");
        assert_eq!(multi_select_wrapper_size("200"), "sm");
        assert_eq!(multi_select_wrapper_size("300"), "md");
        assert_eq!(multi_select_wrapper_size("xs"), "xs");
        assert_eq!(multi_select_wrapper_size("md"), "md");
        assert_eq!(
            chip_wrapper_class("300", false),
            "d-combobox-multi-select__chip-wrapper d-combobox-multi-select__chip-wrapper--md"
        );
        assert_eq!(
            chip_wrapper_class("300", true),
            "d-combobox-multi-select__chip-wrapper d-combobox-multi-select__chip-wrapper--md d-combobox-multi-select__chip-wrapper-md--collapsed"
        );
        assert_eq!(
            chip_wrapper_class("xs", true),
            "d-combobox-multi-select__chip-wrapper d-combobox-multi-select__chip-wrapper--xs d-combobox-multi-select__chip-wrapper-xs--collapsed"
        );
    }

    #[test]
    fn chip_top_is_geometric() {
        assert_eq!(chip_row_top(10.0, 32.0, 20.0, 4.0), 12.0);
        assert_eq!(caret_row_top(0.0, 20.0, 4.0, true, false), None);
        assert_eq!(caret_row_top(24.0, 20.0, 4.0, true, true), Some(24.0));
        assert_eq!(caret_row_top(24.0, 20.0, 4.0, false, true), Some(48.0));
    }
}
