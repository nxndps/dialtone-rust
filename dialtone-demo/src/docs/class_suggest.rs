//! Suggestions for the playground Class tab.
//!
//! The catalog is generated from the Dialtone stylesheets and the component
//! docs (`build.rs`). An empty field offers classes for the page's component.
//! Typed text also searches the rest of the stylesheet.

include!(concat!(env!("OUT_DIR"), "/class_catalog.rs"));

/// Page id from a demo hash such as `#/components/button`.
pub(crate) fn page_id_from_hash(hash: &str) -> &str {
    let path = hash.trim().trim_start_matches('#').trim_start_matches('/');
    let rest = path.strip_prefix("components/").unwrap_or("");
    rest.split(['/', '?', '#']).next().unwrap_or("")
}

pub(crate) fn current_page_id() -> String {
    let hash = web_sys::window()
        .and_then(|window| window.location().hash().ok())
        .unwrap_or_default();
    page_id_from_hash(&hash).to_string()
}

/// Byte range of the class token that contains `caret`.
pub(crate) fn token_span(value: &str, caret: usize) -> (usize, usize) {
    let bytes = value.as_bytes();
    let caret = caret.min(bytes.len());
    let mut start = caret;
    while start > 0 && !bytes[start - 1].is_ascii_whitespace() {
        start -= 1;
    }
    let mut end = caret;
    while end < bytes.len() && !bytes[end].is_ascii_whitespace() {
        end += 1;
    }
    (start, end)
}

/// Replace the token at `caret` with `class_name` and leave the caret after it.
pub(crate) fn apply_token(value: &str, caret: usize, class_name: &str) -> (String, usize) {
    let (start, end) = token_span(value, caret);
    let mut next = String::with_capacity(value.len() + class_name.len() + 1);
    next.push_str(&value[..start]);
    next.push_str(class_name);
    let rest = &value[end..];
    if rest.is_empty() {
        next.push(' ');
    } else if rest.starts_with(|ch: char| ch.is_ascii_whitespace()) {
        next.push_str(rest);
    } else {
        next.push(' ');
        next.push_str(rest);
    }
    let caret_out = (start + class_name.len() + 1).min(next.len());
    (next, caret_out)
}

pub(crate) fn utf16_to_byte(value: &str, utf16_index: usize) -> usize {
    let mut seen = 0;
    for (byte, ch) in value.char_indices() {
        if seen >= utf16_index {
            return byte;
        }
        seen += ch.len_utf16();
    }
    value.len()
}

pub(crate) fn byte_to_utf16(value: &str, byte_index: usize) -> usize {
    let mut seen = 0;
    for (byte, ch) in value.char_indices() {
        if byte >= byte_index {
            return seen;
        }
        seen += ch.len_utf16();
    }
    seen
}

/// Up to `limit` class names for this component field.
///
/// `label` is the field label (`"Class"`, `"Label class"`). `caret` is a byte
/// index into `value`. The token at the caret is the filter.
pub(crate) fn suggestions(
    page_id: &str,
    label: &str,
    value: &str,
    caret: usize,
    limit: usize,
) -> Vec<&'static str> {
    if limit == 0 {
        return Vec::new();
    }
    let (start, end) = token_span(value, caret);
    let query = &value[start..end];
    let needles = part_needles(label);
    if query.is_empty() && !needles.is_empty() {
        let matched = rank(page_id, query, &needles, value, start, end, true, limit);
        if !matched.is_empty() {
            return matched;
        }
    }
    rank(page_id, query, &needles, value, start, end, false, limit)
}

fn rank(
    page_id: &str,
    query: &str,
    needles: &[String],
    value: &str,
    editing_start: usize,
    editing_end: usize,
    require_part: bool,
    limit: usize,
) -> Vec<&'static str> {
    let query_l = query.to_ascii_lowercase();
    let fallback = format!("d-{page_id}");
    let extras = family_prefixes(page_id);
    let documented = doc_classes(page_id);
    let mut ranked = Vec::new();
    for class in CSS_CLASSES {
        if is_used(value, class, editing_start, editing_end) {
            continue;
        }
        let family = in_family(class, extras, &fallback);
        let in_docs = documented.binary_search(class).is_ok();
        let part = matches_part(class, needles);
        let score = if query.is_empty() {
            if !family {
                continue;
            }
            if require_part {
                if !part {
                    continue;
                }
            } else if class.contains("__") {
                continue;
            }
            idle_score(class, in_docs)
        } else {
            let starts = starts_with_ignore_ascii(class, &query_l);
            let contains = starts || contains_ignore_ascii(class, &query_l);
            if !contains {
                continue;
            }
            if query_l.len() < 2 && !family && !in_docs {
                continue;
            }
            typed_score(class, starts, family, in_docs, part)
        };
        ranked.push((score, *class));
    }
    ranked.sort_by(|left, right| right.0.cmp(&left.0).then_with(|| left.1.cmp(right.1)));
    ranked.truncate(limit);
    ranked.into_iter().map(|(_, class)| class).collect()
}

