use super::*;

#[component]
pub(crate) fn CollapsiblePage() -> impl IntoView {
    provide_context(VariantSource(include_str!("collapsible.rs")));
    let anchor = RwSignal::new("Rotation details".to_string());
    let anchor_slot = RwSignal::new(String::new());
    let id = RwSignal::new(String::new());
    let max_width = RwSignal::new(String::new());
    let max_height = RwSignal::new(String::new());
    let aria_label = RwSignal::new(String::new());
    let aria_labelled_by = RwSignal::new(String::new());
    let open = RwSignal::new(true);
    let body = RwSignal::new("Primary and secondary swap every Monday.".to_string());
    let anchor_class = RwSignal::new(String::new());
    let content_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let body = body.get();
                let anchor_view = text_slot(anchor_slot.get());
                open_one!(DtCollapsible {
                    id=id.get()
                    aria_label=aria_label.get()
                    aria_labelled_by=aria_labelled_by.get()
                    anchor_text=anchor.get()
                    max_width=max_width.get()
                    max_height=max_height.get()
                    open=Signal::derive(move || open.get())
                    on_open_change=move |next| open.set(next)
                    anchor_class=anchor_class.get()
                    content_class=content_class.get()
                } {
                    <p class="m-0 text-sm">{body.clone()}</p>
                }, anchor_view, anchor)
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Anchor" value=anchor />
            <SettingText label="Id" value=id />
            <SettingText label="Max width" value=max_width />
            <SettingText label="Max height" value=max_height />
            <SettingText label="Aria label" value=aria_label />
            <SettingText label="Aria labelled by" value=aria_labelled_by />
            <SettingCheck label="Open" value=open />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "id", &id.get());
        push_str(&mut props, "aria_label", &aria_label.get());
        push_str(&mut props, "aria_labelled_by", &aria_labelled_by.get());
        push_str(&mut props, "anchor_text", &anchor.get());
        push_str(&mut props, "max_width", &max_width.get());
        push_str(&mut props, "max_height", &max_height.get());
        push_span_slot(&mut props, "anchor", &anchor_slot.get());
        push_flag(&mut props, "open", open.get());
        push_expr(&mut props, "on_open_change", "move |next| open.set(next)");
        push_str(&mut props, "anchor_class", &anchor_class.get());
        push_str(&mut props, "content_class", &content_class.get());
        let body = element(
            "p",
            &[("class", rust_string("m-0 text-sm"))],
            Some(&rust_string(&body.get())),
        );
        view_of(&element("DtCollapsible", &props, Some(&body)))
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingText label="Anchor" hint="Replaces the anchor button." value=anchor_slot />
            <SettingText label="Default" hint="Collapsible content." value=body />
        }
    });
    let classes = class_panel(vec![
        ClassField {
            label: "Anchor class",
            hint: "Extra class on the anchor.",
            value: anchor_class,
            off: None,
        },
        ClassField {
            label: "Content class",
            hint: "Extra class on the content.",
            value: content_class,
            off: None,
        },
    ]);
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"The open signal is controlled. The anchor click and the setting both write it."</p>
            <Play stack=true preview=preview settings=settings slots=slots classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Starts closed">
                    <ClosedDetails />
                </Variant>
            </div>
        </section>
    }
}

#[component]
fn ClosedDetails() -> impl IntoView {
    let open = RwSignal::new(false);
    view! {
        <DtCollapsible
            anchor_text="More"
            open=Signal::derive(move || open.get())
            on_open_change=move |next| open.set(next)
        >
            <p class="m-0 text-sm">"Hidden until the anchor is pressed."</p>
        </DtCollapsible>
    }
}
