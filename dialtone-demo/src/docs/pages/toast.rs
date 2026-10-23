use super::*;

fn toast_duration(value: &str) -> Option<u32> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        trimmed.parse().ok()
    }
}

fn toast_action(label: String, important: bool, kind: &str) -> Option<ViewFn> {
    if label.is_empty() {
        return None;
    }
    let inverted = important && matches!(kind, "base" | "critical" | "info");
    Some(ViewFn::from(move || {
        let label = label.clone();
        let kind = if inverted { "inverted" } else { "muted" };
        view! { <DtButton importance="outlined" kind=kind size="sm">{label}</DtButton> }
    }))
}

macro_rules! toast_node {
    ($icon:expr, $header:expr, $action:expr, $body:expr, { $($prop:tt)* }) => {
        match ($icon, $header, $action) {
            (Some(icon_slot), Some(header_slot), Some(action_slot)) => {
                let body_text = $body;
                if body_text.is_empty() {
                    view! { <DtToast icon=icon_slot header=header_slot action=action_slot $($prop)* /> }.into_any()
                } else {
                    view! { <DtToast icon=icon_slot header=header_slot action=action_slot $($prop)*>{body_text.clone()}</DtToast> }.into_any()
                }
            }
            (Some(icon_slot), Some(header_slot), None) => {
                let body_text = $body;
                if body_text.is_empty() {
                    view! { <DtToast icon=icon_slot header=header_slot $($prop)* /> }.into_any()
                } else {
                    view! { <DtToast icon=icon_slot header=header_slot $($prop)*>{body_text.clone()}</DtToast> }.into_any()
                }
            }
            (Some(icon_slot), None, Some(action_slot)) => {
                let body_text = $body;
                if body_text.is_empty() {
                    view! { <DtToast icon=icon_slot action=action_slot $($prop)* /> }.into_any()
                } else {
                    view! { <DtToast icon=icon_slot action=action_slot $($prop)*>{body_text.clone()}</DtToast> }.into_any()
                }
            }
            (None, Some(header_slot), Some(action_slot)) => {
                let body_text = $body;
                if body_text.is_empty() {
                    view! { <DtToast header=header_slot action=action_slot $($prop)* /> }.into_any()
                } else {
                    view! { <DtToast header=header_slot action=action_slot $($prop)*>{body_text.clone()}</DtToast> }.into_any()
                }
            }
            (Some(icon_slot), None, None) => {
                let body_text = $body;
                if body_text.is_empty() {
                    view! { <DtToast icon=icon_slot $($prop)* /> }.into_any()
                } else {
                    view! { <DtToast icon=icon_slot $($prop)*>{body_text.clone()}</DtToast> }.into_any()
                }
            }
            (None, Some(header_slot), None) => {
                let body_text = $body;
                if body_text.is_empty() {
                    view! { <DtToast header=header_slot $($prop)* /> }.into_any()
                } else {
                    view! { <DtToast header=header_slot $($prop)*>{body_text.clone()}</DtToast> }.into_any()
                }
            }
            (None, None, Some(action_slot)) => {
                let body_text = $body;
                if body_text.is_empty() {
                    view! { <DtToast action=action_slot $($prop)* /> }.into_any()
                } else {
                    view! { <DtToast action=action_slot $($prop)*>{body_text.clone()}</DtToast> }.into_any()
                }
            }
            (None, None, None) => {
                let body_text = $body;
                if body_text.is_empty() {
                    view! { <DtToast $($prop)* /> }.into_any()
                } else {
                    view! { <DtToast $($prop)*>{body_text.clone()}</DtToast> }.into_any()
                }
            }
        }
    };
}

