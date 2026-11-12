use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{CompositionEvent, Event, FocusEvent, HtmlDivElement, HtmlInputElement, Node};

use super::radio::{radio_message_state, radio_messages_view};
use super::text::{text_classes, TextStyle};
use super::unique_id;
use super::validation::DtValidationMessage;

const INPUT_TYPES: &[&str] = &[
    "text", "textarea", "password", "email", "number", "date", "time", "file", "tel", "search",
    "color",
];

/// Length rule for `DtInput` (`validate.length`).
///
/// `warn` is the Vue `warn` threshold. `None` means it was omitted. `max` of
/// `0` or an empty `description` turns length validation off, matching the
/// Vue truthy checks.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DtInputLength {
    pub description: String,
    pub max: u32,
    pub warn: Option<u32>,
    pub message: String,
    pub limit_max_length: bool,
}

/// Dialtone `INPUT_SIZE_CLASSES.input`. `300` and `md` add no modifier.
pub(crate) fn input_size_class(size: &str) -> &'static str {
    match size {
        "100" | "xs" => "d-input--xs",
        "200" | "sm" => "d-input--sm",
        "400" | "lg" => "d-input--lg",
        "500" | "xl" => "d-input--xl",
        "300" | "md" | "" => "",
        _ => {
            leptos::logging::warn!("DtInput size `{size}` is not 100–500 or xs–xl");
            ""
        }
    }
}

/// Dialtone `INPUT_SIZE_CLASSES.textarea`.
pub(crate) fn textarea_size_class(size: &str) -> &'static str {
    match size {
        "100" | "xs" => "d-textarea--xs",
        "200" | "sm" => "d-textarea--sm",
        "400" | "lg" => "d-textarea--lg",
        "500" | "xl" => "d-textarea--xl",
        "300" | "md" | "" => "",
        _ => {
            leptos::logging::warn!("DtInput size `{size}` is not 100–500 or xs–xl");
            ""
        }
    }
}

/// Icon token for `size`. Dialtone `INPUT_ICON_SIZES`.
///
/// A Leptos slot does not receive the Vue `icon-size` slot prop. Size a
/// slotted icon with this value. `300` and `md` are `200`. An unknown size is
/// `200` as well. [`input_size_class`] warns for that size.
pub fn input_icon_size(size: &str) -> &'static str {
    match size {
        "100" | "xs" => "100",
        "200" | "sm" => "200",
        "400" | "lg" => "400",
        "500" | "xl" => "500",
        _ => "200",
    }
}

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Vue `resolvedLabelSize`. `xl` and `500` cap at the label maximum.
fn label_size_token(size: &str) -> &str {
    match size {
        "xl" => "lg",
        "500" => "400",
        "" => "300",
        other => other,
    }
}

/// Vue `resolvedDescriptionSize`.
fn description_size_token(size: &str) -> &'static str {
    match size {
        "100" | "200" | "xs" | "sm" => "xs",
        "500" | "xl" => "md",
        _ => "sm",
    }
}

fn text_style<'a>(
    kind: &'a str,
    size: &'a str,
    tone: &'a str,
    strength: &'a str,
    density: &'a str,
) -> TextStyle<'a> {
    TextStyle {
        kind,
        variant: "",
        size,
        family: "",
        italic: false,
        tone,
        align: "",
        truncate: false,
        max_lines: 0,
        numeric: false,
        wrap: "",
        text_box_trim: "",
        strength,
        density,
    }
}

pub(crate) fn input_label_classes(
    size: &str,
    label_size: &str,
    strength: &str,
    extra: &str,
) -> String {
    let size = if label_size.is_empty() {
        label_size_token(size)
    } else {
        label_size
    };
    let text = text_classes(&text_style("label", size, "secondary", strength, ""));
    join_class([text.as_str(), "d-input__label-text", extra])
}

pub(crate) fn input_description_classes(size: &str, extra: &str) -> String {
    let density = match size {
        "500" | "xl" => "300",
        _ => "",
    };
    let text = text_classes(&text_style(
        "body",
        description_size_token(size),
        "tertiary",
        "",
        density,
    ));
    join_class([
        text.as_str(),
        "d-input__description",
        "d-description",
        extra,
    ])
}

