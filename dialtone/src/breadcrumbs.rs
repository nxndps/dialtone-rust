use leptos::prelude::*;

/// en-US `DIALTONE_BREADCRUMBS_ARIA_LABEL`.
const BREADCRUMBS_ARIA_LABEL: &str = "breadcrumbs";

/// Vue `DtLink` fallback when `href` is empty.
const HREF_FALLBACK: &str = "javascript:void(0)";

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// One entry in the `DtBreadcrumbs` `breadcrumbs` list.
///
/// Vue requires `href` and `label` to be present (`!== undefined`). An empty
/// string still counts. `inverted` `None` uses the nav value. `Some` wins,
/// matching `:inverted` then `v-bind="item"`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DtBreadcrumb {
    pub href: String,
    pub label: String,
    pub selected: bool,
    pub class: String,
    pub inverted: Option<bool>,
}

impl DtBreadcrumb {
    pub fn new(href: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            href: href.into(),
            label: label.into(),
            selected: false,
            class: String::new(),
            inverted: None,
        }
    }

    /// Mark this crumb as the current page.
    pub fn selected(mut self) -> Self {
        self.selected = true;
        self
    }
}

pub(crate) fn breadcrumbs_classes(inverted: bool, class: &str) -> String {
    join_class([
        "d-breadcrumbs",
        if inverted {
            "d-breadcrumbs--inverted"
        } else {
            ""
        },
        class,
    ])
}

pub(crate) fn breadcrumbs_list_classes(list_class: &str) -> String {
    join_class(["d-breadcrumbs__list", list_class])
}

pub(crate) fn breadcrumb_item_classes(selected: bool, class: &str) -> String {
    join_class([
        "d-breadcrumbs__item",
        if selected {
            "d-breadcrumbs__item--selected"
        } else {
            ""
        },
        class,
    ])
}

/// `DtLink` classes for tone `muted` and `underline` false, or the inverted
/// default tone. `d-breadcrumbs__link` is the class the item passes through.
pub(crate) fn breadcrumb_link_classes(inverted: bool) -> String {
    if inverted {
        "d-link d-link--inverted d-link--no-underline d-breadcrumbs__link".to_string()
    } else {
        "d-link d-link--muted d-link--no-underline d-breadcrumbs__link".to_string()
    }
}

pub(crate) fn breadcrumb_href(href: &str) -> String {
    if href.is_empty() {
        HREF_FALLBACK.to_string()
    } else {
        href.to_string()
    }
}

pub(crate) fn breadcrumb_aria_label(label: &str) -> String {
    if label.is_empty() {
        BREADCRUMBS_ARIA_LABEL.to_string()
    } else {
        label.to_string()
    }
}

pub(crate) fn breadcrumb_inverted(parent: bool, item: Option<bool>) -> bool {
    item.unwrap_or(parent)
}

pub(crate) fn breadcrumb_aria_current(selected: bool) -> Option<&'static str> {
    selected.then_some("location")
}

/// Dialtone Vue `DtBreadcrumbs` (`dialtone/v10.7.0`).
///
/// The root is `<nav data-qa="dt-breadcrumbs">`. `aria_label` is `ariaLabel`.
/// Empty uses the en-US label `breadcrumbs`. `inverted` is deprecated and still
/// adds `d-breadcrumbs--inverted`. `list_class` is `listClass` on the `<ol>`.
/// `class` is `$attrs.class` on the nav. The default slot replaces
/// `breadcrumbs`. Generated items receive the nav `inverted` unless the item
/// sets its own. `v-dt-mode` is not ported.
#[component]
pub fn DtBreadcrumbs(
    /// Crumb list (`breadcrumbs`). Ignored when the default slot is set.
    #[prop(optional)]
    breadcrumbs: Vec<DtBreadcrumb>,
    /// Deprecated inverted colors (`inverted`).
    #[prop(optional)]
    inverted: bool,
    /// Extra class on the `<ol>` (`listClass`).
    #[prop(optional, into)]
    list_class: String,
    /// Nav label (`ariaLabel`). Empty uses `breadcrumbs`.
    #[prop(optional, into)]
    aria_label: String,
    /// Extra class on the nav (`$attrs.class`).
    #[prop(optional, into)]
    class: String,
    /// Default slot. Replaces `breadcrumbs`.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let nav_class = breadcrumbs_classes(inverted, &class);
    let list_classes = breadcrumbs_list_classes(&list_class);
    let label = breadcrumb_aria_label(&aria_label);
    let body = match children {
        Some(children) => children().into_any(),
        None => breadcrumbs
            .into_iter()
            .map(|crumb| {
                let item_inverted = breadcrumb_inverted(inverted, crumb.inverted);
                view! {
                    <DtBreadcrumbItem
                        href=crumb.href
                        label=crumb.label
                        selected=crumb.selected
                        inverted=item_inverted
                        class=crumb.class
                    />
                }
            })
            .collect_view()
            .into_any(),
    };
    view! {
        <nav class=nav_class data-qa="dt-breadcrumbs" aria-label=label>
            <ol class=list_classes>{body}</ol>
        </nav>
    }
}

