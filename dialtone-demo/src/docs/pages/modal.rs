use super::*;

#[component]
pub(crate) fn ModalPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("modal.rs")));
    let show = RwSignal::new(false);
    let title = RwSignal::new("Edit mapping");
    let subtitle = RwSignal::new("Choose a Dialpad target".to_string());
    let message = RwSignal::new("This schedule is not mapped yet.".to_string());
    let footer_label = RwSignal::new("Close".to_string());
    let dialog_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            <DtButton on_click=move || show.set(true)>"Open modal"</DtButton>
            {move || {
                view! {
                    <DtModal
                        show=show
                        on_close=move || show.set(false)
                        labelled_by="docs-modal-title"
                        title=title.get()
                        subtitle=subtitle.get()
                        dialog_class=dialog_class.get()
                        footer=ViewFn::from(move || {
                            let footer_label = footer_label.get();
                            view! {
                                <DtButton importance="outlined" on_click=move || show.set(false)>{footer_label}</DtButton>
                            }
                        })
                    >
                        <p class="m-0 text-sm">{move || message.get()}</p>
                    </DtModal>
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingSelect
                label="Title"
                value=title
                options=vec![
                    ("Edit mapping", "Edit mapping"),
                    ("Confirm", "Confirm"),
                    ("Schedule", "Schedule"),
                ]
            />
            <SettingText label="Subtitle" value=subtitle />
            <SettingText label="Body" value=message />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_expr(&mut props, "show", "open");
        push_expr(&mut props, "on_close", "move || open.set(false)");
        push_str(&mut props, "title", title.get());
        push_str(&mut props, "subtitle", &subtitle.get());
        push_str(&mut props, "dialog_class", &dialog_class.get());
        push_expr(
            &mut props,
            "footer",
            format!(
                "ViewFn::from(|| view! {{ <DtButton importance=\"outlined\">{}</DtButton> }})",
                rust_string(&footer_label.get())
            ),
        );
        let body = element(
            "p",
            &[("class", rust_string("m-0 text-sm"))],
            Some(&rust_string(&message.get())),
        );
        view_of(&element("DtModal", &props, Some(&body)))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"The dialog keeps the mapping editor width, at least 800px. The title is one of the port's static strings."</p>
            <Play
                preview=preview
                settings=settings
                slots=ViewFn::from(move || view! {
                    <SettingText label="Default" hint="Dialog body." value=message />
                    <SettingText label="Footer" hint="Footer button label." value=footer_label />
                })
                classes=class_panel(vec![ClassField {
                    label: "Dialog class",
                    hint: "Extra class on the dialog.",
                    value: dialog_class,
                    off: None,
                }])
                code=code
            />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Closed">
                    <p class="docs-note">"Open the dialog from the playground. Closing it uses the close button, the footer, or the overlay."</p>
                </Variant>
            </div>
        </section>
    }
}
