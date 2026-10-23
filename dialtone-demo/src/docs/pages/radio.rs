use super::*;

const STATES: &[(&str, &str)] = &[
    ("", "None"),
    ("warning", "Warning"),
    ("critical", "Critical"),
    ("positive", "Positive"),
];

const SIZES: &[(&str, &str)] = &[
    ("", "Default (300)"),
    ("100", "100"),
    ("200", "200"),
    ("300", "300"),
    ("400", "400"),
];

const STRENGTHS: &[(&str, &str)] = &[
    ("", "Default (normal)"),
    ("bold", "Bold"),
    ("semibold", "Semibold"),
    ("medium", "Medium"),
    ("normal", "Normal"),
];

fn radio_messages(state: &str) -> Vec<DtValidationMessage> {
    match state {
        "warning" => vec![DtValidationMessage::warning("So they can hear your voice")],
        "critical" => vec![DtValidationMessage::critical(
            "Because they probably don't need to leave a message anyway.",
        )],
        "positive" => vec![DtValidationMessage::positive(
            "Because someone else might be able to talk to them.",
        )],
        _ => vec![],
    }
}

fn radio_messages_expr(state: &str) -> Option<&'static str> {
    Some(match state {
        "warning" => "vec![DtValidationMessage::warning(\"So they can hear your voice\")]",
        "critical" => {
            "vec![DtValidationMessage::critical(\"Because they probably don't need to leave a message anyway.\")]"
        }
        "positive" => {
            "vec![DtValidationMessage::positive(\"Because someone else might be able to talk to them.\")]"
        }
        _ => return None,
    })
}

