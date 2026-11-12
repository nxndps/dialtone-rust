use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, KeyboardEvent};

use super::notice_action::DtNoticeAction;
use super::notice_content::DtNoticeContent;
use super::notice_icon::DtNoticeIcon;
use super::unique_id;

const FOCUSABLE: &str = "button,[href],input,select,textarea,details,[tabindex]";
const FOCUSABLE_ATTRS: &str =
    ":not(:disabled):not([aria-disabled='true']):not([role='presentation'])";

fn kind_class(kind: &str) -> &'static str {
    match kind {
        "critical" => "d-banner--critical",
        "info" => "d-banner--info",
        "positive" => "d-banner--positive",
        "warning" => "d-banner--warning",
        "base" | "" => "d-banner--base",
        _ => {
            leptos::logging::warn!("[DtBanner] Unknown kind \"{kind}\".");
            ""
        }
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

fn active_element() -> Option<HtmlElement> {
    web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.active_element())
        .and_then(|element| element.dyn_into::<HtmlElement>().ok())
}

/// `DtNoticeContent.header` and `children` are optional props whose callers
/// pass the inner value. Pass each only in the arm where it is set.
fn render_banner_content(
    header_text: String,
    header_id: String,
    content_id: String,
    header_class: String,
    content_class: String,
    header: Option<ViewFn>,
    children: Option<ChildrenFn>,
) -> AnyView {
    match (header, children) {
        (Some(header), Some(children)) => view! {
            <DtNoticeContent
                header_text=header_text
                header_id=header_id
                content_id=content_id
                header_class=header_class
                content_class=content_class
                header=header
            >
                {children()}
            </DtNoticeContent>
        }
        .into_any(),
        (Some(header), None) => view! {
            <DtNoticeContent
                header_text=header_text
                header_id=header_id
                content_id=content_id
                header_class=header_class
                content_class=content_class
                header=header
            />
        }
        .into_any(),
        (None, Some(children)) => view! {
            <DtNoticeContent
                header_text=header_text
                header_id=header_id
                content_id=content_id
                header_class=header_class
                content_class=content_class
            >
                {children()}
            </DtNoticeContent>
        }
        .into_any(),
        (None, None) => view! {
            <DtNoticeContent
                header_text=header_text
                header_id=header_id
                content_id=content_id
                header_class=header_class
                content_class=content_class
            />
        }
        .into_any(),
    }
}

/// `DtNoticeAction.on_close` is an optional prop. Pass the callback only when set.
fn render_banner_action(
    show_action: bool,
    show_close: bool,
    action_class: String,
    on_close: Option<Callback<()>>,
    action: Option<ViewFn>,
) -> AnyView {
    match (action, on_close) {
        (Some(action), Some(on_close)) => view! {
            <DtNoticeAction
                show_action=show_action
                show_close=show_close
                action_class=action_class
                on_close=on_close
            >
                {action.run()}
            </DtNoticeAction>
        }
        .into_any(),
        (Some(action), None) => view! {
            <DtNoticeAction
                show_action=show_action
                show_close=show_close
                action_class=action_class
            >
                {action.run()}
            </DtNoticeAction>
        }
        .into_any(),
        (None, Some(on_close)) => view! {
            <DtNoticeAction
                show_action=show_action
                show_close=show_close
                action_class=action_class
                on_close=on_close
            />
        }
        .into_any(),
        (None, None) => view! {
            <DtNoticeAction
                show_action=show_action
                show_close=show_close
                action_class=action_class
            />
        }
        .into_any(),
    }
}

