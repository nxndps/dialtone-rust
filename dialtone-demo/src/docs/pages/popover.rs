use super::*;

const CONTENT_MODES: &[(&str, &str)] = &[
    ("", "None"),
    ("light", "Light"),
    ("dark", "Dark"),
    ("invert", "Invert"),
];

const OPEN_MODES: &[(&str, &str)] = &[("", "Default"), ("true", "True"), ("false", "False")];

const CONTENT_WIDTHS: &[(&str, &str)] = &[("", "Default"), ("anchor", "Anchor")];

const TRANSITIONS: &[(&str, &str)] = &[("", "None"), ("fade", "Fade")];

fn intern(value: &str) -> &'static str {
    use std::cell::RefCell;
    use std::collections::HashMap;
    thread_local! {
        static CACHE: RefCell<HashMap<String, &'static str>> = RefCell::new(HashMap::new());
    }
    CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some(found) = cache.get(value) {
            return *found;
        }
        // Role and fallback placements are `&'static str`, so each distinct value is leaked once.
        let leaked: &'static str = Box::leak(value.to_string().into_boxed_str());
        cache.insert(value.to_string(), leaked);
        leaked
    })
}

fn static_or(value: &str, fallback: &'static str) -> &'static str {
    if value.is_empty() {
        fallback
    } else {
        intern(value)
    }
}

fn offset_px(raw: &str) -> f64 {
    if raw.trim().is_empty() {
        return 4.0;
    }
    raw.split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .last()
        .and_then(|token| token.parse().ok())
        .unwrap_or(4.0)
}

fn rust_f64(value: f64) -> String {
    if value.fract() == 0.0 && value.is_finite() {
        format!("{value:.1}")
    } else {
        value.to_string()
    }
}

fn popover_anchor(label: String) -> ViewFn {
    ViewFn::from(move || {
        let label = label.clone();
        view! { <DtButton importance="outlined">{label}</DtButton> }
    })
}

#[component]
pub(crate) fn PopoverPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("popover.rs")));
    let placement = RwSignal::new("bottom");
    let padding = RwSignal::new("small");
    let append_to = RwSignal::new("body");
    let modal = RwSignal::new(false);
    let content_mode = RwSignal::new("");
    let open_mode = RwSignal::new("");
    let open_state = RwSignal::new(false);
    Effect::new(move |_| match open_mode.get() {
        "true" => open_state.set(true),
        "false" => open_state.set(false),
        _ => {}
    });
    let open_on_context = RwSignal::new(false);
    let transition = RwSignal::new("");
    let role_text = RwSignal::new(String::new());
    let content_width = RwSignal::new("");
    let id_text = RwSignal::new(String::new());
    let offset_text = RwSignal::new(String::new());
    let focus_trap = RwSignal::new(false);
    let fallback_text = RwSignal::new(String::new());
    let tether = RwSignal::new(false);
    let max_height = RwSignal::new(String::new());
    let max_width = RwSignal::new(String::new());
    let content = RwSignal::new("The rotation repeats every week.".to_string());
    let anchor_label = RwSignal::new("Open popover".to_string());
    let header = RwSignal::new(String::new());
    let footer = RwSignal::new(String::new());
    let dialog_class = RwSignal::new(String::new());
    let content_class = RwSignal::new(String::new());
    let anchor_class = RwSignal::new(String::new());
    let header_class = RwSignal::new(String::new());
    let footer_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let placement = placement.get();
                let padding = padding.get();
                let append_to = append_to.get();
                let modal = modal.get();
                let content_mode = content_mode.get();
                let open_mode = open_mode.get();
                let open_on_context = open_on_context.get();
                let transition = transition.get();
                let role = static_or(&role_text.get(), "dialog");
                let content_width = content_width.get();
                let id_value = id_text.get();
                let offset = offset_px(&offset_text.get());
                let focus_trap = focus_trap.get();
                let fallback = static_or(&fallback_text.get(), "auto");
                let tether = tether.get();
                let max_height = max_height.get();
                let max_width = max_width.get();
                let dialog_class_value = dialog_class.get();
                let content_class_value = content_class.get();
                let anchor_class_value = anchor_class.get();
                let header_class_value = header_class.get();
                let footer_class_value = footer_class.get();
                let body = content.get();
                let anchor_text = anchor_label.get();
                let header_text = header.get();
                let footer_text = footer.get();
                let controlled = matches!(open_mode, "true" | "false");
                macro_rules! preview_popover {
                    ($($extra:tt)*) => {
                        open_two!(DtPopover {
                            id=id_value.clone()
                            placement=placement
                            fallback_placements=fallback
                            offset=offset
                            padding=padding
                            role=role
                            append_to=append_to
                            modal=modal
                            content_width=content_width
                            max_height=max_height.clone()
                            max_width=max_width.clone()
                            dialog_class=dialog_class_value.clone()
                            content_class=content_class_value.clone()
                            header_class=header_class_value.clone()
                            footer_class=footer_class_value.clone()
                            content_mode=content_mode
                            open_on_context=open_on_context
                            tether=tether
                            transition=transition
                            anchor_class=anchor_class_value.clone()
                            focus_trap=focus_trap
                            anchor=popover_anchor(anchor_text.clone())
                            $($extra)*
                        } {
                            <p class="m-0 max-w-xs text-sm">{body.clone()}</p>
                        }, text_slot(header_text.clone()), header, text_slot(footer_text.clone()), footer)
                    };
                }
                if controlled {
                    preview_popover!(
                        open=Signal::derive(move || open_state.get())
                        on_open_change=move |next| open_state.set(next)
                    )
                } else {
                    preview_popover!()
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Content" value=content />
            <SettingSelect label="Placement" value=placement options=PLACEMENTS.to_vec() />
            <SettingSelect
                label="Padding"
                value=padding
                options=vec![("none", "None"), ("small", "Small"), ("medium", "Medium"), ("large", "Large")]
            />
            <SettingSelect label="Append to" value=append_to options=APPEND.to_vec() />
            <SettingCheck label="Modal" value=modal />
            <SettingSelect label="Content mode" value=content_mode options=CONTENT_MODES.to_vec() />
            <SettingSelect label="Open" value=open_mode options=OPEN_MODES.to_vec() />
            <SettingCheck label="Open on context" value=open_on_context />
            <SettingSelect label="Transition" value=transition options=TRANSITIONS.to_vec() />
            <SettingText label="Role" hint="Empty uses dialog." value=role_text />
            <SettingSelect label="Content width" value=content_width options=CONTENT_WIDTHS.to_vec() />
            <SettingText label="Id" hint="Empty generates one." value=id_text />
            <SettingText label="Offset" hint="Distance in px, or skid, distance. Empty uses 4." value=offset_text />
            <SettingCheck label="Focus trap" value=focus_trap />
            <SettingText label="Fallback placements" hint="Comma-separated. Empty uses auto." value=fallback_text />
            <SettingCheck label="Tether" value=tether />
            <SettingText label="Max height" hint="CSS length. Empty leaves it unset." value=max_height />
            <SettingText label="Max width" hint="CSS length. Empty leaves it unset." value=max_width />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "placement", placement.get());
        push_str(&mut props, "padding", padding.get());
        push_str(&mut props, "append_to", append_to.get());
        push_bool(&mut props, "modal", modal.get());
        push_str(&mut props, "content_mode", content_mode.get());
        if matches!(open_mode.get(), "true" | "false") {
            push_expr(&mut props, "open", "open");
            push_expr(&mut props, "on_open_change", "move |next| open.set(next)");
        }
        push_bool(&mut props, "open_on_context", open_on_context.get());
        push_str(&mut props, "transition", transition.get());
        push_str(&mut props, "role", &role_text.get());
        push_str(&mut props, "content_width", content_width.get());
        push_str(&mut props, "id", &id_text.get());
        let offset = offset_px(&offset_text.get());
        if (offset - 4.0).abs() > f64::EPSILON {
            push_expr(&mut props, "offset", rust_f64(offset));
        }
        push_bool(&mut props, "focus_trap", focus_trap.get());
        push_str(&mut props, "fallback_placements", &fallback_text.get());
        push_bool(&mut props, "tether", tether.get());
        push_str(&mut props, "max_height", &max_height.get());
        push_str(&mut props, "max_width", &max_width.get());
        push_str(&mut props, "dialog_class", &dialog_class.get());
        push_str(&mut props, "content_class", &content_class.get());
        push_str(&mut props, "anchor_class", &anchor_class.get());
        push_str(&mut props, "header_class", &header_class.get());
        push_str(&mut props, "footer_class", &footer_class.get());
        if !header.get().is_empty() {
            push_expr(
                &mut props,
                "header",
                format!(
                    "ViewFn::from(|| view! {{ <span>{}</span> }})",
                    rust_string(&header.get())
                ),
            );
        }
        if !footer.get().is_empty() {
            push_expr(
                &mut props,
                "footer",
                format!(
                    "ViewFn::from(|| view! {{ <span>{}</span> }})",
                    rust_string(&footer.get())
                ),
            );
        }
        push_expr(
            &mut props,
            "anchor",
            format!(
                "ViewFn::from(|| view! {{ <DtButton importance=\"outlined\">{}</DtButton> }})",
                rust_string(&anchor_label.get())
            ),
        );
        let body = element(
            "p",
            &[("class", rust_string("m-0 max-w-xs text-sm"))],
            Some(&rust_string(&content.get())),
        );
        view_of(&element("DtPopover", &props, Some(&body)))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"Unset open toggles from the anchor. Modal is off so the rest of the page stays clickable."</p>
            <Play
                preview=preview
                settings=settings
                slots=ViewFn::from(move || view! {
                    <SettingText label="Anchor" hint="Anchor slot. The control that opens the popover." value=anchor_label />
                    <SettingText label="Default" hint="Popover content." value=content />
                    <SettingText label="Header" hint="Header slot. Empty hides it." value=header />
                    <SettingText label="Footer" hint="Footer slot. Empty hides it." value=footer />
                })
                classes=class_panel(vec![
                    ClassField {
                        label: "Dialog class",
                        hint: "Extra class on the dialog.",
                        value: dialog_class,
                        off: None,
                    },
                    ClassField {
                        label: "Content class",
                        hint: "Extra class on the popover content.",
                        value: content_class,
                        off: None,
                    },
                    ClassField {
                        label: "Anchor class",
                        hint: "Extra class on the anchor wrapper.",
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
                ])
                code=code
            />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Right" row=true>
                    <DtPopover
                        placement="right"
                        padding="medium"
                        modal=false
                        anchor=ViewFn::from(|| view! { <DtButton importance="clear">"More"</DtButton> })
                    >
                        <p class="m-0 text-sm">"Placed to the right of the anchor."</p>
                    </DtPopover>
                </Variant>
            </div>
        </section>
    }
}
