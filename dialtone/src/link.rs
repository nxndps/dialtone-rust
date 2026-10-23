use leptos::prelude::*;
use web_sys::MouseEvent;

/// Vue `href` fallback when neither `href` nor `to` is set.
const HREF_FALLBACK: &str = "javascript:void(0)";

const TONES: &[&str] = &[
    "", "critical", "warning", "positive", "info", "muted", "mention", "unstyled",
];

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn known_tone(value: &str) -> bool {
    TONES.contains(&value)
}

/// `kind` wins over `tone`. Empty `kind` uses `tone`. Vue `kind ?? tone`.
fn resolved_tone<'a>(tone: &'a str, kind: &'a str) -> &'a str {
    if kind.is_empty() {
        tone
    } else {
        kind
    }
}

fn kind_modifier(tone: &str, inverted: bool) -> &'static str {
    if inverted {
        return match tone {
            "" | "default" => "d-link--inverted",
            "critical" => "d-link--inverted-critical",
            "warning" => "d-link--inverted-warning",
            "positive" => "d-link--inverted-positive",
            "info" => "d-link--inverted-info",
            "muted" => "d-link--inverted-muted",
            "mention" => "d-link--inverted-mention",
            _ => "",
        };
    }
    match tone {
        "critical" => "d-link--critical",
        "warning" => "d-link--warning",
        "positive" => "d-link--positive",
        "info" => "d-link--info",
        "muted" => "d-link--muted",
        "mention" => "d-link--mention",
        _ => "",
    }
}

fn link_classes(tone: &str, kind: &str, inverted: bool, underline: bool, class: &str) -> String {
    let resolved = resolved_tone(tone, kind);
    if resolved == "unstyled" {
        return join_class(["d-link--unstyled", class]);
    }
    let tone = if known_tone(resolved) { resolved } else { "" };
    join_class([
        "d-link",
        kind_modifier(tone, inverted),
        if underline {
            ""
        } else {
            "d-link--no-underline"
        },
        class,
    ])
}

fn link_href(href: &str) -> String {
    if href.is_empty() {
        HREF_FALLBACK.to_string()
    } else {
        href.to_string()
    }
}

/// Dialtone Vue `DtLink` (`dialtone/v10.7.0`).
///
/// The root is `<a data-qa="dt-link">`. `tone` is `""`, `critical`, `warning`,
/// `positive`, `info`, `muted`, `mention`, or `unstyled`. `kind` is deprecated
/// and wins over `tone`. `unstyled` emits only `d-link--unstyled`. `inverted`
/// is deprecated and still adds the inverted modifier. `underline` false adds
/// `d-link--no-underline`. An empty `href` is `javascript:void(0)`. Vue Router
/// `to` and `replace` are not ported. `class` is Vue `$attrs.class`.
#[component]
pub fn DtLink(
    /// Destination (`href`). Empty uses `javascript:void(0)`.
    #[prop(optional, into)]
    href: String,
    /// Link color (`tone`).
    #[prop(optional, into)]
    tone: String,
    /// Deprecated alias of `tone` (`kind`). A non-empty value wins.
    #[prop(optional, into)]
    kind: String,
    /// Deprecated inverted colors (`inverted`).
    #[prop(optional)]
    inverted: bool,
    /// Underline. False adds `d-link--no-underline`.
    #[prop(optional, default = true)]
    underline: bool,
    /// `target`. Empty omits the attribute.
    #[prop(optional, into)]
    target: String,
    /// `rel`. Empty omits the attribute.
    #[prop(optional, into)]
    rel: String,
    /// Element id. Empty omits the attribute.
    #[prop(optional, into)]
    id: String,
    /// Extra class (`$attrs.class`).
    #[prop(optional, into)]
    class: String,
    /// Native click.
    #[prop(optional, into)]
    on_click: Option<Callback<MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let resolved = resolved_tone(&tone, &kind);
    if !known_tone(resolved) {
        leptos::logging::warn!("DtLink tone `{resolved}` is not a link variant");
    }
    let class_name = link_classes(&tone, &kind, inverted, underline, &class);
    let href = link_href(&href);
    let id_attr = (!id.is_empty()).then_some(id);
    let target_attr = (!target.is_empty()).then_some(target);
    let rel_attr = (!rel.is_empty()).then_some(rel);
    view! {
        <a
            id=id_attr
            class=class_name
            data-qa="dt-link"
            href=href
            target=target_attr
            rel=rel_attr
            on:click=move |event: MouseEvent| {
                if let Some(on_click) = on_click {
                    on_click.run(event);
                }
            }
        >
            {children()}
        </a>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes_follow_the_vue_template() {
        assert_eq!(link_classes("", "", false, true, ""), "d-link");
        assert_eq!(
            link_classes("", "", false, false, ""),
            "d-link d-link--no-underline"
        );
        assert_eq!(
            link_classes("", "", true, true, ""),
            "d-link d-link--inverted"
        );
        assert_eq!(
            link_classes("critical", "", false, true, ""),
            "d-link d-link--critical"
        );
        assert_eq!(
            link_classes("warning", "", true, true, ""),
            "d-link d-link--inverted-warning"
        );
        assert_eq!(
            link_classes("positive", "", true, false, ""),
            "d-link d-link--inverted-positive d-link--no-underline"
        );
        assert_eq!(
            link_classes("info", "", false, true, ""),
            "d-link d-link--info"
        );
        assert_eq!(
            link_classes("muted", "", false, true, ""),
            "d-link d-link--muted"
        );
        assert_eq!(
            link_classes("mention", "", true, true, ""),
            "d-link d-link--inverted-mention"
        );
        assert_eq!(
            link_classes("unstyled", "", true, false, ""),
            "d-link--unstyled"
        );
        assert_eq!(
            link_classes("unstyled", "muted", false, true, ""),
            "d-link d-link--muted"
        );
        assert_eq!(
            link_classes("", "unstyled", true, false, "extra"),
            "d-link--unstyled extra"
        );
        assert_eq!(
            link_classes("muted", "", false, true, "extra"),
            "d-link d-link--muted extra"
        );
    }

    #[test]
    fn unknown_tone_omits_the_modifier() {
        assert_eq!(link_classes("nope", "", false, true, ""), "d-link");
        assert_eq!(
            link_classes("nope", "", true, true, ""),
            "d-link d-link--inverted"
        );
    }

    #[test]
    fn empty_href_uses_the_vue_fallback() {
        assert_eq!(link_href(""), HREF_FALLBACK);
        assert_eq!(link_href("https://example.com"), "https://example.com");
        assert_eq!(link_href("#link"), "#link");
    }
}
