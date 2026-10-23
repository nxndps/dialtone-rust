use leptos::prelude::*;

use crate::button::button_classes;
use crate::{DtIconChevronLeft, DtIconChevronRight, DtIconMoreHorizontal};

const NAV_ICON_CLASS: &str = "base-button__icon d-btn__icon d-btn__icon--left";
const ITEM_LABEL_CLASS: &str = "base-button__label d-btn__label d-pagination__item-label";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PageSlot {
    Page(i32),
    Ellipsis,
}

/// Page buttons from `Pagination.vue` `pages`. `...` is the separator.
fn pagination_pages(current: i32, total: i32, max_visible: i32, show_edges: bool) -> Vec<PageSlot> {
    if max_visible == 0 {
        return Vec::new();
    }
    if total <= max_visible {
        return range(1, total);
    }

    let mut start = max_visible - 1;
    let mut end = total - start + 1;
    if !show_edges {
        start += 1;
        end -= 1;
    }

    if current < start {
        let mut pages = range(1, start);
        pages.push(PageSlot::Ellipsis);
        if show_edges {
            pages.push(PageSlot::Page(total));
        }
        return pages;
    }

    if current > end {
        let mut pages = vec![PageSlot::Ellipsis];
        pages.extend(range(end, total));
        if show_edges {
            pages.insert(0, PageSlot::Page(1));
        }
        return pages;
    }

    // Even `maxVisible` rounds down so the current page stays in the middle.
    let window = max_visible - (3 - max_visible % 2);
    let center = window / 2;
    let mut left = current - center;
    let mut right = current + center;
    if !show_edges {
        left -= 1;
        right += 1;
    }
    let mut pages = vec![PageSlot::Ellipsis];
    pages.extend(range(left, right));
    pages.push(PageSlot::Ellipsis);
    if show_edges {
        pages.insert(0, PageSlot::Page(1));
        pages.push(PageSlot::Page(total));
    }
    pages
}

fn range(from: i32, to: i32) -> Vec<PageSlot> {
    let from = if from > 0 { from } else { 1 };
    (from..=to).map(PageSlot::Page).collect()
}

fn nav_button_class() -> String {
    format!(
        "{} d-pagination__button",
        button_classes(
            false, "default", false, "muted", "clear", "md", false, false, true, false, false,
        )
    )
}

fn page_button_class(active: bool) -> String {
    let (kind, importance) = if active {
        ("default", "primary")
    } else {
        ("muted", "clear")
    };
    format!(
        "{} d-pagination__item",
        button_classes(
            false, "default", false, kind, importance, "md", false, false, false, false, false,
        )
    )
}

/// `DIALTONE_PAGINATION_FIRST_PAGE` or `DIALTONE_PAGINATION_PREVIOUS_PAGE`.
fn prev_aria_label(current: i32) -> &'static str {
    if current == 1 {
        "First page"
    } else {
        "Previous page"
    }
}

/// `DIALTONE_PAGINATION_LAST_PAGE` or `DIALTONE_PAGINATION_NEXT_PAGE`.
fn next_aria_label(current: i32, total: i32) -> &'static str {
    if current == total {
        "Last page"
    } else {
        "Next page"
    }
}

/// `DIALTONE_PAGINATION_PAGE_NUMBER`, with Fluent isolate marks around the number.
/// The last page is `DIALTONE_PAGINATION_LAST_PAGE` plus the number.
fn page_aria_label(page: i32, total: i32) -> String {
    if page == total {
        format!("Last page {page}")
    } else {
        format!("Page number \u{2068}{page}\u{2069}")
    }
}

fn select_page(current: RwSignal<i32>, page: i32, on_change: StoredValue<Option<Callback<i32>>>) {
    current.set(page);
    if let Some(callback) = on_change.get_value().as_ref() {
        callback.run(page);
    }
}

