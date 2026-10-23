use leptos::prelude::*;

use super::button::DtButton;
use super::icon::DtIconClose;

/// Close control label. Dialtone `DIALTONE_CLOSE_BUTTON` in en-US.
const CLOSE_LABEL: &str = "Click to close";

/// Visible close icon size. Vue sets `size="200"` on the icon, not from the button.
const CLOSE_ICON_SIZE: &str = "200";

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn action_classes(action_class: &str) -> String {
    join_class(["d-notice__actions", action_class])
}

#[cfg(target_arch = "wasm32")]
fn active_html_element() -> Option<web_sys::HtmlElement> {
    use wasm_bindgen::JsCast;
    web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.active_element())
        .and_then(|element| element.dyn_into().ok())
}

#[cfg(target_arch = "wasm32")]
fn focus_is_in_a_field() -> bool {
    let Some(active) = active_html_element() else {
        return false;
    };
    let tag = active.tag_name().to_ascii_lowercase();
    matches!(tag.as_str(), "input" | "textarea" | "select") || active.is_content_editable()
}

/// Dialtone Vue `DtNoticeAction`.
///
/// `showClose` shows `DtButton` importance `clear`, kind `muted`, size `sm`
/// (Vue `200`), `data-qa="dt-notice-action-close-button"`. The icon is
/// `DtIconClose` size `200`. `showClose` false renders the visually hidden
/// close button (`data-qa="dt-sr-only-close-button"`) and the icon is its label.
/// `on_close` is `close`. While the visible close button is mounted, unmount
/// returns focus to the element that was active on mount, unless focus is in
/// an input, textarea, select, or contenteditable.
#[component]
pub fn DtNoticeAction(
    /// Shows the action slot (`showAction`).
    #[prop(optional, default = true)]
    show_action: bool,
    /// Shows the visible close button (`showClose`).
    #[prop(optional, default = true)]
    show_close: bool,
    /// Extra class on the wrapper (`actionClass`).
    #[prop(optional, into)]
    action_class: String,
    /// Vue `close`.
    #[prop(optional, into)]
    on_close: Option<Callback<()>>,
    /// Action slot.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    if show_close {
        let previous = active_html_element();
        on_cleanup(move || {
            if focus_is_in_a_field() {
                return;
            }
            if let Some(element) = previous {
                let _ = element.focus();
            }
        });
    }

    let class_name = action_classes(&action_class);
    let dismiss = Callback::new(move |()| {
        if let Some(on_close) = on_close {
            on_close.run(());
        }
    });
    view! {
        <div class=class_name data-qa="notice-content-actions">
            {show_action.then(|| children.map(|children| children()))}
            {if show_close {
                view! {
                    <DtButton
                        importance="clear"
                        kind="muted"
                        size="sm"
                        data_qa="dt-notice-action-close-button"
                        aria_label=CLOSE_LABEL
                        title=CLOSE_LABEL
                        icon=ViewFn::from(|| view! { <DtIconClose size=CLOSE_ICON_SIZE /> })
                        on_click=dismiss
                    />
                }
                .into_any()
            } else {
                view! {
                    <DtButton
                        id="sr-only-close-button"
                        class="d-vi-visible-sr"
                        data_qa="dt-sr-only-close-button"
                        aria_label=CLOSE_LABEL
                        title=CLOSE_LABEL
                        on_click=dismiss
                    >
                        <DtIconClose />
                    </DtButton>
                }
                .into_any()
            }}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_class_order_matches_the_vue_array() {
        assert_eq!(action_classes(""), "d-notice__actions");
        assert_eq!(action_classes("extra"), "d-notice__actions extra");
    }
}