fn input_state_class(state: Option<&str>) -> &'static str {
    match state {
        Some("critical") => "d-input--critical",
        Some("warning") => "d-input--warning",
        Some("positive") => "d-input--positive",
        _ => "",
    }
}

/// Vue `inputClasses` order.
pub(crate) fn input_field_classes(
    textarea: bool,
    size: &str,
    state: Option<&str>,
    has_left: bool,
    has_right: bool,
    input_class: &str,
) -> String {
    join_class([
        "d-input__input",
        if textarea { "d-textarea" } else { "d-input" },
        input_state_class(state),
        if has_left { "d-input-icon--left" } else { "" },
        if has_right { "d-input-icon--right" } else { "" },
        if textarea {
            textarea_size_class(size)
        } else {
            input_size_class(size)
        },
        input_class,
    ])
}

/// Vue `inputWrapperClasses`. `hidden` returns no classes.
pub(crate) fn input_wrapper_classes(hidden: bool, state: Option<&str>, extra: &str) -> String {
    if hidden {
        return String::new();
    }
    join_class(["d-input__wrapper", input_state_class(state), extra])
}

pub(crate) fn input_root_class(hidden: bool, class_name: &str) -> String {
    join_class([
        "d-input__root",
        if hidden { "d-input--hidden" } else { "" },
        class_name,
    ])
}

pub(crate) fn should_validate_length(rules: &DtInputLength) -> bool {
    !rules.description.is_empty() && rules.max > 0
}

/// Vue `inputLengthState`. `warn` of `None` or `0` does not produce a warning.
pub(crate) fn length_state(count: u32, max: u32, warn: Option<u32>) -> Option<&'static str> {
    if let Some(warn) = warn {
        if warn > 0 && count < warn {
            return None;
        }
    }
    if count <= max {
        return match warn {
            Some(warn) if warn > 0 => Some("warning"),
            _ => None,
        };
    }
    Some("critical")
}

pub(crate) fn show_length_message(
    state: Option<&str>,
    message: &str,
    retain: bool,
    focused: bool,
    invalid: bool,
) -> bool {
    state.is_some() && !message.is_empty() && (retain || focused || invalid)
}

/// Vue `currentLength ? currentLength : calculateLength`. `0` is omitted.
pub(crate) fn resolved_length(current: Option<u32>, value: &str) -> u32 {
    match current {
        Some(count) if count > 0 => count,
        _ => value.chars().count() as u32,
    }
}

fn merge_state(messages: Option<&str>, length: Option<&str>) -> Option<&'static str> {
    match (messages, length) {
        (Some("critical"), _) | (_, Some("critical")) => Some("critical"),
        (Some("warning"), _) | (_, Some("warning")) => Some("warning"),
        (Some("positive"), _) => Some("positive"),
        (Some("info"), _) => Some("info"),
        _ => None,
    }
}

fn known_message_kind(kind: &str) -> bool {
    matches!(kind, "" | "critical" | "warning" | "positive" | "info")
}

fn live_length_kind(
    rules: &DtInputLength,
    count: u32,
    retain: bool,
    focused: bool,
) -> Option<&'static str> {
    if !should_validate_length(rules) {
        return None;
    }
    let state = length_state(count, rules.max, rules.warn);
    let invalid = count > rules.max;
    if show_length_message(state, &rules.message, retain, focused, invalid) {
        state
    } else {
        None
    }
}

fn clear_after_this_task(flag: RwSignal<bool>) {
    let promise = js_sys::Promise::resolve(&wasm_bindgen::JsValue::UNDEFINED);
    let closure = Closure::once(move |_value: wasm_bindgen::JsValue| flag.set(false));
    let _ = promise.then(&closure);
    closure.forget();
}

fn file_names(event: &Event) -> String {
    let Some(target) = event.target() else {
        return String::new();
    };
    let Ok(input) = target.dyn_into::<HtmlInputElement>() else {
        return String::new();
    };
    let Some(files) = input.files() else {
        return String::new();
    };
    let mut names = Vec::new();
    for index in 0..files.length() {
        if let Some(file) = files.item(index) {
            names.push(file.name());
        }
    }
    names.join(",")
}

fn control_value(event: &Event, input_type: &str) -> String {
    if input_type == "file" {
        file_names(event)
    } else {
        event_target_value(event)
    }
}

