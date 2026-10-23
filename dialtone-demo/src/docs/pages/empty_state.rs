use super::*;

macro_rules! empty_preview {
    ($size:expr, $header:expr, $body:expr $(, $prop:ident = $value:expr)* $(,)?) => {
        view! {
            <DtEmptyState size=$size header_text=$header body_text=$body $($prop=$value)* />
        }
        .into_any()
    };
}

fn shown(value: &str, fallback: &str) -> String {
    if value.trim().is_empty() {
        fallback.to_string()
    } else {
        value.to_string()
    }
}

#[component]
pub(crate) fn EmptyStatePage() -> impl IntoView {
    provide_context(VariantSource(include_str!("empty_state.rs")));
    let header = RwSignal::new("No incidents".to_string());
    let body = RwSignal::new("Alerts land here when a service pages you.".to_string());
    let size = RwSignal::new("lg");
    let show_icon = RwSignal::new(true);
    let illustration = RwSignal::new("");
    let extra_body = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let size = size.get();
                let header_text = shown(&header.get(), "No incidents");
                let body_text = shown(&body.get(), "Alerts land here.");
                let illustration_name = illustration.get();
                let illustration_slot = if illustration_name.is_empty() {
                    None
                } else {
                    let illustration_name = illustration_name.to_string();
                    Some(ViewFn::from(move || {
                        let illustration_name = illustration_name.clone();
                        view! { <DtIllustration name=illustration_name /> }
                    }))
                };
                let icon = show_icon
                    .get()
                    .then(|| ViewFn::from(|| view! { <DtIconBell size="800" /> }));
                let body_slot = text_slot(extra_body.get());
                match (illustration_slot, icon, body_slot) {
                    (None, None, None) => empty_preview!(size, header_text, body_text),
                    (Some(illustration_slot), None, None) => {
                        empty_preview!(size, header_text, body_text, illustration = illustration_slot)
                    }
                    (None, Some(icon), None) => empty_preview!(size, header_text, body_text, icon = icon),
                    (None, None, Some(body_slot)) => {
                        empty_preview!(size, header_text, body_text, body = body_slot)
                    }
                    (Some(illustration_slot), Some(icon), None) => empty_preview!(
                        size,
                        header_text,
                        body_text,
                        illustration = illustration_slot,
                        icon = icon
                    ),
                    (Some(illustration_slot), None, Some(body_slot)) => empty_preview!(
                        size,
                        header_text,
                        body_text,
                        illustration = illustration_slot,
                        body = body_slot
                    ),
                    (None, Some(icon), Some(body_slot)) => {
                        empty_preview!(size, header_text, body_text, icon = icon, body = body_slot)
                    }
                    (Some(illustration_slot), Some(icon), Some(body_slot)) => empty_preview!(
                        size,
                        header_text,
                        body_text,
                        illustration = illustration_slot,
                        icon = icon,
                        body = body_slot
                    ),
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Header" value=header />
            <SettingText label="Body" value=body />
            <SettingSelect
                label="Size"
                value=size
                options=vec![("sm", "sm"), ("md", "md"), ("lg", "lg")]
            />
            <SettingCheck label="Icon" value=show_icon />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "size", size.get());
        push_str(
            &mut props,
            "header_text",
            &shown(&header.get(), "No incidents"),
        );
        push_str(
            &mut props,
            "body_text",
            &shown(&body.get(), "Alerts land here."),
        );
        if show_icon.get() {
            push_expr(
                &mut props,
                "icon",
                "ViewFn::from(|| view! { <DtIconBell size=\"800\" /> })",
            );
        }
        if !illustration.get().is_empty() {
            push_expr(
                &mut props,
                "illustration",
                format!(
                    "ViewFn::from(|| view! {{ <DtIllustration name={} /> }})",
                    rust_string(illustration.get())
                ),
            );
        }
        push_span_slot(&mut props, "body", &extra_body.get());
        view_of(&element("DtEmptyState", &props, None))
    });
    let slots = ViewFn::from(move || {
        let options = std::iter::once(("", "None"))
            .chain(ILLUSTRATION_NAMES.iter().copied().map(|name| (name, name)))
            .collect::<Vec<_>>();
        view! {
            <SettingSelect label="Illustration" value=illustration options=options />
            <SettingText label="Body" hint="Extra body slot. Empty hides it." value=extra_body />
        }
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"Body text stays filled so the port does not warn. The bell uses size 800, which is the icon size Vue passes into the slot. sm keeps the icon. md and lg would hide it when an illustration is set."</p>
            <Play stack=true preview=preview settings=settings slots=slots code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Small">
                    <DtEmptyState
                        size="sm"
                        header_text="No incidents"
                        body_text="Alerts land here."
                        icon=ViewFn::from(|| view! { <DtIconBell size="800" /> })
                    />
                </Variant>
                <Variant title="Large">
                    <DtEmptyState
                        size="lg"
                        header_text="No incidents"
                        body_text="Alerts land here when a service pages you."
                    />
                </Variant>
            </div>
        </section>
    }
}
