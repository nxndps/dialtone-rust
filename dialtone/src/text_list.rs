use leptos::context::Provider;
use leptos::html;
use leptos::prelude::*;

use super::text::known_text_tone;

fn warn_direct_children(element: &web_sys::Node) {
    let children = element.child_nodes();
    for index in 0..children.length() {
        let Some(child) = children.item(index) else {
            continue;
        };
        if child.node_type() != web_sys::Node::ELEMENT_NODE {
            continue;
        }
        if child.node_name() != "LI" {
            warn_list("Use DtTextListItem or a native li as the direct child of DtTextList.");
            break;
        }
    }
}

#[derive(Clone, Copy)]
struct TextListContext {
    ordered: bool,
}

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn known_gap(gap: &str) -> bool {
    matches!(
        gap,
        "0" | "1"
            | "25"
            | "50"
            | "75"
            | "100"
            | "125"
            | "150"
            | "175"
            | "200"
            | "250"
            | "300"
            | "350"
            | "400"
    )
}

fn unordered_marker(marker: &str) -> bool {
    matches!(marker, "disc" | "circle" | "square")
}

fn ordered_marker(marker: &str) -> bool {
    matches!(
        marker,
        "decimal" | "lower-alpha" | "upper-alpha" | "lower-roman" | "upper-roman"
    )
}

fn known_marker(marker: &str) -> bool {
    marker == "none" || unordered_marker(marker) || ordered_marker(marker)
}

fn warn_list(message: &str) {
    leptos::logging::warn!("[DtTextList] {message}");
}

/// `true` when `type` is `ordered`. Any other value renders a `ul`.
pub(crate) fn text_list_ordered(type_name: &str) -> bool {
    type_name == "ordered"
}

/// Safari drops list semantics when the marker is `none`. Vue restores `role="list"`.
pub(crate) fn text_list_role(marker: &str) -> Option<&'static str> {
    (marker == "none").then_some("list")
}

/// Dialtone `DtTextList` classes. `TextList.vue` at `dialtone/v10.7.0`.
pub(crate) fn text_list_classes(
    type_name: &str,
    gap: &str,
    marker: &str,
    marker_tone: &str,
) -> String {
    if !type_name.is_empty() && type_name != "unordered" && type_name != "ordered" {
        warn_list(&format!("Unsupported type \"{type_name}\"."));
    }
    let gap_class = if gap.is_empty() {
        String::new()
    } else if known_gap(gap) {
        format!("d-text-list--gap-{gap}")
    } else {
        warn_list(&format!("Unsupported gap \"{gap}\"."));
        String::new()
    };
    let marker_class = if marker.is_empty() {
        String::new()
    } else if !known_marker(marker) {
        warn_list(&format!("Unsupported marker \"{marker}\"."));
        String::new()
    } else {
        if type_name == "ordered" && unordered_marker(marker) {
            warn_list(&format!(
                "The marker=\"{marker}\" value is usually used with type=\"unordered\"."
            ));
        }
        if type_name == "unordered" && ordered_marker(marker) {
            warn_list(&format!(
                "The marker=\"{marker}\" value is usually used with type=\"ordered\"."
            ));
        }
        format!("d-text-list--marker-{marker}")
    };
    let tone_class = if marker_tone.is_empty() {
        String::new()
    } else if known_text_tone(marker_tone) {
        format!("d-text-list--marker-tone-{marker_tone}")
    } else {
        warn_list(&format!("Unsupported markerTone \"{marker_tone}\"."));
        String::new()
    };
    join_class([
        "d-text-list",
        gap_class.as_str(),
        if text_list_ordered(type_name) {
            "d-text-list--ordered"
        } else {
            ""
        },
        marker_class.as_str(),
        tone_class.as_str(),
    ])
}

pub(crate) fn text_list_item_classes(custom_marker: bool, marker_tone: &str) -> String {
    let tone_class = if marker_tone.is_empty() {
        String::new()
    } else if known_text_tone(marker_tone) {
        format!("d-text-list__item--marker-tone-{marker_tone}")
    } else {
        leptos::logging::warn!("[DtTextListItem] Unsupported markerTone \"{marker_tone}\".");
        String::new()
    };
    join_class([
        "d-text-list__item",
        if custom_marker {
            "d-text-list__item--custom-marker"
        } else {
            ""
        },
        tone_class.as_str(),
    ])
}

fn warn_ordered_only(type_name: &str, start: Option<i32>, reversed: bool) {
    if text_list_ordered(type_name) {
        return;
    }
    if start.is_some() {
        warn_list("The start prop only applies when type=\"ordered\".");
    }
    if reversed {
        warn_list("The reversed prop only applies when type=\"ordered\".");
    }
}

