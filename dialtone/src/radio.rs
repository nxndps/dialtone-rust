use std::sync::Arc;

use leptos::prelude::*;

use super::text::{text_classes, TextStyle};
use super::unique_id;
use super::validation::DtValidationMessage;

/// Group state read by a slotted [`DtRadio`](crate::DtRadio).
#[derive(Clone)]
pub(crate) struct RadioGroupContext {
    pub(crate) name: Arc<str>,
    pub(crate) disabled: bool,
    pub(crate) validation_state: Option<String>,
    pub(crate) selected: Signal<String>,
    pub(crate) on_change: Option<Callback<String>>,
}

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn text_style<'a>(kind: &'a str, size: &'a str, tone: &'a str, strength: &'a str) -> TextStyle<'a> {
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
        density: "",
    }
}

/// Input class from `RADIO_INPUT_VALIDATION_CLASSES`. `info` is a valid state
/// and adds no class. An unknown token warns and adds no class.
pub(crate) fn radio_validation_class(state: &str) -> &'static str {
    match state {
        "" | "info" => "",
        "warning" => "d-radio--warning",
        "critical" => "d-radio--critical",
        "positive" => "d-radio--positive",
        other => {
            leptos::logging::warn!(
                "[DtRadio] validationState \"{other}\" is not warning, critical, positive, or info."
            );
            ""
        }
    }
}

pub(crate) fn radio_input_class(state: &str, input_class: &str) -> String {
    join_class(["d-radio", radio_validation_class(state), input_class])
}

/// Group `validationState` wins when it is non-empty. Vue uses `||`.
pub(crate) fn resolved_validation_state(group_state: Option<&str>, own: &str) -> String {
    match group_state {
        Some(state) if !state.is_empty() => state.to_string(),
        _ => own.to_string(),
    }
}

/// Empty `type` is `critical` (`DEFAULT_VALIDATION_MESSAGE_TYPE` at v10).
fn radio_message_kind(kind: &str) -> &str {
    if kind.is_empty() {
        "critical"
    } else {
        kind
    }
}

/// Dialtone `getValidationState` at `dialtone/v10.7.0`.
/// Priority is critical, then warning, then positive, then info.
pub(crate) fn radio_message_state(messages: &[DtValidationMessage]) -> Option<&'static str> {
    let mut warning = false;
    let mut positive = false;
    let mut info = false;
    let mut critical = false;
    for message in messages {
        match radio_message_kind(&message.kind) {
            "critical" => critical = true,
            "warning" => warning = true,
            "positive" => positive = true,
            "info" => info = true,
            other => {
                leptos::logging::warn!(
                    "[DtRadio] message type \"{other}\" is not critical, warning, positive, or info."
                );
            }
        }
    }
    if critical {
        Some("critical")
    } else if warning {
        Some("warning")
    } else if positive {
        Some("positive")
    } else if info {
        Some("info")
    } else {
        None
    }
}

fn message_row_class(kind: &str) -> String {
    let modifier = match kind {
        "warning" => "base-input__message--warning d-validation-message--warning",
        "critical" => "base-input__message--critical d-validation-message--critical",
        "positive" => "base-input__message--positive d-validation-message--positive",
        "info" => "base-input__message--info d-validation-message--info",
        _ => "",
    };
    join_class(["base-input__message", "d-validation-message", modifier])
}

pub(crate) fn radio_messages_container_class(class_name: &str) -> String {
    join_class([
        "base-input__messages",
        "d-validation-message__container",
        class_name,
    ])
}

/// v10 `DtValidationMessages` shell. The container stays mounted.
/// Rows use the vendored mask (`:not(:has(.d-icon))::before`). Vue renders a
/// `DtIcon` in the row, which suppresses that mask.
pub(crate) fn radio_messages_view(
    messages: &[DtValidationMessage],
    show: bool,
    class_name: &str,
    data_qa: &'static str,
    id: &str,
) -> AnyView {
    let state = radio_message_state(messages);
    let container = radio_messages_container_class(class_name);
    let rows = if show {
        let Some(state) = state else {
            return mounted_messages(container, data_qa, id, ().into_any());
        };
        messages
            .iter()
            .filter(|message| {
                !message.message.is_empty() && radio_message_kind(&message.kind) == state
            })
            .map(|message| {
                let class_name = message_row_class(radio_message_kind(&message.kind));
                let text = message.message.clone();
                view! {
                    <div data-qa="validation-message" class=class_name>
                        <p>{text}</p>
                    </div>
                }
            })
            .collect_view()
            .into_any()
    } else {
        ().into_any()
    };
    mounted_messages(container, data_qa, id, rows)
}

