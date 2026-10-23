use super::*;

const SIZES: &[(&str, &str)] = &[
    ("", "Default"),
    ("100", "100"),
    ("200", "200"),
    ("300", "300"),
    ("400", "400"),
    ("500", "500"),
];

const DEFAULT_OPTIONS: &str = "voice|Voice\nmeetings|Meetings\nsms|SMS";

fn select_options(text: &str) -> Vec<DtSelectOption> {
    text.lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            let (value, label) = line.split_once('|').unwrap_or((line, line));
            let value = value.trim();
            let label = label.trim();
            if label.is_empty() {
                None
            } else {
                Some(DtSelectOption::new(value, label))
            }
        })
        .collect()
}

fn select_messages(text: &str) -> Vec<DtValidationMessage> {
    text.lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                None
            } else {
                Some(DtValidationMessage::error(line))
            }
        })
        .collect()
}

fn options_expr(text: &str) -> String {
    let parts = select_options(text)
        .into_iter()
        .map(|option| {
            format!(
                "DtSelectOption::new({}, {})",
                rust_string(&option.value),
                rust_string(&option.label)
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("vec![{parts}]")
}

fn messages_expr(text: &str) -> String {
    let parts = select_messages(text)
        .into_iter()
        .map(|row| format!("DtValidationMessage::error({})", rust_string(&row.message)))
        .collect::<Vec<_>>()
        .join(", ");
    format!("vec![{parts}]")
}

fn option_nodes(text: String) -> impl IntoView {
    select_options(&text)
        .into_iter()
        .map(|option| {
            let value = option.value;
            let label = option.label;
            view! { <option value=value>{label}</option> }
        })
        .collect_view()
}

#[component]
fn SelectPreview(
    label: String,
    description: String,
    value: RwSignal<String>,
    options: Vec<DtSelectOption>,
    size: &'static str,
    disabled: bool,
    show_messages: bool,
    messages: Vec<DtValidationMessage>,
    label_class: String,
    description_class: String,
    select_class: String,
    option_class: String,
    messages_class: String,
    label_slot: Option<ViewFn>,
    description_slot: Option<ViewFn>,
    slot_text: String,
) -> impl IntoView {
    let has_children = !slot_text.trim().is_empty();
    match (label_slot, description_slot, has_children) {
        (Some(label_slot), Some(description_slot), true) => view! {
            <DtSelectMenu
                label=label
                description=description
                value=value
                on_change=move |next| value.set(next)
                options=options
                size=size
                disabled=disabled
                show_messages=show_messages
                messages=messages
                label_class=label_class
                description_class=description_class
                select_class=select_class
                option_class=option_class
                messages_class=messages_class
                label_slot=label_slot
                description_slot=description_slot
            >
                {option_nodes(slot_text)}
            </DtSelectMenu>
        }
        .into_any(),
        (Some(label_slot), Some(description_slot), false) => view! {
            <DtSelectMenu
                label=label
                description=description
                value=value
                on_change=move |next| value.set(next)
                options=options
                size=size
                disabled=disabled
                show_messages=show_messages
                messages=messages
                label_class=label_class
                description_class=description_class
                select_class=select_class
                option_class=option_class
                messages_class=messages_class
                label_slot=label_slot
                description_slot=description_slot
            />
        }
        .into_any(),
        (Some(label_slot), None, true) => view! {
            <DtSelectMenu
                label=label
                description=description
                value=value
                on_change=move |next| value.set(next)
                options=options
                size=size
                disabled=disabled
                show_messages=show_messages
                messages=messages
                label_class=label_class
                description_class=description_class
                select_class=select_class
                option_class=option_class
                messages_class=messages_class
                label_slot=label_slot
            >
                {option_nodes(slot_text)}
            </DtSelectMenu>
        }
        .into_any(),
        (Some(label_slot), None, false) => view! {
            <DtSelectMenu
                label=label
                description=description
                value=value
                on_change=move |next| value.set(next)
                options=options
                size=size
                disabled=disabled
                show_messages=show_messages
                messages=messages
                label_class=label_class
                description_class=description_class
                select_class=select_class
                option_class=option_class
                messages_class=messages_class
                label_slot=label_slot
            />
        }
        .into_any(),
        (None, Some(description_slot), true) => view! {
            <DtSelectMenu
                label=label
                description=description
                value=value
                on_change=move |next| value.set(next)
                options=options
                size=size
                disabled=disabled
                show_messages=show_messages
                messages=messages
                label_class=label_class
                description_class=description_class
                select_class=select_class
                option_class=option_class
                messages_class=messages_class
                description_slot=description_slot
            >
                {option_nodes(slot_text)}
            </DtSelectMenu>
        }
        .into_any(),
        (None, Some(description_slot), false) => view! {
            <DtSelectMenu
                label=label
                description=description
                value=value
                on_change=move |next| value.set(next)
                options=options
                size=size
                disabled=disabled
                show_messages=show_messages
                messages=messages
                label_class=label_class
                description_class=description_class
                select_class=select_class
                option_class=option_class
                messages_class=messages_class
                description_slot=description_slot
            />
        }
        .into_any(),
        (None, None, true) => view! {
            <DtSelectMenu
                label=label
                description=description
                value=value
                on_change=move |next| value.set(next)
                options=options
                size=size
                disabled=disabled
                show_messages=show_messages
                messages=messages
                label_class=label_class
                description_class=description_class
                select_class=select_class
                option_class=option_class
                messages_class=messages_class
            >
                {option_nodes(slot_text)}
            </DtSelectMenu>
        }
        .into_any(),
        (None, None, false) => view! {
            <DtSelectMenu
                label=label
                description=description
                value=value
                on_change=move |next| value.set(next)
                options=options
                size=size
                disabled=disabled
                show_messages=show_messages
                messages=messages
                label_class=label_class
                description_class=description_class
                select_class=select_class
                option_class=option_class
                messages_class=messages_class
            />
        }
        .into_any(),
    }
}

#[component]
pub(crate) fn SelectMenuPage() -> impl IntoView {
    let value = RwSignal::new("voice".to_string());
    let label = RwSignal::new("Product".to_string());
    let description = RwSignal::new(String::new());
    let options_text = RwSignal::new(DEFAULT_OPTIONS.to_string());
    let size = RwSignal::new("");
    let disabled = RwSignal::new(false);
    let show_messages = RwSignal::new(true);
    let messages_text = RwSignal::new(String::new());
    let label_slot = RwSignal::new(String::new());
    let description_slot = RwSignal::new(String::new());
    let default_slot = RwSignal::new(String::new());
    let label_class = RwSignal::new(String::new());
    let description_class = RwSignal::new(String::new());
    let select_class = RwSignal::new(String::new());
    let option_class = RwSignal::new(String::new());
    let messages_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let size = if size.get().is_empty() { "md" } else { size.get() };
                view! {
                    <SelectPreview
                        label=label.get()
                        description=description.get()
                        value=value
                        options=select_options(&options_text.get())
                        size=size
                        disabled=disabled.get()
                        show_messages=show_messages.get()
                        messages=select_messages(&messages_text.get())
                        label_class=label_class.get()
                        description_class=description_class.get()
                        select_class=select_class.get()
                        option_class=option_class.get()
                        messages_class=messages_class.get()
                        label_slot=text_slot(label_slot.get())
                        description_slot=text_slot(description_slot.get())
                        slot_text=default_slot.get()
                    />
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Label" value=label />
            <SettingText label="Value" value=value />
            <SettingText label="Description" value=description />
            <SettingArea
                label="Options"
                hint="One option per line, value|label."
                value=options_text
            />
            <SettingSelect label="Size" value=size options=SIZES.to_vec() />
            <SettingCheck label="Disabled" value=disabled />
            <SettingCheck label="Show messages" value=show_messages />
            <SettingArea label="Messages" hint="One message per line." value=messages_text />
        }
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingText label="Label" hint="Label slot. Replaces the label text." value=label_slot />
            <SettingText
                label="Description"
                hint="Description slot. Replaces the description."
                value=description_slot
            />
            <SettingArea
                label="Default"
                hint="Option lines, value|label. Empty uses the options prop."
                value=default_slot
            />
        }
    });
    let classes = class_panel(vec![
        ClassField {
            label: "Messages class",
            hint: "Extra class on the messages.",
            value: messages_class,
            off: None,
        },
        ClassField {
            label: "Label class",
            hint: "Extra class on the label.",
            value: label_class,
            off: None,
        },
        ClassField {
            label: "Description class",
            hint: "Extra class on the description.",
            value: description_class,
            off: None,
        },
        ClassField {
            label: "Select class",
            hint: "Extra class on the select.",
            value: select_class,
            off: None,
        },
        ClassField {
            label: "Option class",
            hint: "Extra class on each generated option.",
            value: option_class,
            off: None,
        },
    ]);
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "label", &label.get());
        push_expr(&mut props, "value", "value");
        push_expr(&mut props, "on_change", "move |next| value.set(next)");
        if default_slot.get().trim().is_empty() {
            push_expr(&mut props, "options", options_expr(&options_text.get()));
        }
        if !size.get().is_empty() {
            push_str(&mut props, "size", size.get());
        }
        push_str(&mut props, "description", &description.get());
        push_bool(&mut props, "disabled", disabled.get());
        if !show_messages.get() {
            push_expr(&mut props, "show_messages", "false");
        }
        if !messages_text.get().trim().is_empty() {
            push_expr(&mut props, "messages", messages_expr(&messages_text.get()));
        }
        push_str(&mut props, "label_class", &label_class.get());
        push_str(&mut props, "description_class", &description_class.get());
        push_str(&mut props, "select_class", &select_class.get());
        push_str(&mut props, "option_class", &option_class.get());
        push_str(&mut props, "messages_class", &messages_class.get());
        push_span_slot(&mut props, "label_slot", &label_slot.get());
        push_span_slot(&mut props, "description_slot", &description_slot.get());
        let child = if default_slot.get().trim().is_empty() {
            None
        } else {
            let rows = select_options(&default_slot.get())
                .into_iter()
                .map(|option| {
                    element(
                        "option",
                        &[("value", rust_string(&option.value))],
                        Some(&rust_string(&option.label)),
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");
            Some(rows)
        };
        view_of(&element("DtSelectMenu", &props, child.as_deref()))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <Play stack=true preview=preview settings=settings slots=slots classes=classes code=code />
        </section>
    }
}
