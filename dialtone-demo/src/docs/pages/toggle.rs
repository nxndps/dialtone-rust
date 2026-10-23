use super::*;

#[component]
pub(crate) fn TogglePage() -> impl IntoView {
    provide_context(VariantSource(include_str!("toggle.rs")));
    let on = RwSignal::new(true);
    let label = RwSignal::new("Sync enabled".to_string());
    let size = RwSignal::new("md");
    let disabled = RwSignal::new(false);
    let id = RwSignal::new(String::new());
    let toggle_on_click = RwSignal::new(true);
    let show_label = RwSignal::new(true);
    let show_icon = RwSignal::new(true);
    let class_name = RwSignal::new(String::new());
    let label_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let size = size.get();
                let disabled = disabled.get();
                let label = label.get();
                let id = id.get();
                let toggle_on_click = toggle_on_click.get();
                let show_label = show_label.get();
                let show_icon = show_icon.get();
                let class_name = class_name.get();
                let label_class = label_class.get();
                view! {
                    <DtToggle
                        checked=on
                        size=size
                        disabled=disabled
                        id=id
                        toggle_on_click=toggle_on_click
                        show_label=show_label
                        show_icon=show_icon
                        class=class_name
                        label_class=label_class
                        on_change=move |next| on.set(next)
                    >
                        {label}
                    </DtToggle>
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Label" value=label />
            <SettingText label="Id" value=id />
            <SettingSelect label="Size" value=size options=vec![("sm", "sm"), ("md", "md")] />
            <SettingCheck label="Checked" value=on />
            <SettingCheck label="Disabled" value=disabled />
            <SettingCheck label="Toggle on click" value=toggle_on_click />
            <SettingCheck label="Show label" value=show_label />
            <SettingCheck label="Show icon" value=show_icon />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_flag(&mut props, "checked", on.get());
        push_str(&mut props, "size", size.get());
        push_bool(&mut props, "disabled", disabled.get());
        push_str(&mut props, "id", &id.get());
        if !toggle_on_click.get() {
            push_expr(&mut props, "toggle_on_click", "false");
        }
        if !show_label.get() {
            push_expr(&mut props, "show_label", "false");
        }
        if !show_icon.get() {
            push_expr(&mut props, "show_icon", "false");
        }
        push_str(&mut props, "class", &class_name.get());
        push_str(&mut props, "label_class", &label_class.get());
        push_expr(&mut props, "on_change", "move |next| checked.set(next)");
        view_of(&element(
            "DtToggle",
            &props,
            Some(&rust_string(&label.get())),
        ))
    });
    let slots = ViewFn::from(move || {
        view! { <SettingText label="Default" hint="Toggle label." value=label /> }
    });
    let classes = class_panel(vec![
        ClassField {
            label: "Class",
            hint: "Extra class on the toggle wrapper.",
            value: class_name,
            off: None,
        },
        ClassField {
            label: "Label class",
            hint: "Extra class on the label.",
            value: label_class,
            off: None,
        },
    ]);
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <Play preview=preview settings=settings slots=slots classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Small" row=true>
                    <SmallSwitch />
                </Variant>
            </div>
        </section>
    }
}

#[component]
fn SmallSwitch() -> impl IntoView {
    let on = RwSignal::new(false);
    view! {
        <DtToggle checked=on size="sm" on_change=move |next| on.set(next)>"Compact"</DtToggle>
    }
}