#[component]
pub(crate) fn ToastPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("toast.rs")));
    let open = RwSignal::new(false);
    let header_id = RwSignal::new(String::new());
    let content_id = RwSignal::new(String::new());
    let pinned = RwSignal::new(true);
    let timed = RwSignal::new(false);
    let header_text = RwSignal::new("Saved".to_string());
    let message = RwSignal::new("Schedule saved".to_string());
    let kind = RwSignal::new("positive");
    let layout = RwSignal::new("default");
    let role = RwSignal::new("status");
    let important = RwSignal::new(false);
    let show_close = RwSignal::new(true);
    let show_icon = RwSignal::new(true);
    let show_action = RwSignal::new(true);
    let duration_text = RwSignal::new("6000".to_string());
    let icon = RwSignal::new("");
    let header_slot = RwSignal::new(String::new());
    let body = RwSignal::new(String::new());
    let action = RwSignal::new(String::new());
    let class_name = RwSignal::new(String::new());
    let icon_class = RwSignal::new(String::new());
    let header_class = RwSignal::new(String::new());
    let content_class = RwSignal::new(String::new());
    let action_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            <div class="docs-toast-host">
                <aside class="d-toast-wrapper">
                    {move || {
                        let header_id = header_id.get();
                        let content_id = content_id.get();
                        let message = message.get();
                        let header_text = header_text.get();
                        let kind = kind.get();
                        let layout = layout.get();
                        let role = role.get();
                        let important = important.get();
                        let show_close = show_close.get();
                        let show_icon = show_icon.get();
                        let show_action = show_action.get();
                        let class_name = class_name.get();
                        let icon_class = icon_class.get();
                        let header_class = header_class.get();
                        let content_class = content_class.get();
                        let action_class = action_class.get();
                        let icon = icon_slot(icon.get());
                        let header = text_slot(header_slot.get());
                        let action = toast_action(action.get(), important, kind);
                        let body = body.get();
                        let duration = toast_duration(&duration_text.get());
                        if let Some(ms) = duration {
                            toast_node!(icon.clone(), header.clone(), action.clone(), body.clone(), {
                                open=open
                                header_id=header_id.clone()
                                content_id=content_id.clone()
                                message=message.clone()
                                header_text=header_text.clone()
                                kind=kind
                                layout=layout
                                role=role
                                important=important
                                show_close=show_close
                                show_icon=show_icon
                                show_action=show_action
                                duration=ms
                                class=class_name.clone()
                                icon_class=icon_class.clone()
                                header_class=header_class.clone()
                                content_class=content_class.clone()
                                action_class=action_class.clone()
                                on_open=move |next| open.set(next)
                                on_close=move || open.set(false)
                            })
                        } else {
                            toast_node!(icon.clone(), header.clone(), action.clone(), body.clone(), {
                                open=open
                                header_id=header_id.clone()
                                content_id=content_id.clone()
                                message=message.clone()
                                header_text=header_text.clone()
                                kind=kind
                                layout=layout
                                role=role
                                important=important
                                show_close=show_close
                                show_icon=show_icon
                                show_action=show_action
                                class=class_name.clone()
                                icon_class=icon_class.clone()
                                header_class=header_class.clone()
                                content_class=content_class.clone()
                                action_class=action_class.clone()
                                on_open=move |next| open.set(next)
                                on_close=move || open.set(false)
                            })
                        }
                    }}
                </aside>
                <DtButton importance="outlined" size="sm" on_click=move || open.set(true)>"Show toast"</DtButton>
            </div>
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingCheck label="Open" value=open />
            <SettingText label="Header id" value=header_id />
            <SettingText label="Content id" value=content_id />
            <SettingText label="Header text" value=header_text />
            <SettingText label="Message" value=message />
            <SettingSelect
                label="Kind"
                value=kind
                options=vec![
                    ("base", "Base"),
                    ("critical", "Critical"),
                    ("info", "Info"),
                    ("positive", "Positive"),
                    ("warning", "Warning"),
                    ("gradient", "Gradient"),
                ]
            />
            <SettingSelect
                label="Layout"
                value=layout
                options=vec![("default", "Default"), ("alternate", "Alternate")]
            />
            <SettingSelect
                label="Role"
                value=role
                options=vec![("status", "Status"), ("alert", "Alert")]
            />
            <SettingText label="Duration" hint="Milliseconds. Empty keeps the toast open. Minimum 6000." value=duration_text />
            <SettingCheck label="Important" value=important />
            <SettingCheck label="Show close" value=show_close />
            <SettingCheck label="Show icon" value=show_icon />
            <SettingCheck label="Show action" value=show_action />
        }
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingSelect label="Icon" value=icon options=ICONS.to_vec() />
            <SettingText label="Header" hint="Header slot. Replaces the header text." value=header_slot />
            <SettingText label="Default" hint="Default slot. Replaces the message." value=body />
            <SettingText label="Action" hint="Action label. The alternate layout hides it." value=action />
        }
    });
    let classes = class_panel(vec![
        ClassField {
            label: "Class",
            hint: ROOT_CLASS,
            value: class_name,
            off: None,
        },
        ClassField {
            label: "Icon class",
            hint: "Extra class on the icon wrapper.",
            value: icon_class,
            off: None,
        },
        ClassField {
            label: "Header class",
            hint: "Extra class on the title.",
            value: header_class,
            off: None,
        },
        ClassField {
            label: "Content class",
            hint: "Extra class on the notice content.",
            value: content_class,
            off: None,
        },
        ClassField {
            label: "Action class",
            hint: "Extra class on the actions row.",
            value: action_class,
            off: None,
        },
    ]);
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_expr(&mut props, "open", "open");
        push_str(&mut props, "header_id", &header_id.get());
        push_str(&mut props, "content_id", &content_id.get());
        push_str(&mut props, "header_text", &header_text.get());
        if body.get().is_empty() {
            push_str(&mut props, "message", &message.get());
        }
        push_str(&mut props, "kind", kind.get());
        if layout.get() != "default" {
            push_str(&mut props, "layout", layout.get());
        }
        if role.get() != "status" {
            push_str(&mut props, "role", role.get());
        }
        push_bool(&mut props, "important", important.get());
        if !show_close.get() {
            push_flag(&mut props, "show_close", false);
        }
        if !show_icon.get() {
            push_flag(&mut props, "show_icon", false);
        }
        if !show_action.get() {
            push_flag(&mut props, "show_action", false);
        }
        if let Some(ms) = toast_duration(&duration_text.get()) {
            push_expr(&mut props, "duration", ms.to_string());
        }
        push_str(&mut props, "class", &class_name.get());
        push_str(&mut props, "icon_class", &icon_class.get());
        push_str(&mut props, "header_class", &header_class.get());
        push_str(&mut props, "content_class", &content_class.get());
        push_str(&mut props, "action_class", &action_class.get());
        push_icon(&mut props, "icon", icon.get());
        push_span_slot(&mut props, "header", &header_slot.get());
        if layout.get() != "alternate" && show_action.get() && !action.get().is_empty() {
            let inverted = important.get() && matches!(kind.get(), "base" | "critical" | "info");
            let kind_token = if inverted { "inverted" } else { "muted" };
            push_expr(
                &mut props,
                "action",
                format!(
                    "ViewFn::from(|| view! {{ <DtButton importance=\"outlined\" kind=\"{kind_token}\" size=\"sm\">{}</DtButton> }})",
                    rust_string(&action.get())
                ),
            );
        }
        push_expr(&mut props, "on_open", "move |next| open.set(next)");
        push_expr(&mut props, "on_close", "move || open.set(false)");
        let child = if body.get().is_empty() {
            None
        } else {
            Some(rust_string(&body.get()))
        };
        let toast = element("DtToast", &props, child.as_deref());
        let wrapped = format!("<aside class=\"d-toast-wrapper\">\n    {toast}\n</aside>");
        format!("let open = RwSignal::new(false);\n{}", view_of(&wrapped))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"Duration must be at least 6000 ms. Leave it empty to keep the toast until it is closed. Gradient applies to the alternate layout. The wrapper that pins the toast sits outside DtToast."</p>
            <Play stack=true preview=preview settings=settings slots=slots classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Kinds">
                    <DtToast open=pinned header_text="Base" message="Neutral notice." kind="base" show_action=false />
                    <DtToast open=pinned header_text="Critical" message="Something failed." kind="critical" show_action=false />
                    <DtToast open=pinned header_text="Info" message="Changes apply on the next sync." kind="info" show_action=false />
                    <DtToast open=pinned header_text="Positive" message="Schedule saved." kind="positive" show_action=false />
                    <DtToast open=pinned header_text="Warning" message="Two schedules still need a target." kind="warning" show_action=false />
                    <DtToast open=pinned header_text="Important" message="Needs attention." kind="critical" important=true show_action=false />
                </Variant>
                <Variant title="Alternate">
                    <DtToast open=pinned layout="alternate" header_text="Base" message="A shorter toast." kind="base" />
                    <DtToast open=pinned layout="alternate" header_text="Positive" message="Schedule saved." kind="positive" />
                    <DtToast open=pinned layout="alternate" header_text="Gradient" message="Uses the sparkle icon." kind="gradient" />
                </Variant>
                <Variant title="Slots">
                    <DtToast
                        open=pinned
                        header_text="Custom icon"
                        message="The bell replaces the kind icon."
                        kind="positive"
                        show_action=false
                        icon=ViewFn::from(|| view! { <DtIconBell size="400" /> })
                    />
                    <DtToast
                        open=pinned
                        kind="info"
                        message="Undo is available."
                        header=ViewFn::from(|| view! { <span>"Review the change"</span> })
                        action=ViewFn::from(|| view! {
                            <DtButton importance="outlined" kind="muted" size="sm">"Undo"</DtButton>
                        })
                    />
                    <DtToast
                        open=pinned
                        important=true
                        kind="base"
                        header_text="Important"
                        message="The action uses the inverted kind."
                        action=ViewFn::from(|| view! {
                            <DtButton importance="outlined" kind="inverted" size="sm">"Review"</DtButton>
                        })
                    />
                </Variant>
                <Variant title="Duration">
                    <div class="docs-toast-host">
                        <aside class="d-toast-wrapper">
                            <DtToast
                                open=timed
                                header_text="Saved"
                                message="This toast closes after 6000 ms."
                                kind="positive"
                                duration=6000
                                show_action=false
                                on_open=move |next| timed.set(next)
                                on_close=move || timed.set(false)
                            />
                        </aside>
                        <DtButton importance="outlined" size="sm" on_click=move || timed.set(true)>"Show for 6 seconds"</DtButton>
                    </div>
                </Variant>
            </div>
        </section>
    }
}
