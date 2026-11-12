use super::*;

#[component]
pub(crate) fn RootLayoutPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("root_layout.rs")));
    let side = RwSignal::new("left");
    let breakpoint = RwSignal::new("");
    let sticky = RwSignal::new(false);
    let fixed = RwSignal::new(false);
    let header_text = RwSignal::new("Header".to_string());
    let sidebar_text = RwSignal::new("Sidebar".to_string());
    let content_text = RwSignal::new("Content".to_string());
    let footer_text = RwSignal::new("Footer".to_string());
    let header_class = RwSignal::new(String::new());
    let sidebar_class = RwSignal::new(String::new());
    let content_class = RwSignal::new(String::new());
    let footer_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let header_class_value = header_class.get();
                let sidebar_class_value = sidebar_class.get();
                let content_class_value = content_class.get();
                let footer_class_value = footer_class.get();
                let content_label = content_text.get();
                view! {
                    <div class="docs-layout-frame">
                        {open_one!(DtRootLayout {
                            fixed=fixed.get()
                            header_sticky=sticky.get()
                            sidebar_position=side.get()
                            responsive_breakpoint=breakpoint.get().to_string()
                            header_class=header_class_value
                            sidebar_class=sidebar_class_value
                            content_class=content_class_value
                            footer_class=footer_class_value
                            header=layout_bar(header_text.get())
                            sidebar=layout_bar(sidebar_text.get())
                        } {
                            <p class="m-0 p-3 text-sm">{content_label.clone()}</p>
                        }, layout_bar_slot(footer_text.get()), footer)}
                    </div>
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingSelect
                label="Sidebar"
                value=side
                options=vec![
                    ("start", "Start"),
                    ("end", "End"),
                    ("left", "Left"),
                    ("right", "Right"),
                ]
            />
            <SettingSelect
                label="Breakpoint"
                value=breakpoint
                options=vec![("", "Default"), ("sm", "sm"), ("md", "md"), ("lg", "lg")]
            />
            <SettingCheck label="Sticky header" value=sticky />
            <SettingCheck label="Fixed" value=fixed />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_bool(&mut props, "fixed", fixed.get());
        push_bool(&mut props, "header_sticky", sticky.get());
        push_str(&mut props, "sidebar_position", side.get());
        push_str(&mut props, "responsive_breakpoint", breakpoint.get());
        push_str(&mut props, "header_class", &header_class.get());
        push_str(&mut props, "sidebar_class", &sidebar_class.get());
        push_str(&mut props, "content_class", &content_class.get());
        push_str(&mut props, "footer_class", &footer_class.get());
        let bar = |text: &str| {
            format!(
                "ViewFn::from(|| view! {{ <div class=\"docs-layout-bar\">{}</div> }})",
                rust_string(text)
            )
        };
        push_expr(&mut props, "header", bar(&header_text.get()));
        push_expr(&mut props, "sidebar", bar(&sidebar_text.get()));
        if !footer_text.get().is_empty() {
            push_expr(&mut props, "footer", bar(&footer_text.get()));
        }
        let body = element(
            "p",
            &[("class", rust_string("m-0 p-3 text-sm"))],
            Some(&rust_string(&content_text.get())),
        );
        view_of(&element("DtRootLayout", &props, Some(&body)))
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingText label="Header" hint="Header slot." value=header_text />
            <SettingText label="Sidebar" hint="Sidebar slot." value=sidebar_text />
            <SettingText label="Default" hint="Main content." value=content_text />
            <SettingText label="Footer" hint="Footer slot. The footer element stays mounted." value=footer_text />
        }
    });
    let classes = class_panel(vec![
        ClassField {
            label: "Header class",
            hint: "Extra class on the header.",
            value: header_class,
            off: None,
        },
        ClassField {
            label: "Sidebar class",
            hint: "Extra class on the sidebar.",
            value: sidebar_class,
            off: None,
        },
        ClassField {
            label: "Content class",
            hint: "Extra class on the content.",
            value: content_class,
            off: None,
        },
        ClassField {
            label: "Footer class",
            hint: "Extra class on the footer.",
            value: footer_class,
            off: None,
        },
    ]);
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"The frame holds the layout. Fixed positioning stays inside that frame."</p>
            <Play stack=true preview=preview settings=settings slots=slots classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Sidebar on the right">
                    <div class="docs-layout-frame">
                        <DtRootLayout
                            sidebar_position="right"
                            header=ViewFn::from(|| view! { <div class="docs-layout-bar">"Header"</div> })
                            sidebar=ViewFn::from(|| view! { <div class="docs-layout-bar">"Sidebar"</div> })
                        >
                            <p class="m-0 p-3 text-sm">"Content"</p>
                        </DtRootLayout>
                    </div>
                </Variant>
            </div>
        </section>
    }
}
