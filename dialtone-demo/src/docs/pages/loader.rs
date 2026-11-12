use super::*;

#[component]
pub(crate) fn LoaderPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("loader.rs")));
    let size = RwSignal::new("300");
    let label = RwSignal::new("Loading schedules".to_string());
    let root_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || view! { <DtLoader size=size.get() aria_label=label.get() class=root_class.get() /> }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingSelect
                label="Size"
                value=size
                options=vec![("200", "200"), ("300", "300"), ("400", "400"), ("500", "500"), ("600", "600")]
            />
            <SettingText label="Accessible name" value=label />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
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
            <p class="docs-note">"An omitted size uses 500. This playground passes size. This port draws the v10.7 spinner paths and does not add d-loader--spinner, so the vendored spin does not run. Dialtone 10.11.0 defaults the loader to four dots."</p>
            <Play preview=preview settings=settings classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Sizes" row=true>
                    <DtLoader size="200" />
                    <DtLoader size="400" />
                    <DtLoader size="600" />
                </Variant>
            </div>
        </section>
    }
}