/// Dialtone Vue `DtBreadcrumbItem` (`dialtone/v10.7.0`).
///
/// The root is `<li data-qa="dt-breadcrumb-item">`. The link is an `<a>` with
/// `data-qa="breadcrumb-item"`, not `DtLink`, because `DtLink` hardcodes
/// `data-qa="dt-link"` and has no `aria-current`. A non-inverted item uses
/// tone `muted` and `underline` false. `inverted` uses the default tone plus
/// the inverted modifier. `selected` adds `d-breadcrumbs__item--selected` and
/// `aria-current="location"`. `class` and `style` are `$attrs` on the `<li>`.
/// The default slot replaces `label`. An empty `href` is `javascript:void(0)`.
#[component]
pub fn DtBreadcrumbItem(
    /// Destination (`href`). Empty uses `javascript:void(0)`.
    #[prop(optional, into)]
    href: String,
    /// Text when the default slot is absent (`label`).
    #[prop(optional, into)]
    label: String,
    /// Current page (`selected`).
    #[prop(optional)]
    selected: bool,
    /// Deprecated inverted link color (`inverted`).
    #[prop(optional)]
    inverted: bool,
    /// Extra class on the `<li>` (`$attrs.class`).
    #[prop(optional, into)]
    class: String,
    /// Inline style on the `<li>` (`$attrs.style`).
    #[prop(optional, into)]
    style: String,
    /// `target` on the link. Empty omits the attribute.
    #[prop(optional, into)]
    target: String,
    /// `rel` on the link. Empty omits the attribute.
    #[prop(optional, into)]
    rel: String,
    /// Element id on the link. Empty omits the attribute.
    #[prop(optional, into)]
    id: String,
    /// Default slot. Replaces `label`.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let item_class = breadcrumb_item_classes(selected, &class);
    let link_class = breadcrumb_link_classes(inverted);
    let href = breadcrumb_href(&href);
    let current = breadcrumb_aria_current(selected);
    let style_attr = (!style.is_empty()).then_some(style);
    let id_attr = (!id.is_empty()).then_some(id);
    let target_attr = (!target.is_empty()).then_some(target);
    let rel_attr = (!rel.is_empty()).then_some(rel);
    let body = match children {
        Some(children) => children().into_any(),
        None => label.into_any(),
    };
    view! {
        <li data-qa="dt-breadcrumb-item" class=item_class style=style_attr>
            <a
                id=id_attr
                class=link_class
                data-qa="breadcrumb-item"
                href=href
                target=target_attr
                rel=rel_attr
                aria-current=current
            >
                {body}
            </a>
        </li>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes_follow_the_vue_template() {
        assert_eq!(breadcrumbs_classes(false, ""), "d-breadcrumbs");
        assert_eq!(
            breadcrumbs_classes(true, ""),
            "d-breadcrumbs d-breadcrumbs--inverted"
        );
        assert_eq!(
            breadcrumbs_classes(true, "custom-class"),
            "d-breadcrumbs d-breadcrumbs--inverted custom-class"
        );
        assert_eq!(breadcrumbs_list_classes(""), "d-breadcrumbs__list");
        assert_eq!(
            breadcrumbs_list_classes("d-py-200"),
            "d-breadcrumbs__list d-py-200"
        );
        assert_eq!(breadcrumb_item_classes(false, ""), "d-breadcrumbs__item");
        assert_eq!(
            breadcrumb_item_classes(true, "custom-class"),
            "d-breadcrumbs__item d-breadcrumbs__item--selected custom-class"
        );
        assert_eq!(
            breadcrumb_link_classes(false),
            "d-link d-link--muted d-link--no-underline d-breadcrumbs__link"
        );
        assert_eq!(
            breadcrumb_link_classes(true),
            "d-link d-link--inverted d-link--no-underline d-breadcrumbs__link"
        );
    }

    #[test]
    fn empty_aria_label_uses_the_english_default() {
        assert_eq!(breadcrumb_aria_label(""), "breadcrumbs");
        assert_eq!(breadcrumb_aria_label("newAria"), "newAria");
    }

    #[test]
    fn item_inverted_overrides_the_parent() {
        assert!(breadcrumb_inverted(true, None));
        assert!(!breadcrumb_inverted(true, Some(false)));
        assert!(breadcrumb_inverted(false, Some(true)));
    }

    #[test]
    fn selected_sets_aria_current_location() {
        assert_eq!(breadcrumb_aria_current(true), Some("location"));
        assert_eq!(breadcrumb_aria_current(false), None);
        let crumb = DtBreadcrumb::new("#", "Current Page").selected();
        assert!(crumb.selected);
        assert!(!DtBreadcrumb::new("#", "Root").selected);
    }

    #[test]
    fn empty_href_uses_the_link_fallback() {
        assert_eq!(breadcrumb_href(""), "javascript:void(0)");
        assert_eq!(breadcrumb_href("#"), "#");
    }
}
