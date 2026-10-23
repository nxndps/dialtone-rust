use super::*;

fn skeleton_preview(kind: &'static str, rows: u8, animate: bool, aria_label: String) -> AnyView {
    let labeled = !aria_label.is_empty();
    match (kind, labeled) {
        ("list", false) => view! {
            <DtSkeleton animate=animate list_item_option=DtSkeletonListItemOption::default() />
        }
        .into_any(),
        ("list", true) => view! {
            <DtSkeleton
                animate=animate
                aria_label=aria_label
                list_item_option=DtSkeletonListItemOption::default()
            />
        }
        .into_any(),
        ("circle", false) => view! {
            <DtSkeleton
                animate=animate
                shape_option=DtSkeletonShapeOption { shape: DtSkeletonShapeKind::Circle, size: "lg" }
            />
        }
        .into_any(),
        ("circle", true) => view! {
            <DtSkeleton
                animate=animate
                aria_label=aria_label
                shape_option=DtSkeletonShapeOption { shape: DtSkeletonShapeKind::Circle, size: "lg" }
            />
        }
        .into_any(),
        ("square", false) => view! {
            <DtSkeleton
                animate=animate
                shape_option=DtSkeletonShapeOption { shape: DtSkeletonShapeKind::Square, size: "lg" }
            />
        }
        .into_any(),
        ("square", true) => view! {
            <DtSkeleton
                animate=animate
                aria_label=aria_label
                shape_option=DtSkeletonShapeOption { shape: DtSkeletonShapeKind::Square, size: "lg" }
            />
        }
        .into_any(),
        ("text", false) => view! {
            <DtSkeleton animate=animate text_option=DtSkeletonTextOption::default() />
        }
        .into_any(),
        ("text", true) => view! {
            <DtSkeleton
                animate=animate
                aria_label=aria_label
                text_option=DtSkeletonTextOption::default()
            />
        }
        .into_any(),
        (_, true) => view! {
            <DtSkeleton
                animate=animate
                aria_label=aria_label
                paragraph_option=DtSkeletonParagraphOption { rows, ..Default::default() }
            />
        }
        .into_any(),
        _ => view! {
            <DtSkeleton
                animate=animate
                paragraph_option=DtSkeletonParagraphOption { rows, ..Default::default() }
            />
        }
        .into_any(),
    }
}

#[component]
pub(crate) fn SkeletonPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("skeleton.rs")));
    let kind = RwSignal::new("paragraph");
    let rows = RwSignal::new("3");
    let animate = RwSignal::new(true);
    let aria_label = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let animate = animate.get();
                let rows = rows.get().parse::<u8>().unwrap_or(3);
                let aria_label = aria_label.get();
                skeleton_preview(kind.get(), rows, animate, aria_label)
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingSelect
                label="Shape"
                value=kind
                options=vec![
                    ("paragraph", "Paragraph"),
                    ("text", "Text"),
                    ("list", "List item"),
                    ("circle", "Circle"),
                    ("square", "Square"),
                ]
            />
            <SettingSelect label="Paragraph rows" value=rows options=vec![("2", "2"), ("3", "3"), ("4", "4")] />
            <SettingCheck label="Animate" value=animate />
            <SettingText label="Aria label" value=aria_label />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_flag(&mut props, "animate", animate.get());
        push_str(&mut props, "aria_label", &aria_label.get());
        let rows = rows.get().parse::<u8>().unwrap_or(3);
        match kind.get() {
            "list" => push_expr(
                &mut props,
                "list_item_option",
                "DtSkeletonListItemOption::default()",
            ),
            "circle" => push_expr(
                &mut props,
                "shape_option",
                "DtSkeletonShapeOption { shape: DtSkeletonShapeKind::Circle, size: \"lg\" }",
            ),
            "square" => push_expr(
                &mut props,
                "shape_option",
                "DtSkeletonShapeOption { shape: DtSkeletonShapeKind::Square, size: \"lg\" }",
            ),
            "text" => push_expr(&mut props, "text_option", "DtSkeletonTextOption::default()"),
            _ => push_expr(
                &mut props,
                "paragraph_option",
                format!("DtSkeletonParagraphOption {{ rows: {rows}, ..Default::default() }}"),
            ),
        }
        view_of(&element("DtSkeleton", &props, None))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"Pass one option. Paragraph rows apply when the shape is Paragraph."</p>
            <Play stack=true preview=preview settings=settings code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="List and shapes" row=true>
                    <DtSkeleton list_item_option=DtSkeletonListItemOption::default() />
                    <DtSkeleton shape_option=DtSkeletonShapeOption { shape: DtSkeletonShapeKind::Circle, size: "lg" } />
                    <DtSkeleton shape_option=DtSkeletonShapeOption { shape: DtSkeletonShapeKind::Square, size: "lg" } />
                </Variant>
            </div>
        </section>
    }
}
