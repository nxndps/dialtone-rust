use super::*;

#[component]
pub(crate) fn BadgePage() -> impl IntoView {
    provide_context(VariantSource(include_str!("badge.rs")));
    let text = RwSignal::new("Info".to_string());
    let slot_text = RwSignal::new(String::new());
    let icon_size = RwSignal::new("");
    let kind = RwSignal::new("label");
    let type_ = RwSignal::new("info");
    let decoration = RwSignal::new("");
    let subtle = RwSignal::new(false);
    let outlined = RwSignal::new(false);
    let left_icon = RwSignal::new("");
    let right_icon = RwSignal::new("");
    let label_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let text = text.get();
                let kind = kind.get();
                let type_ = type_.get();
                let decoration = decoration.get().to_string();
                let subtle = subtle.get();
                let outlined = outlined.get();
                let label_class_value = label_class.get();
                let icon_size = if icon_size.get().is_empty() {
                    "200"
                } else {
                    icon_size.get()
                };
                let slot_text = slot_text.get();
                let left = icon_slot(left_icon.get());
                let right = icon_slot(right_icon.get());
                if slot_text.is_empty() {
                    void_two!(DtBadge {
                        icon_size=icon_size
                        text=text
                        kind=kind
                        type_=type_
                        decoration=decoration
                        subtle=subtle
                        outlined=outlined
                        label_class=label_class_value
                    }, left, left_icon, right, right_icon)
                } else {
                    open_two!(DtBadge {
                        icon_size=icon_size
                        text=text
                        kind=kind
                        type_=type_
                        decoration=decoration
                        subtle=subtle
                        outlined=outlined
                        label_class=label_class_value
                    } {
                        {slot_text}
                    }, left, left_icon, right, right_icon)
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Text" value=text />
            <SettingSelect
                label="Icon size"
                value=icon_size
                options={
                    let mut options = vec![("", "Default")];
                    options.extend(ICON_SIZE.iter().copied());
                    options
                }
            />
            <SettingSelect label="Kind" value=kind options=vec![("label", "Label"), ("count", "Count")] />
            <SettingSelect
                label="Type"
                value=type_
                options=vec![
                    ("default", "Default"),
                    ("info", "Info"),
                    ("positive", "Positive"),
                    ("warning", "Warning"),
                    ("critical", "Critical"),
                    ("bulletin", "Bulletin"),
                    ("ai", "AI"),
                ]
            />
            <SettingSelect
                label="Decoration"
                value=decoration
                options=vec![("", "None"), ("purple-400", "Purple"), ("green-400", "Green"), ("gold-400", "Gold")]
            />
            <SettingCheck label="Subtle" value=subtle />
            <SettingCheck label="Outlined" value=outlined />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        if !icon_size.get().is_empty() {
            push_str(&mut props, "icon_size", icon_size.get());
        }
        push_str(&mut props, "text", &text.get());
        push_str(&mut props, "kind", kind.get());
        push_str(&mut props, "type_", type_.get());
        push_str(&mut props, "decoration", decoration.get());
        push_bool(&mut props, "subtle", subtle.get());
        push_bool(&mut props, "outlined", outlined.get());
        push_str(&mut props, "label_class", &label_class.get());
        push_icon(&mut props, "left_icon", left_icon.get());
        push_icon(&mut props, "right_icon", right_icon.get());
        let body = if slot_text.get().is_empty() {
            None
        } else {
            Some(rust_string(&slot_text.get()))
        };
        view_of(&element("DtBadge", &props, body.as_deref()))
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingText label="Default" hint="Replaces the text." value=slot_text />
            <SettingSelect label="Left icon" value=left_icon options=ICONS.to_vec() />
            <SettingSelect label="Right icon" value=right_icon options=ICONS.to_vec() />
        }
    });
    let classes = class_panel(vec![ClassField {
        label: "Label class",
        hint: "Extra class on the label.",
        value: label_class,
        off: None,
    }]);
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <Play preview=preview settings=settings slots=slots classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Types" row=true>
                    <DtBadge text="Default" />
                    <DtBadge text="Info" type_="info" />
                    <DtBadge text="Positive" type_="positive" />
                    <DtBadge text="Warning" type_="warning" />
                    <DtBadge text="Critical" type_="critical" />
                    <DtBadge text="Bulletin" type_="bulletin" />
                    <DtBadge text="AI" type_="ai" />
                </Variant>
                <Variant title="Count" row=true>
                    <DtBadge text="3" kind="count" type_="info" />
                    <DtBadge text="12" kind="count" type_="critical" />
                </Variant>
            </div>
        </section>
    }
}