/// Dialtone Vue `DtTextList` (`dialtone/v10.7.0`).
///
/// `d-text-list*` is in the vendored Dialtone 10 CSS. A direct child that is
/// not an `li` warns after mount.
///
/// `type` is `unordered` (default, `ul`) or `ordered` (`ol`). `gap` is `0`,
/// `1`, `25`, `50`, `75`, `100`, `125`, `150`, `175`, `200`, `250`, `300`,
/// `350`, or `400`. `marker` is `disc`, `circle`, `square`, `decimal`,
/// `lower-alpha`, `upper-alpha`, `lower-roman`, `upper-roman`, or `none`.
/// `marker_tone` is `markerTone` and uses the `DtText` tone names. `start` and
/// `reversed` apply only when `type` is `ordered`.
#[component]
pub fn DtTextList(
    /// `unordered` (default) or `ordered` (`type`).
    #[prop(optional, default = "unordered")]
    r#type: &'static str,
    /// Marker style (`marker`). Empty keeps the automatic marker.
    #[prop(optional, default = "")]
    marker: &'static str,
    /// Marker tone (`markerTone`).
    #[prop(optional, default = "")]
    marker_tone: &'static str,
    /// Space between items (`gap`).
    #[prop(optional, default = "")]
    gap: &'static str,
    /// Ordered-list start (`start`).
    #[prop(optional)]
    start: Option<i32>,
    /// Reverse ordered numbering (`reversed`).
    #[prop(optional)]
    reversed: bool,
    children: Children,
) -> impl IntoView {
    warn_ordered_only(r#type, start, reversed);
    let ordered = text_list_ordered(r#type);
    let class_name = text_list_classes(r#type, gap, marker, marker_tone);
    let role = text_list_role(marker);
    let context = TextListContext { ordered };
    let body = view! {
        <Provider value=context>
            {children()}
        </Provider>
    };
    if ordered {
        let reversed_attr = reversed.then_some(true);
        let list_ref = NodeRef::<html::Ol>::new();
        list_ref.on_load(|element| warn_direct_children(&element));
        view! {
            <ol
                node_ref=list_ref
                data-qa="dt-text-list"
                class=class_name
                role=role
                start=start
                reversed=reversed_attr
            >
                {body}
            </ol>
        }
        .into_any()
    } else {
        let list_ref = NodeRef::<html::Ul>::new();
        list_ref.on_load(|element| warn_direct_children(&element));
        view! {
            <ul node_ref=list_ref data-qa="dt-text-list" class=class_name role=role>
                {body}
            </ul>
        }
        .into_any()
    }
}

/// Dialtone Vue `DtTextListItem` (`dialtone/v10.7.0`).
///
/// `value` is the native `li` value and is set only when the parent
/// `DtTextList` `type` is `ordered`. `marker` replaces the list marker.
/// `marker_class` is `markerClass`. `content_class` is `contentClass`.
/// `marker_tone` is `markerTone`.
#[component]
pub fn DtTextListItem(
    /// Native `li` value. Applied only inside an ordered list (`value`).
    #[prop(optional)]
    value: Option<i32>,
    /// Marker tone for this item (`markerTone`).
    #[prop(optional, default = "")]
    marker_tone: &'static str,
    /// Extra class on the marker (`markerClass`).
    #[prop(optional, into)]
    marker_class: String,
    /// Extra class on the content (`contentClass`).
    #[prop(optional, into)]
    content_class: String,
    /// Custom marker slot (`marker`).
    #[prop(optional)]
    marker: Option<ViewFn>,
    children: Children,
) -> impl IntoView {
    let parent = use_context::<TextListContext>();
    if parent.is_none() {
        leptos::logging::warn!("[DtTextListItem] DtTextListItem must be used inside DtTextList.");
    }
    let ordered = parent.is_some_and(|parent| parent.ordered);
    if value.is_some() && !ordered {
        leptos::logging::warn!(
            "[DtTextListItem] The value prop only applies inside a DtTextList with type=\"ordered\"."
        );
    }
    let class_name = text_list_item_classes(marker.is_some(), marker_tone);
    let marker_class_name = join_class(["d-text-list__item-marker", marker_class.as_str()]);
    let content_class_name = join_class(["d-text-list__item-content", content_class.as_str()]);
    let value_attr = ordered.then_some(value).flatten();
    view! {
        <li data-qa="dt-text-list-item" class=class_name value=value_attr>
            {marker.map(|slot| view! {
                <span
                    data-qa="dt-text-list-item-marker"
                    class=marker_class_name
                    aria-hidden="true"
                >
                    {slot.run()}
                </span>
            })}
            <div data-qa="dt-text-list-item-content" class=content_class_name>
                {children()}
            </div>
        </li>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes_follow_the_vue_order() {
        assert_eq!(text_list_classes("unordered", "", "", ""), "d-text-list");
        assert_eq!(
            text_list_classes("ordered", "", "", ""),
            "d-text-list d-text-list--ordered"
        );
        assert_eq!(
            text_list_classes("ordered", "200", "decimal", "muted"),
            "d-text-list d-text-list--gap-200 d-text-list--ordered d-text-list--marker-decimal d-text-list--marker-tone-muted"
        );
        assert_eq!(
            text_list_classes("unordered", "", "none", ""),
            "d-text-list d-text-list--marker-none"
        );
    }

    #[test]
    fn unknown_tokens_are_omitted_and_marker_none_sets_role() {
        assert_eq!(
            text_list_classes("sideways", "999", "arrow", "brand"),
            "d-text-list"
        );
        assert_eq!(text_list_role("none"), Some("list"));
        assert_eq!(text_list_role("disc"), None);
        assert!(text_list_ordered("ordered"));
        assert!(!text_list_ordered("unordered"));
    }

    #[test]
    fn mismatched_marker_family_still_emits_the_class() {
        assert_eq!(
            text_list_classes("ordered", "", "disc", ""),
            "d-text-list d-text-list--ordered d-text-list--marker-disc"
        );
        assert_eq!(
            text_list_classes("unordered", "", "decimal", ""),
            "d-text-list d-text-list--marker-decimal"
        );
    }

    #[test]
    fn item_classes_include_custom_marker_and_tone() {
        assert_eq!(text_list_item_classes(false, ""), "d-text-list__item");
        assert_eq!(
            text_list_item_classes(true, "critical"),
            "d-text-list__item d-text-list__item--custom-marker d-text-list__item--marker-tone-critical"
        );
        assert_eq!(text_list_item_classes(false, "nope"), "d-text-list__item");
    }
}
