use super::*;

#[component]
pub(crate) fn CardPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("card.rs")));
    let title = RwSignal::new("Schedule".to_string());
    let body = RwSignal::new("Primary on-call this week.".to_string());
    let show_header = RwSignal::new(true);
    let show_footer = RwSignal::new(true);
    let footer_text = RwSignal::new(String::new());
    let max_height = RwSignal::new(String::new());
    let container_extra = RwSignal::new(String::new());
    let content_class = RwSignal::new(String::new());
    let header_class = RwSignal::new(String::new());
    let footer_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let show_header = show_header.get();
                let show_footer = show_footer.get();
                let footer_text = footer_text.get();
                let max_height = max_height.get();
                let container = join_extra("w-full", &container_extra.get());
                let content = content_class.get();
                let header_class_value = header_class.get();
                let footer_class_value = footer_class.get();
                let header = ViewFn::from(move || view! { <span>{move || title.get()}</span> });
                let footer = if !footer_text.is_empty() {
                    let footer_text = footer_text.clone();
                    ViewFn::from(move || {
                        let footer_text = footer_text.clone();
                        view! { <span>{footer_text}</span> }
                    })
                } else {
                    ViewFn::from(|| {
                        view! { <DtButton importance="outlined" size="sm">"Edit"</DtButton> }
                    })
                };
                match (show_header, show_footer) {
                    (true, true) => view! {
                        <DtCard
                            max_height=max_height.clone()
                            container_class=container
                            content_class=content
                            header_class=header_class_value
                            footer_class=footer_class_value
                            header=header.clone()
                            footer=footer.clone()
                        >
                            <p class="m-0 text-sm text-fg-muted">{move || body.get()}</p>
                        </DtCard>
                    }.into_any(),
                    (true, false) => view! {
                        <DtCard
                            max_height=max_height.clone()
                            container_class=container
                            content_class=content
                            header_class=header_class_value
                            header=header.clone()
                        >
                            <p class="m-0 text-sm text-fg-muted">{move || body.get()}</p>
                        </DtCard>
                    }.into_any(),
                    (false, true) => view! {
                        <DtCard
                            max_height=max_height.clone()
                            container_class=container
                            content_class=content
                            footer_class=footer_class_value
                            footer=footer.clone()
                        >
                            <p class="m-0 text-sm text-fg-muted">{move || body.get()}</p>
                        </DtCard>
                    }.into_any(),
                    (false, false) => view! {
                        <DtCard max_height=max_height container_class=container content_class=content>
                            <p class="m-0 text-sm text-fg-muted">{move || body.get()}</p>
                        </DtCard>
                    }.into_any(),
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Header" value=title />
            <SettingText label="Body" value=body />
            <SettingText label="Max height" value=max_height />
            <SettingCheck label="Show header" value=show_header />
            <SettingCheck label="Show footer" value=show_footer />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "max_height", &max_height.get());
        push_str(
            &mut props,
            "container_class",
            &join_extra("w-full", &container_extra.get()),
        );
        push_str(&mut props, "content_class", &content_class.get());
        if show_header.get() {
            push_str(&mut props, "header_class", &header_class.get());
        }
        if show_footer.get() {
            push_str(&mut props, "footer_class", &footer_class.get());
        }
        if show_header.get() {
            push_expr(
                &mut props,
                "header",
                format!(
                    "ViewFn::from(|| view! {{ <span>{}</span> }})",
                    rust_string(&title.get())
                ),
            );
        }
        if show_footer.get() {
            if footer_text.get().is_empty() {
                push_expr(
                    &mut props,
                    "footer",
                    "ViewFn::from(|| view! { <DtButton importance=\"outlined\" size=\"sm\">\"Edit\"</DtButton> })",
                );
            } else {
                push_expr(
                    &mut props,
                    "footer",
                    format!(
                        "ViewFn::from(|| view! {{ <span>{}</span> }})",
                        rust_string(&footer_text.get())
                    ),
                );
            }
        }
        let body = element(
            "p",
            &[("class", rust_string("m-0 text-sm text-fg-muted"))],
            Some(&rust_string(&body.get())),
        );
        view_of(&element("DtCard", &props, Some(&body)))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <Play
                stack=true
                preview=preview
                settings=settings
                slots=ViewFn::from(move || {
                    view! {
                        <SettingText label="Header" hint="Header slot." value=title />
                        <SettingText label="Footer" hint="Footer slot. Empty keeps the Edit button." value=footer_text />
                    }
                })
                classes=class_panel(vec![
                    ClassField {
                        label: "Class",
                        hint: ROOT_CLASS,
                        value: container_extra,
                        off: None,
                    },
                    ClassField {
                        label: "Content class",
                        hint: "Extra class on the content.",
                        value: content_class,
                        off: None,
                    },
                    ClassField {
                        label: "Header class",
                        hint: "Extra class on the header.",
                        value: header_class,
                        off: None,
                    },
                    ClassField {
                        label: "Footer class",
                        hint: "Extra class on the footer.",
                        value: footer_class,
                        off: None,
                    },
                ])
                code=code
            />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Body only">
                    <DtCard container_class="w-full">
                        <p class="m-0 text-sm">"No header or footer."</p>
                    </DtCard>
                </Variant>
            </div>
        </section>
    }
}
