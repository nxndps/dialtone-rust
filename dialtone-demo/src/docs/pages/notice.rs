use super::*;

fn action_kind(important: bool) -> &'static str {
    if important {
        "inverted"
    } else {
        "muted"
    }
}

fn action_class(important: bool, kind: &str) -> &'static str {
    if important && kind == "warning" {
        "d-bc-neutral-black"
    } else {
        ""
    }
}

fn link_class(important: bool, kind: &str) -> &'static str {
    if important && kind == "warning" {
        "d-link d-fc-neutral-black"
    } else if important {
        "d-link d-fc-primary-inverted"
    } else {
        "d-link d-fc-primary"
    }
}

fn notice_action(label: String, important: bool, kind: &str) -> Option<ViewFn> {
    if label.is_empty() {
        return None;
    }
    let button_kind = action_kind(important);
    let class_name = action_class(important, kind);
    Some(ViewFn::from(move || {
        let label = label.clone();
        view! {
            <DtButton importance="outlined" kind=button_kind size="sm" class=class_name>
                {label}
            </DtButton>
        }
    }))
}

fn action_expr(label: &str, important: bool, kind: &str) -> String {
    let button_kind = action_kind(important);
    let class_name = action_class(important, kind);
    let class_attr = if class_name.is_empty() {
        String::new()
    } else {
        format!(" class=\"{class_name}\"")
    };
    format!(
        "ViewFn::from(|| view! {{ <DtButton importance=\"outlined\" kind=\"{button_kind}\" size=\"sm\"{class_attr}>{}</DtButton> }})",
        rust_string(label)
    )
}

macro_rules! notice_node {
    ($icon:expr, $header:expr, $action:expr, $body:expr, { $($prop:tt)* }) => {
        match ($icon, $header, $action) {
            (Some(icon_slot), Some(header_slot), Some(action_slot)) => {
                let body_text = $body;
                if body_text.is_empty() {
                    view! { <DtNotice icon=icon_slot header=header_slot action=action_slot $($prop)* /> }.into_any()
                } else {
                    view! { <DtNotice icon=icon_slot header=header_slot action=action_slot $($prop)*>{body_text.clone()}</DtNotice> }.into_any()
                }
            }
            (Some(icon_slot), Some(header_slot), None) => {
                let body_text = $body;
                if body_text.is_empty() {
                    view! { <DtNotice icon=icon_slot header=header_slot $($prop)* /> }.into_any()
                } else {
                    view! { <DtNotice icon=icon_slot header=header_slot $($prop)*>{body_text.clone()}</DtNotice> }.into_any()
                }
            }
            (Some(icon_slot), None, Some(action_slot)) => {
                let body_text = $body;
                if body_text.is_empty() {
                    view! { <DtNotice icon=icon_slot action=action_slot $($prop)* /> }.into_any()
                } else {
                    view! { <DtNotice icon=icon_slot action=action_slot $($prop)*>{body_text.clone()}</DtNotice> }.into_any()
                }
            }
            (None, Some(header_slot), Some(action_slot)) => {
                let body_text = $body;
                if body_text.is_empty() {
                    view! { <DtNotice header=header_slot action=action_slot $($prop)* /> }.into_any()
                } else {
                    view! { <DtNotice header=header_slot action=action_slot $($prop)*>{body_text.clone()}</DtNotice> }.into_any()
                }
            }
            (Some(icon_slot), None, None) => {
                let body_text = $body;
                if body_text.is_empty() {
                    view! { <DtNotice icon=icon_slot $($prop)* /> }.into_any()
                } else {
                    view! { <DtNotice icon=icon_slot $($prop)*>{body_text.clone()}</DtNotice> }.into_any()
                }
            }
            (None, Some(header_slot), None) => {
                let body_text = $body;
                if body_text.is_empty() {
                    view! { <DtNotice header=header_slot $($prop)* /> }.into_any()
                } else {
                    view! { <DtNotice header=header_slot $($prop)*>{body_text.clone()}</DtNotice> }.into_any()
                }
            }
            (None, None, Some(action_slot)) => {
                let body_text = $body;
                if body_text.is_empty() {
                    view! { <DtNotice action=action_slot $($prop)* /> }.into_any()
                } else {
                    view! { <DtNotice action=action_slot $($prop)*>{body_text.clone()}</DtNotice> }.into_any()
                }
            }
            (None, None, None) => {
                let body_text = $body;
                if body_text.is_empty() {
                    view! { <DtNotice $($prop)* /> }.into_any()
                } else {
                    view! { <DtNotice $($prop)*>{body_text.clone()}</DtNotice> }.into_any()
                }
            }
        }
    };
}