fn mounted_messages(container: String, data_qa: &str, id: &str, rows: AnyView) -> AnyView {
    let id = id.to_string();
    let data_qa = data_qa.to_string();
    view! {
        <div id=id aria-live="assertive" class=container data-qa=data_qa>
            {rows}
        </div>
    }
    .into_any()
}

/// `DtText` label classes plus `d-radio__copy d-radio__label`.
/// Empty `label_size` is `300`. Empty `label_strength` is `normal`.
pub(crate) fn radio_label_class(
    label_class: &str,
    label_size: &str,
    label_strength: &str,
    disabled: bool,
) -> String {
    let size = if label_size.is_empty() {
        "300"
    } else {
        label_size
    };
    let strength = if label_strength.is_empty() {
        "normal"
    } else {
        label_strength
    };
    let tone = if disabled { "disabled" } else { "primary" };
    let text = text_classes(&text_style("label", size, tone, strength));
    join_class([text.as_str(), label_class, "d-radio__copy d-radio__label"])
}

pub(crate) fn radio_description_class(description_class: &str) -> String {
    let text = text_classes(&text_style("body", "200", "tertiary", ""));
    join_class([text.as_str(), "d-description", description_class])
}

pub(crate) fn radio_legend_class(legend_class: &str) -> String {
    let text = text_classes(&text_style("label", "", "secondary", ""));
    join_class([text.as_str(), "d-input-group__legend-text", legend_class])
}

/// Report `update:modelValue` for this change.
/// A disabled control does not report. Inside a group, the current value does not.
pub(crate) fn should_report(disabled: bool, selected: Option<&str>, next: &str) -> bool {
    if disabled {
        return false;
    }
    match selected {
        Some(current) => current != next,
        None => true,
    }
}

