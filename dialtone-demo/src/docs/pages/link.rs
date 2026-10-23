use super::*;

const TONES: &[(&str, &str)] = &[
    ("", "Default"),
    ("critical", "Critical"),
    ("warning", "Warning"),
    ("positive", "Positive"),
    ("info", "Info"),
    ("muted", "Muted"),
    ("mention", "Mention"),
    ("unstyled", "Unstyled"),
];

const KINDS: &[(&str, &str)] = &[
    ("", "Unset"),
    ("critical", "Critical"),
    ("warning", "Warning"),
    ("positive", "Positive"),
    ("info", "Info"),
    ("muted", "Muted"),
    ("mention", "Mention"),
    ("unstyled", "Unstyled"),
];

#[component]
pub(crate) fn LinkPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("link.rs")));
    let label = RwSignal::new("Default link".to_string());
    let href = RwSignal::new("#link".to_string());
    let tone = RwSignal::new("");
    let kind = RwSignal::new("");
    let inverted = RwSignal::new(false);
    let underline = RwSignal::new(true);
    let target = RwSignal::new(String::new());
    let rel = RwSignal::new(String::new());
    let root_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let label = label.get();
                let href = href.get();
                let tone = tone.get();
                let kind = kind.get();
                let inverted = inverted.get();
                let underline = underline.get();
                let target = target.get();
                let rel = rel.get();
                let class_name = root_class.get();
                view! {
                    <DtLink
                        href=href
                        tone=tone
                        kind=kind
                        inverted=inverted
                        underline=underline
                        target=target
                        rel=rel
                        class=class_name
                    >
                        {label}
                    </DtLink>
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Label" value=label />
            <SettingText label="Href" value=href />
            <SettingSelect label="Tone" value=tone options=TONES.to_vec() />
            <SettingSelect label="Kind (Deprecated)" value=kind options=KINDS.to_vec() />
            <SettingCheck label="Underline" value=underline />
            <SettingCheck label="Inverted (Deprecated)" value=inverted />
            <SettingText label="Target" value=target hint="Empty omits the attribute." />
            <SettingText label="Rel" value=rel />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "href", &href.get());
        push_str(&mut props, "tone", tone.get());
        push_str(&mut props, "kind", kind.get());
        push_bool(&mut props, "inverted", inverted.get());
        if !underline.get() {
            props.push(("underline", "false".to_string()));
        }
        push_str(&mut props, "target", &target.get());
        push_str(&mut props, "rel", &rel.get());
        push_str(&mut props, "class", &root_class.get());
        let child = rust_string(&label.get());
        view_of(&element("DtLink", &props, Some(&child)))
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
                "Kind is deprecated and wins over tone. Unstyled drops the link color and the underline. Inverted is deprecated and still changes the color. An empty href is javascript:void(0). Vue Router to is not ported."
            </p>
            <Play preview=preview settings=settings classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Tones">
                    <DtStack direction="row" gap="100" align="baseline" class="d-fw-wrap">
                        <DtLink href="#link">"Default link"</DtLink>
                        <DtLink href="#link" tone="critical">"Critical link"</DtLink>
                        <DtLink href="#link" tone="warning">"Warning link"</DtLink>
                        <DtLink href="#link" tone="positive">"Positive link"</DtLink>
                        <DtLink href="#link" tone="info">"Info link"</DtLink>
                        <DtLink href="#link" tone="muted">"Muted link"</DtLink>
                        <DtLink href="#link" tone="mention">"@Mention link"</DtLink>
                        <DtLink href="#link" tone="unstyled">"Unstyled link"</DtLink>
                    </DtStack>
                </Variant>
                <Variant title="No underline">
                    <DtLink href="#link" underline=false>"No underline link"</DtLink>
                </Variant>
                <Variant title="Inverted">
                    <div class="d-bgc-contrast d-p-200">
                        <DtStack direction="row" gap="100" align="baseline" class="d-fw-wrap">
                            <DtLink href="#link" inverted=true>"Base link"</DtLink>
                            <DtLink href="#link" tone="critical" inverted=true>"Critical link"</DtLink>
                            <DtLink href="#link" tone="positive" inverted=true>"Positive link"</DtLink>
                            <DtLink href="#link" tone="warning" inverted=true>"Warning link"</DtLink>
                            <DtLink href="#link" tone="info" inverted=true>"Info link"</DtLink>
                            <DtLink href="#link" tone="muted" inverted=true>"Muted link"</DtLink>
                            <DtLink href="#link" tone="mention" inverted=true>"@Mention link"</DtLink>
                        </DtStack>
                    </div>
                </Variant>
                <Variant title="External">
                    <DtLink
                        href="https://github.com/dialpad/dialtone"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        "GitHub"
                    </DtLink>
                </Variant>
            </div>
        </section>
    }
}
