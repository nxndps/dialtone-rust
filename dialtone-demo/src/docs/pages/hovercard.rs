use super::*;

const HOVER_PLACEMENTS: &[(&str, &str)] = &[
    ("top-start", "Top start"),
    ("top", "Top"),
    ("top-end", "Top end"),
    ("right-start", "Right start"),
    ("right", "Right"),
    ("right-end", "Right end"),
    ("bottom-start", "Bottom start"),
    ("bottom", "Bottom"),
    ("bottom-end", "Bottom end"),
    ("left-start", "Left start"),
    ("left", "Left"),
    ("left-end", "Left end"),
];

const PADDINGS: &[(&str, &str)] = &[
    ("none", "None"),
    ("small", "Small"),
    ("medium", "Medium"),
    ("large", "Large"),
];

const CONTENT_MODES: &[(&str, &str)] = &[
    ("", "None"),
    ("light", "Light"),
    ("dark", "Dark"),
    ("invert", "Invert"),
];

const DELAYS: &[(&str, &str)] = &[("0", "0"), ("300", "300")];

fn or_static<'a>(value: &'a str, default: &'a str) -> &'a str {
    if value.is_empty() {
        default
    } else {
        value
    }
}

macro_rules! hover_preview {
    (
        $placement:expr, $fallback:expr, $padding:expr, $offset:expr, $content_mode:expr,
        $transition:expr, $enter_delay:expr, $leave_delay:expr, $id:expr, $append_to:expr,
        $anchor_class:expr, $header_class:expr, $footer_class:expr, $dialog_class:expr,
        $content_class:expr, $anchor:expr, $body:expr, $header:expr, $footer:expr;
        $($extra:tt)*
    ) => {
        open_two!(DtHovercard {
            placement=$placement
            fallback_placements=$fallback
            padding=$padding
            offset=$offset
            content_mode=$content_mode
            transition=$transition
            enter_delay=$enter_delay
            leave_delay=$leave_delay
            id=$id
            append_to=$append_to
            anchor_class=$anchor_class
            header_class=$header_class
            footer_class=$footer_class
            dialog_class=$dialog_class
            content_class=$content_class
            anchor=$anchor
            $($extra)*
        } {
            <p class="m-0 max-w-xs text-sm">{$body.clone()}</p>
        }, $header, header, $footer, footer)
    };
}

fn hover_anchor() -> ViewFn {
    ViewFn::from(|| {
        view! { <DtButton importance="outlined" kind="muted">"Hover over me"</DtButton> }
    })
}

