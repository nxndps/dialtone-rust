use leptos::prelude::*;

use super::icon::{DtIconAlertCircle, DtIconAlertTriangle, DtIconCheckCircle, DtIconInfo};
use super::unique_id;

/// One Dialtone validation message.
///
/// `kind` is the Vue `type`. [`DtValidationMessages`](crate::DtValidationMessages)
/// reads `critical`, `warning`, `positive`, and `info`. An empty kind means
/// `critical` there. [`DtSelectMenu`](crate::DtSelectMenu) still reads `error`,
/// `warning`, and `success` through [`validation_state`]. An empty kind means
/// `error` for that helper. [`DtRadio`](crate::DtRadio),
/// [`DtRadioGroup`](crate::DtRadioGroup), [`DtCheckbox`](crate::DtCheckbox),
/// [`DtCheckboxGroup`](crate::DtCheckboxGroup), and
/// [`DtComboboxMultiSelect`](crate::DtComboboxMultiSelect) read the v10 names.
/// An empty kind means `critical` for those. The text is rendered as text.
/// Vue's `v-html` is not used.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DtValidationMessage {
    pub message: String,
    pub kind: String,
}

impl DtValidationMessage {
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            kind: "error".into(),
        }
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            kind: "warning".into(),
        }
    }

    pub fn success(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            kind: "success".into(),
        }
    }

    /// v10 message type `critical`. [`DtRadio`](crate::DtRadio) reads this name.
    pub fn critical(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            kind: "critical".into(),
        }
    }

    /// v10 message type `positive`. [`DtRadio`](crate::DtRadio) reads this name.
    pub fn positive(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            kind: "positive".into(),
        }
    }

    /// v10 message type `info`. [`DtRadio`](crate::DtRadio) reads this name.
    pub fn info(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            kind: "info".into(),
        }
    }
}

fn normalized_kind(kind: &str) -> &str {
    if kind.is_empty() {
        "error"
    } else {
        kind
    }
}

/// v9 priority still used by [`DtSelectMenu`](crate::DtSelectMenu).
/// Error, then warning, then success. An empty kind means `error`.
pub(crate) fn validation_state(messages: &[DtValidationMessage]) -> Option<&'static str> {
    let mut warning = false;
    let mut success = false;
    for message in messages {
        match normalized_kind(&message.kind) {
            "error" => return Some("error"),
            "warning" => warning = true,
            "success" => success = true,
            _ => {}
        }
    }
    if warning {
        Some("warning")
    } else if success {
        Some("success")
    } else {
        None
    }
}

fn shown_messages(messages: &[DtValidationMessage]) -> Vec<DtValidationMessage> {
    let Some(state) = validation_state(messages) else {
        return Vec::new();
    };
    messages
        .iter()
        .filter(|message| !message.message.is_empty() && normalized_kind(&message.kind) == state)
        .cloned()
        .collect()
}

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn message_class(kind: &str) -> String {
    let kind_class = match normalized_kind(kind) {
        "warning" => "base-input__message--warning d-validation-message--warning",
        "success" => "base-input__message--success d-validation-message--success",
        "error" => "base-input__message--error d-validation-message--error",
        _ => "",
    };
    join_class(["base-input__message", "d-validation-message", kind_class])
}

/// Dialtone `DtValidationMessages` markup (`dialtone/v9.190.0`).
///
/// `data_qa` is the container `data-qa`. The public component uses
/// `validation-messages-container`. Other inputs pass their own value.
pub(crate) fn validation_messages(
    messages: Vec<DtValidationMessage>,
    show: bool,
    class_name: &str,
    data_qa: &'static str,
) -> AnyView {
    let shown = shown_messages(&messages);
    if !show || shown.is_empty() {
        return ().into_any();
    }
    let container = join_class([
        "base-input__messages",
        "d-validation-message__container",
        class_name,
    ]);
    let rows = shown
        .into_iter()
        .map(|message| {
            let class_name = message_class(&message.kind);
            view! {
                <div role="status" aria-live="polite" data-qa="validation-message" class=class_name>
                    <p>{message.message}</p>
                </div>
            }
        })
        .collect_view();
    let data_qa = (!data_qa.is_empty()).then_some(data_qa);
    view! {
        <div class=container data-qa=data_qa>
            {rows}
        </div>
    }
    .into_any()
}

fn formatted_kind(kind: &str) -> &str {
    if kind.is_empty() { "critical" } else { kind }
}

