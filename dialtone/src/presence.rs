use leptos::prelude::*;

const PRESENCE_CHECK: &str = "M8.63559 2.44125C8.87637 2.1042 9.34444 2.02566 9.68149 2.26644C10.0185 2.5072 10.096 2.9753 9.85531 3.31234L5.3934 9.55941C5.25261 9.7564 5.02519 9.87289 4.78305 9.87289C4.54091 9.87288 4.31348 9.75641 4.1727 9.55941L2.14438 6.71957C1.90382 6.38254 1.98223 5.91439 2.31918 5.67367C2.65612 5.43319 3.12431 5.51077 3.36508 5.8475L4.78305 7.83285L8.63559 2.44125Z";
const PRESENCE_MINUS: &str = "M8.2334 5.25C8.64754 5.25009 8.9834 5.58584 8.9834 6C8.9834 6.41416 8.64754 6.74991 8.2334 6.75H3.7666C3.35249 6.74988 3.0166 6.41414 3.0166 6C3.0166 5.58586 3.35249 5.25012 3.7666 5.25H8.2334Z";

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn presence_modifier(presence: &str) -> &'static str {
    match presence {
        "active" => "d-presence__inner--active",
        "away" => "d-presence__inner--away",
        "busy" => "d-presence__inner--busy",
        "offline" => "d-presence__inner--offline",
        "dnd" => "d-presence__inner--dnd",
        _ => {
            leptos::logging::warn!(
                "[DtPresence] `presence` must be one of busy, away, active, offline, dnd. Received {presence}."
            );
            ""
        }
    }
}

fn presence_root_class(class: &str) -> String {
    join_class(["d-presence", class])
}

fn presence_inner_class(presence: &str) -> String {
    join_class(["d-presence__inner", presence_modifier(presence)])
}

/// Glyph for `active`, `busy`, and `dnd`. `away` is an empty SVG. `offline` and
/// unknown states draw nothing.
fn presence_icon(presence: &str) -> Option<AnyView> {
    let qa = match presence {
        "active" => "dt-presence-active-icon",
        "away" => "dt-presence-away-icon",
        "busy" => "dt-presence-busy-icon",
        "dnd" => "dt-presence-dnd-icon",
        _ => return None,
    };
    let path = match presence {
        "active" => Some(PRESENCE_CHECK),
        "busy" | "dnd" => Some(PRESENCE_MINUS),
        _ => None,
    };
    Some(
        view! {
            <svg
                class="d-presence__icon"
                data-qa=qa
                viewBox="0 0 12 12"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
            >
                {path.map(|d| view! { <path d=d fill="currentColor" /> })}
            </svg>
        }
        .into_any(),
    )
}

/// Dialtone Vue `DtPresence`.
///
/// `presence` is `busy`, `away`, `active` (default), `offline`, or `dnd`.
/// `sr_text` is `srText`. `show_icon` is `showIcon` and defaults to true.
/// `aria_live` is the fallthrough `ariaLive` attribute and defaults to `off`.
/// `class` is `$attrs.class` on the root.
#[component]
pub fn DtPresence(
    /// Status color (`presence`). Default `active`.
    #[prop(optional, default = "active")]
    presence: &'static str,
    /// Screen-reader text (`srText`). Empty omits the span.
    #[prop(optional, into)]
    sr_text: String,
    /// Draws the state glyph (`showIcon`). Default true.
    #[prop(optional, default = true)]
    show_icon: bool,
    /// `aria-live` on the root (`ariaLive`). Default `off`.
    #[prop(optional, default = "off")]
    aria_live: &'static str,
    /// Extra class on the root (`$attrs.class`).
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let root = presence_root_class(&class);
    let inner = presence_inner_class(presence);
    let live = if aria_live.is_empty() {
        "off"
    } else {
        aria_live
    };
    let icon = (show_icon && presence != "offline").then(|| presence_icon(presence));
    view! {
        <div class=root data-qa="dt-presence" role="status" aria-live=live>
            {(!sr_text.is_empty()).then(|| view! {
                <span data-qa="dt-presence-sr-text" class="sr-only">{sr_text}</span>
            })}
            <div class=inner data-qa="dt-presence-inner">
                {icon.flatten()}
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_class_appends_the_extra_class() {
        assert_eq!(presence_root_class(""), "d-presence");
        assert_eq!(presence_root_class("m-0"), "d-presence m-0");
    }

    #[test]
    fn inner_class_matches_each_state() {
        assert_eq!(
            presence_inner_class("active"),
            "d-presence__inner d-presence__inner--active"
        );
        assert_eq!(
            presence_inner_class("away"),
            "d-presence__inner d-presence__inner--away"
        );
        assert_eq!(
            presence_inner_class("busy"),
            "d-presence__inner d-presence__inner--busy"
        );
        assert_eq!(
            presence_inner_class("offline"),
            "d-presence__inner d-presence__inner--offline"
        );
        assert_eq!(
            presence_inner_class("dnd"),
            "d-presence__inner d-presence__inner--dnd"
        );
    }

    #[test]
    fn unknown_presence_omits_the_modifier() {
        assert_eq!(presence_inner_class("online"), "d-presence__inner");
    }
}
