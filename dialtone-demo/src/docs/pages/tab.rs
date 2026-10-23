use super::*;

#[component]
pub(crate) fn TabPage() -> impl IntoView {
    let selected = RwSignal::new("overview");
    let preview = ViewFn::from(move || {
        view! {
            <div class="flex flex-col gap-3">
                <div class="flex gap-2" role="tablist">
                    {move || {
                        let current = selected.get();
                        view! {
                            <DtTab
                                id="overview"
                                panel_id="overview"
                                selected={current == "overview"}
                                on_click=move || selected.set("overview")
                            >
                                "Overview"
                            </DtTab>
                            <DtTab
                                id="people"
                                panel_id="people"
                                selected={current == "people"}
                                on_click=move || selected.set("people")
                            >
                                "People"
                            </DtTab>
                        }
                    }}
                </div>
                <p class="m-0 text-sm">
                    {move || if selected.get() == "overview" { "Overview panel" } else { "People panel" }}
                </p>
            </div>
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingSelect
                label="Selected"
                value=selected
                options=vec![("overview", "Overview"), ("people", "People")]
            />
        }
    });
    let code = example_panel(move || {
        let current = selected.get();
        let tab = |id: &str, label: &str| {
            let mut props = vec![("id", rust_string(id)), ("panel_id", rust_string(id))];
            push_bool(&mut props, "selected", current == id);
            element("DtTab", &props, Some(&rust_string(label)))
        };
        format!(
            "{}\n{}",
            tab("overview", "Overview"),
            tab("people", "People")
        )
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"A tab outside a group uses selected as its own state. The button is the v10 clear tab: muted when idle, and d-tab--is-selected on the default kind. Disabled sets aria-disabled and does not set the native disabled attribute. Grouped tabs are on the Tab group page."</p>
            <Play stack=true preview=preview settings=settings code=code />
        </section>
    }
}
