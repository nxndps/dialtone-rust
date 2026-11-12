use leptos::prelude::*;

use super::stack::{DtStack, StackAt};
use super::text::{text_classes, TextStyle};

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn empty_text<'a>(kind: &'a str, size: &'a str, tone: &'a str, density: &'a str) -> TextStyle<'a> {
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
        wrap: "balance",
        text_box_trim: "",
        strength: "",
        density,
    }
}

fn is_small(size: &str) -> bool {
    size == "sm" || size == "200"
}

/// Root classes for `DtEmptyState`, in the Vue computed order.
pub(crate) fn empty_state_classes(size: &str) -> String {
    let size_class = match size {
        "200" | "sm" => "d-empty-state--size-sm",
        "300" | "md" => "d-empty-state--size-md",
        "400" | "lg" => "d-empty-state--size-lg",
        _ => {
            leptos::logging::warn!(
                "DtEmptyState size `{size}` is not 200, 300, 400, sm, md, or lg"
            );
            ""
        }
    };
    join_class(["d-empty-state", size_class])
}

pub(crate) fn empty_state_content_class(size: &str) -> &'static str {
    match size {
        "200" | "sm" => "d-empty-state__content--sm",
        "300" | "md" => "d-empty-state__content--md",
        "400" | "lg" => "d-empty-state__content--lg",
        _ => "",
    }
}

/// Headline `DtText` classes plus `d-empty-state__header-text`.
/// `200`/`sm` is headline `md` density `200`. `300`/`md` is `xl` density `200`.
/// `400`/`lg` is `2xl` with no density.
pub(crate) fn empty_state_headline_class(size: &str) -> String {
    let text = match size {
        "200" | "sm" => text_classes(&empty_text("headline", "md", "", "200")),
        "300" | "md" => text_classes(&empty_text("headline", "xl", "", "200")),
        "400" | "lg" => text_classes(&empty_text("headline", "2xl", "", "")),
        _ => String::new(),
    };
    join_class([text.as_str(), "d-empty-state__header-text"])
}

/// Body `DtText` classes plus `d-empty-state__body-text`.
/// Tone is `secondary` and wrap is `balance`.
pub(crate) fn empty_state_body_class(size: &str) -> String {
    let text = match size {
        "200" | "sm" => text_classes(&empty_text("body", "sm", "secondary", "")),
        "300" | "md" => text_classes(&empty_text("body", "md", "secondary", "300")),
        "400" | "lg" => text_classes(&empty_text("body", "lg", "secondary", "300")),
        _ => String::new(),
    };
    join_class([text.as_str(), "d-empty-state__body-text"])
}

/// Icon shows for `sm` and `200` even when an illustration is set.
/// Larger sizes prefer the illustration.
pub(crate) fn show_empty_icon(has_icon: bool, has_illustration: bool, size: &str) -> bool {
    has_icon && (!has_illustration || is_small(size))
}

/// Illustration is hidden at `sm` and `200`.
pub(crate) fn show_empty_illustration(has_illustration: bool, size: &str) -> bool {
    has_illustration && !is_small(size)
}

