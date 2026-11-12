use super::*;

#[component]
pub(crate) fn LoaderPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("loader.rs")));
    let kind = RwSignal::new("dots");
    let size = RwSignal::new("500");
    let label = RwSignal::new("Loading schedules".to_string());
    let root_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || view! {
                <DtLoader
                    kind=kind.get()
                    size=size.get()
                    aria_label=label.get()
                    class=root_class.get()
                />
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingSelect
                label="Kind"
                value=kind
                options=vec![("dots", "dots"), ("spinner", "spinner")]
            />
            <SettingSelect
                label="Size"
                value=size
                options=vec![
                    ("100", "100"),
                    ("200", "200"),
                    ("300", "300"),
                    ("400", "400"),
                    ("500", "500"),
                    ("600", "600"),
                    ("700", "700"),
                    ("800", "800"),
                ]
            />
            <SettingText label="Accessible name" value=label />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "kind", kind.get());
        push_str(&mut props, "size", size.get());
        push_str(&mut props, "aria_label", &label.get());
        push_str(&mut props, "class", &root_class.get());
        view_of(&element("DtLoader", &props, None))
    });
    let classes = class_panel(vec![ClassField {
        label: "Class",
        hint: ROOT_CLASS,
        value: root_class,
        off: None,
    }]);
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"An omitted kind is dots. An omitted size is 500. This playground passes both. spinner adds d-loader--spinner and draws the circular mark."</p>
            <Play preview=preview settings=settings classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Spinner">
                    <DtLoader kind="spinner" />
                </Variant>
                <Variant title="Sizes" row=true>
                    <DtLoader size="200" />
                    <DtLoader size="400" />
                    <DtLoader size="600" />
                </Variant>
            </div>
        </section>
    }
}
