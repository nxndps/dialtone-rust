use leptos::prelude::*;

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Classes for [`DtTable`]. `striped` adds `d-table--striped`.
pub(crate) fn table_classes(striped: bool, class: &str) -> String {
    join_class([
        "d-table",
        if striped { "d-table--striped" } else { "" },
        class,
    ])
}

/// Dialtone `d-table` (`dialtone/v10.7.0`).
///
/// There is no Vue `DtTable`. The markup is
/// `packages/dialtone-css/lib/build/less/components/table.less` and the docs
/// page `apps/dialtone-documentation/docs/components/table.md`. The root is
/// `<table class="d-table">`. `striped` adds `d-table--striped`. A non-empty
/// `caption` renders `<caption class="d-table__caption">`. The default slot
/// is the rest of the table (`thead`, `tbody`, `th`, `td`). `class` is
/// appended on the table. An empty `id` is omitted. `d-table--inverted` is
/// deprecated on that page. This port does not emit it. Invert with
/// [`DtModeIsland`](crate::DtModeIsland). There is no `data-qa`. `d-table`,
/// `d-table__caption`, and `d-table--striped` are in the vendored CSS.
#[component]
pub fn DtTable(
    /// Caption text. Empty omits the `<caption>`.
    #[prop(optional, into)]
    caption: String,
    /// Even-row background (`d-table--striped`).
    #[prop(optional)]
    striped: bool,
    /// Extra class on the `<table>`.
    #[prop(optional, into)]
    class: String,
    /// `id` on the `<table>`. Empty omits it.
    #[prop(optional, into)]
    id: String,
    /// `thead`, `tbody`, and the rows.
    children: Children,
) -> impl IntoView {
    let class_name = table_classes(striped, &class);
    let id = (!id.is_empty()).then_some(id);
    let caption = (!caption.is_empty()).then(|| {
        view! {
            <caption class="d-table__caption">{caption}</caption>
        }
    });
    view! {
        <table class=class_name id=id>
            {caption}
            {children()}
        </table>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_class_is_d_table() {
        assert_eq!(table_classes(false, ""), "d-table");
    }

    #[test]
    fn striped_appends_the_modifier_and_an_extra_class() {
        assert_eq!(
            table_classes(true, "office-list"),
            "d-table d-table--striped office-list"
        );
    }
}