#[component]
fn SampleNotice(
    kind: &'static str,
    title: &'static str,
    #[prop(optional)] important: bool,
    #[prop(optional)] truncate_text: bool,
    #[prop(optional)] message: &'static str,
) -> impl IntoView {
    let button_kind = action_kind(important);
    let button_class = action_class(important, kind);
    let anchor_class = link_class(important, kind);
    if !message.is_empty() {
        view! {
            <DtNotice
                header_text=title
                kind=kind
                important=important
                truncate_text=truncate_text
                show_action=false
            >
                {message}
            </DtNotice>
        }
        .into_any()
    } else {
        let action = ViewFn::from(move || {
            if button_class.is_empty() {
                view! {
                    <DtButton importance="outlined" kind=button_kind size="sm">
                        "Action"
                    </DtButton>
                }
                .into_any()
            } else {
                view! {
                    <DtButton
                        importance="outlined"
                        kind=button_kind
                        size="sm"
                        class=button_class
                    >
                        "Action"
                    </DtButton>
                }
                .into_any()
            }
        });
        view! {
            <DtNotice
                header_text=title
                kind=kind
                important=important
                truncate_text=truncate_text
                action=action
            >
                "Message body with "
                <a href="#" class=anchor_class>"a link"</a>
                "."
            </DtNotice>
        }
        .into_any()
    }
}

