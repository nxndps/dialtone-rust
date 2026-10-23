use super::*;

fn illustration_options() -> Vec<(&'static str, &'static str)> {
    ILLUSTRATION_NAMES
        .iter()
        .copied()
        .map(|name| (name, name))
        .collect()
}

#[component]
pub(crate) fn IllustrationPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("illustration.rs")));
    let name = RwSignal::new("mind");
    let root_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let name = name.get();
                let class_name = root_class.get();
                view! { <DtIllustration name=name class=class_name /> }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! { <SettingSelect label="Name" value=name options=illustration_options() /> }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "name", name.get());
        push_str(&mut props, "class", &root_class.get());
        view_of(&element("DtIllustration", &props, None))
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
            <p class="docs-note">
                "Name is the kebab-case illustration. An unknown name renders nothing. The SVG is inlined, and ids are prefixed so two copies do not share a gradient. Class is added on the SVG."
            </p>
            <Play preview=preview settings=settings classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Catalog">
                    <div class="demo-icons">
                        {ILLUSTRATION_NAMES.iter().copied().map(|name| {
                            view! {
                                <div class="demo-icon">
                                    <DtIllustration name=name />
                                    <span>{name}</span>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </Variant>
            </div>
        </section>
    }
}
