use leptos::prelude::*;

use super::list_nav::join_class;

const DIRECTIONS: &[&str] = &["row", "column"];
const GAPS: &[&str] = &[
    "0", "1", "25", "50", "75", "100", "125", "150", "175", "200", "250", "300", "350", "400",
    "450", "500", "525", "550", "600", "650", "700", "750", "800",
];

/// One `DtDescriptionList` row (`term` / `description`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DtDescriptionListItem {
    pub term: String,
    pub description: String,
}

impl DtDescriptionListItem {
    pub fn new(term: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            term: term.into(),
            description: description.into(),
        }
    }
}

/// Vue rejects an item whose `term` or `description` is empty.
pub fn description_items_valid(items: &[DtDescriptionListItem]) -> bool {
    items
        .iter()
        .all(|item| !item.term.is_empty() && !item.description.is_empty())
}

pub(crate) fn description_list_classes(direction: &str, gap: &str) -> String {
    if !DIRECTIONS.contains(&direction) {
        leptos::logging::warn!("DtDescriptionList direction `{direction}` is not row or column");
    }
    if !GAPS.contains(&gap) {
        leptos::logging::warn!("DtDescriptionList gap `{gap}` is not a stack gap");
    }
    join_class([
        "d-description-list",
        &format!("d-description-list--{direction}"),
        &format!("d-description-list--gap-{gap}"),
    ])
}

/// Dialtone Vue `DtDescriptionList` (`dialtone/v10.7.0`).
///
/// `direction` is `row` (default) or `column`. The template still emits
/// `d-description-list--{direction}` for any other value, and this port warns.
/// `d-description-list--row` is not in the vendored CSS. `.d-description-list`
/// is already `flex-flow: row wrap`. `column` stacks the pairs
/// (`d-description-list--column`). `gap` is a `DtStack` space token, including
/// `50` and `1`. The default is `100`. An unknown gap warns and is still
/// emitted. An item with an empty term or description warns. An empty list
/// does not. There is no `data-qa`.
#[component]
pub fn DtDescriptionList(
    /// `row` (default) or `column`.
    #[prop(optional, default = "row")]
    direction: &'static str,
    /// Space token (`gap`). Default `100`.
    #[prop(optional, default = "100")]
    gap: &'static str,
    /// Extra class on each `dt` (`termClass`).
    #[prop(optional, into)]
    term_class: String,
    /// Extra class on each `dd` (`descriptionClass`).
    #[prop(optional, into)]
    description_class: String,
    items: Vec<DtDescriptionListItem>,
) -> impl IntoView {
    if !description_items_valid(&items) {
        leptos::logging::warn!("DtDescriptionList items need a term and a description");
    }
    let root_class = description_list_classes(direction, gap);
    let term_class = join_class(["d-description-list__term", term_class.as_str()]);
    let description_class = join_class([
        "d-description-list__description",
        description_class.as_str(),
    ]);
    let rows = items
        .into_iter()
        .map(|item| {
            let term_class = term_class.clone();
            let description_class = description_class.clone();
            view! {
                <dt class=term_class>{item.term}</dt>
                <dd class=description_class>{item.description}</dd>
            }
        })
        .collect_view();
    view! { <dl class=root_class>{rows}</dl> }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes_include_direction_and_known_gap() {
        assert_eq!(
            description_list_classes("row", "400"),
            "d-description-list d-description-list--row d-description-list--gap-400"
        );
        assert_eq!(
            description_list_classes("column", "500"),
            "d-description-list d-description-list--column d-description-list--gap-500"
        );
        assert_eq!(
            description_list_classes("row", "50"),
            "d-description-list d-description-list--row d-description-list--gap-50"
        );
        assert_eq!(
            description_list_classes("sideways", "999"),
            "d-description-list d-description-list--sideways d-description-list--gap-999"
        );
    }

    #[test]
    fn empty_term_or_description_is_invalid() {
        assert!(description_items_valid(&[DtDescriptionListItem::new(
            "A", "B"
        )]));
        assert!(!description_items_valid(&[DtDescriptionListItem::new(
            "", "B"
        )]));
        assert!(!description_items_valid(&[DtDescriptionListItem::new(
            "A", ""
        )]));
    }
}
