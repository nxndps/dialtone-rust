use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, KeyboardEvent};

use super::icon::{
    DtIconAlertCircle, DtIconAlertTriangle, DtIconBell, DtIconCheckCircle, DtIconClose, DtIconInfo,
};
use super::unique_id;

/// Close button label. Dialtone `DIALTONE_CLOSE_BUTTON` in English.
const CLOSE_LABEL: &str = "Close";

/// Notice icon size in `DtNoticeIcon` (`size="400"`).
const NOTICE_ICON_SIZE: &str = "400";

const FOCUSABLE: &str = "button,[href],input,select,textarea,details,[tabindex]";
const FOCUSABLE_ATTRS: &str =
    ":not(:disabled):not([aria-disabled='true']):not([role='presentation'])";

fn kind_class(kind: &str) -> &'static str {
    match kind {
        "error" => "d-banner--error",
        "info" => "d-banner--info",
        "success" => "d-banner--success",
        "warning" => "d-banner--warning",
        "base" => "d-banner--base",
        _ => "",
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

fn banner_class(kind: &str, important: bool, pinned: bool) -> String {
    join_class([
        "d-banner",
        kind_class(kind),
        if important { "d-banner--important" } else { "" },
        if pinned { "d-banner--pinned" } else { "" },
    ])
}

fn background_style(image: &str, size: &str) -> Option<String> {
    if image.is_empty() {
        None
    } else {
        Some(format!(
            "background-image: url({image}); background-size: {size};"
        ))
    }
}

fn is_visible(element: &Element) -> bool {
    let Some(window) = web_sys::window() else {
        return true;
    };
    let Ok(Some(style)) = window.get_computed_style(element) else {
        return true;
    };
    let display = style.get_property_value("display").unwrap_or_default();
    let visibility = style.get_property_value("visibility").unwrap_or_default();
    display != "none" && visibility != "hidden"
}

fn matches_selector(element: &Element, selector: &str) -> bool {
    element.matches(selector).unwrap_or(false)
}

fn focusable_elements(root: &Element, include_negative_tab: bool) -> Vec<Element> {
    let Ok(list) = root.query_selector_all(FOCUSABLE) else {
        return Vec::new();
    };
    let selector = if include_negative_tab {
        FOCUSABLE_ATTRS
    } else {
        ":not(:disabled):not([aria-disabled='true']):not([role='presentation']):not([tabindex='-1'])"
    };
    let mut elements = Vec::new();
    for index in 0..list.length() {
        let Some(node) = list.item(index) else {
            continue;
        };
        let Ok(element) = node.dyn_into::<Element>() else {
            continue;
        };
        if is_visible(&element) && matches_selector(&element, selector) {
            elements.push(element);
        }
    }
    elements
}

fn first_focus<'a>(elements: &'a [Element]) -> Option<&'a Element> {
    let first = elements.first()?;
    if matches_selector(first, "[type='radio']:not(:checked)") {
        let name = first.get_attribute("name").unwrap_or_default();
        if let Some(checked) = elements.iter().find(|element| {
            matches_selector(element, "[type='radio']:checked")
                && element.get_attribute("name").as_deref() == Some(name.as_str())
        }) {
            return Some(checked);
        }
    }
    Some(first)
}

fn focus_element(element: &Element) {
    if let Some(html) = element.dyn_ref::<web_sys::HtmlElement>() {
        let _ = html.focus();
    }
}

fn focus_first(root: &Element) {
    let elements = focusable_elements(root, true);
    if let Some(element) = first_focus(&elements) {
        focus_element(element);
    }
}

