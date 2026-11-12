use super::*;

const MODAL_TITLES: &[(&str, &str)] = &[
    ("Edit mapping", "Edit mapping"),
    ("Confirm", "Confirm"),
    ("Schedule", "Schedule"),
];

const MODAL_KINDS: &[(&str, &str)] = &[("default", "Default"), ("critical", "Critical")];

#[component]
pub(crate) fn ModalPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("modal.rs")));
    let show = RwSignal::new(false);
    let title = RwSignal::new("Edit mapping");
    let kind = RwSignal::new("default");
    let modal = RwSignal::new(false);
    let close_on_click = RwSignal::new(true);
    let show_close = RwSignal::new(true);
    let fullscreen = RwSignal::new(false);
    let message = RwSignal::new("This schedule is not mapped yet.".to_string());
    let footer_label = RwSignal::new("Close".to_string());
    let dialog_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            <DtButton on_click=move || show.set(true)>"Open modal"</DtButton>
            {move || {
                let header_text = title.get();
                let kind = kind.get();
                let modal = modal.get();
                let close_on_click = close_on_click.get();
                let show_close = show_close.get();
                let fullscreen = fullscreen.get();
                let dialog_class = dialog_class.get();
                view! {
                    <DtModal
                        open=show
                        on_open=move |next| show.set(next)
                        labelled_by_id="docs-modal-title"
                        header_text=header_text
                        kind=kind
                        modal=modal
                        close_on_click=close_on_click
                        show_close=show_close
                        fullscreen=fullscreen
                        dialog_class=dialog_class
                        footer=ViewFn::from(move || {
                            view! {
                                <DtButton importance="outlined" on_click=move || show.set(false)>
                                    {move || footer_label.get()}
                                </DtButton>
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
            <SettingSelect label="Header" value=title options=MODAL_TITLES.to_vec() />
            <SettingSelect label="Kind" value=kind options=MODAL_KINDS.to_vec() />
            <SettingCheck label="Modal" value=modal />
            <SettingCheck label="Close on click" value=close_on_click />
            <SettingCheck label="Show close" value=show_close />
            <SettingCheck label="Fullscreen" value=fullscreen />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_expr(&mut props, "open", "open");
        push_expr(&mut props, "on_open", "move |next| open.set(next)");
        push_str(&mut props, "header_text", title.get());
        push_str(&mut props, "labelled_by_id", "docs-modal-title");
        if kind.get() != "default" {
            push_str(&mut props, "kind", kind.get());
        }
        if modal.get() {
            push_bool(&mut props, "modal", true);
        }
        if !close_on_click.get() {
            props.push(("close_on_click", "false".to_string()));
        }
        if !show_close.get() {
            props.push(("show_close", "false".to_string()));
        }
        if fullscreen.get() {
            push_bool(&mut props, "fullscreen", true);
        }
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
            <p class="docs-note">"The dialog is a native dialog. The default calls show(), so it is not the top layer. A backdrop click closes when the press and the release are both on the backdrop. close_on_click defaults to true. The CSS max-inline-size is the layout token."</p>
            <Play
                preview=preview
                settings=settings
                slots=ViewFn::from(move || view! {
                    <SettingText label="Default" hint="Dialog body." value=message />
                    <SettingText label="Footer" hint="Footer button label." value=footer_label />
                })
                classes=class_panel(vec![ClassField {
                    label: "Dialog class",
                    hint: "Extra class on the dialog panel.",
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
                    <p class="docs-note">"Open the dialog from the playground. The close button, the footer, and a backdrop click can close it."</p>
                </Variant>
            </div>
        </section>
    }
}
