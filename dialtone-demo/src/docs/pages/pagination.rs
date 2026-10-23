use super::*;

fn number_setting(value: &str, fallback: i32) -> i32 {
    value.trim().parse().unwrap_or(fallback)
}

#[component]
pub(crate) fn PaginationPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("pagination.rs")));
    let aria_label = RwSignal::new("pagination".to_string());
    let total_pages = RwSignal::new("5".to_string());
    let active_page = RwSignal::new("1".to_string());
    let max_visible = RwSignal::new("5".to_string());
    let show_edges = RwSignal::new(true);
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let label = aria_label.get();
                let total = number_setting(&total_pages.get(), 5);
                let page = number_setting(&active_page.get(), 1);
                let visible = number_setting(&max_visible.get(), 5);
                let edges = show_edges.get();
                view! {
                    <DtPagination
                        aria_label=label
                        total_pages=total
                        active_page=page
                        max_visible=visible
                        show_edges=edges
                        on_change=move |next: i32| active_page.set(next.to_string())
                    />
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Aria label" value=aria_label />
            <SettingText label="Total pages" value=total_pages />
            <SettingText label="Active page" value=active_page />
            <SettingText label="Max visible" value=max_visible hint="Odd numbers keep the current page centered." />
            <SettingCheck label="Show edges" value=show_edges />
        }
    });
    let code = example_panel(move || {
        let mut lines = Vec::new();
        lines.push(format!(
            "let page = RwSignal::new({});",
            number_setting(&active_page.get(), 1)
        ));
        let mut props = Vec::new();
        push_str(&mut props, "aria_label", &aria_label.get());
        push_expr(
            &mut props,
            "total_pages",
            number_setting(&total_pages.get(), 5).to_string(),
        );
        let page = number_setting(&active_page.get(), 1);
        if page != 1 {
            push_expr(&mut props, "active_page", "page");
        }
        let visible = number_setting(&max_visible.get(), 5);
        if visible != 5 {
            push_expr(&mut props, "max_visible", visible.to_string());
        }
        if !show_edges.get() {
            push_flag(&mut props, "show_edges", false);
        }
        push_expr(&mut props, "on_change", "move |next| page.set(next)");
        lines.push(view_of(&element("DtPagination", &props, None)));
        lines.join("\n")
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"Click a page or an arrow. The first page disables the previous arrow, and the last page disables the next arrow. An even max visible rounds down in the middle so the current page stays centered."</p>
            <Play preview=preview settings=settings code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="With active page">
                    <ActivePage />
                </Variant>
                <Variant title="With max visible">
                    <MaxVisible />
                </Variant>
            </div>
        </section>
    }
}

#[component]
fn ActivePage() -> impl IntoView {
    let page = RwSignal::new(5);
    view! {
        <DtPagination
            aria_label="pagination".to_string()
            total_pages=25
            active_page=page
            on_change=move |next| page.set(next)
        />
    }
}

#[component]
fn MaxVisible() -> impl IntoView {
    let page = RwSignal::new(1);
    view! {
        <DtPagination
            aria_label="pagination".to_string()
            total_pages=25
            active_page=page
            max_visible=7
            on_change=move |next| page.set(next)
        />
    }
}