/// Tab cycle from Dialtone `focusTrappedTabPress`.
fn trap_tab(event: &KeyboardEvent, root: &Element) {
    if event.key() != "Tab" {
        return;
    }
    let elements = focusable_elements(root, false);
    if elements.is_empty() {
        event.prevent_default();
        return;
    }
    let Some(first) = first_focus(&elements) else {
        event.prevent_default();
        return;
    };
    let last = elements.last().unwrap_or(first);
    let active = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.active_element());
    let on_edge = active.is_some_and(|current| {
        if event.shift_key() {
            current.is_same_node(Some(first))
        } else {
            current.is_same_node(Some(last))
        }
    });
    if on_edge {
        focus_element(if event.shift_key() { last } else { first });
        event.prevent_default();
    }
}

fn notice_icon(kind: &str) -> AnyView {
    match kind {
        "info" => view! { <DtIconInfo size=NOTICE_ICON_SIZE /> }.into_any(),
        "success" => view! { <DtIconCheckCircle size=NOTICE_ICON_SIZE /> }.into_any(),
        "warning" => view! { <DtIconAlertTriangle size=NOTICE_ICON_SIZE /> }.into_any(),
        "error" => view! { <DtIconAlertCircle size=NOTICE_ICON_SIZE /> }.into_any(),
        _ => view! { <DtIconBell size=NOTICE_ICON_SIZE /> }.into_any(),
    }
}

fn run_close(on_close: Option<Callback<()>>) {
    if let Some(on_close) = on_close {
        on_close.run(());
    }
}

