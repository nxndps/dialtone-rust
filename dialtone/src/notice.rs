use leptos::prelude::*;

use super::notice_action::DtNoticeAction;
use super::notice_content::DtNoticeContent;
use super::notice_icon::DtNoticeIcon;

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn kind_class(kind: &str) -> Option<&'static str> {
    Some(match kind {
        "" | "base" => "d-notice--base",
        "critical" => "d-notice--critical",
        "info" => "d-notice--info",
        "positive" => "d-notice--positive",
        "warning" => "d-notice--warning",
        _ => return None,
    })
}

fn notice_role(role: &str) -> Option<&'static str> {
    Some(match role {
        "" | "status" => "status",
        "alert" => "alert",
        "alertdialog" => "alertdialog",
        _ => return None,
    })
}

/// Root class list. Order is `d-notice`, the kind, `d-notice--important`,
/// then `d-notice--truncate`. An unknown kind omits the kind class.
fn notice_class(kind: &str, important: bool, truncate: bool) -> String {
    let kind = if kind.is_empty() { "base" } else { kind };
    join_class([
        "d-notice",
        kind_class(kind).unwrap_or(""),
        if important { "d-notice--important" } else { "" },
        if truncate { "d-notice--truncate" } else { "" },
    ])
}

/// `iconClass` plus the title modifier Vue puts on the icon component.
fn passed_icon_class(icon_class: &str, has_title: bool) -> String {
    join_class([
        icon_class,
        if has_title {
            "d-notice__icon--has-title"
        } else {
            ""
        },
    ])
}

fn render_icon(
    show: bool,
    kind: &'static str,
    icon_class: String,
    icon: Option<ViewFn>,
) -> AnyView {
    if !show {
        return ().into_any();
    }
    if let Some(icon) = icon {
        view! { <DtNoticeIcon kind=kind icon_class=icon_class>{icon.run()}</DtNoticeIcon> }
            .into_any()
    } else {
        view! { <DtNoticeIcon kind=kind icon_class=icon_class /> }.into_any()
    }
}

fn render_content(
    header_text: String,
    header_id: String,
    content_id: String,
    header_class: String,
    content_class: String,
    role: &'static str,
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
                role=role
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
                role=role
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
                role=role
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
                role=role
            />
        }
        .into_any(),
    }
}

fn render_action(
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

/// Dialtone Vue `DtNotice` (`dialtone/v10.7.0`).
///
/// The root is `<aside class="d-notice" data-qa="notice">`. `kind` is `base`,
/// `critical`, `info`, `positive`, or `warning`. An unknown kind warns and
/// omits the kind class. `important` adds `d-notice--important` and does not
/// change `role`. `truncate_text` is `truncateText`. `role` is `status`
/// (default), `alert`, or `alertdialog`, and it is rendered on
/// `DtNoticeContent`. `on_close` is `close`. The Vue `click` emit is not
/// wired in the template.
#[component]
pub fn DtNotice(
    /// Title element id (`headerId`). Empty omits the attribute.
    #[prop(optional, into)]
    header_id: String,
    /// Message element id (`contentId`). Empty omits the attribute.
    #[prop(optional, into)]
    content_id: String,
    /// Title text (`headerText`). Empty omits the title unless `header` is set.
    #[prop(optional, into)]
    header_text: String,
    /// `status` (default), `alert`, or `alertdialog` (`role`).
    #[prop(optional, default = "status")]
    role: &'static str,
    /// Visually dominant notice (`important`).
    #[prop(optional)]
    important: bool,
    /// `base` (default), `critical`, `info`, `positive`, or `warning` (`kind`).
    #[prop(optional, default = "base")]
    kind: &'static str,
    /// Shows the close button (`showClose`).
    #[prop(optional, default = true)]
    show_close: bool,
    /// Shows the icon (`showIcon`).
    #[prop(optional, default = true)]
    show_icon: bool,
    /// Shows the action slot (`showAction`).
    #[prop(optional, default = true)]
    show_action: bool,
    /// Single-line title and message (`truncateText`).
    #[prop(optional)]
    truncate_text: bool,
    /// Extra class on the icon wrapper (`iconClass`).
    #[prop(optional, into)]
    icon_class: String,
    /// Extra class on the title (`headerClass`).
    #[prop(optional, into)]
    header_class: String,
    /// Extra class on `.d-notice__content` (`contentClass`).
    #[prop(optional, into)]
    content_class: String,
    /// Extra class on `.d-notice__actions` (`actionClass`).
    #[prop(optional, into)]
    action_class: String,
    /// Vue `close`.
    #[prop(optional, into)]
    on_close: Option<Callback<()>>,
    /// Custom icon slot (`icon`).
    #[prop(optional)]
    icon: Option<ViewFn>,
    /// Header slot (`header`). Replaces `headerText`.
    #[prop(optional)]
    header: Option<ViewFn>,
    /// Action slot (`action`).
    #[prop(optional)]
    action: Option<ViewFn>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let kind = if kind.is_empty() { "base" } else { kind };
    if kind_class(kind).is_none() {
        leptos::logging::warn!("[DtNotice] Unknown kind \"{kind}\".");
    }
    let role = match notice_role(role) {
        Some(role) => role,
        None => {
            leptos::logging::warn!("[DtNotice] Unknown role \"{role}\". Using \"status\".");
            "status"
        }
    };
    let class_name = notice_class(kind, important, truncate_text);
    let has_title = !header_text.is_empty() || header.is_some();
    let icon_class_name = passed_icon_class(&icon_class, has_title);
    let icon_view = render_icon(show_icon, kind, icon_class_name, icon);
    let content_view = render_content(
        header_text,
        header_id,
        content_id,
        header_class,
        content_class,
        role,
        header,
        children,
    );
    let action_view = render_action(show_action, show_close, action_class, on_close, action);
    view! {
        <aside class=class_name data-qa="notice">
            {icon_view}
            {content_view}
            {action_view}
        </aside>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notice_class_order_matches_the_vue_computed_array() {
        assert_eq!(
            notice_class("base", false, false),
            "d-notice d-notice--base"
        );
        assert_eq!(notice_class("", false, false), "d-notice d-notice--base");
        assert_eq!(
            notice_class("critical", true, true),
            "d-notice d-notice--critical d-notice--important d-notice--truncate"
        );
        assert_eq!(
            notice_class("positive", false, false),
            "d-notice d-notice--positive"
        );
        assert_eq!(notice_class("nope", false, false), "d-notice");
    }

    #[test]
    fn roles_are_status_alert_or_alertdialog() {
        assert_eq!(notice_role("status"), Some("status"));
        assert_eq!(notice_role(""), Some("status"));
        assert_eq!(notice_role("alert"), Some("alert"));
        assert_eq!(notice_role("alertdialog"), Some("alertdialog"));
        assert_eq!(notice_role("dialog"), None);
    }

    #[test]
    fn title_modifier_follows_the_icon_component_class() {
        assert_eq!(passed_icon_class("", false), "");
        assert_eq!(
            passed_icon_class("extra", true),
            "extra d-notice__icon--has-title"
        );
    }
}
