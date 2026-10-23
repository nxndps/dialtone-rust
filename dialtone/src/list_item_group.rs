use leptos::prelude::*;

use super::unique_id;

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn group_list_class(list_class: &str) -> String {
    join_class(["d-list-item-group", list_class])
}

fn group_heading_class(heading_class: &str) -> String {
    join_class(["dt-dropdown-list--header", heading_class])
}

/// Dialtone Vue `DtListItemGroup` (`dialtone/v10.7.0`).
///
/// The root is `ul.d-list-item-group` with `role="group"` and
/// `data-qa="dt-dropdown-list-wrapper"`. `aria-labelledby` is `{id}-heading`
/// even when the heading row is absent. The heading row renders when `heading`
/// is non-empty. `heading_slot` is the `heading` slot and replaces that text.
/// The string still has to be set, which is the Vue `v-if="heading"`.
/// `heading_class` (`headingClass`) is appended to `dt-dropdown-list--header`.
/// `list_class` (`listClass`) is appended to the root. `dt-dropdown-list--header`
/// is not in the vendored CSS.
#[component]
pub fn DtListItemGroup(
    /// Element id. Empty generates one.
    #[prop(optional, into)]
    id: String,
    /// Heading text. Empty hides the heading row.
    #[prop(optional, into)]
    heading: String,
    /// Extra classes on the heading (`headingClass`).
    #[prop(optional, into)]
    heading_class: String,
    /// Extra classes on the list (`listClass`).
    #[prop(optional, into)]
    list_class: String,
    /// `heading` slot. Replaces the heading text when `heading` is set.
    #[prop(optional)]
    heading_slot: Option<ViewFn>,
    children: Children,
) -> impl IntoView {
    let id = if id.is_empty() {
        unique_id("dt-list-item-group")
    } else {
        id
    };
    let heading_id = format!("{id}-heading");
    let list_class = group_list_class(&list_class);
    let heading_class = group_heading_class(&heading_class);
    let show_heading = !heading.is_empty();
    view! {
        <ul
            id=id
            class=list_class
            role="group"
            data-qa="dt-dropdown-list-wrapper"
            aria-labelledby=heading_id.clone()
        >
            {show_heading.then(|| view! {
                <li
                    id=heading_id.clone()
                    role="presentation"
                    data-qa="dt-dropdown-list-heading"
                    class=heading_class.clone()
                >
                    {if let Some(slot) = heading_slot {
                        slot.run().into_any()
                    } else {
                        heading.into_any()
                    }}
                </li>
            })}
            {children()}
        </ul>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_classes_append_the_extras() {
        assert_eq!(group_list_class(""), "d-list-item-group");
        assert_eq!(group_list_class("d-w100p"), "d-list-item-group d-w100p");
        assert_eq!(group_heading_class(""), "dt-dropdown-list--header");
        assert_eq!(
            group_heading_class("d-fw-bold"),
            "dt-dropdown-list--header d-fw-bold"
        );
    }
}
