use leptos::prelude::*;

/// Extra class on `.d-card`. Empty skips the extra, which is the Rust
/// `container_class` behavior. Vue `DtCard` has no `containerClass`.
pub(crate) fn card_root_class(container_class: &str) -> String {
    if container_class.is_empty() {
        "d-card".to_string()
    } else {
        format!("d-card {container_class}")
    }
}

/// Base part class plus an extra class. Empty skips the extra.
pub(crate) fn card_part_class(part: &str, extra: &str) -> String {
    if extra.is_empty() {
        part.to_string()
    } else {
        format!("{part} {extra}")
    }
}

/// `max-height` for `.d-card__content`. Empty omits the style attribute.
pub(crate) fn card_content_style(max_height: &str) -> Option<String> {
    if max_height.is_empty() {
        None
    } else {
        Some(format!("max-height:{max_height}"))
    }
}

/// Dialtone Vue `DtCard` (`dialtone/v10.7.0`).
///
/// Slots: `header` (optional), default/`content`, `footer` (optional).
/// `max_height` (`maxHeight`) sets `max-height` on `.d-card__content` and
/// makes that area scroll. An empty value omits the style. `container_class`
/// is a Rust extra on `.d-card`. Vue has no `containerClass`. An empty value
/// is omitted. Header and footer render only when the slot is set.
#[component]
pub fn DtCard(
    /// Max height of the content area (`maxHeight`). Makes content scrollable.
    #[prop(optional, into)]
    max_height: String,
    /// Extra class on `.d-card`. Not a Vue prop. Empty is omitted.
    #[prop(optional, into)]
    container_class: String,
    /// Extra class on `.d-card__content` (`contentClass`).
    #[prop(optional, into)]
    content_class: String,
    /// Extra class on `.d-card__header` (`headerClass`).
    #[prop(optional, into)]
    header_class: String,
    /// Extra class on `.d-card__footer` (`footerClass`).
    #[prop(optional, into)]
    footer_class: String,
    /// Header slot. Omitted when unset, matching Vue `v-if="hasSlotContent($slots.header)"`.
    #[prop(optional)]
    header: Option<ViewFn>,
    /// Footer slot. Omitted when unset.
    #[prop(optional)]
    footer: Option<ViewFn>,
    /// Main content (`#content` slot in Vue).
    children: ChildrenFn,
) -> impl IntoView {
    let container = card_root_class(&container_class);
    let header_cls = card_part_class("d-card__header", &header_class);
    let content_cls = card_part_class("d-card__content", &content_class);
    let footer_cls = card_part_class("d-card__footer", &footer_class);
    let content_style = card_content_style(&max_height);

    view! {
        <div class=container data-qa="dt-card">
            {header.as_ref().map(|slot| {
                view! {
                    <div class=header_cls.clone()>
                        {slot.run()}
                    </div>
                }
            })}
            <div class=content_cls style=content_style>
                {children()}
            </div>
            {footer.as_ref().map(|slot| {
                view! {
                    <div class=footer_cls.clone()>
                        {slot.run()}
                    </div>
                }
            })}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_skips_an_empty_container_class() {
        assert_eq!(card_root_class(""), "d-card");
        assert_eq!(card_root_class("extra"), "d-card extra");
    }

    #[test]
    fn part_class_appends_an_extra_class() {
        assert_eq!(card_part_class("d-card__header", ""), "d-card__header");
        assert_eq!(
            card_part_class("d-card__content", "d-of-y-auto"),
            "d-card__content d-of-y-auto"
        );
    }

    #[test]
    fn content_style_omits_an_empty_max_height() {
        assert_eq!(card_content_style(""), None);
        assert_eq!(
            card_content_style("12rem"),
            Some("max-height:12rem".to_string())
        );
    }
}
