use leptos::prelude::*;

/// One Dialtone validation message.
///
/// `kind` is the Vue `type`. The shared renderer uses `error`, `warning`, or
/// `success`. An empty kind means `error` there, matching a bare string in the
/// v9 `messages` prop. [`DtRadio`](crate::DtRadio) and
/// [`DtRadioGroup`](crate::DtRadioGroup) read the v10 names `critical`,
/// `warning`, `positive`, and `info`. An empty kind means `critical` for those
/// two. The text is rendered as text. Vue's `v-html` is not used.
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

/// Dialtone `getValidationState`. Priority is error, then warning, then success.
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

fn render_validation_messages(messages: Vec<DtValidationMessage>, show: bool) -> AnyView {
    validation_messages(messages, show, "", "validation-messages-container")
}

/// Dialtone Vue `DtValidationMessages` (`dialtone/v9.190.0`).
///
/// `validation_messages` is `validationMessages`. `show_messages` is
/// `showMessages` and defaults to true. `id` is the Vue list key and is not
/// rendered. Message text is escaped. Vue's `v-html` is not used. One kind
/// wins: error, then warning, then success.
#[component]
pub fn DtValidationMessages(
    /// Vue list key. Not rendered (`id`).
    #[prop(optional, into)]
    id: String,
    /// Messages (`validationMessages`).
    #[prop(optional)]
    validation_messages: Vec<DtValidationMessage>,
    /// Show the messages (`showMessages`).
    #[prop(optional, default = true)]
    show_messages: bool,
) -> impl IntoView {
    let _ = id;
    render_validation_messages(validation_messages, show_messages)
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
}
