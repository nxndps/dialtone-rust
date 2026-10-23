use super::*;

#[component]
pub(crate) fn InputGroupPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("input_group.rs")));
    let legend = RwSignal::new("Name".to_string());
    let first = RwSignal::new(String::new());
    let last = RwSignal::new(String::new());
    let root_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let group_class = join_extra("flex flex-col gap-3", &root_class.get());
                view! {
                    <DtInputGroup legend=legend.get() class=group_class>
                        <DtInput label="First" placeholder="Ada" value=first on_input=move |next| first.set(next) />
                        <DtInput label="Last" placeholder="Lovelace" value=last on_input=move |next| last.set(next) />
                    </DtInputGroup>
                }
            }}
        }
    });
    let settings = ViewFn::from(move || view! { <SettingText label="Legend" value=legend /> });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "legend", &legend.get());
        push_str(
            &mut props,
            "class",
            &join_extra("flex flex-col gap-3", &root_class.get()),
        );
        let field = |label: &str, placeholder: &str| {
            let mut field_props = Vec::new();
            push_str(&mut field_props, "label", label);
            push_str(&mut field_props, "placeholder", placeholder);
            push_expr(&mut field_props, "value", "value");
            push_expr(&mut field_props, "on_input", "move |next| value.set(next)");
            element("DtInput", &field_props, None)
        };
        let body = format!("{}\n{}", field("First", "Ada"), field("Last", "Lovelace"));
        view_of(&element("DtInputGroup", &props, Some(&body)))
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
            <Play stack=true preview=preview settings=settings classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Two fields">
                    <p class="docs-note">"The legend names the group. Each input keeps its own value."</p>
                </Variant>
            </div>
        </section>
    }
}
