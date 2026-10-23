use super::*;

#[component]
pub(crate) fn KeyboardShortcutPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("keyboard_shortcut.rs")));
    let shortcut = RwSignal::new("{cmd}+Ctrl+X".to_string());
    let screen_reader_text = RwSignal::new(String::new());
    let inverted = RwSignal::new(false);
    let root_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let shortcut = shortcut.get();
                let screen_reader_text = screen_reader_text.get();
                let inverted = inverted.get();
                let class_name = root_class.get();
                view! {
                    <DtKeyboardShortcut
                        shortcut=shortcut
                        screen_reader_text=screen_reader_text
                        inverted=inverted
                        class=class_name
                    />
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Shortcut" value=shortcut />
            <SettingText
                label="Screen reader text"
                value=screen_reader_text
                hint="Empty uses the generated label."
            />
            <SettingCheck label="Inverted (Deprecated)" value=inverted />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        props.push(("shortcut", rust_string(&shortcut.get())));
        push_str(&mut props, "screen_reader_text", &screen_reader_text.get());
        push_bool(&mut props, "inverted", inverted.get());
        push_str(&mut props, "class", &root_class.get());
        view_of(&element("DtKeyboardShortcut", &props, None))
    });
    let classes = class_panel(vec![ClassField {
        label: "Class",
        hint: ROOT_CLASS,
        value: root_class,
        off: None,
    }]);
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">
                "A plus sign is an icon. "
                <code>"{cmd}"</code>
                ", "
                <code>"{opt}"</code>
                ", "
                <code>"{win}"</code>
                ", and the arrow aliases are icons too. An empty screen reader text is generated from the shortcut. Inverted is deprecated and still changes the border and icon colors."
            </p>
            <Play preview=preview settings=settings classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Default shortcut">
                    <DtKeyboardShortcut shortcut="Option+{cmd}+{win}+Shift+{arrow-up}" />
                </Variant>
                <Variant title="Inverted">
                    <div class="d-bgc-contrast d-p-200">
                        <DtKeyboardShortcut
                            inverted=true
                            shortcut="{win}+{arrow-right}+{arrow-left}+{arrow-up}+{arrow-down}+E+B"
                        />
                    </div>
                </Variant>
                <Variant title="Shortcut size variation">
                    <DtKeyboardShortcut class="d-fs-200" shortcut="{cmd}+Y" />
                </Variant>
                <Variant title="All available shortcut aliases">
                    <DtKeyboardShortcut shortcut="{cmd}+{opt}+{win}+{arrow-right}+{arrow-left}+{arrow-up}+{arrow-down}+A" />
                </Variant>
                <Variant title="Inline with text and screen reader text">
                    <DtText kind="body" size="300">
                        "Press "
                        <DtKeyboardShortcut
                            screen_reader_text="Control plus F5"
                            shortcut="Ctrl + F5"
                        />
                        " to hard refresh the page."
                    </DtText>
                </Variant>
            </div>
        </section>
    }
}
