//! Shared list and popover class helpers for dropdown and combobox ports.

use wasm_bindgen::JsCast;
use web_sys::{Element, EventTarget, HtmlElement, MouseEvent, Node};

/// Dialtone `DROPDOWN_PADDING_CLASSES` (`dialtone/v9.190.0`).
pub(crate) fn dropdown_padding_class(padding: &str) -> &'static str {
    match padding {
        "large" => "d-py4",
        "none" => "",
        _ => "d-py0",
    }
}

/// Dialtone popover padding (`none` / `small` / `medium` / `large`).
pub(crate) fn popover_padding_class(padding: &str) -> &'static str {
    match padding {
        "small" => "d-p4",
        "medium" => "d-p8",
        "large" => "d-p16",
        _ => "",
    }
}

pub(crate) fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn role_items(root: &Element, selector: &str) -> Vec<Element> {
    let Ok(nodes) = root.query_selector_all(selector) else {
        return Vec::new();
    };
    let mut items = Vec::with_capacity(nodes.length() as usize);
    for index in 0..nodes.length() {
        if let Some(node) = nodes.item(index) {
            if let Ok(element) = node.dyn_into::<Element>() {
                items.push(element);
            }
        }
    }
    items
}

pub(crate) fn ensure_ids(items: &[Element], prefix: &str) {
    for (index, element) in items.iter().enumerate() {
        if element.id().is_empty() {
            let _ = element.set_attribute("id", &format!("{prefix}-{index}"));
        }
    }
}

pub(crate) fn paint_highlight(items: &[Element], index: i32) {
    for (item_index, element) in items.iter().enumerate() {
        let class_list = element.class_list();
        if item_index as i32 == index {
            let _ = class_list.add_1("d-list-item--highlighted");
        } else {
            let _ = class_list.remove_1("d-list-item--highlighted");
        }
    }
}

pub(crate) fn move_index(len: usize, index: i32, delta: i32, wrap: bool) -> i32 {
    if len == 0 {
        return -1;
    }
    let last = len as i32 - 1;
    if index < 0 {
        return if delta < 0 { last } else { 0 };
    }
    let next = index + delta;
    if wrap {
        if next < 0 {
            last
        } else if next > last {
            0
        } else {
            next
        }
    } else {
        next.clamp(0, last)
    }
}

pub(crate) fn item_index_at(
    list: &Element,
    event: &MouseEvent,
    item_selector: &str,
) -> Option<usize> {
    let target = event.target()?.dyn_into::<Element>().ok()?;
    let item = target.closest(item_selector).ok().flatten()?;
    if item.get_attribute("role").is_none() {
        return None;
    }
    role_items(list, item_selector)
        .iter()
        .position(|element| element.is_same_node(Some(item.as_ref())))
}

pub(crate) fn focus_item(items: &[Element], index: i32) {
    let Some(element) = index
        .try_into()
        .ok()
        .and_then(|index: usize| items.get(index))
    else {
        return;
    };
    if let Ok(element) = element.clone().dyn_into::<HtmlElement>() {
        let _ = element.focus();
    }
}

pub(crate) fn event_inside(event: &MouseEvent, element: &EventTarget) -> bool {
    let Some(target) = event.target() else {
        return false;
    };
    let Some(node) = element.dyn_ref::<Node>() else {
        return false;
    };
    let Some(target) = target.dyn_ref::<Node>() else {
        return false;
    };
    node.contains(Some(target))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dropdown_padding_matches_vue() {
        assert_eq!(dropdown_padding_class("small"), "d-py0");
        assert_eq!(dropdown_padding_class("large"), "d-py4");
        assert_eq!(dropdown_padding_class("none"), "");
        assert_eq!(dropdown_padding_class(""), "d-py0");
    }

    #[test]
    fn popover_padding_matches_vue() {
        assert_eq!(popover_padding_class("none"), "");
        assert_eq!(popover_padding_class("small"), "d-p4");
        assert_eq!(popover_padding_class("medium"), "d-p8");
        assert_eq!(popover_padding_class("large"), "d-p16");
    }

    #[test]
    fn highlight_index_wraps() {
        assert_eq!(move_index(3, 2, 1, true), 0);
        assert_eq!(move_index(3, 0, -1, true), 2);
        assert_eq!(move_index(3, 2, 1, false), 2);
        assert_eq!(move_index(0, 0, 1, true), -1);
        assert_eq!(move_index(4, -1, 1, true), 0);
        assert_eq!(move_index(4, -1, -1, true), 3);
    }
}
