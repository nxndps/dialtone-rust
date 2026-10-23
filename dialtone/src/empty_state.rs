use leptos::prelude::*;

use super::stack::{DtStack, StackAt};

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Root classes for `DtEmptyState`, in the Vue computed order.
pub(crate) fn empty_state_classes(size: &str) -> String {
    let size_class = match size {
        "sm" => "d-empty-state--size-sm",
        "md" => "d-empty-state--size-md",
        "lg" => "d-empty-state--size-lg",
        _ => {
            leptos::logging::warn!("DtEmptyState size `{size}` is not sm, md, or lg");
            ""
        }
    };
    join_class(["d-empty-state", size_class])
}

pub(crate) fn empty_state_content_class(size: &str) -> &'static str {
    match size {
        "sm" => "d-empty-state__content--sm",
        "md" => "d-empty-state__content--md",
        "lg" => "d-empty-state__content--lg",
        _ => "",
    }
}

pub(crate) fn empty_state_headline_class(size: &str) -> &'static str {
    match size {
        "sm" => "d-headline--md",
        "md" => "d-headline--xl",
        "lg" => "d-headline--xxl",
        _ => "",
    }
}

pub(crate) fn empty_state_body_class(size: &str) -> &'static str {
    match size {
        "sm" | "md" => "d-body--sm",
        "lg" => "d-body--md",
        _ => "",
    }
}

/// Icon shows for `sm` even when an illustration is set. Larger sizes prefer the illustration.
pub(crate) fn show_empty_icon(has_icon: bool, has_illustration: bool, size: &str) -> bool {
    has_icon && (!has_illustration || size == "sm")
}

/// Illustration is hidden at `sm`.
pub(crate) fn show_empty_illustration(has_illustration: bool, size: &str) -> bool {
    has_illustration && size != "sm"
}

/// Dialtone Vue `DtEmptyState` (`dialtone/v9.190.0`).
///
/// The root is [`DtStack`](crate::DtStack). The content stack uses `gap` `450`.
/// Vue passes `icon-size` `800` into the icon slot. Size the icon in that slot.
/// `d-empty-state--size-lg`, `d-empty-state__content--md`, and
/// `d-empty-state__content--lg` are emitted and are not in the vendored CSS.
/// Provide `body_text` or the `body` slot. Vue logs an error when both are absent.
#[component]
pub fn DtEmptyState(
    /// `sm`, `md`, or `lg` (`size`). Default `lg`.
    #[prop(optional, default = "lg")]
    size: &'static str,
    /// Header text (`headerText`).
    #[prop(into)]
    header_text: String,
    /// Body copy (`bodyText`). Empty omits the paragraph.
    #[prop(optional, into)]
    body_text: String,
    /// Illustration slot. Hidden when `size` is `sm`. Replaces the icon at `md` and `lg`.
    #[prop(optional)]
    illustration: Option<ViewFn>,
    /// Icon slot. Shown when there is no illustration, and always at `sm`.
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
    let headline_class = join_class([
        "d-empty-state__header-text",
        empty_state_headline_class(size),
    ]);
    let body_class = join_class(["d-empty-state__body-text", empty_state_body_class(size)]);
    let body_present = !body_text.is_empty();
    view! {
        <DtStack class=root_class>
            <Show when=move || show_illustration>
                <span class="d-empty-state__illustration">
                    {illustration.clone().map(|slot| slot.run())}
                </span>
            </Show>
            <Show when=move || show_icon>
                <span class="d-empty-state__icon">
                    {icon.clone().map(|slot| slot.run())}
                </span>
            </Show>
            <DtStack gap=StackAt::all("450") class=content_class>
                <div class=headline_class>{header_text}</div>
                <Show when=move || body_present>
                    <p class=body_class.clone()>{body_text.clone()}</p>
                </Show>
            </DtStack>
            {body.map(|slot| slot.run())}
        </DtStack>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes_follow_the_size_map() {
        assert_eq!(
            empty_state_classes("lg"),
            "d-empty-state d-empty-state--size-lg"
        );
        assert_eq!(
            empty_state_classes("md"),
            "d-empty-state d-empty-state--size-md"
        );
        assert_eq!(empty_state_headline_class("lg"), "d-headline--xxl");
        assert_eq!(empty_state_headline_class("md"), "d-headline--xl");
        assert_eq!(empty_state_body_class("md"), "d-body--sm");
        assert_eq!(empty_state_body_class("lg"), "d-body--md");
        assert_eq!(
            empty_state_content_class("sm"),
            "d-empty-state__content--sm"
        );
        assert_eq!(empty_state_classes("xl"), "d-empty-state");
    }

    #[test]
    fn small_size_prefers_the_icon() {
        assert!(show_empty_illustration(true, "lg"));
        assert!(!show_empty_illustration(true, "sm"));
        assert!(!show_empty_icon(true, true, "lg"));
        assert!(show_empty_icon(true, true, "sm"));
        assert!(show_empty_icon(true, false, "md"));
    }
}
