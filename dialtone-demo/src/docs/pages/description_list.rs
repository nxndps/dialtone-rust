use super::*;

#[component]
pub(crate) fn DescriptionListPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("description_list.rs")));
    let direction = RwSignal::new("row");
    let gap = RwSignal::new("100");
    let term = RwSignal::new("Owner".to_string());
    let description = RwSignal::new("Ada Lovelace".to_string());
    let term_class = RwSignal::new(String::new());
    let description_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                view! {
                    <DtDescriptionList
                        direction=direction.get()
                        gap=gap.get()
                        term_class=term_class.get()
                        description_class=description_class.get()
                        items=vec![
                            DtDescriptionListItem::new("Service", "Voice"),
                            DtDescriptionListItem::new(term.get(), description.get()),
                        ]
                    />
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingSelect label="Direction" value=direction options=vec![("row", "Row"), ("column", "Column")] />
            <SettingSelect
                label="Gap"
                value=gap
                options=vec![
                    ("0", "0"),
                    ("100", "100"),
                    ("200", "200"),
                    ("400", "400"),
                    ("600", "600"),
                ]
            />
            <SettingText label="Term" value=term />
            <SettingText label="Description" value=description />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "direction", direction.get());
        push_str(&mut props, "gap", gap.get());
        push_str(&mut props, "term_class", &term_class.get());
        push_str(&mut props, "description_class", &description_class.get());
        push_expr(
            &mut props,
            "items",
            format!(
                "vec![DtDescriptionListItem::new(\"Service\", \"Voice\"), DtDescriptionListItem::new({}, {})]",
                rust_string(&term.get()),
                rust_string(&description.get()),
            ),
        );
        view_of(&element("DtDescriptionList", &props, None))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <Play
                stack=true
                preview=preview
                settings=settings
                classes=class_panel(vec![
                    ClassField {
                        label: "Term class",
                        hint: "Extra class on each term.",
                        value: term_class,
                        off: None,
                    },
                    ClassField {
                        label: "Description class",
                        hint: "Extra class on each description.",
                        value: description_class,
                        off: None,
                    },
                ])
                code=code
            />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Default">
                    <div class="d-w-500">
                        <DtDescriptionList
                            direction="row"
                            gap="100"
                            items=vec![
                                DtDescriptionListItem::new("Local time", "10:36 AM (PST)"),
                                DtDescriptionListItem::new("Dialpad", "(985) 241-8617"),
                                DtDescriptionListItem::new("work", "+1 604-900-7909"),
                                DtDescriptionListItem::new("email", "jackeline.na@dialpad.com"),
                            ]
                        />
                    </div>
                </Variant>
                <Variant title="Column direction">
                    <div class="d-w-500">
                        <DtDescriptionList
                            direction="column"
                            gap="100"
                            items=vec![
                                DtDescriptionListItem::new("Local time", "10:36 AM (PST)"),
                                DtDescriptionListItem::new("Dialpad", "(985) 241-8617"),
                                DtDescriptionListItem::new("work", "+1 604-900-7909"),
                                DtDescriptionListItem::new("email", "jackeline.na@dialpad.com"),
                            ]
                        />
                    </div>
                </Variant>
                <Variant title="Long text">
                    <div class="d-w-500">
                        <DtDescriptionList
                            direction="row"
                            gap="100"
                            items=vec![
                                DtDescriptionListItem::new(
                                    "Customer Intent",
                                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.",
                                ),
                                DtDescriptionListItem::new(
                                    "Three word term",
                                    "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
                                ),
                            ]
                        />
                    </div>
                </Variant>
                <Variant title="Term and description styles">
                    <div class="d-w-500">
                        <DtDescriptionList
                            direction="row"
                            gap="100"
                            term_class="d-fc-critical d-fw-bold"
                            description_class="d-fc-positive"
                            items=vec![
                                DtDescriptionListItem::new("Local time", "10:36 AM (PST)"),
                                DtDescriptionListItem::new("Dialpad", "(985) 241-8617"),
                                DtDescriptionListItem::new("work", "+1 604-900-7909"),
                                DtDescriptionListItem::new("email", "jackeline.na@dialpad.com"),
                            ]
                        />
                    </div>
                </Variant>
            </div>
        </section>
    }
}