fn idle_score(class: &str, in_docs: bool) -> i32 {
    let mut score = if !class.contains("--") && !class.contains("__") {
        300
    } else if !class.contains("__") {
        200
    } else {
        100
    };
    if in_docs {
        score += 80;
    }
    score -= class.len() as i32;
    score
}

fn typed_score(class: &str, starts: bool, family: bool, in_docs: bool, part: bool) -> i32 {
    let mut score = if starts { 400 } else { 200 };
    if family {
        score += 80;
    }
    if in_docs {
        score += 40;
    }
    if part {
        score += 60;
    }
    score -= class.len() as i32;
    score
}

fn family_prefixes(id: &str) -> &'static [&'static str] {
    match id {
        "button" => &["d-btn", "base-button"],
        "button-group" => &["d-btn-group"],
        "split-button" => &["d-split-btn"],
        "dropdown" => &[
            "d-dropdown-list",
            "d-context-menu-list",
            "d-list-item",
            "d-list-item-separator",
            "d-popover",
        ],
        "select-menu" => &["d-select"],
        "skeleton" => &[
            "d-skeleton-list-item",
            "d-skeleton-paragraph",
            "d-skeleton-placeholder",
            "d-skeleton-placeholder-throb",
            "d-skeleton-text",
        ],
        "tab" | "tab-group" => &["d-tab", "d-tablist"],
        "toast" => &[
            "d-toast",
            "d-toast-alternate",
            "d-toast-layout-alternate",
            "d-toast-wrapper",
            "d-notice",
        ],
        "banner" => &["d-banner", "d-notice"],
        "box" => &["d-box"],
        "breadcrumbs" => &["d-breadcrumbs", "d-link"],
        "notice" => &["d-notice"],
        "validation-messages" => &["d-validation-message"],
        "codeblock" => &["d-codeblock"],
        "combobox" => &["d-combobox"],
        "combobox-multi-select" => &["d-combobox-multi-select", "d-combobox"],
        "combobox-with-popover" => &["d-combobox-with-popover", "d-combobox", "d-popover"],
        "emoji" => &["d-emoji", "d-icon"],
        "emoji-picker" => &["d-emoji-picker", "d-emoji"],
        "emoji-text-wrapper" => &["d-emoji-text-wrapper", "d-emoji"],
        "rich-text-editor" => &["d-rich-text-editor", "d-rich-text-editor-bubble-menu"],
        "toggle" => &["d-toggle", "d-toggle-group", "d-toggle-wrapper"],
        "list-item" => &["d-list-item", "d-item-layout"],
        "list-item-group" => &["d-list-item-group", "d-list-item", "dt-dropdown-list"],
        "keyboard-shortcut" => &["d-keyboard-shortcut"],
        "link" => &["d-link"],
        "hovercard" => &["d-popover"],
        "illustration" => &["d-illustration"],
        "image-viewer" => &["d-image-viewer", "d-modal"],
        "loader" => &["d-loader"],
        "slider" => &["d-slider", "d-tooltip"],
        "table" => &["d-table"],
        _ => &[],
    }
}

fn in_family(class: &str, extras: &[&str], fallback: &str) -> bool {
    let block = block_of(class);
    extras.iter().any(|prefix| *prefix == block) || block == fallback
}

fn block_of(class: &str) -> &str {
    let without_element = class
        .split_once("__")
        .map(|(head, _)| head)
        .unwrap_or(class);
    without_element
        .split_once("--")
        .map(|(head, _)| head)
        .unwrap_or(without_element)
}

