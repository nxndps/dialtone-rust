use super::*;

const TAGS: &[(&str, &str)] = &[
    ("", "Default"),
    ("div", "div"),
    ("section", "section"),
    ("span", "span"),
    ("nav", "nav"),
    ("ul", "ul"),
    ("ol", "ol"),
    ("li", "li"),
    ("article", "article"),
    ("header", "header"),
    ("footer", "footer"),
    ("main", "main"),
    ("aside", "aside"),
    ("p", "p"),
    ("form", "form"),
];

const ALIGNS: &[(&str, &str)] = &[
    ("start", "Start"),
    ("center", "Center"),
    ("end", "End"),
    ("stretch", "Stretch"),
    ("baseline", "Baseline"),
];

const JUSTIFY: &[(&str, &str)] = &[
    ("", "Default"),
    ("start", "Start"),
    ("center", "Center"),
    ("end", "End"),
    ("space-around", "Space around"),
    ("space-between", "Space between"),
    ("space-evenly", "Space evenly"),
    ("around", "Around"),
    ("between", "Between"),
    ("evenly", "Evenly"),
];

fn sample_children() -> String {
    [
        element(
            "DtBadge",
            &[
                ("text", rust_string("Voice")),
                ("type_", rust_string("info")),
            ],
            None,
        ),
        element(
            "DtBadge",
            &[
                ("text", rust_string("On call")),
                ("type_", rust_string("positive")),
            ],
            None,
        ),
        element(
            "DtButton",
            &[
                ("size", rust_string("sm")),
                ("importance", rust_string("outlined")),
            ],
            Some(&rust_string("Edit")),
        ),
    ]
    .join("\n")
}

#[component]
pub(crate) fn StackPage() -> impl IntoView {
    let tag = RwSignal::new("");
    let direction = RwSignal::new("row");
    let gap = RwSignal::new("400");
    let align = RwSignal::new("center");
    let justify = RwSignal::new("");
    let class_name = RwSignal::new(String::new());
    let default_slot = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let tag = if tag.get().is_empty() { "div" } else { tag.get() };
                let direction = direction.get();
                let gap = gap.get();
                let align = align.get();
                let justify = if justify.get().is_empty() {
                    "start"
                } else {
                    justify.get()
                };
                let class_name = class_name.get();
                let custom = default_slot.get();
                if custom.is_empty() {
                    view! {
                        <DtStack
                            r#as=tag
                            direction=direction
                            gap=gap
                            align=align
                            justify=justify
                            class=class_name.clone()
                        >
                            <DtBadge text="Voice" type_="info" />
                            <DtBadge text="On call" type_="positive" />
                            <DtButton size="sm" importance="outlined">"Edit"</DtButton>
                        </DtStack>
                    }
                    .into_any()
                } else {
                    view! {
                        <DtStack
                            r#as=tag
                            direction=direction
                            gap=gap
                            align=align
                            justify=justify
                            class=class_name
                        >
                            {custom}
                        </DtStack>
                    }
                    .into_any()
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingSelect label="As" value=tag options=TAGS.to_vec() />
            <SettingSelect
                label="Direction"
                value=direction
                options=vec![("row", "Row"), ("column", "Column")]
            />
            <SettingSelect
                label="Gap"
                value=gap
                options=vec![("200", "200"), ("400", "400"), ("600", "600")]
            />
            <SettingSelect label="Align" value=align options=ALIGNS.to_vec() />
            <SettingSelect label="Justify" value=justify options=JUSTIFY.to_vec() />
        }
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingText
                label="Default"
                hint="Default slot. Empty keeps the sample items."
                value=default_slot
            />
        }
    });
    let classes = class_panel(vec![ClassField {
        label: "Class",
        hint: ROOT_CLASS,
        value: class_name,
        off: None,
    }]);
    let code = example_panel(move || {
        let mut props = Vec::new();
        if !tag.get().is_empty() {
            push_str(&mut props, "r#as", tag.get());
        }
        push_str(&mut props, "direction", direction.get());
        push_str(&mut props, "gap", gap.get());
        push_str(&mut props, "align", align.get());
        if !justify.get().is_empty() {
            push_str(&mut props, "justify", justify.get());
        }
        push_str(&mut props, "class", &class_name.get());
        let body = if default_slot.get().is_empty() {
            sample_children()
        } else {
            rust_string(&default_slot.get())
        };
        view_of(&element("DtStack", &props, Some(&body)))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <Play preview=preview settings=settings slots=slots classes=classes code=code />
        </section>
    }
}
