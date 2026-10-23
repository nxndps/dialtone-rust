use leptos::prelude::*;

use crate::text::{text_classes, TextStyle};
use crate::{
    DtIconArrowDown, DtIconArrowLeft, DtIconArrowRight, DtIconArrowUp, DtIconCommand,
    DtIconLayoutGrid, DtIconOption, DtIconPlus,
};

/// Vue `SHORTCUTS_ALIASES_LIST`. `{plus}` is the `+` separator, not an alias.
pub const SHORTCUTS_ALIASES_LIST: &[&str] = &[
    "{win}",
    "{arrow-right}",
    "{arrow-left}",
    "{arrow-up}",
    "{arrow-down}",
    "{cmd}",
    "{opt}",
];

/// Split tokens, in the same order as Vue's `icons` key list (`{plus}` last).
const SPLIT_ALIASES: &[&str] = &[
    "{win}",
    "{arrow-right}",
    "{arrow-left}",
    "{arrow-up}",
    "{arrow-down}",
    "{cmd}",
    "{opt}",
    "{plus}",
];

#[derive(Debug, PartialEq, Eq)]
enum Part {
    Icon(&'static str),
    Text(String),
}

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn starts_with_ignore_ascii_case(haystack: &str, needle: &str) -> bool {
    let hay = haystack.as_bytes();
    let ned = needle.as_bytes();
    hay.len() >= ned.len() && hay[..ned.len()].eq_ignore_ascii_case(ned)
}

/// Vue splits on the alias regex after replacing `+` with `{plus}`.
/// The match is case-insensitive. The captured text keeps its original case.
fn split_shortcut(input: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut index = 0;
    let mut start = 0;
    while index < input.len() {
        if let Some(alias) = SPLIT_ALIASES
            .iter()
            .copied()
            .find(|alias| starts_with_ignore_ascii_case(&input[index..], alias))
        {
            if index > start {
                parts.push(&input[start..index]);
            }
            let end = index + alias.len();
            parts.push(&input[index..end]);
            index = end;
            start = end;
        } else {
            let step = input[index..]
                .chars()
                .next()
                .map(|ch| ch.len_utf8())
                .unwrap_or(1);
            index += step;
        }
    }
    if start < input.len() {
        parts.push(&input[start..]);
    }
    parts.retain(|part| !part.is_empty());
    parts
}

fn visible_parts(shortcut: &str) -> Vec<Part> {
    let substituted = shortcut.replace('+', "{plus}");
    split_shortcut(&substituted)
        .into_iter()
        .filter_map(|item| {
            if let Some(alias) = SPLIT_ALIASES.iter().copied().find(|alias| *alias == item) {
                Some(Part::Icon(alias))
            } else if item.trim().is_empty() {
                None
            } else {
                Some(Part::Text(item.to_string()))
            }
        })
        .collect()
}

fn alias_text(item: &str) -> Option<&'static str> {
    Some(match item {
        "{cmd}" => "Command",
        "{opt}" => "Option",
        "{win}" => "Windows",
        "{arrow-right}" => "Right Arrow",
        "{arrow-left}" => "Left Arrow",
        "{arrow-up}" => "Up Arrow",
        "{arrow-down}" => "Down Arrow",
        "{plus}" => "plus",
        _ => return None,
    })
}

fn key_abbreviation(item: &str) -> Option<&'static str> {
    Some(match item.to_ascii_lowercase().as_str() {
        "ctrl" => "Control",
        "alt" => "Alt",
        "esc" => "Escape",
        "del" => "Delete",
        "ins" => "Insert",
        "pgup" => "Page Up",
        "pgdn" => "Page Down",
        "num" => "Number",
        "caps" => "Caps Lock",
        _ => return None,
    })
}

fn readable_key(item: &str) -> String {
    if let Some(name) = alias_text(item) {
        return name.to_string();
    }
    if let Some(name) = key_abbreviation(item) {
        return name.to_string();
    }
    item.to_string()
}

