use super::*;

const KINDS: &[(&str, &str)] = &[
    ("", "None"),
    ("positive", "Positive"),
    ("warning", "Warning"),
    ("critical", "Critical"),
];

fn group_messages(kind: &str) -> Vec<DtValidationMessage> {
    match kind {
        "positive" => vec![DtValidationMessage::positive("Positive validation message")],
        "warning" => vec![DtValidationMessage::warning("Warning validation message")],
        "critical" => vec![DtValidationMessage::critical("Critical validation message")],
        _ => vec![],
    }
}

fn intern(value: &str) -> &'static str {
    use std::cell::RefCell;
    use std::collections::HashMap;
    thread_local! {
        static CACHE: RefCell<HashMap<String, &'static str>> = RefCell::new(HashMap::new());
    }
    CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some(found) = cache.get(value) {
            return *found;
        }
        // `data_qa_*` props are `&'static str`, so each distinct value is leaked once.
        let leaked: &'static str = Box::leak(value.to_string().into_boxed_str());
        cache.insert(value.to_string(), leaked);
        leaked
    })
}

fn qa_or(value: &str, fallback: &'static str) -> &'static str {
    if value.is_empty() {
        fallback
    } else {
        intern(value)
    }
}

fn group_messages_expr(kind: &str) -> Option<&'static str> {
    Some(match kind {
        "positive" => "vec![DtValidationMessage::positive(\"Positive validation message\")]",
        "warning" => "vec![DtValidationMessage::warning(\"Warning validation message\")]",
        "critical" => "vec![DtValidationMessage::critical(\"Critical validation message\")]",
        _ => return None,
    })
}

