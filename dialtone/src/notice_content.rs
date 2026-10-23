use leptos::prelude::*;

use super::text::{text_classes, TextStyle};

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn text_style<'a>(kind: &'a str, size: &'a str, density: &'a str, wrap: &'a str) -> TextStyle<'a> {
    TextStyle {
        kind,
        variant: "",
        size,
        family: "",
        italic: false,
        tone: "",
        align: "",
        truncate: false,
        max_lines: 0,
        numeric: false,
        wrap,
        text_box_trim: "",
        strength: "",
        density,
    }
}

fn content_classes(content_class: &str) -> String {
    join_class(["d-notice__content", content_class])
}

/// Title classes. Vue `DtText` headline `300`, density `200`, wrap `pretty`,
/// plus `d-notice__title` and `headerClass`.
fn title_classes(header_class: &str) -> String {
    let text = text_classes(&text_style("headline", "300", "200", "pretty"));
    join_class([text.as_str(), "d-notice__title", header_class])
}

/// Message classes. Vue `DtText` body `200`, wrap `pretty`, plus `d-notice__message`.
fn message_classes() -> String {
    let text = text_classes(&text_style("body", "200", "", "pretty"));
    join_class([text.as_str(), "d-notice__message"])
}

/// Dialtone Vue `DtNoticeContent`.
///
/// The title renders when `headerText` is non-empty or the `header` slot is
/// set. The message renders when `message` is non-empty or the default slot
/// is set. `DtText` has no `id`, `class`, or `data-qa`, so those sit on a
/// `<p>` with the `DtText` class list. `role` is the Vue fallthrough attribute
/// on this root. An empty role omits it.
#[component]
pub fn DtNoticeContent(
    /// Header text (`headerText`). Empty omits the title unless `header` is set.
    #[prop(optional, into)]
    header_text: String,
    /// Message text (`message`). The default slot replaces it.
    #[prop(optional, into)]
    message: String,
    /// Title element id (`headerId`). Empty omits the attribute.
    #[prop(optional, into)]
    header_id: String,
    /// Message element id (`contentId`). Empty omits the attribute.
    #[prop(optional, into)]
    content_id: String,
    /// Extra class on the title (`headerClass`).
    #[prop(optional, into)]
    header_class: String,
    /// Extra class on the root (`contentClass`).
    #[prop(optional, into)]
    content_class: String,
    /// Fallthrough `role` from `DtNotice`. Empty omits the attribute.
    #[prop(optional, default = "")]
    role: &'static str,
    /// Header slot (`header`). Replaces `headerText`.
    #[prop(optional)]
    header: Option<ViewFn>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let class_name = content_classes(&content_class);
    let title_class = title_classes(&header_class);
    let message_class = message_classes();
    let show_title = !header_text.is_empty() || header.is_some();
    let show_message = !message.is_empty() || children.is_some();
    let header_id = (!header_id.is_empty()).then_some(header_id);
    let content_id = (!content_id.is_empty()).then_some(content_id);
    let role = (!role.is_empty()).then_some(role);
    view! {
        <div class=class_name data-qa="notice-content" role=role>
            {show_title.then(|| view! {
                <p id=header_id class=title_class data-qa="notice-content-title">
                    {if let Some(header) = header {
                        header.run().into_any()
                    } else {
                        header_text.into_any()
                    }}
                </p>
            })}
            {show_message.then(|| view! {
                <p id=content_id class=message_class data-qa="notice-content-message">
                    {if let Some(children) = children {
                        children().into_any()
                    } else {
                        message.into_any()
                    }}
                </p>
            })}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_title_and_message_classes_follow_vue() {
        assert_eq!(content_classes(""), "d-notice__content");
        assert_eq!(content_classes("extra"), "d-notice__content extra");
        assert_eq!(
            title_classes(""),
            "d-text d-text-headline--md d-text--wrap-pretty d-text--lh-200 d-notice__title"
        );
        assert_eq!(
            title_classes("extra"),
            "d-text d-text-headline--md d-text--wrap-pretty d-text--lh-200 d-notice__title extra"
        );
        assert_eq!(
            message_classes(),
            "d-text d-text-body--sm d-text--wrap-pretty d-notice__message"
        );
    }
}
