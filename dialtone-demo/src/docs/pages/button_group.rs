use super::*;

#[component]
pub(crate) fn ButtonGroupPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("button_group.rs")));
    let alignment = RwSignal::new("start");
    let range = RwSignal::new("week");
    let slot_text = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let alignment = alignment.get();
                let current = range.get();
                let slot_text = slot_text.get();
                view! {
                    <DtButtonGroup alignment=alignment>
                        <DtButton importance="outlined" active=current == "day" on_click=move || range.set("day")>"Day"</DtButton>
                        <DtButton importance="outlined" active=current == "week" on_click=move || range.set("week")>"Week"</DtButton>
                        <DtButton importance="outlined" active=current == "month" on_click=move || range.set("month")>"Month"</DtButton>
                        {(!slot_text.is_empty()).then(|| {
                            let slot_text = slot_text.clone();
                            view! { <span>{slot_text}</span> }
                        })}
                    </DtButtonGroup>
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingSelect
                label="Alignment"
                value=alignment
                options=vec![("start", "Start"), ("end", "End"), ("space-between", "Space between")]
            />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "alignment", alignment.get());
        let current = range.get();
        let button = |name: &str, active: bool| {
            let mut button_props = Vec::new();
            push_str(&mut button_props, "importance", "outlined");
            push_bool(&mut button_props, "active", active);
            element("DtButton", &button_props, Some(&rust_string(name)))
        };
        let mut body = format!(
            "{}\n{}\n{}",
            button("Day", current == "day"),
            button("Week", current == "week"),
            button("Month", current == "month"),
        );
        if !slot_text.get().is_empty() {
            body.push('\n');
            body.push_str(&element("span", &[], Some(&rust_string(&slot_text.get()))));
        }
        view_of(&element("DtButtonGroup", &props, Some(&body)))
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingText label="Default" hint="Added after the buttons. Empty leaves them alone." value=slot_text />
        }
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <Play preview=preview settings=settings slots=slots code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="End">
                    <DtButtonGroup alignment="end">
                        <DtButton importance="outlined">"Cancel"</DtButton>
                        <DtButton>"Save"</DtButton>
                    </DtButtonGroup>
                </Variant>
                <Variant title="Space between">
                    <DtButtonGroup alignment="space-between">
                        <DtButton importance="clear">"Back"</DtButton>
                        <DtButton>"Continue"</DtButton>
                    </DtButtonGroup>
                </Variant>
            </div>
        </section>
    }
}