#[component]
pub(crate) fn RadioGroupPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("radio_group.rs")));
    let name = RwSignal::new("fruits".to_string());
    let selected = RwSignal::new("apple".to_string());
    let legend = RwSignal::new("Fruits".to_string());
    let aria_label = RwSignal::new(String::new());
    let disabled = RwSignal::new(false);
    let show_messages = RwSignal::new(true);
    let kind = RwSignal::new("");
    let data_qa_group = RwSignal::new(String::new());
    let data_qa_legend = RwSignal::new(String::new());
    let data_qa_messages = RwSignal::new(String::new());
    let legend_slot = RwSignal::new(String::new());
    let default_slot = RwSignal::new(String::new());
    let legend_class = RwSignal::new(String::new());
    let messages_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let name_value = name.get();
                let legend_text = legend.get();
                let aria = aria_label.get();
                let legend_class_value = legend_class.get();
                let messages_class_value = messages_class.get();
                let show = show_messages.get();
                let disabled_value = disabled.get();
                let kind_value = kind.get();
                let group_qa = qa_or(&data_qa_group.get(), "radio-group");
                let legend_qa = qa_or(&data_qa_legend.get(), "radio-group-legend");
                let messages_qa = qa_or(&data_qa_messages.get(), "radio-group-messages");
                let custom = default_slot.get();
                let legend_text_slot = legend_slot.get();
                view! {
                    <div class="docs-stack">
                        {if custom.is_empty() {
                            open_one!(DtRadioGroup {
                                name=name_value.clone()
                                selected=selected
                                on_change=move |next| selected.set(next)
                                legend=legend_text.clone()
                                aria_label=aria.clone()
                                disabled=disabled_value
                                show_messages=show
                                messages=group_messages(kind_value)
                                legend_class=legend_class_value.clone()
                                messages_class=messages_class_value.clone()
                                data_qa_group=group_qa
                                data_qa_group_legend=legend_qa
                                data_qa_group_messages=messages_qa
                            } {
                                <DtRadio value="apple" label="Apple" />
                                <DtRadio value="banana" label="Banana" />
                                <DtRadio value="other" label="Other" />
                            }, text_slot(legend_text_slot.clone()), legend_slot)
                        } else {
                            open_one!(DtRadioGroup {
                                name=name_value
                                selected=selected
                                on_change=move |next| selected.set(next)
                                legend=legend_text
                                aria_label=aria
                                disabled=disabled_value
                                show_messages=show
                                messages=group_messages(kind_value)
                                legend_class=legend_class_value
                                messages_class=messages_class_value
                                data_qa_group=group_qa
                                data_qa_group_legend=legend_qa
                                data_qa_group_messages=messages_qa
                            } {
                                {custom}
                            }, text_slot(legend_text_slot), legend_slot)
                        }}
                        <p class="docs-readout">
                            {move || {
                                let value = selected.get();
                                if value.is_empty() {
                                    "None selected".to_string()
                                } else {
                                    format!("Selected: {value}")
                                }
                            }}
                        </p>
                    </div>
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Name" value=name />
            <SettingText label="Legend" value=legend />
            <SettingText label="Aria label" value=aria_label hint="Used when the legend is empty." />
            <SettingSelect label="Message" value=kind options=KINDS.to_vec() />
            <SettingText label="Data qa group" hint="Empty uses radio-group." value=data_qa_group />
            <SettingText label="Data qa group legend" hint="Empty uses radio-group-legend." value=data_qa_legend />
            <SettingText label="Data qa group messages" hint="Empty uses radio-group-messages." value=data_qa_messages />
            <SettingCheck label="Disabled" value=disabled />
            <SettingCheck label="Show messages" value=show_messages />
        }
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingText label="Legend" value=legend_slot hint="Replaces the legend text." />
            <SettingText label="Default" hint="Default slot. Empty keeps the fruit options." value=default_slot />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "name", &name.get());
        push_str(&mut props, "legend", &legend.get());
        push_str(&mut props, "aria_label", &aria_label.get());
        push_str(&mut props, "legend_class", &legend_class.get());
        push_str(&mut props, "messages_class", &messages_class.get());
        push_str(&mut props, "data_qa_group", &data_qa_group.get());
        push_str(&mut props, "data_qa_group_legend", &data_qa_legend.get());
        push_str(
            &mut props,
            "data_qa_group_messages",
            &data_qa_messages.get(),
        );
        push_expr(&mut props, "selected", "selected");
        push_expr(&mut props, "on_change", "move |next| selected.set(next)");
        push_bool(&mut props, "disabled", disabled.get());
        if let Some(expr) = group_messages_expr(kind.get()) {
            push_expr(&mut props, "messages", expr);
        }
        if !show_messages.get() {
            push_flag(&mut props, "show_messages", false);
        }
        push_span_slot(&mut props, "legend_slot", &legend_slot.get());
        let option = |value: &str, label: &str| {
            element(
                "DtRadio",
                &[("value", rust_string(value)), ("label", rust_string(label))],
                None,
            )
        };
        let custom = default_slot.get();
        let body = if custom.is_empty() {
            format!(
                "{}\n{}\n{}",
                option("apple", "Apple"),
                option("banana", "Banana"),
                option("other", "Other"),
            )
        } else {
            rust_string(&custom)
        };
        let chosen = selected.get();
        format!(
            "// selected: {}\n{}",
            if chosen.is_empty() {
                "none".to_string()
            } else {
                chosen
            },
            view_of(&element("DtRadioGroup", &props, Some(&body)))
        )
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"on_change writes the selected signal. Choosing the current value does not emit. Disabled does not emit. An empty legend expects an aria-label."</p>
            <Play
                stack=true
                preview=preview
                settings=settings
                slots=slots
                classes=class_panel(vec![
                    ClassField { label: "Legend class", hint: "Extra class on the legend.", value: legend_class, off: None },
                    ClassField { label: "Messages class", hint: "Extra class on the messages.", value: messages_class, off: None },
                ])
                code=code
            />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Without legend">
                    <FruitRadios name="fruits-aria" legend="" aria_label="Fruits" />
                </Variant>
                <Variant title="Validation">
                    <FruitRadios
                        name="fruits-positive"
                        legend="With Positive Message"
                        aria_label=""
                        messages=vec![DtValidationMessage::positive("Positive validation message")]
                    />
                    <FruitRadios
                        name="fruits-warning"
                        legend="With Warning Message"
                        aria_label=""
                        messages=vec![DtValidationMessage::warning("Warning validation message")]
                    />
                    <FruitRadios
                        name="fruits-critical"
                        legend="With Critical Message"
                        aria_label=""
                        messages=vec![DtValidationMessage::critical("Critical validation message")]
                    />
                </Variant>
            </div>
        </section>
    }
}

#[component]
fn FruitRadios(
    name: &'static str,
    legend: &'static str,
    aria_label: &'static str,
    #[prop(optional)] messages: Vec<DtValidationMessage>,
) -> impl IntoView {
    let selected = RwSignal::new(String::new());
    view! {
        <DtRadioGroup
            name=name
            legend=legend
            aria_label=aria_label
            selected=selected
            on_change=move |next| selected.set(next)
            messages=messages
        >
            <DtRadio value="apple" label="Apple" />
            <DtRadio value="banana" label="Banana" />
            <DtRadio value="other" label="Other" />
        </DtRadioGroup>
    }
}