fn part_needles(label: &str) -> Vec<String> {
    let lower = label.to_ascii_lowercase();
    let stem = lower.strip_suffix(" class").unwrap_or(&lower).trim();
    if stem.is_empty() || stem == "class" || stem == "root" {
        return Vec::new();
    }
    let mut needles = vec![stem.to_string()];
    if let Some(singular) = stem.strip_suffix('s') {
        if singular.len() >= 3 {
            needles.push(singular.to_string());
        }
    }
    match stem {
        "header" => needles.push("title".to_string()),
        "title" => needles.push("header".to_string()),
        "action" | "actions" => {
            needles.push("action".to_string());
            needles.push("actions".to_string());
        }
        "message" | "messages" => {
            needles.push("message".to_string());
            needles.push("messages".to_string());
        }
        _ => {}
    }
    needles.sort();
    needles.dedup();
    needles
}

fn matches_part(class: &str, needles: &[String]) -> bool {
    if needles.is_empty() {
        return false;
    }
    let lower = class.to_ascii_lowercase();
    needles.iter().any(|needle| {
        let element = format!("__{needle}");
        let modifier = format!("--{needle}");
        lower.contains(&element) || lower.contains(&modifier)
    })
}

fn is_used(value: &str, class: &str, editing_start: usize, editing_end: usize) -> bool {
    let bytes = value.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }
        let start = index;
        while index < bytes.len() && !bytes[index].is_ascii_whitespace() {
            index += 1;
        }
        if start == index {
            break;
        }
        if start == editing_start && index == editing_end {
            continue;
        }
        if &value[start..index] == class {
            return true;
        }
    }
    false
}

fn starts_with_ignore_ascii(hay: &str, needle: &str) -> bool {
    hay.len() >= needle.len()
        && hay.as_bytes()[..needle.len()].eq_ignore_ascii_case(needle.as_bytes())
}

fn contains_ignore_ascii(hay: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    let hay = hay.as_bytes();
    let needle = needle.as_bytes();
    hay.windows(needle.len())
        .any(|window| window.eq_ignore_ascii_case(needle))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_includes_stylesheet_and_button_docs() {
        assert!(CSS_CLASSES.len() > 1000);
        assert!(CSS_CLASSES.binary_search(&"d-btn").is_ok());
        assert!(CSS_CLASSES.binary_search(&"d-btn--disabled").is_ok());
        assert!(CSS_CLASSES.binary_search(&"d-checkbox__label").is_ok());
        assert!(doc_classes("button")
            .binary_search(&"d-btn--disabled")
            .is_ok());
    }

    #[test]
    fn page_id_comes_from_the_component_hash() {
        assert_eq!(page_id_from_hash("#/components/button"), "button");
        assert_eq!(page_id_from_hash("#/components/tab-group"), "tab-group");
        assert_eq!(page_id_from_hash(""), "");
    }

    #[test]
    fn apply_token_replaces_the_word_at_the_caret() {
        assert_eq!(
            apply_token("d-bt", 4, "d-btn--disabled"),
            ("d-btn--disabled ".to_string(), "d-btn--disabled ".len())
        );
        assert_eq!(
            apply_token("pre d-bt post", 6, "d-btn"),
            ("pre d-btn post".to_string(), 10)
        );
    }

    #[test]
    fn button_field_prefers_documented_component_classes() {
        let names = suggestions("button", "Class", "", 0, 10);
        assert_eq!(names.first().copied(), Some("d-btn"));
        assert!(names.contains(&"d-btn--disabled"), "{names:?}");
        assert!(!names.contains(&"d-w100p"), "{names:?}");
        assert!(names.iter().all(|name| !name.contains("__")), "{names:?}");
    }

    #[test]
    fn typed_text_searches_utility_classes() {
        let names = suggestions("button", "Class", "d-fc-mu", 7, 10);
        assert!(names.contains(&"d-fc-muted"), "{names:?}");
    }

    #[test]
    fn part_label_prefers_the_matching_element() {
        let names = suggestions("checkbox", "Label class", "", 0, 10);
        assert!(names.contains(&"d-checkbox__label"), "{names:?}");
        assert!(
            names.iter().all(|name| name.contains("__label")),
            "{names:?}"
        );
    }

    #[test]
    fn a_class_already_entered_is_not_offered_again() {
        let names = suggestions("button", "Class", "d-btn ", 6, 10);
        assert!(!names.contains(&"d-btn"), "{names:?}");
        assert!(names.contains(&"d-btn--disabled"), "{names:?}");
    }

    #[test]
    fn tab_page_uses_the_tab_stylesheet_family() {
        let names = suggestions("tab", "Class", "", 0, 10);
        assert_eq!(names.first().copied(), Some("d-tab"));
        assert!(
            names.iter().any(|name| name.starts_with("d-tablist")),
            "{names:?}"
        );
    }
}