#[component]
pub(crate) fn HovercardPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("hovercard.rs")));
    let placement = RwSignal::new("top-start");
    let padding = RwSignal::new("large");
    let content_mode = RwSignal::new("");
    let transition = RwSignal::new(false);
    let enter_delay = RwSignal::new("300");
    let content = RwSignal::new("Content".to_string());
    let header = RwSignal::new(String::new());
    let footer = RwSignal::new(String::new());
    let dialog_class = RwSignal::new(String::new());
    let open_mode = RwSignal::new("");
    let shown = RwSignal::new(false);
    let fallback = RwSignal::new("");
    let offset_text = RwSignal::new(String::new());
    let card_id = RwSignal::new(String::new());
    let append_to = RwSignal::new("");
    let leave_text = RwSignal::new(String::new());
    let anchor_label = RwSignal::new("Hover over me".to_string());
    let anchor_class = RwSignal::new(String::new());
    let header_class = RwSignal::new(String::new());
    let footer_class = RwSignal::new(String::new());
    let content_class = RwSignal::new(String::new());
    let unset_open: Option<bool> = None;
    Effect::new(move |_| match open_mode.get() {
        "true" => shown.set(true),
        "false" => shown.set(false),
        _ => {}
    });
    let on_open = Callback::new(move |next: bool| shown.set(next));
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let placement = placement.get();
                let padding = padding.get();
                let content_mode = content_mode.get();
                let transition = transition.get();
                let enter_delay = enter_delay.get().parse::<u64>().unwrap_or(300);
                let body = content.get();
                let dialog_class_value = dialog_class.get();
                let fallback = or_static(fallback.get(), "auto");
                let offset = offset_text.get().parse().unwrap_or(16.0);
                let leave_delay = leave_text.get().parse().unwrap_or(300u64);
                let card_id = card_id.get();
                let append_to = or_static(append_to.get(), "body");
                let anchor_class = anchor_class.get();
                let header_class = header_class.get();
                let footer_class = footer_class.get();
                let content_class = content_class.get();
                let anchor_text = anchor_label.get();
                let anchor = ViewFn::from(move || {
                    let anchor_text = anchor_text.clone();
                    view! { <DtButton importance="outlined" kind="muted">{anchor_text}</DtButton> }
                });
                let header_view = text_slot(header.get());
                let footer_view = text_slot(footer.get());
                let mode = open_mode.get();
                match mode {
                    "true" | "false" => hover_preview!(
                        placement, fallback, padding, offset, content_mode, transition, enter_delay,
                        leave_delay, card_id, append_to, anchor_class, header_class, footer_class,
                        dialog_class_value, content_class, anchor, body, header_view, footer_view;
                        open=shown.into() on_open=on_open
                    ),
                    "null" => hover_preview!(
                        placement, fallback, padding, offset, content_mode, transition, enter_delay,
                        leave_delay, card_id, append_to, anchor_class, header_class, footer_class,
                        dialog_class_value, content_class, anchor, body, header_view, footer_view;
                        open=unset_open.into()
                    ),
                    _ => hover_preview!(
                        placement, fallback, padding, offset, content_mode, transition, enter_delay,
                        leave_delay, card_id, append_to, anchor_class, header_class, footer_class,
                        dialog_class_value, content_class, anchor, body, header_view, footer_view;
                    ),
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Content" value=content />
            <SettingSelect label="Placement" value=placement options=HOVER_PLACEMENTS.to_vec() />
            <SettingSelect label="Padding" value=padding options=PADDINGS.to_vec() />
            <SettingSelect label="Content mode" value=content_mode options=CONTENT_MODES.to_vec() />
            <SettingCheck label="Transition" value=transition />
            <SettingSelect label="Enter delay" value=enter_delay options=DELAYS.to_vec() />
            <SettingSelect
                label="Open"
                value=open_mode
                options=vec![("", "Default"), ("null", "Null"), ("true", "True"), ("false", "False")]
            />
            <SettingSelect
                label="Fallback"
                value=fallback
                options={
                    let mut options = vec![("", "Default"), ("auto", "Auto")];
                    options.extend(HOVER_PLACEMENTS.iter().copied());
                    options
                }
            />
            <SettingText label="Offset" hint="Empty uses 16." value=offset_text />
            <SettingText label="Id" hint="Empty generates an id." value=card_id />
            <SettingSelect
                label="Append to"
                value=append_to
                options=vec![
                    ("", "Default"),
                    ("body", "Body"),
                    ("parent", "Parent"),
                    ("root", "Root"),
                    ("HTMLElement", "HTMLElement"),
                ]
            />
            <SettingText label="Leave delay" hint="Empty uses 300." value=leave_text />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        if placement.get() != "top-start" {
            push_str(&mut props, "placement", placement.get());
        }
        if padding.get() != "large" {
            push_str(&mut props, "padding", padding.get());
        }
        push_str(&mut props, "content_mode", content_mode.get());
        push_bool(&mut props, "transition", transition.get());
        let delay = enter_delay.get().parse::<u64>().unwrap_or(300);
        if delay != 300 {
            push_expr(&mut props, "enter_delay", delay.to_string());
        }
        push_str(&mut props, "dialog_class", &dialog_class.get());
        match open_mode.get() {
            "true" | "false" => {
                push_expr(&mut props, "open", "shown");
                push_expr(&mut props, "on_open", "move |next: bool| shown.set(next)");
            }
            "null" => push_expr(&mut props, "open", "None"),
            _ => {}
        }
        if !fallback.get().is_empty() && fallback.get() != "auto" {
            push_str(&mut props, "fallback_placements", fallback.get());
        }
        if let Ok(offset) = offset_text.get().parse::<f64>() {
            push_expr(&mut props, "offset", offset.to_string());
        }
        push_str(&mut props, "id", &card_id.get());
        push_str(&mut props, "append_to", append_to.get());
        if let Ok(delay) = leave_text.get().parse::<u64>() {
            push_expr(&mut props, "leave_delay", delay.to_string());
        }
        push_str(&mut props, "anchor_class", &anchor_class.get());
        push_str(&mut props, "header_class", &header_class.get());
        push_str(&mut props, "footer_class", &footer_class.get());
        push_str(&mut props, "content_class", &content_class.get());
        push_span_slot(&mut props, "header", &header.get());
        push_span_slot(&mut props, "footer", &footer.get());
        push_expr(
            &mut props,
            "anchor",
            format!(
                "ViewFn::from(|| view! {{ <DtButton importance=\"outlined\" kind=\"muted\">{}</DtButton> }})",
                rust_string(&anchor_label.get())
            ),
        );
        let body = element(
            "p",
            &[("class", rust_string("m-0 max-w-xs text-sm"))],
            Some(&rust_string(&content.get())),
        );
        view_of(&element("DtHovercard", &props, Some(&body)))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">
                "Click does not toggle, and focus on the anchor does not open the card. The enter delay is 300 ms. Focus inside the content keeps it open. Escape and an outside click close it and restore focus. OverlayScrollbars, an HTMLElement append target, offset skid, and the docs warm-up are not in this port."
            </p>
            <Play
                preview=preview
                settings=settings
                slots=ViewFn::from(move || view! {
                    <SettingText label="Anchor" hint="Anchor slot. The control that opens the card." value=anchor_label />
                    <SettingText label="Header" hint="Header slot. Empty hides it." value=header />
                    <SettingText label="Footer" hint="Footer slot. Empty hides it." value=footer />
                })
                classes=class_panel(vec![
                    ClassField {
                        label: "Anchor class",
                        hint: "Extra class on the anchor.",
                        value: anchor_class,
                        off: None,
                    },
                    ClassField {
                        label: "Header class",
                        hint: "Extra class on the header.",
                        value: header_class,
                        off: Some(Signal::derive(move || header.get().is_empty())),
                    },
                    ClassField {
                        label: "Footer class",
                        hint: "Extra class on the footer.",
                        value: footer_class,
                        off: Some(Signal::derive(move || footer.get().is_empty())),
                    },
                    ClassField {
                        label: "Content class",
                        hint: "Extra class on the content.",
                        value: content_class,
                        off: None,
                    },
                    ClassField {
                        label: "Dialog class",
                        hint: "Extra class on the dialog.",
                        value: dialog_class,
                        off: None,
                    },
                ])
                code=code
            />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Header, content, and footer" row=true>
                    <DtHovercard
                        placement="bottom-start"
                        anchor=hover_anchor()
                        header=ViewFn::from(|| view! { <span>"Header"</span> })
                        footer=ViewFn::from(|| view! { <span>"Footer"</span> })
                    >
                        <div>"Content"</div>
                    </DtHovercard>
                </Variant>
                <Variant title="Content mode" row=true>
                    <DtHovercard
                        content_mode="invert"
                        anchor=ViewFn::from(|| view! {
                            <DtButton importance="outlined" kind="muted" size="sm">"Invert"</DtButton>
                        })
                    >
                        <p class="m-0 text-sm">"Inverted content."</p>
                    </DtHovercard>
                    <DtHovercard
                        content_mode="dark"
                        anchor=ViewFn::from(|| view! {
                            <DtButton importance="outlined" kind="muted" size="sm">"Dark"</DtButton>
                        })
                    >
                        <p class="m-0 text-sm">"Dark content."</p>
                    </DtHovercard>
                    <DtHovercard
                        content_mode="light"
                        anchor=ViewFn::from(|| view! {
                            <DtButton importance="outlined" kind="muted" size="sm">"Light"</DtButton>
                        })
                    >
                        <p class="m-0 text-sm">"Light content."</p>
                    </DtHovercard>
                </Variant>
            </div>
        </section>
    }
}