/// Dialtone Vue `DtEmptyState` (`dialtone/v10.7.0`).
///
/// The root is [`DtStack`](crate::DtStack). The content stack uses `gap` `150`.
/// Size is `200`, `300`, or `400` (default `400`). `sm`, `md`, and `lg` are the
/// deprecated aliases. Vue passes `icon-size` `800` into the icon slot. Size the
/// icon in that slot. Headline and body use the `DtText` class list because
/// `DtText` has no `class`. `d-empty-state--size-lg`, `d-empty-state__content--md`,
/// and `d-empty-state__content--lg` are emitted and are not in the vendored CSS.
/// Provide `body_text` or the `body` slot. Vue logs an error when both are absent.
#[component]
pub fn DtEmptyState(
    /// `200`, `300`, or `400` (`size`). `sm`, `md`, and `lg` still work. Default `400`.
    #[prop(optional, default = "400")]
    size: &'static str,
    /// Header text (`headerText`).
    #[prop(into)]
    header_text: String,
    /// Body copy (`bodyText`). Empty omits the paragraph.
    #[prop(optional, into)]
    body_text: String,
    /// Extra class on the illustration wrapper (`illustrationClass`).
    #[prop(optional, into)]
    illustration_class: String,
    /// Extra class on the icon wrapper (`iconClass`).
    #[prop(optional, into)]
    icon_class: String,
    /// Extra class on the body slot wrapper (`bodyClass`).
    #[prop(optional, into)]
    body_class: String,
    /// Illustration slot. Hidden when `size` is `sm` or `200`. Replaces the icon at larger sizes.
    #[prop(optional)]
    illustration: Option<ViewFn>,
    /// Icon slot. Shown when there is no illustration, and always at `sm` or `200`.
    #[prop(optional)]
    icon: Option<ViewFn>,
    /// Extra body (`body` slot).
    #[prop(optional)]
    body: Option<ViewFn>,
) -> impl IntoView {
    if body_text.is_empty() && body.is_none() {
        leptos::logging::warn!(
            "DtEmptyState: You should provide either bodyText or content on body slot."
        );
    }
    let show_illustration = show_empty_illustration(illustration.is_some(), size);
    let show_icon = show_empty_icon(icon.is_some(), illustration.is_some(), size);
    let root_class = empty_state_classes(size);
    let content_class = join_class(["d-empty-state__content", empty_state_content_class(size)]);
    let headline_class = empty_state_headline_class(size);
    let body_text_class = empty_state_body_class(size);
    let illustration_class_name =
        join_class(["d-empty-state__illustration", illustration_class.as_str()]);
    let icon_class_name = join_class(["d-empty-state__icon", icon_class.as_str()]);
    let body_present = !body_text.is_empty();
    view! {
        <DtStack class=root_class>
            <Show when=move || show_illustration>
                <span class=illustration_class_name.clone()>
                    {illustration.clone().map(|slot| slot.run())}
                </span>
            </Show>
            <Show when=move || show_icon>
                <span class=icon_class_name.clone()>
                    {icon.clone().map(|slot| slot.run())}
                </span>
            </Show>
            <DtStack gap=StackAt::all("150") class=content_class>
                <div class=headline_class>{header_text}</div>
                <Show when=move || body_present>
                    <p class=body_text_class.clone()>{body_text.clone()}</p>
                </Show>
            </DtStack>
            {body.map(|slot| view! { <div class=body_class>{slot.run()}</div> })}
        </DtStack>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes_follow_the_size_map() {
        assert_eq!(
            empty_state_classes("400"),
            "d-empty-state d-empty-state--size-lg"
        );
        assert_eq!(
            empty_state_classes("lg"),
            "d-empty-state d-empty-state--size-lg"
        );
        assert_eq!(
            empty_state_classes("300"),
            "d-empty-state d-empty-state--size-md"
        );
        assert_eq!(
            empty_state_classes("md"),
            "d-empty-state d-empty-state--size-md"
        );
        assert_eq!(
            empty_state_classes("200"),
            "d-empty-state d-empty-state--size-sm"
        );
        assert_eq!(
            empty_state_headline_class("lg"),
            "d-text d-text-headline--2xl d-text--wrap-balance d-empty-state__header-text"
        );
        assert_eq!(
            empty_state_headline_class("400"),
            "d-text d-text-headline--2xl d-text--wrap-balance d-empty-state__header-text"
        );
        assert_eq!(
            empty_state_headline_class("md"),
            "d-text d-text-headline--xl d-text--wrap-balance d-text--lh-200 d-empty-state__header-text"
        );
        assert_eq!(
            empty_state_headline_class("sm"),
            "d-text d-text-headline--md d-text--wrap-balance d-text--lh-200 d-empty-state__header-text"
        );
        assert_eq!(
            empty_state_body_class("sm"),
            "d-text d-text-body--sm d-text--tone-secondary d-text--wrap-balance d-empty-state__body-text"
        );
        assert_eq!(
            empty_state_body_class("md"),
            "d-text d-text-body--md d-text--tone-secondary d-text--wrap-balance d-text--lh-300 d-empty-state__body-text"
        );
        assert_eq!(
            empty_state_body_class("lg"),
            "d-text d-text-body--lg d-text--tone-secondary d-text--wrap-balance d-text--lh-300 d-empty-state__body-text"
        );
        assert_eq!(
            empty_state_content_class("sm"),
            "d-empty-state__content--sm"
        );
        assert_eq!(
            empty_state_content_class("200"),
            "d-empty-state__content--sm"
        );
        assert_eq!(empty_state_classes("xl"), "d-empty-state");
        assert_eq!(
            empty_state_headline_class("xl"),
            "d-empty-state__header-text"
        );
    }

    #[test]
    fn small_size_prefers_the_icon() {
        assert!(show_empty_illustration(true, "lg"));
        assert!(show_empty_illustration(true, "400"));
        assert!(!show_empty_illustration(true, "sm"));
        assert!(!show_empty_illustration(true, "200"));
        assert!(!show_empty_icon(true, true, "lg"));
        assert!(!show_empty_icon(true, true, "300"));
        assert!(show_empty_icon(true, true, "sm"));
        assert!(show_empty_icon(true, true, "200"));
        assert!(show_empty_icon(true, false, "md"));
    }
}