/// Dialtone Vue `DtBanner` (`dialtone/v9.190.0`).
///
/// `kind` is `base` (default), `error`, `info`, `success`, or `warning`.
/// `important` sets `role="alertdialog"` and traps Tab inside the banner.
/// The default slot is the message. `title_override` replaces `title`.
#[component]
pub fn DtBanner(
    /// Title element id (`titleId`).
    #[prop(optional, into)]
    title_id: String,
    /// Message element id (`contentId`).
    #[prop(optional, into)]
    content_id: String,
    /// Header text (`title`). Empty omits the title.
    #[prop(optional, into)]
    title: String,
    /// Replaces `title` (`titleOverride`).
    #[prop(optional)]
    title_override: Option<ViewFn>,
    #[prop(optional)] important: bool,
    #[prop(optional)] pinned: bool,
    /// `base`, `error`, `info`, `success`, or `warning`.
    #[prop(optional, default = "base")]
    kind: &'static str,
    /// Hides the close button (`hideClose`).
    #[prop(optional)]
    hide_close: bool,
    /// Hides the kind icon (`hideIcon`).
    #[prop(optional)]
    hide_icon: bool,
    /// Hides the action slot (`hideAction`).
    #[prop(optional)]
    hide_action: bool,
    /// Extra class on `.d-banner__dialog` (`dialogClass`).
    #[prop(optional, into)]
    dialog_class: String,
    /// CSS `background-image` URL (`backgroundImage`).
    #[prop(optional, into)]
    background_image: String,
    /// CSS `background-size` (`backgroundSize`). Default `cover`.
    #[prop(optional, default = "cover")]
    background_size: &'static str,
    #[prop(optional, into)] on_close: Option<Callback<()>>,
    /// Custom icon slot (`icon`).
    #[prop(optional)]
    icon: Option<ViewFn>,
    /// Action slot (`action`).
    #[prop(optional)]
    action: Option<ViewFn>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let title_id = if title_id.is_empty() {
        unique_id("dt-banner-title")
    } else {
        title_id
    };
    let content_id = if content_id.is_empty() {
        unique_id("dt-banner-content")
    } else {
        content_id
    };
    let root_class = banner_class(kind, important, pinned);
    let dialog_class = join_class(["d-banner__dialog", dialog_class.as_str()]);
    let style = background_style(&background_image, background_size).unwrap_or_default();
    let role = if important { "alertdialog" } else { "status" };
    let has_title = !title.is_empty() || title_override.is_some();
    let node_ref = NodeRef::<html::Aside>::new();

    if !hide_close {
        let previous = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.active_element())
            .and_then(|element| element.dyn_into::<web_sys::HtmlElement>().ok());
        on_cleanup(move || {
            if let Some(element) = previous {
                let _ = element.focus();
            }
        });
    }
    if important {
        let node_ref = node_ref;
        request_animation_frame(move || {
            if let Some(element) = node_ref.get() {
                let element: HtmlElement = element.unchecked_into();
                focus_first(&element);
            }
        });
    }

    let close = on_close;
    view! {
        <aside
            node_ref=node_ref
            class=root_class
            style=style
            on:keydown=move |event: KeyboardEvent| {
                if !important {
                    return;
                }
                if let Some(element) = node_ref.get() {
                    let element: HtmlElement = element.unchecked_into();
                    trap_tab(&event, &element);
                }
            }
        >
            <div
                class=dialog_class
                role=role
                aria-labelledby=title_id.clone()
                aria-describedby=content_id.clone()
            >
                {(!hide_icon).then(|| {
                    view! {
                        <div class="d-notice__icon" aria-hidden="true">
                            {if let Some(icon) = icon {
                                icon.run().into_any()
                            } else {
                                notice_icon(kind)
                            }}
                        </div>
                    }
                })}
                <div class="d-notice__content" data-qa="notice-content">
                    {has_title.then(|| {
                        view! {
                            <p id=title_id.clone() class="d-notice__title" data-qa="notice-content-title">
                                {if let Some(title_override) = title_override {
                                    title_override.run().into_any()
                                } else {
                                    title.clone().into_any()
                                }}
                            </p>
                        }
                    })}
                    <p id=content_id.clone() class="d-notice__message" data-qa="notice-content-message">
                        {children.map(|children| children())}
                    </p>
                </div>
                <div class="d-notice__actions" data-qa="notice-content-actions">
                    {(!hide_action).then(|| action.map(|slot| slot.run()))}
                    {if hide_close {
                        let close = close;
                        view! {
                            <button
                                type="button"
                                class="base-button__button d-btn d-btn--primary d-vi-visible-sr"
                                data-qa="dt-sr-only-close-button"
                                aria-label=CLOSE_LABEL
                                title=CLOSE_LABEL
                                on:click=move |_| run_close(close)
                                on:keydown=move |event: KeyboardEvent| {
                                    if event.key() == " " || event.code() == "Space" {
                                        event.prevent_default();
                                        event.stop_propagation();
                                        run_close(close);
                                    }
                                }
                            >
                                <span data-qa="dt-button-label" class="base-button__label d-btn__label">
                                    {CLOSE_LABEL}
                                </span>
                            </button>
                        }
                        .into_any()
                    } else {
                        let close = close;
                        view! {
                            <button
                                type="button"
                                class="base-button__button d-btn d-btn--sm d-btn--circle d-btn--icon-only"
                                data-qa="dt-notice-action-close-button"
                                aria-label=CLOSE_LABEL
                                title=CLOSE_LABEL
                                on:click=move |_| run_close(close)
                            >
                                <span data-qa="dt-button-icon" class="base-button__icon d-btn__icon d-btn__icon--left">
                                    <DtIconClose size="200" />
                                </span>
                            </button>
                        }
                        .into_any()
                    }}
                </div>
            </div>
        </aside>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind_and_pinned_classes_match_vue() {
        assert_eq!(
            banner_class("base", false, false),
            "d-banner d-banner--base"
        );
        assert_eq!(
            banner_class("error", true, true),
            "d-banner d-banner--error d-banner--important d-banner--pinned"
        );
        assert_eq!(kind_class("success"), "d-banner--success");
        assert_eq!(kind_class("critical"), "");
    }

    #[test]
    fn background_style_is_omitted_when_the_image_is_empty() {
        assert_eq!(background_style("", "cover"), None);
        assert_eq!(
            background_style("/banner.png", "contain"),
            Some("background-image: url(/banner.png); background-size: contain;".to_string())
        );
    }
}