#[component]
pub(crate) fn NoticePage() -> impl IntoView {
    provide_context(VariantSource(include_str!("notice.rs")));
    let visible = RwSignal::new(true);
    let header_text = RwSignal::new("Base title (optional)".to_string());
    let message = RwSignal::new("Message body.".to_string());
    let kind = RwSignal::new("base");
    let role = RwSignal::new("status");
    let important = RwSignal::new(false);
    let truncate_text = RwSignal::new(false);
    let show_close = RwSignal::new(true);
    let show_icon = RwSignal::new(true);
    let show_action = RwSignal::new(true);
    let icon = RwSignal::new("");
    let header_slot = RwSignal::new(String::new());
    let action = RwSignal::new("Action".to_string());
    let header_id = RwSignal::new(String::new());
    let content_id = RwSignal::new(String::new());
    let icon_class = RwSignal::new(String::new());
    let header_class = RwSignal::new(String::new());
    let content_class = RwSignal::new(String::new());
    let action_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                if !visible.get() {
                    view! {
                        <DtButton importance="outlined" size="sm" on_click=move || visible.set(true)>
                            "Show notice"
                        </DtButton>
                    }
                    .into_any()
                } else {
                    let header_text = header_text.get();
                    let message = message.get();
                    let kind = kind.get();
                    let role = role.get();
                    let important = important.get();
                    let truncate_text = truncate_text.get();
                    let show_close = show_close.get();
                    let show_icon = show_icon.get();
                    let show_action = show_action.get();
                    let header_id = header_id.get();
                    let content_id = content_id.get();
                    let icon_class = icon_class.get();
                    let header_class = header_class.get();
                    let content_class = content_class.get();
                    let action_class = action_class.get();
                    let icon = icon_slot(icon.get());
                    let header = text_slot(header_slot.get());
                    let action = show_action
                        .then(|| notice_action(action.get(), important, kind))
                        .flatten();
                    notice_node!(icon, header, action, message, {
                        header_text=header_text
                        header_id=header_id
                        content_id=content_id
                        kind=kind
                        role=role
                        important=important
                        truncate_text=truncate_text
                        show_close=show_close
                        show_icon=show_icon
                        show_action=show_action
                        icon_class=icon_class
                        header_class=header_class
                        content_class=content_class
                        action_class=action_class
                        on_close=move || visible.set(false)
                    })
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Header text" value=header_text />
            <SettingText label="Message" value=message />
            <SettingText label="Header id" value=header_id />
            <SettingText label="Content id" value=content_id />
            <SettingSelect
                label="Kind"
                value=kind
                options=vec![
                    ("base", "Base"),
                    ("critical", "Critical"),
                    ("info", "Info"),
                    ("positive", "Positive"),
                    ("warning", "Warning"),
                ]
            />
            <SettingSelect
                label="Role"
                value=role
                options=vec![
                    ("status", "Status"),
                    ("alert", "Alert"),
                    ("alertdialog", "Alert dialog"),
                ]
            />
            <SettingCheck label="Important" value=important />
            <SettingCheck label="Truncate text" value=truncate_text />
            <SettingCheck label="Show close" value=show_close />
            <SettingCheck label="Show icon" value=show_icon />
            <SettingCheck label="Show action" value=show_action />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "header_text", &header_text.get());
        push_str(&mut props, "header_id", &header_id.get());
        push_str(&mut props, "content_id", &content_id.get());
        push_str(&mut props, "kind", kind.get());
        if role.get() != "status" {
            push_str(&mut props, "role", role.get());
        }
        push_bool(&mut props, "important", important.get());
        push_bool(&mut props, "truncate_text", truncate_text.get());
        if !show_close.get() {
            push_expr(&mut props, "show_close", "false");
        }
        if !show_icon.get() {
            push_expr(&mut props, "show_icon", "false");
        }
        if !show_action.get() {
            push_expr(&mut props, "show_action", "false");
        }
        push_str(&mut props, "icon_class", &icon_class.get());
        push_str(&mut props, "header_class", &header_class.get());
        push_str(&mut props, "content_class", &content_class.get());
        push_str(&mut props, "action_class", &action_class.get());
        push_icon(&mut props, "icon", icon.get());
        push_span_slot(&mut props, "header", &header_slot.get());
        if show_action.get() && !action.get().is_empty() {
            push_expr(
                &mut props,
                "action",
                action_expr(&action.get(), important.get(), kind.get()),
            );
        }
        if show_close.get() {
            push_expr(&mut props, "on_close", "move || visible.set(false)");
        }
        view_of(&element(
            "DtNotice",
            &props,
            Some(&rust_string(&message.get())),
        ))
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingText label="Default" hint="Message body. Empty hides it." value=message />
            <SettingText label="Header" hint="Header slot. Empty uses the header text." value=header_slot />
            <SettingSelect label="Icon" value=icon options=ICONS.to_vec() />
            <SettingText label="Action" hint="Action button label. Empty hides it." value=action />
        }
    });
    let classes = class_panel(vec![
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
            hint: "Extra class on the content wrapper.",
            value: content_class,
            off: None,
        },
        ClassField {
            label: "Action class",
            hint: "Extra class on the action wrapper.",
            value: action_class,
            off: None,
        },
    ]);
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <Play preview=preview settings=settings slots=slots classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Kinds">
                    <DtStack gap="100">
                        <SampleNotice kind="base" title="Base title (optional)" />
                        <SampleNotice kind="critical" title="Critical title (optional)" />
                        <SampleNotice kind="info" title="Info title (optional)" />
                        <SampleNotice kind="positive" title="Positive title (optional)" />
                        <SampleNotice kind="warning" title="Warning title (optional)" />
                    </DtStack>
                </Variant>
                <Variant title="Important">
                    <DtStack gap="100">
                        <SampleNotice kind="base" title="Base title (optional)" important=true />
                        <SampleNotice kind="critical" title="Critical title (optional)" important=true />
                        <SampleNotice kind="info" title="Info title (optional)" important=true />
                        <SampleNotice kind="positive" title="Positive title (optional)" important=true />
                        <SampleNotice kind="warning" title="Warning title (optional)" important=true />
                    </DtStack>
                </Variant>
                <Variant title="Truncate text">
                    <SampleNotice
                        kind="base"
                        title="Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
                        truncate_text=true
                        message="Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur."
                    />
                </Variant>
            </div>
        </section>
    }
}
