use super::*;

#[component]
pub(crate) fn ValidationMessagesPage() -> impl IntoView {
    let kind = RwSignal::new("critical");
    let message = RwSignal::new("Choose a product".to_string());
    let id = RwSignal::new(String::new());
    let show_messages = RwSignal::new(true);
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let message = message.get();
                let id = id.get();
                let show_messages = show_messages.get();
                let row = match kind.get() {
                    "warning" => DtValidationMessage::warning(message),
                    "positive" => DtValidationMessage::positive(message),
                    "info" => DtValidationMessage::info(message),
                    _ => DtValidationMessage::critical(message),
                };
                view! {
                    <DtValidationMessages
                        id=id
                        show_messages=show_messages
                        validation_messages=vec![row]
                    />
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Id" value=id />
            <SettingSelect
                label="Kind"
                value=kind
                options=vec![
                    ("critical", "Critical"),
                    ("warning", "Warning"),
                    ("positive", "Positive"),
                    ("info", "Info"),
                ]
            />
            <SettingText label="Message" value=message />
            <SettingCheck label="Show messages" value=show_messages />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "id", &id.get());
        let constructor = match kind.get() {
            "warning" => "warning",
            "positive" => "positive",
            "info" => "info",
            _ => "critical",
        };
        let body = format!(
            "vec![DtValidationMessage::{constructor}({})]",
            rust_string(&message.get())
        );
        push_expr(&mut props, "validation_messages", body);
        if !show_messages.get() {
            push_expr(&mut props, "show_messages", "false");
        }
        view_of(&element("DtValidationMessages", &props, None))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"When a list mixes kinds, critical hides warning, positive, and info."</p>
            <Play stack=true preview=preview settings=settings code=code />
        </section>
    }
}
