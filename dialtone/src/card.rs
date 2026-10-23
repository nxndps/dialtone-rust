use leptos::prelude::*;

/// Dialtone Vue `DtCard`.
///
/// Slots: `header` (optional), default/`content`, `footer` (optional).
/// When `max_height` is set, the content area scrolls.
#[component]
pub fn DtCard(
    /// Max height of the content area (`maxHeight` in Vue). Makes content scrollable.
    #[prop(optional, into)]
    max_height: String,
    /// Extra class on `.d-card` (`containerClass`).
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
    let container = if container_class.is_empty() {
        "d-card".to_string()
    } else {
        format!("d-card {container_class}")
    };
    let header_cls = if header_class.is_empty() {
        "d-card__header".to_string()
    } else {
        format!("d-card__header {header_class}")
    };
    let content_cls = if content_class.is_empty() {
        "d-card__content".to_string()
    } else {
        format!("d-card__content {content_class}")
    };
    let footer_cls = if footer_class.is_empty() {
        "d-card__footer".to_string()
    } else {
        format!("d-card__footer {footer_class}")
    };
    let content_style = if max_height.is_empty() {
        String::new()
    } else {
        format!("max-height:{max_height}")
    };

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