/// Dialtone `getValidationState` at `dialtone/v10.7.0`.
/// Priority is critical, then warning, then positive, then info.
fn formatted_message_state(messages: &[DtValidationMessage]) -> Option<&'static str> {
    let mut warning = false;
    let mut positive = false;
    let mut info = false;
    let mut critical = false;
    for message in messages {
        match formatted_kind(&message.kind) {
            "critical" => critical = true,
            "warning" => warning = true,
            "positive" => positive = true,
            "info" => info = true,
            other => {
                leptos::logging::warn!(
                    "[DtValidationMessages] message type \"{other}\" is not critical, warning, positive, or info."
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

fn formatted_row_class(kind: &str) -> String {
    let modifier = match kind {
        "warning" => "base-input__message--warning d-validation-message--warning",
        "critical" => "base-input__message--critical d-validation-message--critical",
        "positive" => "base-input__message--positive d-validation-message--positive",
        "info" => "base-input__message--info d-validation-message--info",
        _ => "",
    };
    join_class(["base-input__message", "d-validation-message", modifier])
}

fn validation_icon(kind: &str, icon_class: &str) -> AnyView {
    let class_name = join_class(["d-validation-message__icon", icon_class]);
    match kind {
        "warning" => view! { <DtIconAlertTriangle size="300" class=class_name /> }.into_any(),
        "critical" => view! { <DtIconAlertCircle size="300" class=class_name /> }.into_any(),
        "positive" => view! { <DtIconCheckCircle size="300" class=class_name /> }.into_any(),
        "info" => view! { <DtIconInfo size="300" class=class_name /> }.into_any(),
        _ => ().into_any(),
    }
}

/// Dialtone Vue `DtValidationMessages` (`dialtone/v10.7.0`).
///
/// The container stays mounted. `id` is rendered. An empty id uses
/// [`unique_id`](crate::unique_id). `show_messages` false leaves the container
/// empty. One kind wins: critical, then warning, then positive, then info.
/// The default icon is size `300`. `icon` replaces it and does not receive
/// Vue's `{ type }` slot prop. `DtIcon` keeps `data-qa="dt-icon"`. Message
/// text is escaped. Vue's `v-html` is not used. [`DtSelectMenu`](crate::DtSelectMenu)
/// still renders the v9 rows through [`validation_messages`].
#[component]
pub fn DtValidationMessages(
    /// Container id (`id`). Empty generates one.
    #[prop(optional, into)]
    id: String,
    /// Messages (`validationMessages`).
    #[prop(optional)]
    validation_messages: Vec<DtValidationMessage>,
    /// Show the rows (`showMessages`).
    #[prop(optional, default = true)]
    show_messages: bool,
    /// Extra class on the default icon (`iconClass`).
    #[prop(optional, into)]
    icon_class: String,
    /// Replaces the default icon (`icon`).
    #[prop(optional)]
    icon: Option<ViewFn>,
) -> impl IntoView {
    let id = if id.is_empty() {
        unique_id("dt")
    } else {
        id
    };
    let state = formatted_message_state(&validation_messages);
    let rows = if let Some(state) = state.filter(|_| show_messages) {
        validation_messages
            .into_iter()
            .filter(|message| !message.message.is_empty() && formatted_kind(&message.kind) == state)
            .map(|message| {
                let kind = formatted_kind(&message.kind).to_string();
                let class_name = formatted_row_class(&kind);
                let text = message.message;
                let icon_view = if let Some(slot) = &icon {
                    slot.run().into_any()
                } else {
                    validation_icon(&kind, &icon_class)
                };
                view! {
                    <div data-qa="validation-message" class=class_name>
                        {icon_view}
                        <p>{text}</p>
                    </div>
                }
            })
            .collect_view()
            .into_any()
    } else {
        ().into_any()
    };
    view! {
        <div
            id=id
            aria-live="assertive"
            class="base-input__messages d-validation-message__container"
            data-qa="validation-messages-container"
        >
            {rows}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_prefers_error_then_warning_then_success() {
        assert_eq!(validation_state(&[]), None);
        assert_eq!(
            validation_state(&[DtValidationMessage::success("ok")]),
            Some("success")
        );
        assert_eq!(
            validation_state(&[
                DtValidationMessage::success("ok"),
                DtValidationMessage::warning("careful"),
            ]),
            Some("warning")
        );
        assert_eq!(
            validation_state(&[
                DtValidationMessage::warning("careful"),
                DtValidationMessage::error("bad"),
            ]),
            Some("error")
        );
        assert_eq!(
            validation_state(&[DtValidationMessage {
                message: "bad".into(),
                kind: String::new(),
            }]),
            Some("error")
        );
    }

    #[test]
    fn shown_messages_keep_only_the_winning_non_empty_text() {
        let shown = shown_messages(&[
            DtValidationMessage::error(""),
            DtValidationMessage::error("bad"),
            DtValidationMessage::warning("careful"),
        ]);
        assert_eq!(shown, vec![DtValidationMessage::error("bad")]);
        assert!(shown_messages(&[DtValidationMessage::error("")]).is_empty());
    }

    #[test]
    fn v10_state_prefers_critical_then_warning_then_positive_then_info() {
        assert_eq!(formatted_message_state(&[]), None);
        assert_eq!(
            formatted_message_state(&[DtValidationMessage::info("note")]),
            Some("info")
        );
        assert_eq!(
            formatted_message_state(&[
                DtValidationMessage::info("note"),
                DtValidationMessage::positive("ok"),
            ]),
            Some("positive")
        );
        assert_eq!(
            formatted_message_state(&[
                DtValidationMessage::positive("ok"),
                DtValidationMessage::warning("careful"),
            ]),
            Some("warning")
        );
        assert_eq!(
            formatted_message_state(&[
                DtValidationMessage::warning("careful"),
                DtValidationMessage::critical("stop"),
            ]),
            Some("critical")
        );
        assert_eq!(
            formatted_message_state(&[DtValidationMessage {
                message: "stop".into(),
                kind: String::new(),
            }]),
            Some("critical")
        );
        assert_eq!(
            formatted_row_class("critical"),
            "base-input__message d-validation-message base-input__message--critical d-validation-message--critical"
        );
        assert_eq!(
            formatted_row_class("info"),
            "base-input__message d-validation-message base-input__message--info d-validation-message--info"
        );
    }
}