/// Dialtone Vue `DtRadio` (`dialtone/v10.7.0`).
///
/// `checked` is the boolean `modelValue`. `on_change` is `update:modelValue`
/// and reports the option `value`. Inside [`DtRadioGroup`](crate::DtRadioGroup),
/// the checked state follows `selected`. `label_size` is `labelSize`.
/// `label_strength` is `labelStrength`. `show_label` is `showLabel`.
/// `validation_state` is `validationState`.
#[component]
pub fn DtRadio(
    /// Option value (`value`). This is what `on_change` reports.
    #[prop(optional, into)]
    value: String,
    /// Checked state (`modelValue`). Ignored for display inside a radio group.
    #[prop(optional)]
    checked: Option<Signal<bool>>,
    /// Called with the option value (`update:modelValue`).
    #[prop(optional, into)]
    on_change: Option<Callback<String>>,
    /// Called on `focusin`.
    #[prop(optional, into)]
    on_focus_in: Option<Callback<()>>,
    /// Called on `focusout`.
    #[prop(optional, into)]
    on_focus_out: Option<Callback<()>>,
    #[prop(optional, into)] name: String,
    /// Label text. The default slot replaces it.
    #[prop(optional, into)]
    label: String,
    /// Show the label (`showLabel`). False sets `aria-label` from `label`.
    #[prop(optional, default = true)]
    show_label: bool,
    /// Label size (`labelSize`). Empty uses `300`.
    #[prop(optional, default = "")]
    label_size: &'static str,
    /// Label weight (`labelStrength`). Empty uses `normal`.
    #[prop(optional, default = "")]
    label_strength: &'static str,
    #[prop(optional, into)] description: String,
    #[prop(optional)] disabled: bool,
    /// `warning`, `critical`, `positive`, or `info` (`validationState`).
    /// A group state wins. `info` adds no input class.
    #[prop(optional, default = "")]
    validation_state: &'static str,
    /// Extra class on the input (`inputClass`).
    #[prop(optional, into)]
    input_class: String,
    /// Extra class on the label (`labelClass`).
    #[prop(optional, into)]
    label_class: String,
    /// Extra class on the description (`descriptionClass`).
    #[prop(optional, into)]
    description_class: String,
    /// Extra class on the messages (`messagesClass`).
    #[prop(optional, into)]
    messages_class: String,
    /// Extra class on the root (`$attrs.class`).
    #[prop(optional, into)]
    class: String,
    /// Show `messages` (`showMessages`).
    #[prop(optional, default = true)]
    show_messages: bool,
    #[prop(optional)] messages: Vec<DtValidationMessage>,
    /// Accessible name when the label is hidden.
    #[prop(optional, into)]
    aria_label: String,
    /// Description slot (`description`). Replaces `description`.
    #[prop(optional)]
    description_slot: Option<ViewFn>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let checked = checked.unwrap_or_else(|| Signal::derive(|| false));
    let group = use_context::<RadioGroupContext>();
    let has_label_content = children.is_some() || !label.is_empty();
    if !has_label_content && aria_label.is_empty() {
        leptos::logging::warn!(
            "[Dialtone] A label is required for accessibility. Provide a label prop and use show-label=\"false\" to hide it visually."
        );
    }
    if let Some(group) = group.as_ref() {
        if !name.is_empty() && group.name.as_ref() != name.as_str() {
            leptos::logging::warn!(
                "Component is being used inside a Group. Did you mean to override the name prop value ({}) with ({})? It is recommended to only set name at the Group level.",
                group.name,
                name
            );
        }
    }
    let is_disabled = disabled || group.as_ref().is_some_and(|group| group.disabled);
    let input_name = if !name.is_empty() {
        name
    } else {
        group
            .as_ref()
            .map(|group| group.name.to_string())
            .unwrap_or_default()
    };
    let state = resolved_validation_state(
        group
            .as_ref()
            .and_then(|group| group.validation_state.as_deref()),
        validation_state,
    );
    let input_class_name = radio_input_class(&state, &input_class);
    let label_class_name = radio_label_class(&label_class, label_size, label_strength, is_disabled);
    let description_class_name = radio_description_class(&description_class);
    let root_class = (!class.is_empty()).then_some(class);
    let name_attr = (!input_name.is_empty()).then_some(input_name);
    let input_label = if !show_label && !label.is_empty() {
        Some(label.clone())
    } else if !aria_label.is_empty() {
        Some(aria_label)
    } else {
        None
    };
    let value_for_check = value.clone();
    let group_for_check = group.clone();
    let is_checked = move || {
        if let Some(group) = group_for_check.clone() {
            let value_for_check = value_for_check.clone();
            group.selected.with(|selected| selected == &value_for_check)
        } else {
            checked.get()
        }
    };
    let messages_id = unique_id("dt-radio-messages");
    let described_by = (show_messages && !messages.is_empty()).then(|| messages_id.clone());
    let aria_invalid = (radio_message_state(&messages) == Some("critical")).then_some("true");
    let messages_view = radio_messages_view(
        &messages,
        show_messages,
        &messages_class,
        "dt-radio-validation-messages",
        &messages_id,
    );
    let has_description = description_slot.is_some() || !description.is_empty();
    let description_body = if let Some(slot) = description_slot {
        Some(slot.run().into_any())
    } else if !description.is_empty() {
        Some(description.into_any())
    } else {
        None
    };
    let label_body = if show_label {
        if let Some(children) = children {
            Some(children().into_any())
        } else if !label.is_empty() {
            Some(label.into_any())
        } else {
            None
        }
    } else {
        None
    };
    let group_for_change = group.clone();
    view! {
        <div class=root_class>
            <label class="d-radio-group" class=("d-radio-group--disabled", is_disabled)>
                <div class="d-radio__input">
                    <input
                        type="radio"
                        class=input_class_name
                        name=name_attr
                        value=value.clone()
                        prop:checked=is_checked
                        prop:disabled=is_disabled
                        aria-label=input_label
                        aria-invalid=aria_invalid
                        aria-describedby=described_by
                        on:change=move |event| {
                            let next = event_target_value(&event);
                            let selected = group_for_change
                                .as_ref()
                                .map(|group| group.selected.get_untracked());
                            if !should_report(is_disabled, selected.as_deref(), &next) {
                                return;
                            }
                            if let Some(group) = group_for_change.clone() {
                                if let Some(on_change) = group.on_change {
                                    on_change.run(next.clone());
                                }
                            }
                            if let Some(on_change) = on_change {
                                on_change.run(next);
                            }
                        }
                        on:focusin=move |_| {
                            if let Some(on_focus_in) = on_focus_in {
                                on_focus_in.run(());
                            }
                        }
                        on:focusout=move |_| {
                            if let Some(on_focus_out) = on_focus_out {
                                on_focus_out.run(());
                            }
                        }
                    />
                </div>
                {label_body.map(|body| view! {
                    <div class=label_class_name.clone() data-qa="radio-label">
                        {body}
                    </div>
                })}
            </label>
            <div class="d-radio__messages" data-qa="radio-description-messages">
                {has_description.then(|| view! {
                    <div class=description_class_name.clone() data-qa="radio-description">
                        {description_body}
                    </div>
                })}
                {messages_view}
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_class_matches_the_vue_map() {
        assert_eq!(radio_validation_class("warning"), "d-radio--warning");
        assert_eq!(radio_validation_class("critical"), "d-radio--critical");
        assert_eq!(radio_validation_class("positive"), "d-radio--positive");
        assert_eq!(radio_validation_class("info"), "");
        assert_eq!(radio_validation_class(""), "");
        assert_eq!(radio_validation_class("error"), "");
    }

    #[test]
    fn input_class_joins_in_vue_order() {
        assert_eq!(radio_input_class("", ""), "d-radio");
        assert_eq!(
            radio_input_class("critical", "my-custom-class"),
            "d-radio d-radio--critical my-custom-class"
        );
    }

    #[test]
    fn group_state_wins_when_it_is_set() {
        assert_eq!(
            resolved_validation_state(Some("warning"), "critical"),
            "warning"
        );
        assert_eq!(resolved_validation_state(Some(""), "critical"), "critical");
        assert_eq!(resolved_validation_state(None, "positive"), "positive");
    }

    #[test]
    fn message_state_follows_v10_priority() {
        assert_eq!(radio_message_state(&[]), None);
        assert_eq!(
            radio_message_state(&[DtValidationMessage {
                message: "plain".into(),
                kind: String::new(),
            }]),
            Some("critical")
        );
        assert_eq!(
            radio_message_state(&[DtValidationMessage::info("note")]),
            Some("info")
        );
        assert_eq!(
            radio_message_state(&[
                DtValidationMessage::info("note"),
                DtValidationMessage::positive("ok"),
            ]),
            Some("positive")
        );
        assert_eq!(
            radio_message_state(&[
                DtValidationMessage::positive("ok"),
                DtValidationMessage::warning("careful"),
            ]),
            Some("warning")
        );
        assert_eq!(
            radio_message_state(&[
                DtValidationMessage::warning("careful"),
                DtValidationMessage::critical("stop"),
            ]),
            Some("critical")
        );
    }

    #[test]
    fn message_row_uses_the_v10_class_pair() {
        assert_eq!(
            message_row_class("critical"),
            "base-input__message d-validation-message base-input__message--critical d-validation-message--critical"
        );
        assert_eq!(
            radio_messages_container_class("my-custom-class"),
            "base-input__messages d-validation-message__container my-custom-class"
        );
    }

    #[test]
    fn label_and_description_classes_follow_dt_text() {
        assert_eq!(
            radio_label_class("", "", "", false),
            "d-text d-text-label--md d-text--tone-primary d-text--fw-normal d-radio__copy d-radio__label"
        );
        assert_eq!(
            radio_label_class("my-custom-class", "400", "bold", true),
            "d-text d-text-label--lg d-text--tone-disabled d-text--fw-bold my-custom-class d-radio__copy d-radio__label"
        );
        assert_eq!(
            radio_label_class("", "lg", "", false),
            "d-text d-text-label--lg d-text--tone-primary d-text--fw-normal d-radio__copy d-radio__label"
        );
        assert_eq!(
            radio_description_class("my-custom-class"),
            "d-text d-text-body--sm d-text--tone-tertiary d-description my-custom-class"
        );
        assert_eq!(
            radio_legend_class("my-custom-class"),
            "d-text d-text-label--md d-text--tone-secondary d-input-group__legend-text my-custom-class"
        );
    }

    #[test]
    fn report_skips_disabled_and_the_current_group_value() {
        assert!(should_report(false, None, "apple"));
        assert!(!should_report(true, None, "apple"));
        assert!(!should_report(false, Some("apple"), "apple"));
        assert!(should_report(false, Some("apple"), "banana"));
        assert!(!should_report(true, Some(""), "banana"));
    }
}
