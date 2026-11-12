use leptos::prelude::*;

/// `end` and `right` flip the sidebar to the other side.
fn sidebar_inverted(position: &str) -> bool {
    match position {
        "end" | "right" => true,
        "start" | "left" | "" => false,
        _ => {
            leptos::logging::warn!(
                "DtRootLayout sidebar_position `{position}` is not start, end, left, or right"
            );
            false
        }
    }
}

/// Empty is Vue `null`: `d-root-layout__responsive--default`.
fn responsive_class(breakpoint: &str) -> &'static str {
    match breakpoint {
        "" => "d-root-layout__responsive--default",
        "sm" => "d-root-layout__responsive--sm",
        "md" => "d-root-layout__responsive--md",
        "lg" => "d-root-layout__responsive--lg",
        _ => {
            leptos::logging::warn!(
                "DtRootLayout responsive_breakpoint `{breakpoint}` is not sm, md, lg, or empty"
            );
            "d-root-layout__responsive--default"
        }
    }
}

/// Dialtone Vue `DtRootLayout`. Follows `dialtone/v10.7.0`.
///
/// `sidebar_position` is `start` (default), `end`, `left`, or `right`. `end`
/// and `right` add `d-root-layout--inverted`. `fixed` stays false here. Vue's
/// default is true, and that modifier is `100vh` on the responsive grid.
/// `bodyClass`, `headerHeight`, `sidebarWidth`, and `footerHeight` are unused
/// in the Vue template. The footer element is always rendered.
#[component]
pub fn DtRootLayout(
    /// Lock the header, sidebar, and footer (`fixed`). Default false.
    #[prop(optional)]
    fixed: bool,
    #[prop(optional)] header_sticky: bool,
    /// `start` (default), `end`, `left`, or `right` (`sidebarPosition`).
    #[prop(optional, default = "start")]
    sidebar_position: &'static str,
    /// `sm`, `md`, `lg`, or empty for the default breakpoint (`responsiveBreakpoint`).
    #[prop(optional, into)]
    responsive_breakpoint: String,
    #[prop(optional, into)] header_class: String,
    #[prop(optional, into)] sidebar_class: String,
    #[prop(optional, into)] content_class: String,
    #[prop(optional, into)] footer_class: String,
    #[prop(optional)] header: Option<ViewFn>,
    #[prop(optional)] sidebar: Option<ViewFn>,
    #[prop(optional)] footer: Option<ViewFn>,
    children: ChildrenFn,
) -> impl IntoView {
    let inverted = sidebar_inverted(sidebar_position);
    let responsive = responsive_class(&responsive_breakpoint);
    view! {
        <div
            class=format!("root-layout d-root-layout {responsive}")
            class=("d-root-layout--fixed", fixed)
            class=("d-root-layout--inverted", inverted)
            data-qa="dt-root-layout"
        >
            <header
                class=format!("d-root-layout__header {header_class}")
                class=("d-root-layout__header--sticky", header_sticky)
                data-qa="dt-root-layout-header"
            >
                {header.as_ref().map(|slot| slot.run())}
            </header>
            <aside
                class=format!("d-root-layout__sidebar {sidebar_class}")
                tabindex="0"
                data-qa="dt-root-layout-sidebar"
            >
                {sidebar.as_ref().map(|slot| slot.run())}
            </aside>
            <main
                class=format!("d-root-layout__content {content_class}")
                tabindex="0"
                data-qa="dt-root-layout-content"
            >
                {children()}
            </main>
            <footer
                class=format!("d-root-layout__footer {footer_class}")
                data-qa="dt-root-layout-footer"
            >
                {footer.as_ref().map(|slot| slot.run())}
            </footer>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::{responsive_class, sidebar_inverted};

    #[test]
    fn end_and_right_invert_the_sidebar() {
        assert!(!sidebar_inverted("start"));
        assert!(!sidebar_inverted("left"));
        assert!(!sidebar_inverted(""));
        assert!(sidebar_inverted("end"));
        assert!(sidebar_inverted("right"));
        assert!(!sidebar_inverted("middle"));
    }

    #[test]
    fn breakpoint_tokens_map_to_the_responsive_class() {
        assert_eq!(responsive_class(""), "d-root-layout__responsive--default");
        assert_eq!(responsive_class("sm"), "d-root-layout__responsive--sm");
        assert_eq!(responsive_class("md"), "d-root-layout__responsive--md");
        assert_eq!(responsive_class("lg"), "d-root-layout__responsive--lg");
        assert_eq!(responsive_class("xl"), "d-root-layout__responsive--default");
    }
}
