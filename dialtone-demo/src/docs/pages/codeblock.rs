use super::*;

#[component]
pub(crate) fn CodeblockPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("codeblock.rs")));
    let text = RwSignal::new("<DtButton>\"Save\"</DtButton>".to_string());
    let preview = ViewFn::from(move || {
        view! { {move || view! { <DtCodeblock text=text.get() /> }} }
    });
    let settings = ViewFn::from(move || view! { <SettingArea label="Text" value=text /> });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "text", &text.get());
        view_of(&element("DtCodeblock", &props, None))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <Play stack=true preview=preview settings=settings code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Sample">
                    <DtCodeblock text="let open = RwSignal::new(true);\n<DtToast open=open message=\"Saved\" kind=\"positive\" duration=6000 />" />
                </Variant>
            </div>
        </section>
    }
}
