use super::*;

fn crumb_expr(label: &str, selected: bool) -> String {
    let mut expr = format!("DtBreadcrumb::new(\"#\", {})", rust_string(label));
    if selected {
        expr.push_str(".selected()");
    }
    expr
}

#[component]
pub(crate) fn BreadcrumbsPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("breadcrumbs.rs")));
    let root_label = RwSignal::new("Root".to_string());
    let section_label = RwSignal::new("Section".to_string());
    let current_label = RwSignal::new("Current Page".to_string());
    let aria_label = RwSignal::new(String::new());
    let inverted = RwSignal::new(false);
    let slot_text = RwSignal::new(String::new());
    let selected = RwSignal::new(true);
    let list_class = RwSignal::new(String::new());
    let root_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let mut current = DtBreadcrumb::new("#", current_label.get());
                current.selected = selected.get();
                let crumbs = vec![
                    DtBreadcrumb::new("#", root_label.get()),
                    DtBreadcrumb::new("#", section_label.get()),
                    current,
                ];
                let label = aria_label.get();
                let list_class = list_class.get();
                let class_name = root_class.get();
                let inverted = inverted.get();
                let slot_text = slot_text.get();
                let trail = if slot_text.is_empty() {
                    view! {
                        <DtBreadcrumbs
                            breadcrumbs=crumbs
                            aria_label=label
                            list_class=list_class
                            class=class_name
                            inverted=inverted
                        />
                    }
                    .into_any()
                } else {
                    view! {
                        <DtBreadcrumbs
                            breadcrumbs=crumbs
                            aria_label=label
                            list_class=list_class
                            class=class_name
                            inverted=inverted
                        >
                            {slot_text}
                        </DtBreadcrumbs>
                    }
                    .into_any()
                };
                if inverted {
                    view! { <div class="d-bgc-contrast d-p-200 d-bar-400">{trail}</div> }.into_any()
                } else {
                    trail.into_any()
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Root" value=root_label />
            <SettingText label="Section" value=section_label />
            <SettingText label="Current" value=current_label />
            <SettingText label="Aria label" value=aria_label hint="Empty uses breadcrumbs." />
            <SettingCheck label="Selected" value=selected />
            <SettingCheck label="Inverted (Deprecated)" value=inverted />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_expr(
            &mut props,
            "breadcrumbs",
            format!(
                "vec![{}, {}, {}]",
                crumb_expr(&root_label.get(), false),
                crumb_expr(&section_label.get(), false),
                crumb_expr(&current_label.get(), selected.get()),
            ),
        );
        push_str(&mut props, "aria_label", &aria_label.get());
        push_str(&mut props, "list_class", &list_class.get());
        push_str(&mut props, "class", &root_class.get());
        push_bool(&mut props, "inverted", inverted.get());
        let children = if slot_text.get().is_empty() {
            None
        } else {
            Some(rust_string(&slot_text.get()))
        };
        view_of(&element("DtBreadcrumbs", &props, children.as_deref()))
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingText label="Default" hint="Replaces the breadcrumbs list." value=slot_text />
        }
    });
    let classes = class_panel(vec![
        ClassField {
            label: "Class",
            hint: ROOT_CLASS,
            value: root_class,
            off: None,
        },
        ClassField {
            label: "List class",
            hint: "Extra class on the list (listClass).",
            value: list_class,
            off: None,
        },
    ]);
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">
                "The default slot replaces the breadcrumbs list and does not inherit inverted. Inverted is deprecated and still changes the separator and link color. The current page stays a link and sets aria-current to location. An empty aria label is breadcrumbs. An empty href is javascript:void(0). v-dt-mode is not ported."
            </p>
            <Play preview=preview settings=settings slots=slots classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Default">
                    <DtBreadcrumbs breadcrumbs=vec![
                        DtBreadcrumb::new("#", "Root"),
                        DtBreadcrumb::new("#", "Section"),
                        DtBreadcrumb::new("#", "Section"),
                        DtBreadcrumb::new("#", "Section"),
                        DtBreadcrumb::new("#", "Current Page").selected(),
                    ] />
                </Variant>
                <Variant title="Inverted">
                    <div class="d-bgc-contrast d-p-200 d-bar-400">
                        <DtBreadcrumbs
                            inverted=true
                            breadcrumbs=vec![
                                DtBreadcrumb::new("#", "Root"),
                                DtBreadcrumb::new("#", "Section"),
                                DtBreadcrumb::new("#", "Current Page").selected(),
                            ]
                        />
                    </div>
                </Variant>
                <Variant title="Slotted">
                    <DtBreadcrumbs>
                        <DtBreadcrumbItem href="#" label="Root" />
                        <DtBreadcrumbItem href="#">"Custom section"</DtBreadcrumbItem>
                        <DtBreadcrumbItem href="#" label="Current Page" selected=true />
                    </DtBreadcrumbs>
                </Variant>
            </div>
        </section>
    }
}
