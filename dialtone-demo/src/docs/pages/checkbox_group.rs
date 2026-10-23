use super::*;

fn qa_or(value: &str, fallback: &'static str) -> &'static str {
    if value.is_empty() {
        fallback
    } else {
        Box::leak(value.to_string().into_boxed_str())
    }
}

#[component]
pub(crate) fn CheckboxGroupPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("checkbox_group.rs")));
    let selected = RwSignal::new(vec!["email".to_string()]);
    let name = RwSignal::new("channels".to_string());
    let legend = RwSignal::new("Channels".to_string());
    let legend_slot = RwSignal::new(String::new());
    let slot_text = RwSignal::new(String::new());
    let disabled = RwSignal::new(false);
    let show_messages = RwSignal::new(true);
    let show_error = RwSignal::new(false);
    let data_qa_group = RwSignal::new(String::new());
    let data_qa_group_legend = RwSignal::new(String::new());
    let data_qa_group_messages = RwSignal::new(String::new());
    let legend_class = RwSignal::new(String::new());
    let messages_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let messages = if show_error.get() {
                    vec![DtValidationMessage::error("Choose at least one channel")]
                } else {
                    vec![]
                };
                let legend_view = text_slot(legend_slot.get());
                let slot_text = slot_text.get();
                let name = name.get();
                let data_qa_group = qa_or(&data_qa_group.get(), "checkbox-group");
                let data_qa_group_legend = qa_or(&data_qa_group_legend.get(), "checkbox-group-legend");
                let data_qa_group_messages =
                    qa_or(&data_qa_group_messages.get(), "checkbox-group-messages");
                view! {
                    <div class="docs-stack">
                        {open_one!(DtCheckboxGroup {
                            name=name
                            legend=legend.get()
                            selected_values=selected
                            on_change=move |next| selected.set(next)
                            disabled=disabled.get()
                            show_messages=show_messages.get()
                            messages=messages
                            legend_class=legend_class.get()
                            messages_class=messages_class.get()
                            data_qa_group=data_qa_group
                            data_qa_group_legend=data_qa_group_legend
                            data_qa_group_messages=data_qa_group_messages
                        } {
                            <DtCheckbox value="email" label="Email" />
                            <DtCheckbox value="sms" label="SMS" />
                            <DtCheckbox value="push" label="Push" />
                            {(!slot_text.is_empty()).then(|| {
                                let slot_text = slot_text.clone();
                                view! { <span>{slot_text}</span> }
                            })}
                        }, legend_view, legend_slot)}
                        <p class="docs-readout">
                            {move || {
                                let values = selected.get();
                                if values.is_empty() {
                                    "None selected".to_string()
                                } else {
                                    format!("Selected: {}", values.join(", "))
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
            <SettingText label="Data qa group" value=data_qa_group />
            <SettingText label="Data qa group legend" value=data_qa_group_legend />
            <SettingText label="Data qa group messages" value=data_qa_group_messages />
            <SettingCheck label="Show messages" value=show_messages />
            <SettingCheck label="Disabled" value=disabled />
            <SettingCheck label="Error message" value=show_error />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "name", &name.get());
        push_str(&mut props, "legend", &legend.get());
        push_str(&mut props, "data_qa_group", &data_qa_group.get());
        push_str(
            &mut props,
            "data_qa_group_legend",
            &data_qa_group_legend.get(),
        );
        push_str(
            &mut props,
            "data_qa_group_messages",
            &data_qa_group_messages.get(),
        );
        push_str(&mut props, "legend_class", &legend_class.get());
        push_str(&mut props, "messages_class", &messages_class.get());
        push_expr(&mut props, "selected_values", "selected");
        push_expr(&mut props, "on_change", "move |next| selected.set(next)");
        push_bool(&mut props, "disabled", disabled.get());
        if !show_messages.get() {
            props.push(("show_messages", "false".to_string()));
        }
        if show_error.get() {
            push_expr(
                &mut props,
                "messages",
                "vec![DtValidationMessage::error(\"Choose at least one channel\")]",
            );
        }
        let option = |value: &str, label: &str| {
            element(
                "DtCheckbox",
                &[("value", rust_string(value)), ("label", rust_string(label))],
                None,
            )
        };
        push_span_slot(&mut props, "legend_slot", &legend_slot.get());
        let chosen = selected.get().join(", ");
        let mut body = format!(
            "{}\n{}\n{}",
            option("email", "Email"),
            option("sms", "SMS"),
            option("push", "Push"),
        );
        if !slot_text.get().is_empty() {
            body.push('\n');
            body.push_str(&element("span", &[], Some(&rust_string(&slot_text.get()))));
        }
        format!(
            "// selected: {}\n{}",
            if chosen.is_empty() {
                "none".to_string()
            } else {
                chosen
            },
            view_of(&element("DtCheckboxGroup", &props, Some(&body)))
        )
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <Play
                stack=true
                preview=preview
                settings=settings
                slots=ViewFn::from(move || {
                    view! {
                        <SettingText label="Legend" hint="Replaces the legend text." value=legend_slot />
                        <SettingText label="Default" hint="Added after the checkboxes. Empty leaves them alone." value=slot_text />
                    }
                })
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
                <Variant title="Disabled">
                    <DisabledChannels />
                </Variant>
            </div>
        </section>
    }
}

#[component]
fn DisabledChannels() -> impl IntoView {
    let selected = RwSignal::new(vec!["sms".to_string()]);
    view! {
        <DtCheckboxGroup
            name="channels-disabled"
            legend="Channels"
            selected_values=selected
            disabled=true
            on_change=move |next| selected.set(next)
        >
            <DtCheckbox value="email" label="Email" />
            <DtCheckbox value="sms" label="SMS" />
        </DtCheckboxGroup>
    }
}