/// Dialtone Vue `DtBanner` (`dialtone/v10.7.0`).
///
/// Composes [`DtNoticeIcon`](crate::DtNoticeIcon), [`DtNoticeContent`](crate::DtNoticeContent),
/// and [`DtNoticeAction`](crate::DtNoticeAction). It does not compose `DtNotice`:
/// the role and `aria-modal` sit on `.d-banner__dialog`. `kind` is `base`
/// (default), `critical`, `info`, `positive`, or `warning`. An unknown kind
/// warns and omits the kind class. `header_text` is `headerText`. The `header`
/// slot replaces it. `show_close`, `show_icon`, and `show_action` default to
/// true. `important` sets `role="alertdialog"`, `aria-modal="true"`, traps Tab,
/// and restores focus on unmount. `aria-labelledby` is set only when there is
/// a header. The close control comes from `DtNoticeAction` ("Click to close").
#[component]
pub fn DtBanner(
    /// Header element id (`headerId`). Empty generates one.
    #[prop(optional, into)]
    header_id: String,
    /// Message element id (`contentId`). Empty generates one.
    #[prop(optional, into)]
    content_id: String,
    /// Header text (`headerText`). Empty omits the title unless `header` is set.
    #[prop(optional, into)]
    header_text: String,
    /// Header slot (`header`). Replaces `headerText`.
    #[prop(optional)]
    header: Option<ViewFn>,
    #[prop(optional)] important: bool,
    #[prop(optional)] pinned: bool,
    /// `base`, `critical`, `info`, `positive`, or `warning`.
    #[prop(optional, default = "base")]
    kind: &'static str,
    /// Shows the close button (`showClose`).
    #[prop(optional, default = true)]
    show_close: bool,
    /// Shows the kind icon (`showIcon`).
    #[prop(optional, default = true)]
    show_icon: bool,
    /// Shows the action slot (`showAction`).
    #[prop(optional, default = true)]
    show_action: bool,
    /// Extra class on `.d-banner__dialog` (`dialogClass`).
    #[prop(optional, into)]
    dialog_class: String,
    /// CSS `background-image` URL (`backgroundImage`).
    #[prop(optional, into)]
    background_image: String,
    /// CSS `background-size` (`backgroundSize`). Default `cover`.
    #[prop(optional, default = "cover")]
    background_size: &'static str,
    /// Extra class on the icon wrapper (`iconClass`).
    #[prop(optional, into)]
    icon_class: String,
    /// Extra class on the title (`headerClass`).
    #[prop(optional, into)]
    header_class: String,
    /// Extra class on the content (`contentClass`).
    #[prop(optional, into)]
    content_class: String,
    /// Extra class on the actions (`actionClass`).
    #[prop(optional, into)]
    action_class: String,
    #[prop(optional, into)] on_close: Option<Callback<()>>,
    /// Custom icon slot (`icon`).
    #[prop(optional)]
    icon: Option<ViewFn>,
    /// Action slot (`action`).
    #[prop(optional)]
    action: Option<ViewFn>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let header_id = if header_id.is_empty() {
        unique_id("dt-banner-title")
    } else {
        header_id
    };
    let content_id = if content_id.is_empty() {
        unique_id("dt-banner-content")
    } else {
        content_id
    };
    let root_class = banner_class(kind, important, pinned);
    let dialog_class = join_class(["d-banner__dialog", dialog_class.as_str()]);
    let style = background_style(&background_image, background_size);
    let role = if important { "alertdialog" } else { "status" };
    let has_header = !header_text.is_empty() || header.is_some();
    let labelled_by = has_header.then(|| header_id.clone());
    let aria_modal = important.then_some("true");
    let icon_class_name = join_class([
        icon_class.as_str(),
        if has_header {
            "d-notice__icon--has-title"
        } else {
            ""
        },
    ]);
    let node_ref = NodeRef::<html::Aside>::new();

    if important {
        let previous = active_element();
        on_cleanup(move || {
            if let Some(element) = previous {
                let _ = element.focus();
            }
        });
        let node_ref = node_ref;
        request_animation_frame(move || {
            if let Some(element) = node_ref.get() {
                let element: HtmlElement = element.unchecked_into();
                focus_first(&element);
            }
        });
    }

    let described_by = content_id.clone();
    view! {
        <aside
            node_ref=node_ref
            class=root_class
            style=style
            data-qa="dt-banner"
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
                data-qa="dt-banner-dialog"
                role=role
                aria-modal=aria_modal
                aria-labelledby=labelled_by
                aria-describedby=described_by
            >
                {show_icon.then(|| {
                    let icon_class_name = icon_class_name.clone();
                    if let Some(icon) = icon {
                        view! {
                            <DtNoticeIcon kind=kind icon_class=icon_class_name>
                                {icon.run()}
                            </DtNoticeIcon>
                        }
                        .into_any()
                    } else {
                        view! {
                            <DtNoticeIcon kind=kind icon_class=icon_class_name />
                        }
                        .into_any()
                    }
                })}
                {render_banner_content(
                    header_text,
                    header_id,
                    content_id,
                    header_class,
                    content_class,
                    header,
                    children,
                )}
                {render_banner_action(show_action, show_close, action_class, on_close, action)}
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
            banner_class("critical", true, true),
            "d-banner d-banner--critical d-banner--important d-banner--pinned"
        );
        assert_eq!(kind_class("positive"), "d-banner--positive");
        assert_eq!(kind_class("info"), "d-banner--info");
        assert_eq!(kind_class("warning"), "d-banner--warning");
        assert_eq!(kind_class("error"), "");
        assert_eq!(kind_class("success"), "");
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
