use leptos::prelude::*;

use super::icon::{
    DtIconAlertCircle, DtIconAlertTriangle, DtIconBell, DtIconCheckCircle, DtIconInfo,
};

/// Icon size on the default notice icon. Vue `size="400"`.
const NOTICE_ICON_SIZE: &str = "400";

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// CSS suffix for the default icon. Unknown kinds have no icon.
fn notice_icon_name(kind: &str) -> Option<&'static str> {
    Some(match kind {
        "" | "base" => "bell",
        "info" => "info",
        "positive" => "check-circle",
        "warning" => "alert-triangle",
        "critical" => "alert-circle",
        _ => return None,
    })
}

fn icon_classes(icon_class: &str) -> String {
    join_class(["d-notice__icon", icon_class])
}

fn default_icon(kind: &str) -> Option<AnyView> {
    let size = NOTICE_ICON_SIZE;
    Some(match notice_icon_name(kind)? {
        "info" => view! { <DtIconInfo size=size /> }.into_any(),
        "check-circle" => view! { <DtIconCheckCircle size=size /> }.into_any(),
        "alert-triangle" => view! { <DtIconAlertTriangle size=size /> }.into_any(),
        "alert-circle" => view! { <DtIconAlertCircle size=size /> }.into_any(),
        "bell" => view! { <DtIconBell size=size /> }.into_any(),
        _ => return None,
    })
}

/// Dialtone Vue `DtNoticeIcon`.
///
/// `kind` is `base` (default), `critical`, `info`, `positive`, or `warning`.
/// An unknown kind warns and draws no default icon. The default slot replaces
/// that icon. The wrapper is omitted when there is neither. `iconClass` is
/// `icon_class`.
#[component]
pub fn DtNoticeIcon(
    /// `base`, `critical`, `info`, `positive`, or `warning` (`kind`).
    #[prop(optional, default = "base")]
    kind: &'static str,
    /// Extra class on the wrapper (`iconClass`).
    #[prop(optional, into)]
    icon_class: String,
    /// Custom icon. Replaces the kind icon.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let kind = if kind.is_empty() { "base" } else { kind };
    if notice_icon_name(kind).is_none() {
        leptos::logging::warn!("[DtNoticeIcon] Unknown kind \"{kind}\".");
    }
    let class_name = icon_classes(&icon_class);
    let icon = default_icon(kind);
    if children.is_none() && icon.is_none() {
        return ().into_any();
    }
    view! {
        <div aria-hidden="true" class=class_name>
            {if let Some(children) = children {
                children().into_any()
            } else {
                icon.unwrap_or_else(|| ().into_any())
            }}
        </div>
    }
    .into_any()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kinds_map_to_the_dialtone_icons() {
        assert_eq!(notice_icon_name("base"), Some("bell"));
        assert_eq!(notice_icon_name(""), Some("bell"));
        assert_eq!(notice_icon_name("info"), Some("info"));
        assert_eq!(notice_icon_name("positive"), Some("check-circle"));
        assert_eq!(notice_icon_name("warning"), Some("alert-triangle"));
        assert_eq!(notice_icon_name("critical"), Some("alert-circle"));
        assert_eq!(notice_icon_name("error"), None);
    }

    #[test]
    fn icon_class_order_matches_the_vue_array() {
        assert_eq!(icon_classes(""), "d-notice__icon");
        assert_eq!(
            icon_classes("extra d-notice__icon--has-title"),
            "d-notice__icon extra d-notice__icon--has-title"
        );
    }
}