/// Dialtone Vue `DtPagination` at `dialtone/v10.7.0`.
///
/// `aria_label` is `ariaLabel`. `total_pages` is `totalPages`. `active_page`
/// is `activePage` and defaults to 1. The component keeps the current page
/// and follows later changes to that prop. A `total_pages` change clamps the
/// current page when it falls outside `1..=total_pages`, without `change`.
/// `on_change` is `change`. `max_visible` is `maxVisible` (default 5). An even
/// value rounds down in the middle range so the current page stays centered.
/// `show_edges` is `showEdges` (default true).
///
/// Prev, next, and page controls are native buttons using [`button_classes`].
/// `DtButton` captures `aria-label`, `kind`, and `importance` once, and those
/// change with the current page. Names are the en-US Dialtone strings.
#[component]
pub fn DtPagination(
    /// Descriptive label for the navigation (`ariaLabel`).
    aria_label: String,
    /// `totalPages`.
    #[prop(into)]
    total_pages: Signal<i32>,
    /// `activePage`. Empty means 1.
    #[prop(optional, into)]
    active_page: MaybeProp<i32>,
    /// `maxVisible`. Default 5. `0` shows no page buttons.
    #[prop(optional, default = 5)]
    max_visible: i32,
    /// `showEdges`. Default true.
    #[prop(optional, default = true)]
    show_edges: bool,
    /// `change`.
    #[prop(optional, into)]
    on_change: Option<Callback<i32>>,
) -> impl IntoView {
    let current = RwSignal::new(active_page.get_untracked().unwrap_or(1));
    let on_change = StoredValue::new(on_change);
    let active_seen = StoredValue::new(false);
    Effect::new(move |_| {
        let page = active_page.get();
        if !active_seen.get_value() {
            active_seen.set_value(true);
            return;
        }
        if let Some(page) = page {
            current.set(page);
        }
    });
    let total_seen = StoredValue::new(false);
    Effect::new(move |_| {
        let pages = total_pages.get();
        if !total_seen.get_value() {
            total_seen.set_value(true);
            return;
        }
        let page = current.get_untracked();
        if page > pages || page <= 0 {
            current.set(pages);
        }
    });
    let nav_class = nav_button_class();
    let prev_class = nav_class.clone();
    let next_class = nav_class;
    // A `>` inside a `view!` attribute closes the tag.
    let display = move || {
        if total_pages.get() > 0 {
            "flex"
        } else {
            "none"
        }
    };
    view! {
        <nav
            class="d-pagination"
            aria-label=aria_label
            style:display=display
        >
            <button
                type="button"
                class=prev_class
                data-qa="dt-pagination-prev"
                aria-label=move || prev_aria_label(current.get())
                prop:disabled=move || current.get() == 1
                on:click=move |_| {
                    select_page(current, current.get_untracked() - 1, on_change);
                }
            >
                <span data-qa="dt-button-icon" class=NAV_ICON_CLASS>
                    <DtIconChevronLeft size="300" />
                </span>
            </button>
            {move || {
                pagination_pages(current.get(), total_pages.get(), max_visible, show_edges)
                    .into_iter()
                    .map(|slot| match slot {
                        PageSlot::Ellipsis => view! {
                            <div class="d-pagination__separator">
                                <div
                                    class="d-pagination__separator-icon"
                                    data-qa="dt-pagination-separator"
                                >
                                    <DtIconMoreHorizontal size="300" />
                                </div>
                            </div>
                        }
                        .into_any(),
                        PageSlot::Page(page) => {
                            let label = page_aria_label(page, total_pages.get());
                            let class_name = page_button_class(current.get() == page);
                            view! {
                                <div>
                                    <button
                                        type="button"
                                        class=class_name
                                        data-qa="dt-button"
                                        aria-label=label
                                        on:click=move |_| select_page(current, page, on_change)
                                    >
                                        <span data-qa="dt-button-label" class=ITEM_LABEL_CLASS>
                                            {page}
                                        </span>
                                    </button>
                                </div>
                            }
                            .into_any()
                        }
                    })
                    .collect_view()
            }}
            <button
                type="button"
                class=next_class
                data-qa="dt-pagination-next"
                aria-label=move || next_aria_label(current.get(), total_pages.get())
                prop:disabled=move || current.get() == total_pages.get()
                on:click=move |_| {
                    select_page(current, current.get_untracked() + 1, on_change);
                }
            >
                <span data-qa="dt-button-icon" class=NAV_ICON_CLASS>
                    <DtIconChevronRight size="300" />
                </span>
            </button>
        </nav>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn listed(slots: &[PageSlot]) -> String {
        slots
            .iter()
            .map(|slot| match slot {
                PageSlot::Page(page) => page.to_string(),
                PageSlot::Ellipsis => "...".to_string(),
            })
            .collect::<Vec<_>>()
            .join(",")
    }

    fn button_count(slots: &[PageSlot]) -> usize {
        slots
            .iter()
            .filter(|slot| matches!(slot, PageSlot::Page(_)))
            .count()
            + 2
    }

    fn separators(slots: &[PageSlot]) -> usize {
        slots
            .iter()
            .filter(|slot| matches!(slot, PageSlot::Ellipsis))
            .count()
    }

    #[test]
    fn default_pages_have_no_separator() {
        let slots = pagination_pages(1, 5, 5, true);
        assert_eq!(listed(&slots), "1,2,3,4,5");
        assert_eq!(separators(&slots), 0);
        assert_eq!(prev_aria_label(1), "First page");
        assert_eq!(next_aria_label(1, 5), "Next page");
        assert_eq!(page_aria_label(1, 5), "Page number \u{2068}1\u{2069}");
    }

    #[test]
    fn active_page_near_the_end_has_one_separator() {
        let slots = pagination_pages(9, 10, 5, true);
        assert_eq!(listed(&slots), "1,...,7,8,9,10");
        assert_eq!(separators(&slots), 1);
    }

    #[test]
    fn middle_range_matches_the_vue_window() {
        let odd = pagination_pages(7, 15, 7, true);
        assert_eq!(listed(&odd), "1,...,5,6,7,8,9,...,15");
        assert_eq!(separators(&odd), 2);
        assert_eq!(button_count(&odd), 9);

        let even = pagination_pages(7, 15, 6, true);
        assert_eq!(listed(&even), "1,...,6,7,8,...,15");
        assert_eq!(button_count(&even), 7);
    }

    #[test]
    fn hidden_edges_drop_the_first_and_last_page() {
        let middle = pagination_pages(7, 15, 5, false);
        assert_eq!(listed(&middle), "...,5,6,7,8,9,...");
        assert_eq!(button_count(&middle), 7);

        let start = pagination_pages(1, 15, 5, false);
        assert_eq!(listed(&start), "1,2,3,4,5,...");
        assert_eq!(button_count(&start), 7);
        assert!(!listed(&start).ends_with("15"));

        let end = pagination_pages(14, 15, 5, false);
        assert_eq!(listed(&end), "...,11,12,13,14,15");
        assert_eq!(button_count(&end), 7);
        assert!(!listed(&end).starts_with('1'));
    }

    #[test]
    fn nav_and_page_classes_match_vue_button_order() {
        assert_eq!(
            nav_button_class(),
            "base-button__button d-btn d-btn--muted d-btn--icon-only d-pagination__button"
        );
        assert_eq!(
            page_button_class(true),
            "base-button__button d-btn d-btn--primary d-pagination__item"
        );
        assert_eq!(
            page_button_class(false),
            "base-button__button d-btn d-btn--muted d-pagination__item"
        );
        assert_eq!(next_aria_label(5, 5), "Last page");
        assert_eq!(page_aria_label(5, 5), "Last page 5");
        assert_eq!(prev_aria_label(2), "Previous page");
    }

    #[test]
    fn zero_max_visible_and_zero_total_draw_no_pages() {
        assert!(pagination_pages(1, 5, 0, true).is_empty());
        assert!(pagination_pages(1, 0, 5, true).is_empty());
    }
}