fn generated_screen_reader_text(shortcut: &str) -> String {
    let substituted = shortcut.replace('+', "{plus}");
    split_shortcut(&substituted)
        .into_iter()
        .filter_map(|item| {
            let trimmed = item.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(readable_key(trimmed))
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn announced(screen_reader_text: &str, shortcut: &str) -> String {
    if screen_reader_text.is_empty() {
        generated_screen_reader_text(shortcut)
    } else {
        screen_reader_text.to_string()
    }
}

fn root_class(inverted: bool, class: &str) -> String {
    join_class([
        "d-keyboard-shortcut",
        if inverted {
            "d-keyboard-shortcut--inverted"
        } else {
            ""
        },
        class,
    ])
}

fn icon_extra_class(inverted: bool) -> String {
    join_class([
        "d-keyboard-shortcut__icon",
        if inverted {
            "d-keyboard-shortcut__icon--inverted"
        } else {
            ""
        },
    ])
}

fn item_text_classes(inverted: bool) -> String {
    let style = TextStyle {
        kind: "body",
        variant: "",
        size: "200",
        family: "",
        italic: false,
        tone: "tertiary",
        align: "",
        truncate: false,
        max_lines: 0,
        numeric: true,
        wrap: "",
        text_box_trim: "",
        strength: "",
        density: "400",
    };
    let text = text_classes(&style);
    join_class([
        text.as_str(),
        "d-keyboard-shortcut__item",
        if inverted {
            "d-keyboard-shortcut__item--inverted"
        } else {
            ""
        },
    ])
}

fn shortcut_icon(alias: &str, class_name: String) -> AnyView {
    // Size `100` is the Vue template. The icon package default is `500`.
    match alias {
        "{win}" => view! { <DtIconLayoutGrid size="100" class=class_name /> }.into_any(),
        "{arrow-right}" => view! { <DtIconArrowRight size="100" class=class_name /> }.into_any(),
        "{arrow-left}" => view! { <DtIconArrowLeft size="100" class=class_name /> }.into_any(),
        "{arrow-up}" => view! { <DtIconArrowUp size="100" class=class_name /> }.into_any(),
        "{arrow-down}" => view! { <DtIconArrowDown size="100" class=class_name /> }.into_any(),
        "{cmd}" => view! { <DtIconCommand size="100" class=class_name /> }.into_any(),
        "{opt}" => view! { <DtIconOption size="100" class=class_name /> }.into_any(),
        "{plus}" => view! { <DtIconPlus size="100" class=class_name /> }.into_any(),
        _ => ().into_any(),
    }
}

fn render_part(part: Part, icon_class: &str, text_class: &str) -> AnyView {
    match part {
        Part::Icon(alias) => shortcut_icon(alias, icon_class.to_string()),
        Part::Text(text) => {
            let text_class = text_class.to_string();
            view! {
                <span class=text_class data-qa="dt-text" aria-hidden="true">
                    <span>{text}</span>
                </span>
            }
            .into_any()
        }
    }
}

/// Dialtone Vue `DtKeyboardShortcut` (`dialtone/v10.7.0`).
///
/// The root is `<kbd class="d-keyboard-shortcut">`. `shortcut` is required.
/// `{cmd}`, `{opt}`, `{win}`, `{arrow-right}`, `{arrow-left}`, `{arrow-up}`,
/// and `{arrow-down}` draw icons at size `100`. Each `+` draws `DtIconPlus`.
/// Other text is the `DtText` body `200` list (`numeric`, density `400`, tone
/// `tertiary`) plus `d-keyboard-shortcut__item`. `DtText` has no `class` or
/// `aria-hidden`, so those classes sit on the span. Vue `v-html` is text.
///
/// `screen_reader_text` is `screenReaderText`. Empty uses the generated label.
/// `inverted` is deprecated and still adds the inverted classes. `class` is
/// Vue `$attrs.class` on the `<kbd>`.
///
/// Vue's `formattedShortcut` replace looks for `{{alias}}` because the alias
/// already includes braces, so it does not rewrite tokens at this tag. This
/// port splits the string after replacing `+` with `{plus}`.
#[component]
pub fn DtKeyboardShortcut(
    /// Key combination (`shortcut`).
    #[prop(into)]
    shortcut: String,
    /// Accessible label (`screenReaderText`). Empty uses the generated text.
    #[prop(optional, into)]
    screen_reader_text: String,
    /// Deprecated inverted colors (`inverted`).
    #[prop(optional)]
    inverted: bool,
    /// Extra class on the `<kbd>` (`$attrs.class`).
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let announced = announced(&screen_reader_text, &shortcut);
    let root = root_class(inverted, &class);
    let icon_class = icon_extra_class(inverted);
    let text_class = item_text_classes(inverted);
    let parts = visible_parts(&shortcut);
    view! {
        <kbd class=root>
            <span class="d-keyboard-shortcut--sr-only">{announced}</span>
            {parts
                .into_iter()
                .map(|part| render_part(part, &icon_class, &text_class))
                .collect_view()}
        </kbd>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes_follow_the_vue_template() {
        assert_eq!(root_class(false, ""), "d-keyboard-shortcut");
        assert_eq!(
            root_class(true, "d-fs-200"),
            "d-keyboard-shortcut d-keyboard-shortcut--inverted d-fs-200"
        );
        assert_eq!(icon_extra_class(false), "d-keyboard-shortcut__icon");
        assert_eq!(
            icon_extra_class(true),
            "d-keyboard-shortcut__icon d-keyboard-shortcut__icon--inverted"
        );
        assert_eq!(
            item_text_classes(false),
            "d-text d-text-body--sm d-text--tone-tertiary d-text--numeric d-text--lh-400 d-keyboard-shortcut__item"
        );
        assert_eq!(
            item_text_classes(true),
            "d-text d-text-body--sm d-text--tone-tertiary d-text--numeric d-text--lh-400 d-keyboard-shortcut__item d-keyboard-shortcut__item--inverted"
        );
    }

    #[test]
    fn every_alias_plus_separators_is_thirteen_icons() {
        let shortcut = SHORTCUTS_ALIASES_LIST.join("+");
        let icons = visible_parts(&shortcut)
            .into_iter()
            .filter(|part| matches!(part, Part::Icon(_)))
            .count();
        assert_eq!(icons, 13);
        assert_eq!(
            visible_parts(&shortcut)
                .into_iter()
                .filter_map(|part| match part {
                    Part::Icon(alias) => Some(alias),
                    Part::Text(_) => None,
                })
                .collect::<Vec<_>>(),
            vec![
                "{win}",
                "{plus}",
                "{arrow-right}",
                "{plus}",
                "{arrow-left}",
                "{plus}",
                "{arrow-up}",
                "{plus}",
                "{arrow-down}",
                "{plus}",
                "{cmd}",
                "{plus}",
                "{opt}",
            ]
        );
    }

    #[test]
    fn command_and_letter_keep_the_separator_icon() {
        assert_eq!(
            visible_parts("{cmd}+X"),
            vec![
                Part::Icon("{cmd}"),
                Part::Icon("{plus}"),
                Part::Text("X".to_string()),
            ]
        );
    }

    #[test]
    fn spaces_around_plus_stay_on_the_text() {
        assert_eq!(
            visible_parts("Ctrl + F5"),
            vec![
                Part::Text("Ctrl ".to_string()),
                Part::Icon("{plus}"),
                Part::Text(" F5".to_string()),
            ]
        );
    }

    #[test]
    fn a_different_alias_case_stays_text() {
        assert_eq!(
            visible_parts("{CMD}+X"),
            vec![
                Part::Text("{CMD}".to_string()),
                Part::Icon("{plus}"),
                Part::Text("X".to_string()),
            ]
        );
    }

    #[test]
    fn screen_reader_text_matches_the_vue_tests() {
        assert_eq!(announced("", "{cmd}+X"), "Command plus X");
        assert_eq!(
            announced("", "Ctrl+Alt+Del"),
            "Control plus Alt plus Delete"
        );
        assert_eq!(announced("", "{cmd}+Ctrl+X"), "Command plus Control plus X");
        assert_eq!(
            announced("", "{arrow-up}+{arrow-down}"),
            "Up Arrow plus Down Arrow"
        );
        assert_eq!(announced("", "{win}+D"), "Windows plus D");
        assert_eq!(announced("", "{opt}+C"), "Option plus C");
        assert_eq!(announced("", "Ctrl + F5"), "Control plus F5");
        assert_eq!(
            announced("", "Esc+Ins+PgUp+PgDn+Num+Caps"),
            "Escape plus Insert plus Page Up plus Page Down plus Number plus Caps Lock"
        );
        assert_eq!(
            announced("Custom accessible text", "{cmd}+X"),
            "Custom accessible text"
        );
        assert_eq!(announced("", "{CMD}+X"), "{CMD} plus X");
    }
}
