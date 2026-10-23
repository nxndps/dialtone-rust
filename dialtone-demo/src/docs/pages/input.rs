use super::*;

#[component]
pub(crate) fn InputPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("input.rs")));
    let value = RwSignal::new(String::new());
    let label = RwSignal::new("Name".to_string());
    let name = RwSignal::new(String::new());
    let placeholder = RwSignal::new("Ada Lovelace".to_string());
    let description = RwSignal::new(String::new());
    let input_type = RwSignal::new("text");
    let size = RwSignal::new("");
    let show_label = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let required = RwSignal::new(false);
    let input_wrapper_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let size = match size.get() {
                    "" => "300",
                    value => value,
                };
                view! {
                    <DtInput
                        label=label.get()
                        name=name.get()
                        show_label=show_label.get()
                        placeholder=placeholder.get()
                        description=description.get()
                        size=size
                        input_type=input_type.get()
                        value=value
                        on_input=move |next| value.set(next)
                        disabled=disabled.get()
                        required=required.get()
                        input_wrapper_class=input_wrapper_class.get()
                    />
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Label" value=label />
            <SettingText label="Name" value=name />
            <SettingText label="Value" value=value />
            <SettingText label="Placeholder" value=placeholder />
            <SettingText label="Description" value=description />
            <SettingSelect
                label="Type"
                value=input_type
                options=vec![("text", "Text"), ("email", "Email"), ("password", "Password"), ("search", "Search")]
            />
            <SettingSelect
                label="Size"
                value=size
                options=vec![
                    ("", "Default"),
                    ("100", "100"),
                    ("200", "200"),
                    ("300", "300"),
                    ("400", "400"),
                    ("500", "500"),
                ]
            />
            <SettingCheck label="Show label" value=show_label />
            <SettingCheck label="Required" value=required />
            <SettingCheck label="Disabled" value=disabled />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "label", &label.get());
        push_str(&mut props, "name", &name.get());
        push_str(&mut props, "placeholder", &placeholder.get());
        push_str(&mut props, "description", &description.get());
        let chosen_size = size.get();
        if !chosen_size.is_empty() && chosen_size != "300" {
            push_str(&mut props, "size", chosen_size);
        }
        push_str(&mut props, "input_type", input_type.get());
        push_str(&mut props, "value", &value.get());
        push_expr(&mut props, "on_input", "move |next| value.set(next)");
        if !show_label.get() {
            props.push(("show_label", "false".to_string()));
        }
        push_bool(&mut props, "disabled", disabled.get());
        push_bool(&mut props, "required", required.get());
        push_str(
            &mut props,
            "input_wrapper_class",
            &input_wrapper_class.get(),
        );
        view_of(&element("DtInput", &props, None))
    });
    let classes = class_panel(vec![ClassField {
        label: "Input wrapper class",
        hint: "Extra class on the input wrapper.",
        value: input_wrapper_class,
        off: None,
    }]);
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <Play stack=true preview=preview settings=settings classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Password">
                    <PasswordField />
                </Variant>
            </div>
        </section>
    }
}

#[component]
fn PasswordField() -> impl IntoView {
    let value = RwSignal::new(String::new());
    view! {
        <DtInput
            label="Password"
            input_type="password"
            value=value
            on_input=move |next| value.set(next)
        />
    }
}
