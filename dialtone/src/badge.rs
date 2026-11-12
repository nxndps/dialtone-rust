use leptos::prelude::*;

fn type_class(kind: &str) -> &'static str {
    match kind {
        "info" => "d-badge--info",
        // `success` is the v9 name. v10 emits `d-badge--positive` for both.
        "positive" | "success" => "d-badge--positive",
        "warning" => "d-badge--warning",
        "critical" => "d-badge--critical",
        "bulletin" => "d-badge--bulletin",
        "ai" => "d-badge--ai",
        _ => "",
    }
}

fn decoration_class(decoration: &str) -> &'static str {
    match decoration {
        "black-400" => "d-badge--decorate-black-400",
        "black-500" => "d-badge--decorate-black-500",
        "black-900" => "d-badge--decorate-black-900",
        "red-200" => "d-badge--decorate-red-200",
        "red-300" => "d-badge--decorate-red-300",
        "red-400" => "d-badge--decorate-red-400",
        "purple-200" => "d-badge--decorate-purple-200",
        "purple-300" => "d-badge--decorate-purple-300",
        "purple-400" => "d-badge--decorate-purple-400",
        "purple-500" => "d-badge--decorate-purple-500",
        "blue-200" => "d-badge--decorate-blue-200",
        "blue-300" => "d-badge--decorate-blue-300",
        "blue-400" => "d-badge--decorate-blue-400",
        "green-300" => "d-badge--decorate-green-300",
        "green-400" => "d-badge--decorate-green-400",
        "green-500" => "d-badge--decorate-green-500",
        "gold-300" => "d-badge--decorate-gold-300",
        "gold-400" => "d-badge--decorate-gold-400",
        "gold-500" => "d-badge--decorate-gold-500",
        "magenta-200" => "d-badge--decorate-magenta-200",
        "magenta-300" => "d-badge--decorate-magenta-300",
        "magenta-400" => "d-badge--decorate-magenta-400",
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

fn badge_classes(
    type_name: &str,
    kind: &str,
    decoration: &str,
    subtle: bool,
    outlined: bool,
) -> String {
    join_class([
        "d-badge",
        type_class(type_name),
        if kind == "count" {
            "d-badge--count"
        } else {
            ""
        },
        decoration_class(decoration),
        if subtle { "d-badge--subtle" } else { "" },
        if outlined { "d-badge--outlined" } else { "" },
    ])
}

/// Vue `validateProps`. The class helper stays quiet so tests can assert the alias.
fn warn_badge(type_: &str, kind: &str, decoration: &str, subtle: bool, has_icons: bool) {
    if type_ == "success" {
        leptos::logging::warn!("DtBadge type `success` is deprecated. Use `positive`.");
    } else if !matches!(
        type_,
        "default" | "info" | "positive" | "warning" | "critical" | "bulletin" | "ai"
    ) {
        leptos::logging::warn!(
            "DtBadge type `{type_}` is not default, info, positive, warning, critical, bulletin, or ai"
        );
    }
    if !matches!(kind, "label" | "count") {
        leptos::logging::warn!("DtBadge kind `{kind}` is not label or count");
    }
    if type_ == "ai" && kind == "count" {
        leptos::logging::warn!(
            "DtBadge error: type: 'ai' with kind: 'count' is an invalid combination."
        );
    }
    if type_ != "bulletin" && subtle {
        leptos::logging::warn!("DtBadge error: subtle can only be used with type 'bulletin'");
    }
    if decoration.is_empty() {
        return;
    }
    if kind != "label" || type_ != "default" {
        leptos::logging::warn!(
            "DtBadge error: decoration prop can only be used with kind: 'label' and type: 'default'."
        );
    }
    if has_icons {
        leptos::logging::warn!(
            "DtBadge error: decoration prop cannot be used with startIcon or endIcon."
        );
    }
    if decoration_class(decoration).is_empty() {
        leptos::logging::warn!("DtBadge decoration `{decoration}` is not a badge decoration token");
    }
}

/// Dialtone Vue `DtBadge` (`dialtone/v10.7.0`).
///
/// `type` is `default`, `info`, `positive`, `warning`, `critical`, `bulletin`, or `ai`.
/// `success` is the deprecated alias of `positive` and emits `d-badge--positive`.
/// `kind` is `label` (default) or `count`. `decoration` is a palette token such as
/// `purple-400` and is meant for `kind: label` and `type: default` with no icons.
/// `subtle` only applies to `bulletin`. `ai` with `count` warns.
/// `start_icon` (`startIcon`) wins over the deprecated `left_icon` (`leftIcon`).
/// `end_icon` (`endIcon`) wins over `right_icon` (`rightIcon`).
/// `icon_size` is the token callers should put on slotted icons (default `"200"`).
#[component]
pub fn DtBadge(
    /// Icon token for slotted icons (`iconSize`). Default `"200"`.
    #[prop(optional, default = "200")]
    icon_size: &'static str,
    /// Fallback label when the default slot is absent (`text`).
    #[prop(optional, into)]
    text: String,
    /// `label` (default) or `count`.
    #[prop(optional, default = "label")]
    kind: &'static str,
    /// Color (`type` in Vue).
    #[prop(optional, default = "default")]
    type_: &'static str,
    /// Decorative dot token (`decoration`).
    #[prop(optional, into)]
    decoration: String,
    /// Extra class on `.d-badge__label` (`labelClass`).
    #[prop(optional, into)]
    label_class: String,
    /// Extra class on the start icon (`startIconClass`).
    #[prop(optional, into)]
    start_icon_class: String,
    /// Extra class on the end icon (`endIconClass`).
    #[prop(optional, into)]
    end_icon_class: String,
    /// Subtle bulletin treatment (`subtle`).
    #[prop(optional)]
    subtle: bool,
    #[prop(optional)] outlined: bool,
    /// Start icon (`startIcon`). Wins over `left_icon`.
    #[prop(optional)]
    start_icon: Option<ViewFn>,
    /// Deprecated start icon (`leftIcon`).
    #[prop(optional)]
    left_icon: Option<ViewFn>,
    /// End icon (`endIcon`). Wins over `right_icon`.
    #[prop(optional)]
    end_icon: Option<ViewFn>,
    /// Deprecated end icon (`rightIcon`).
    #[prop(optional)]
    right_icon: Option<ViewFn>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let _icon_size = icon_size;
    let start_slot = start_icon.or(left_icon);
    let end_slot = end_icon.or(right_icon);
    warn_badge(
        type_,
        kind,
        &decoration,
        subtle,
        start_slot.is_some() || end_slot.is_some(),
    );
    let root = badge_classes(type_, kind, &decoration, subtle, outlined);
    let label = join_class(["d-badge__label", label_class.as_str()]);
    let start_class = join_class(["d-badge__icon-left", start_icon_class.as_str()]);
    let end_class = join_class(["d-badge__icon-right", end_icon_class.as_str()]);
    let show_decoration = !decoration.is_empty();
    view! {
        <span class=root data-qa="dt-badge">
            {show_decoration.then(|| view! { <span class="d-badge__decorative"></span> })}
            {start_slot.map(|slot| view! {
                <span class=start_class>{slot.run()}</span>
            })}
            <span class=label>
                {if let Some(children) = children {
                    children().into_any()
                } else {
                    text.into_any()
                }}
            </span>
            {end_slot.map(|slot| view! {
                <span class=end_class>{slot.run()}</span>
            })}
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes_follow_the_vue_modifiers() {
        assert_eq!(
            badge_classes("default", "label", "", false, false),
            "d-badge"
        );
        assert_eq!(
            badge_classes("positive", "count", "", false, true),
            "d-badge d-badge--positive d-badge--count d-badge--outlined"
        );
        assert_eq!(
            badge_classes("success", "count", "", false, true),
            "d-badge d-badge--positive d-badge--count d-badge--outlined"
        );
        assert_eq!(
            badge_classes("bulletin", "label", "", true, false),
            "d-badge d-badge--bulletin d-badge--subtle"
        );
        assert_eq!(
            badge_classes("default", "label", "purple-400", false, false),
            "d-badge d-badge--decorate-purple-400"
        );
        assert_eq!(decoration_class("nope"), "");
        assert_eq!(type_class("ai"), "d-badge--ai");
    }
}
