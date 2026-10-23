use leptos::prelude::*;

use super::list_nav::join_class;

const GAPS: &[&str] = &["0", "100", "200", "300", "400", "500", "600"];

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
    let direction = if direction == "column" {
        "column"
    } else {
        "row"
    };
    let gap_class = if GAPS.contains(&gap) {
        format!("d-description-list--gap-{gap}")
    } else {
        String::new()
    };
    join_class([
        "d-description-list",
        &format!("d-description-list--{direction}"),
        gap_class.as_str(),
    ])
}

/// Dialtone Vue `DtDescriptionList`.
///
/// `direction` is `row` (default) or `column`. `row` keeps each term beside
/// its description (`flex-flow: row wrap`). `column` stacks them
/// (`d-description-list--column`). `gap` is a Dialtone space token. The
/// default is `100`, matching Vue. There is no `data-qa` on this component.
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
            description_list_classes("sideways", "999"),
            "d-description-list d-description-list--row"
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
