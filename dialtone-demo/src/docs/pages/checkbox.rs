use super::*;

#[component]
pub(crate) fn CheckboxPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("checkbox.rs")));
    let alerts = RwSignal::new(true);
    let label = RwSignal::new("Email alerts".to_string());
    let description = RwSignal::new("Send a message when the rotation changes.".to_string());
    let name = RwSignal::new(String::new());
    let value = RwSignal::new(String::new());
    let show_messages = RwSignal::new(true);
    let slot_label = RwSignal::new(String::new());
    let slot_description = RwSignal::new(String::new());
    let disabled = RwSignal::new(false);
    let indeterminate = RwSignal::new(false);
    let state = RwSignal::new("");
    let root_class = RwSignal::new(String::new());
    let input_class = RwSignal::new(String::new());
    let label_class = RwSignal::new(String::new());
    let description_class = RwSignal::new(String::new());
    let messages_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let state = state.get();
                let messages = if state == "error" {
                    vec![DtValidationMessage::error("Select this to continue")]
                } else {
                    vec![]
                };
                let label = label.get();
                let description = description.get();
                let name = name.get();
                let value = value.get();
                let disabled = disabled.get();
                let indeterminate = indeterminate.get();
                let show_messages = show_messages.get();
                let root_class = root_class.get();
                let input_class = input_class.get();
                let label_class = label_class.get();
                let description_class = description_class.get();
                let messages_class = messages_class.get();
                let slot_label = slot_label.get();
                let description_view = text_slot(slot_description.get());
                if slot_label.is_empty() {
                    open_one!(DtCheckbox {
                        checked=Signal::derive(move || alerts.get())
                        on_change=move |next| alerts.set(next)
                        label=label
                        description=description
                        name=name
                        value=value
                        disabled=disabled
                        indeterminate=indeterminate
                        validation_state=state
                        show_messages=show_messages
                        messages=messages
                        root_class=root_class
                        input_class=input_class
                        label_class=label_class
                        description_class=description_class
                        messages_class=messages_class
                    } { }, description_view, description_slot)
                } else {
                    open_one!(DtCheckbox {
                        checked=Signal::derive(move || alerts.get())
                        on_change=move |next| alerts.set(next)
                        label=label
                        description=description
                        name=name
                        value=value
                        disabled=disabled
                        indeterminate=indeterminate
                        validation_state=state
                        show_messages=show_messages
                        messages=messages
                        root_class=root_class
                        input_class=input_class
                        label_class=label_class
                        description_class=description_class
                        messages_class=messages_class
                    } {
                        {slot_label}
                    }, description_view, description_slot)
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Label" value=label />
            <SettingText label="Description" value=description />
            <SettingText label="Name" value=name />
            <SettingText label="Value" value=value />
            <SettingSelect
                label="Validation"
                value=state
                options=vec![("", "None"), ("error", "Error"), ("warning", "Warning"), ("success", "Success")]
            />
            <SettingCheck label="Checked" value=alerts />
            <SettingCheck label="Indeterminate" value=indeterminate />
            <SettingCheck label="Show messages" value=show_messages />
            <SettingCheck label="Disabled" value=disabled />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_flag(&mut props, "checked", alerts.get());
        push_expr(&mut props, "on_change", "move |next| checked.set(next)");
        push_str(&mut props, "label", &label.get());
        push_str(&mut props, "description", &description.get());
        push_str(&mut props, "name", &name.get());
        push_str(&mut props, "value", &value.get());
        push_bool(&mut props, "disabled", disabled.get());
        push_bool(&mut props, "indeterminate", indeterminate.get());
        push_str(&mut props, "validation_state", state.get());
        if !show_messages.get() {
            props.push(("show_messages", "false".to_string()));
        }
        push_str(&mut props, "root_class", &root_class.get());
        push_str(&mut props, "input_class", &input_class.get());
        push_str(&mut props, "label_class", &label_class.get());
        push_str(&mut props, "description_class", &description_class.get());
        push_str(&mut props, "messages_class", &messages_class.get());
        if state.get() == "error" {
            push_expr(
                &mut props,
                "messages",
                "vec![DtValidationMessage::error(\"Select this to continue\")]",
            );
        }
        push_span_slot(&mut props, "description_slot", &slot_description.get());
        let body = if slot_label.get().is_empty() {
            None
        } else {
            Some(rust_string(&slot_label.get()))
        };
        view_of(&element("DtCheckbox", &props, body.as_deref()))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"on_change writes the checked signal. An error state includes a validation message."</p>
            <Play
                stack=true
                preview=preview
                settings=settings
                slots=ViewFn::from(move || {
                    view! {
                        <SettingText label="Default" hint="Replaces the label." value=slot_label />
                        <SettingText label="Description" hint="Replaces the description." value=slot_description />
                    }
                })
                classes=class_panel(vec![
                    ClassField { label: "Class", hint: ROOT_CLASS, value: root_class, off: None },
                    ClassField { label: "Input class", hint: "Extra class on the input.", value: input_class, off: None },
                    ClassField { label: "Label class", hint: "Extra class on the label.", value: label_class, off: None },
                    ClassField { label: "Description class", hint: "Extra class on the description.", value: description_class, off: None },
                    ClassField { label: "Messages class", hint: "Extra class on the messages.", value: messages_class, off: None },
                ])
                code=code
            />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Validation">
                    <DtCheckbox label="Warning" validation_state="warning" />
                    <DtCheckbox label="Success" validation_state="success" />
                    <DtCheckbox
                        label="Error"
                        validation_state="error"
                        messages=vec![DtValidationMessage::error("Select this to continue")]
                    />
                </Variant>
            </div>
        </section>
    }
}
