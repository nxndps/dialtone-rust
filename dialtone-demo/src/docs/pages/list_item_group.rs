use super::*;

const HEADING_CLASS: &str =
    "Extra classes on the heading. The heading is unstyled until you add them.";
const LIST_CLASS: &str = "Extra classes on the list.";

fn group_items() -> impl IntoView {
    view! {
        <DtListItem role="option" navigation_type="tab" class="d-pis-100">"Item 1"</DtListItem>
        <DtListItem role="option" navigation_type="tab" class="d-pis-100">"Item 2"</DtListItem>
        <DtListItem role="option" navigation_type="tab" class="d-pis-100">"Item 3"</DtListItem>
    }
}

const GROUP_ITEMS: &str = r#"<DtListItem role="option" navigation_type="tab" class="d-pis-100">
    "Item 1"
</DtListItem>
<DtListItem role="option" navigation_type="tab" class="d-pis-100">
    "Item 2"
</DtListItem>
<DtListItem role="option" navigation_type="tab" class="d-pis-100">
    "Item 3"
</DtListItem>"#;

#[component]
pub(crate) fn ListItemGroupPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("list_item_group.rs")));
    let id = RwSignal::new(String::new());
    let heading = RwSignal::new("Example Heading".to_string());
    let heading_slot = RwSignal::new(String::new());
    let default_slot = RwSignal::new(String::new());
    let heading_class = RwSignal::new(String::new());
    let list_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let id_value = id.get();
                let heading_value = heading.get();
                let heading_class_value = heading_class.get();
                let list_class_value = list_class.get();
                let custom = default_slot.get();
                let heading_text = heading_slot.get();
                view! {
                    <div role="listbox">
                        {if custom.is_empty() {
                            open_one!(DtListItemGroup {
                                id=id_value.clone()
                                heading=heading_value.clone()
                                heading_class=heading_class_value.clone()
                                list_class=list_class_value.clone()
                            } {
                                {group_items()}
                            }, text_slot(heading_text.clone()), heading_slot)
                        } else {
                            open_one!(DtListItemGroup {
                                id=id_value
                                heading=heading_value
                                heading_class=heading_class_value
                                list_class=list_class_value
                            } {
                                {custom}
                            }, text_slot(heading_text), heading_slot)
                        }}
                    </div>
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Id" hint="Empty generates one." value=id />
            <SettingText label="Heading" hint="Empty hides the heading row." value=heading />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "id", &id.get());
        push_str(&mut props, "heading", &heading.get());
        push_str(&mut props, "heading_class", &heading_class.get());
        push_str(&mut props, "list_class", &list_class.get());
        push_span_slot(&mut props, "heading_slot", &heading_slot.get());
        let custom = default_slot.get();
        let body = if custom.is_empty() {
            GROUP_ITEMS.to_string()
        } else {
            rust_string(&custom)
        };
        view_of(&element("DtListItemGroup", &props, Some(&body)))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <Play
                stack=true
                preview=preview
                settings=settings
                slots=ViewFn::from(move || view! {
                    <SettingText
                        label="Heading"
                        hint="Heading slot. Replaces the heading text when Heading is set."
                        value=heading_slot
                    />
                    <SettingText
                        label="Default"
                        hint="Default slot. Empty keeps the three items."
                        value=default_slot
                    />
                })
                classes=class_panel(vec![
                    ClassField {
                        label: "Heading class",
                        hint: HEADING_CLASS,
                        value: heading_class,
                        off: None,
                    },
                    ClassField {
                        label: "List class",
                        hint: LIST_CLASS,
                        value: list_class,
                        off: None,
                    },
                ])
                code=code
            />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Styled heading">
                    <div role="listbox">
                        <DtListItemGroup
                            heading="Example Heading"
                            heading_class="d-py-50 d-px-100 d-label--sm d-fc-tertiary"
                        >
                            {group_items()}
                        </DtListItemGroup>
                    </div>
                </Variant>
            </div>
        </section>
    }
}