fn focus_left_the_container(event: &FocusEvent, container: &HtmlDivElement) -> bool {
    let Some(target) = event.related_target() else {
        return true;
    };
    let Some(node) = target.dyn_ref::<Node>() else {
        return true;
    };
    !container.contains(Some(node))
}

/// Dialtone Vue `DtInput` (`dialtone/v10.7.0`).
///
/// `value` is `modelValue`. `on_input` is `update:modelValue`. `type`
/// `textarea` renders a `<textarea>`. Other types render an `<input>`.
/// A `file` input omits `prop:value` so the browser keeps the chosen files.
/// `show_label` is `showLabel`. A hidden label becomes `aria-label` unless
/// `aria_label` is set. `size` is `100`–`500` or `xs`–`xl`. `300` and `md`
/// add no size class. State classes are `d-input--critical`, `--warning`, and
/// `--positive` when `show_messages` is set. `messages` use those v10 types.
/// An empty kind means `critical`. `aria-invalid` is `"true"` only for
/// `critical`, including when the messages are hidden.
///
/// `validate` is `validate.length`. `current_length` is `currentLength`. `0`
/// falls back to the character count, matching the Vue truthy check.
/// `on_length` is `update:length` and fires when `current_length` is omitted,
/// including on mount. `on_invalid` is `update:invalid` and fires when that
/// flag changes. `start_icon` wins over deprecated `left_icon`. `end_icon`
/// wins over deprecated `right_icon`. Icon wrappers render only when a slot is
/// set. Size a slotted icon with [`input_icon_size`]. The Vue `clear` slot
/// prop is not forwarded. `defineExpose` `focus`, `blur`, and `select` are not
/// ported. `labelChildProps`, `descriptionChildProps`, and `messagesChildProps`
/// are not ported. A missing label and `aria_label` warns. Vue uses `console.info`.
///
/// `d-input__root`, `d-input__input`, and `d-input__description` are emitted
/// and are not in the vendored CSS. The other `d-input*` classes used here are.
#[component]
pub fn DtInput(
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] label: String,
    /// `showLabel`. Default true. False omits the visible label and sets `aria-label`.
    #[prop(optional, default = true)]
    show_label: bool,
    #[prop(optional, into)] description: String,
    /// Extra class on the description (`descriptionClass`).
    #[prop(optional, into)]
    description_class: String,
    /// `100`–`500`, or `xs` / `sm` / `md` / `lg` / `xl` (`size`).
    #[prop(optional, default = "300")]
    size: &'static str,
    /// Input type (`type`). `textarea` renders a textarea.
    #[prop(optional, default = "text")]
    input_type: &'static str,
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_input: Callback<String>,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] autocomplete: String,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional)] required: bool,
    /// Hides the root (`hidden`).
    #[prop(optional)]
    hidden: bool,
    /// Overrides the label size (`labelSize`). Empty derives it from `size`.
    #[prop(optional, default = "")]
    label_size: &'static str,
    /// Label weight (`labelStrength`).
    #[prop(optional, into)]
    label_strength: String,
    /// Extra class on the label (`labelClass`).
    #[prop(optional, into)]
    label_class: String,
    /// Extra class on the field (`inputClass`).
    #[prop(optional, into)]
    input_class: String,
    #[prop(optional, into)] input_wrapper_class: String,
    /// Extra class on the start icon (`startIconClass`).
    #[prop(optional, into)]
    start_icon_class: String,
    /// Extra class on the end icon (`endIconClass`).
    #[prop(optional, into)]
    end_icon_class: String,
    /// Extra class on the root (`$attrs.class`).
    #[prop(optional, into)]
    class: String,
    /// Accessible name (`aria-label`). Wins over the hidden-label fallback.
    #[prop(optional, into)]
    aria_label: String,
    /// Character count override (`currentLength`). `None` counts `value`.
    #[prop(optional)]
    current_length: Option<u32>,
    /// Keep the length warning after blur (`retainWarning`).
    #[prop(optional)]
    retain_warning: bool,
    /// Length rule (`validate.length`).
    #[prop(optional)]
    validate: DtInputLength,
    #[prop(optional, default = true)] show_messages: bool,
    #[prop(optional)] messages: Vec<DtValidationMessage>,
    /// Extra class on the messages (`messagesClass`).
    #[prop(optional, into)]
    messages_class: String,
    #[prop(optional)] node_ref: NodeRef<html::Input>,
    /// Textarea handle. Used when `input_type` is `textarea`.
    #[prop(optional)]
    textarea_ref: NodeRef<html::Textarea>,
    /// `update:length`. Fires when `current_length` is omitted.
    #[prop(optional, into)]
    on_length: Option<Callback<u32>>,
    /// `update:invalid`. Fires when the length-invalid flag changes.
    #[prop(optional, into)]
    on_invalid: Option<Callback<bool>>,
    #[prop(optional, into)] on_focus: Option<Callback<()>>,
    #[prop(optional, into)] on_blur: Option<Callback<()>>,
    /// Label slot (`label`). Replaces the label text.
    #[prop(optional)]
    label_slot: Option<ViewFn>,
    /// Description slot (`description`). Replaces the description text.
    #[prop(optional)]
    description_slot: Option<ViewFn>,
    /// Start icon (`startIcon`). Wins over `left_icon`.
    #[prop(optional)]
    start_icon: Option<ViewFn>,
    /// Deprecated start icon (`leftIcon`).
    #[prop(optional)]
    left_icon: Option<ViewFn>,
    /// End icon (`endIcon`). Wins over `right_icon`.
    #[prop(optional)]
    end_icon: Option<ViewFn>,
    /// Deprecated end icon (`rightIcon`).
    #[prop(optional)]
    right_icon: Option<ViewFn>,
) -> impl IntoView {
    if !INPUT_TYPES.contains(&input_type) {
        leptos::logging::warn!(
            "DtInput type `{input_type}` is not text, textarea, password, email, number, date, time, file, tel, search, or color."
        );
    }
    if label.is_empty() && aria_label.is_empty() {
        leptos::logging::warn!(
            "[DtInput] A label is required for accessibility. Provide a label prop and use show_label false to hide it visually."
        );
    }
    for message in &messages {
        if !known_message_kind(&message.kind) {
            leptos::logging::warn!(
                "[DtInput] message type \"{}\" is not critical, warning, positive, or info.",
                message.kind
            );
        }
    }
    let messages: Vec<_> = messages
        .into_iter()
        .filter(|message| known_message_kind(&message.kind))
        .collect();
    let static_state = radio_message_state(&messages);
    let textarea = input_type == "textarea";
    let has_left = start_icon.is_some() || left_icon.is_some();
    let has_right = end_icon.is_some() || right_icon.is_some();
    let autocomplete = if autocomplete.is_empty() {
        "off".to_string()
    } else {
        autocomplete
    };
    let id_attr = (!id.is_empty()).then_some(id);
    let name_attr = (!name.is_empty()).then_some(name);
    let placeholder_attr = (!placeholder.is_empty()).then_some(placeholder);
    let aria_name = if !aria_label.is_empty() {
        Some(aria_label)
    } else if !show_label && !label.is_empty() {
        Some(label.clone())
    } else {
        None
    };
    let max_length = (should_validate_length(&validate) && validate.limit_max_length)
        .then_some(validate.max);
    let label_class_name = input_label_classes(size, label_size, &label_strength, &label_class);
    let description_class_name = input_description_classes(size, &description_class);
    let description_id = unique_id("dt-input-description");
    let messages_id = unique_id("dt-input-messages");
    let has_description_text = description_slot.is_some() || !description.is_empty();
    let show_length_copy = should_validate_length(&validate);
    let show_description = has_description_text || show_length_copy;
    let length_copy = validate.description.clone();
    let details = show_description.then(|| description_id.clone());
    let left_class = join_class(["d-input-icon", "d-input-icon--left", start_icon_class.as_str()]);
    let right_class = join_class(["d-input-icon", "d-input-icon--right", end_icon_class.as_str()]);
    let root_class = input_root_class(hidden, &class);
    let rules = StoredValue::new(validate);
    let stored_messages = StoredValue::new(messages);
    let messages_class = StoredValue::new(messages_class);
    let focused = RwSignal::new(false);
    let composing = RwSignal::new(false);
    let just_ended = RwSignal::new(false);
    let container = NodeRef::<html::Div>::new();

    let field_state = move || {
        let count = resolved_length(current_length, &value.get());
        let length_kind = live_length_kind(&rules.get_value(), count, retain_warning, focused.get());
        merge_state(static_state, length_kind)
    };
    let described_messages_id = StoredValue::new(messages_id.clone());
    let described_by = move || {
        let count = resolved_length(current_length, &value.get());
        let length_kind = live_length_kind(&rules.get_value(), count, retain_warning, focused.get());
        let has_length_message = length_kind.is_some();
        let has_messages = !stored_messages.with_value(|messages| messages.is_empty());
        (show_messages && (has_messages || has_length_message))
            .then(|| described_messages_id.get_value())
    };
    let input_class = StoredValue::new(input_class);
    let field_class = move || {
        let state = field_state();
        let painted = show_messages.then_some(state).flatten();
        input_field_classes(
            textarea,
            size,
            painted,
            has_left,
            has_right,
            &input_class.get_value(),
        )
    };
    let wrapper_class = move || {
        let state = field_state();
        let painted = show_messages.then_some(state).flatten();
        input_wrapper_classes(hidden, painted, &input_wrapper_class)
    };
    let aria_invalid = move || (field_state() == Some("critical")).then_some("true");

    if current_length.is_none() {
        if let Some(on_length) = on_length {
            Effect::new(move |_| {
                on_length.run(value.get().chars().count() as u32);
            });
        }
    }
    if on_invalid.is_some() {
        let previous = StoredValue::new(None);
        Effect::new(move |_| {
            let count = resolved_length(current_length, &value.get());
            let rules = rules.get_value();
            let invalid = should_validate_length(&rules) && count > rules.max;
            let last = previous.get_value();
            previous.set_value(Some(invalid));
            if last.is_some() && last != Some(invalid) {
                if let Some(on_invalid) = on_invalid {
                    on_invalid.run(invalid);
                }
            }
        });
    }

    let messages_view = if show_length_copy {
        view! {
            {move || {
                let count = resolved_length(current_length, &value.get());
                let mut messages = stored_messages.get_value();
                if let Some(kind) = live_length_kind(&rules.get_value(), count, retain_warning, focused.get()) {
                    messages.push(DtValidationMessage {
                        message: rules.with_value(|rules| rules.message.clone()),
                        kind: kind.to_string(),
                    });
                }
                let class_name = messages_class.get_value();
                radio_messages_view(
                    &messages,
                    show_messages,
                    &class_name,
                    "dt-input-messages",
                    &messages_id,
                )
            }}
        }
        .into_any()
    } else {
        radio_messages_view(
            &stored_messages.get_value(),
            show_messages,
            &messages_class.get_value(),
            "dt-input-messages",
            &messages_id,
        )
    };

    let report_blur = move |event: FocusEvent| {
        focused.set(false);
        let leaves = container
            .get()
            .is_none_or(|container| focus_left_the_container(&event, &container));
        if leaves {
            if let Some(on_blur) = on_blur {
                on_blur.run(());
            }
        }
    };
    let report_input = move |event: Event| {
        if composing.get() || just_ended.get() {
            return;
        }
        on_input.run(control_value(&event, input_type));
    };
    let start_composition = move |_| composing.set(true);
    let end_composition = move |event: CompositionEvent| {
        composing.set(false);
        just_ended.set(true);
        on_input.run(control_value(&event, input_type));
        clear_after_this_task(just_ended);
    };
    let report_focus = move |_| {
        focused.set(true);
        if let Some(on_focus) = on_focus {
            on_focus.run(());
        }
    };

    let is_file = input_type == "file";
    let field = if textarea {
        view! {
            <textarea
                id=id_attr.clone()
                name=name_attr.clone()
                class=field_class
                node_ref=textarea_ref
                autocomplete=autocomplete.clone()
                placeholder=placeholder_attr.clone()
                required=required
                maxlength=max_length
                aria-label=aria_name.clone()
                aria-invalid=aria_invalid
                aria-describedby=described_by
                prop:value=move || value.get()
                prop:disabled=move || disabled.get().unwrap_or(false)
                data-qa="dt-input-input"
                on:input=report_input
                on:compositionstart=start_composition
                on:compositionend=end_composition
                on:focus=report_focus
                on:blur=report_blur
            />
        }
        .into_any()
    } else if is_file {
        view! {
            <input
                id=id_attr.clone()
                name=name_attr.clone()
                class=field_class
                type=input_type
                node_ref=node_ref
                autocomplete=autocomplete.clone()
                placeholder=placeholder_attr.clone()
                required=required
                maxlength=max_length
                aria-label=aria_name.clone()
                aria-invalid=aria_invalid
                aria-describedby=described_by
                prop:disabled=move || disabled.get().unwrap_or(false)
                data-qa="dt-input-input"
                on:input=report_input
                on:compositionstart=start_composition
                on:compositionend=end_composition
                on:focus=report_focus
                on:blur=report_blur
            />
        }
        .into_any()
    } else {
        view! {
            <input
                id=id_attr
                name=name_attr
                class=field_class
                type=input_type
                node_ref=node_ref
                autocomplete=autocomplete
                placeholder=placeholder_attr
                required=required
                maxlength=max_length
                aria-label=aria_name
                aria-invalid=aria_invalid
                aria-describedby=described_by
                prop:value=move || value.get()
                prop:disabled=move || disabled.get().unwrap_or(false)
                data-qa="dt-input-input"
                on:input=report_input
                on:compositionstart=start_composition
                on:compositionend=end_composition
                on:focus=report_focus
                on:blur=report_blur
            />
        }
        .into_any()
    };

    view! {
        <div node_ref=container class=root_class data-qa="dt-input">
            <label class="d-input__label" aria-details=details data-qa="dt-input-label-wrapper">
                {if let Some(slot) = label_slot {
                    slot.run().into_any()
                } else if show_label && !label.is_empty() {
                    view! {
                        <span class=label_class_name data-qa="dt-input-label">
                            {label}
                        </span>
                    }
                    .into_any()
                } else {
                    ().into_any()
                }}
                {show_description.then(|| view! {
                    <div id=description_id class=description_class_name data-qa="dt-input-description">
                        {has_description_text.then(|| view! {
                            <div>
                                {if let Some(slot) = description_slot {
                                    slot.run().into_any()
                                } else {
                                    description.into_any()
                                }}
                            </div>
                        })}
                        {show_length_copy.then(|| view! {
                            <div data-qa="dt-input-length-description" class="d-input__length-description">
                                {length_copy}
                            </div>
                        })}
                    </div>
                })}
                <div class=wrapper_class attr:read-only=move || disabled.get().unwrap_or(false).then_some("true")>
                    {has_left.then(|| view! {
                        <span class=left_class data-qa="dt-input-left-icon-wrapper" on:focusout=report_blur>
                            {if let Some(slot) = start_icon {
                                slot.run().into_any()
                            } else if let Some(slot) = left_icon {
                                slot.run().into_any()
                            } else {
                                ().into_any()
                            }}
                        </span>
                    })}
                    {field}
                    {has_right.then(|| view! {
                        <span class=right_class data-qa="dt-input-right-icon-wrapper" on:focusout=report_blur>
                            {if let Some(slot) = end_icon {
                                slot.run().into_any()
                            } else if let Some(slot) = right_icon {
                                slot.run().into_any()
                            } else {
                                ().into_any()
                            }}
                        </span>
                    })}
                </div>
            </label>
            {messages_view}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_size_classes_match_vue() {
        assert_eq!(input_size_class("100"), "d-input--xs");
        assert_eq!(input_size_class("xs"), "d-input--xs");
        assert_eq!(input_size_class("200"), "d-input--sm");
        assert_eq!(input_size_class("sm"), "d-input--sm");
        assert_eq!(input_size_class("300"), "");
        assert_eq!(input_size_class("md"), "");
        assert_eq!(input_size_class(""), "");
        assert_eq!(input_size_class("400"), "d-input--lg");
        assert_eq!(input_size_class("lg"), "d-input--lg");
        assert_eq!(input_size_class("500"), "d-input--xl");
        assert_eq!(input_size_class("xl"), "d-input--xl");
        assert_eq!(input_size_class("nope"), "");
        assert_eq!(textarea_size_class("100"), "d-textarea--xs");
        assert_eq!(textarea_size_class("sm"), "d-textarea--sm");
        assert_eq!(textarea_size_class("300"), "");
        assert_eq!(textarea_size_class("lg"), "d-textarea--lg");
        assert_eq!(textarea_size_class("500"), "d-textarea--xl");
        assert_eq!(input_icon_size("100"), "100");
        assert_eq!(input_icon_size("xs"), "100");
        assert_eq!(input_icon_size("200"), "200");
        assert_eq!(input_icon_size("sm"), "200");
        assert_eq!(input_icon_size("300"), "200");
        assert_eq!(input_icon_size("md"), "200");
        assert_eq!(input_icon_size("400"), "400");
        assert_eq!(input_icon_size("lg"), "400");
        assert_eq!(input_icon_size("500"), "500");
        assert_eq!(input_icon_size("xl"), "500");
        assert_eq!(
            input_label_classes("md", "", "", ""),
            "d-text d-text-label--md d-text--tone-secondary d-input__label-text"
        );
        assert_eq!(
            input_label_classes("300", "", "", ""),
            "d-text d-text-label--md d-text--tone-secondary d-input__label-text"
        );
        assert_eq!(
            input_label_classes("xl", "", "", ""),
            "d-text d-text-label--lg d-text--tone-secondary d-input__label-text"
        );
        assert_eq!(
            input_label_classes("500", "", "", ""),
            "d-text d-text-label--lg d-text--tone-secondary d-input__label-text"
        );
        assert_eq!(
            input_label_classes("300", "200", "bold", "extra"),
            "d-text d-text-label--sm d-text--tone-secondary d-text--fw-bold d-input__label-text extra"
        );
        assert_eq!(
            input_description_classes("md", ""),
            "d-text d-text-body--sm d-text--tone-tertiary d-input__description d-description"
        );
        assert_eq!(
            input_description_classes("xl", ""),
            "d-text d-text-body--md d-text--tone-tertiary d-text--lh-300 d-input__description d-description"
        );
        assert_eq!(
            input_field_classes(false, "400", Some("critical"), true, false, "extra"),
            "d-input__input d-input d-input--critical d-input-icon--left d-input--lg extra"
        );
        assert_eq!(
            input_field_classes(true, "100", None, false, true, ""),
            "d-input__input d-textarea d-input-icon--right d-textarea--xs"
        );
        assert_eq!(
            input_wrapper_classes(false, Some("warning"), "extra"),
            "d-input__wrapper d-input--warning extra"
        );
        assert_eq!(input_wrapper_classes(true, Some("warning"), "extra"), "");
        assert_eq!(
            input_root_class(true, "extra"),
            "d-input__root d-input--hidden extra"
        );
    }

    #[test]
    fn length_validation_matches_vue() {
        let off = DtInputLength::default();
        assert!(!should_validate_length(&off));
        assert!(!should_validate_length(&DtInputLength {
            description: "Limit".into(),
            max: 0,
            ..DtInputLength::default()
        }));
        let rules = DtInputLength {
            description: "Limit".into(),
            max: 10,
            warn: Some(8),
            message: "Too long".into(),
            limit_max_length: true,
        };
        assert!(should_validate_length(&rules));
        assert_eq!(length_state(7, 10, Some(8)), None);
        assert_eq!(length_state(8, 10, Some(8)), Some("warning"));
        assert_eq!(length_state(10, 10, Some(8)), Some("warning"));
        assert_eq!(length_state(11, 10, Some(8)), Some("critical"));
        assert_eq!(length_state(4, 10, None), None);
        assert_eq!(length_state(11, 10, None), Some("critical"));
        assert_eq!(length_state(4, 10, Some(0)), None);
        assert!(!show_length_message(Some("warning"), "Too long", false, false, false));
        assert!(show_length_message(Some("warning"), "Too long", false, true, false));
        assert!(show_length_message(Some("critical"), "Too long", false, false, true));
        assert!(!show_length_message(Some("warning"), "", true, true, true));
        assert_eq!(resolved_length(None, "a😀"), 2);
        assert_eq!(resolved_length(Some(0), "abcd"), 4);
        assert_eq!(resolved_length(Some(3), "abcd"), 3);
        assert_eq!(
            live_length_kind(&rules, 9, false, true),
            Some("warning")
        );
        assert_eq!(live_length_kind(&rules, 9, false, false), None);
        assert_eq!(
            live_length_kind(&rules, 11, false, false),
            Some("critical")
        );
    }
}