#[component]
pub(crate) fn RadioPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("radio.rs")));
    let checked = RwSignal::new(false);
    let label = RwSignal::new("To voicemail".to_string());
    let description = RwSignal::new(String::new());
    let value = RwSignal::new("voicemail".to_string());
    let name = RwSignal::new("routing".to_string());
    let disabled = RwSignal::new(false);
    let show_label = RwSignal::new(true);
    let state = RwSignal::new("");
    let show_messages = RwSignal::new(true);
    let label_size = RwSignal::new("");
    let label_strength = RwSignal::new("");
    let slot_label = RwSignal::new(String::new());
    let slot_description = RwSignal::new(String::new());
    let root_class = RwSignal::new(String::new());
    let input_class = RwSignal::new(String::new());
    let label_class = RwSignal::new(String::new());
    let description_class = RwSignal::new(String::new());
    let messages_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let label = label.get();
                let description = description.get();
                let value = value.get();
                let name = name.get();
                let state = state.get();
                let label_size = label_size.get();
                let label_strength = label_strength.get();
                let messages = radio_messages(state);
                let show = show_messages.get();
                let description_view = text_slot(slot_description.get());
                let slot_label = slot_label.get();
                let class_name = root_class.get();
                let input_class = input_class.get();
                let label_class = label_class.get();
                let description_class = description_class.get();
                let messages_class = messages_class.get();
                if slot_label.is_empty() {
                    open_one!(DtRadio {
                        value=value
                        checked=Signal::derive(move || checked.get())
                        on_change=move |_| checked.set(true)
                        label=label
                        description=description
                        name=name
                        disabled=disabled.get()
                        show_label=show_label.get()
                        validation_state=state
                        label_size=label_size
                        label_strength=label_strength
                        messages=messages
                        show_messages=show
                        class=class_name
                        input_class=input_class
                        label_class=label_class
                        description_class=description_class
                        messages_class=messages_class
                    } { }, description_view, description_slot)
                } else {
                    open_one!(DtRadio {
                        value=value
                        checked=Signal::derive(move || checked.get())
                        on_change=move |_| checked.set(true)
                        label=label
                        description=description
                        name=name
                        disabled=disabled.get()
                        show_label=show_label.get()
                        validation_state=state
                        label_size=label_size
                        label_strength=label_strength
                        messages=messages
                        show_messages=show
                        class=class_name
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
            <SettingText label="Value" value=value />
            <SettingText label="Name" value=name />
            <SettingSelect label="Validation" value=state options=STATES.to_vec() />
            <SettingSelect label="Label size" value=label_size options=SIZES.to_vec() />
            <SettingSelect label="Label strength" value=label_strength options=STRENGTHS.to_vec() />
            <SettingCheck label="Checked" value=checked />
            <SettingCheck label="Show label" value=show_label />
            <SettingCheck label="Disabled" value=disabled />
            <SettingCheck label="Show messages" value=show_messages />
        }
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingText label="Default" value=slot_label hint="Replaces the label text." />
            <SettingText label="Description" value=slot_description hint="Replaces the description." />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "value", &value.get());
        push_str(&mut props, "name", &name.get());
        push_str(&mut props, "label", &label.get());
        push_str(&mut props, "description", &description.get());
        push_expr(
            &mut props,
            "checked",
            "Signal::derive(move || checked.get())",
        );
        push_expr(&mut props, "on_change", "move |_| checked.set(true)");
        if !show_label.get() {
            props.push(("show_label", "false".to_string()));
        }
        push_bool(&mut props, "disabled", disabled.get());
        push_str(&mut props, "validation_state", state.get());
        push_str(&mut props, "label_size", label_size.get());
        push_str(&mut props, "label_strength", label_strength.get());
        push_str(&mut props, "class", &root_class.get());
        push_str(&mut props, "input_class", &input_class.get());
        push_str(&mut props, "label_class", &label_class.get());
        push_str(&mut props, "description_class", &description_class.get());
        push_str(&mut props, "messages_class", &messages_class.get());
        if let Some(expr) = radio_messages_expr(state.get()) {
            push_expr(&mut props, "messages", expr);
        }
        if !show_messages.get() {
            push_flag(&mut props, "show_messages", false);
        }
        push_span_slot(&mut props, "description_slot", &slot_description.get());
        let body = if slot_label.get().is_empty() {
            None
        } else {
            Some(slot_label.get())
        };
        view_of(&element("DtRadio", &props, body.as_deref()))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"on_change reports the option value and checks the radio. Show label false moves the label into aria-label. A critical message sets aria-invalid."</p>
            <Play
                stack=true
                preview=preview
                settings=settings
                slots=slots
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
                <Variant title="Disabled">
                    <DtRadio name="disabled" value="disabled" label="Radio label thats been disabled" disabled=true />
                    <DtRadio name="checked-disabled" value="checked" label="Radio label thats been disabled & checked" checked=Signal::derive(|| true) disabled=true />
                </Variant>
                <Variant title="Description">
                    <DtRadio
                        name="described"
                        value="voicemail"
                        label="To voicemail"
                        description="So they can hear your voice"
                    />
                </Variant>
                <Variant title="Validation">
                    <DtRadio
                        name="validation"
                        value="warning"
                        label="To voicemail"
                        validation_state="warning"
                        messages=vec![DtValidationMessage::warning("So they can hear your voice")]
                    />
                    <DtRadio
                        name="validation"
                        value="critical"
                        label="To a message (no voicemail)"
                        validation_state="critical"
                        messages=vec![DtValidationMessage::critical("Because they probably don't need to leave a message anyway.")]
                    />
                    <DtRadio
                        name="validation"
                        value="positive"
                        label="To a team member or room phone"
                        validation_state="positive"
                        messages=vec![DtValidationMessage::positive("Because someone else might be able to talk to them.")]
                    />
                </Variant>
                <Variant title="Label size">
                    <DtRadio name="label-size" value="xs" label="Extra small label" label_size="100" />
                    <DtRadio name="label-size" value="sm" label="Small label" label_size="200" />
                    <DtRadio name="label-size" value="md" label="Medium label (default)" label_size="300" />
                    <DtRadio name="label-size" value="lg" label="Large label" label_size="400" />
                </Variant>
            </div>
        </section>
    }
}
