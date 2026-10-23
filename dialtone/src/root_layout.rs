use leptos::prelude::*;

/// Dialtone Vue `DtRootLayout`.
#[component]
pub fn DtRootLayout(
    #[prop(optional)] fixed: bool,
    #[prop(optional)] header_sticky: bool,
    /// `left` (default) or `right`.
    #[prop(optional, default = "left")]
    sidebar_position: &'static str,
    /// `sm`, `md`, `lg`, or empty for the default two-column layout.
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
    let inverted = sidebar_position == "right";
    let responsive = if responsive_breakpoint.is_empty() {
        "d-root-layout__responsive--default".to_string()
    } else {
        format!("d-root-layout__responsive--{responsive_breakpoint}")
    };
    view! {
        <div
            class=format!("d-root-layout {responsive}")
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
            {footer.as_ref().map(|slot| {
                view! {
                    <footer
                        class=format!("d-root-layout__footer {footer_class}")
                        data-qa="dt-root-layout-footer"
                    >
                        {slot.run()}
                    </footer>
                }
            })}
        </div>
    }
}
