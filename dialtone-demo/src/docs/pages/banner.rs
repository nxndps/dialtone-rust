use super::*;

#[component]
pub(crate) fn BannerPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("banner.rs")));
    let title = RwSignal::new("Rotation updated".to_string());
    let message = RwSignal::new("The primary on-call changed.".to_string());
    let kind = RwSignal::new("positive");
    let important = RwSignal::new(false);
    let pinned = RwSignal::new(false);
    let show_close = RwSignal::new(true);
    let show_icon = RwSignal::new(true);
    let visible = RwSignal::new(true);
    let icon = RwSignal::new("");
    let action = RwSignal::new(String::new());
    let dialog_class = RwSignal::new(String::new());
    let content_id = RwSignal::new(String::new());
    let background_image = RwSignal::new(String::new());
    let background_size = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let title = title.get();
                let message = message.get();
                let kind = kind.get();
                let important = important.get();
                let pinned = pinned.get();
                let show_close = show_close.get();
                let show_icon = show_icon.get();
                if visible.get() {
                    let dialog_class_value = dialog_class.get();
                    let content_id = content_id.get();
                    let background_image = background_image.get();
                    let background_size = {
                        let value = background_size.get();
                        if value.is_empty() {
                            "cover"
                        } else {
                            Box::leak(value.into_boxed_str())
                        }
                    };
                    open_two!(DtBanner {
                        header_text=title
                        content_id=content_id
                        kind=kind
                        important=important
                        pinned=pinned
                        show_close=show_close
                        show_icon=show_icon
                        dialog_class=dialog_class_value
                        background_image=background_image
                        background_size=background_size
                        on_close=move || visible.set(false)
                    } {
                        {message.clone()}
                    }, icon_slot(icon.get()), icon, action_slot(action.get()), action)
                } else {
                    view! {
                        <DtButton importance="outlined" size="sm" on_click=move || visible.set(true)>
                            "Show banner"
                        </DtButton>
                    }.into_any()
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Title" value=title />
            <SettingText label="Message" value=message />
            <SettingSelect
                label="Kind"
                value=kind
                options=vec![
                    ("base", "Base"),
                    ("info", "Info"),
                    ("positive", "Positive"),
                    ("warning", "Warning"),
                    ("critical", "Critical"),
                ]
            />
            <SettingCheck label="Important" value=important />
            <SettingCheck label="Pinned" value=pinned />
            <SettingCheck label="Show close" value=show_close />
            <SettingCheck label="Show icon" value=show_icon />
            <SettingText label="Content id" value=content_id />
            <SettingText label="Background image" value=background_image />
            <SettingText label="Background size" value=background_size />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "header_text", &title.get());
        push_str(&mut props, "kind", kind.get());
        push_bool(&mut props, "important", important.get());
        push_bool(&mut props, "pinned", pinned.get());
        if !show_close.get() {
            props.push(("show_close", "false".to_string()));
        }
        if !show_icon.get() {
            props.push(("show_icon", "false".to_string()));
        }
        push_str(&mut props, "content_id", &content_id.get());
        push_str(&mut props, "background_image", &background_image.get());
        push_str(&mut props, "background_size", &background_size.get());
        push_str(&mut props, "dialog_class", &dialog_class.get());
        push_icon(&mut props, "icon", icon.get());
        if !action.get().is_empty() {
            push_expr(
                &mut props,
                "action",
                format!(
                    "ViewFn::from(|| view! {{ <DtButton importance=\"outlined\" size=\"sm\">{}</DtButton> }})",
                    rust_string(&action.get())
                ),
            );
        }
        if show_close.get() {
            push_expr(&mut props, "on_close", "move || visible.set(false)");
        }
        view_of(&element(
            "DtBanner",
            &props,
            Some(&rust_string(&message.get())),
        ))
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingText label="Default" hint="Banner message." value=message />
            <SettingSelect label="Icon" value=icon options=ICONS.to_vec() />
            <SettingText label="Action" hint="Action slot. Empty hides it." value=action />
        }
    });
    let classes = class_panel(vec![ClassField {
        label: "Dialog class",
        hint: "Extra class on the dialog.",
        value: dialog_class,
        off: None,
    }]);
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"Banners are fixed to the viewport in the library. This preview keeps them in the stage."</p>
            <Play stack=true preview=preview settings=settings slots=slots classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Kinds">
                    <DtBanner header_text="Heads up" kind="info" show_close=false>"Changes apply on the next sync."</DtBanner>
                    <DtBanner header_text="Check the mapping" kind="warning" show_close=false>"Two schedules still need a target."</DtBanner>
                    <DtBanner header_text="Sync failed" kind="critical" show_close=false>"PagerDuty returned 503."</DtBanner>
                </Variant>
            </div>
        </section>
    }
}
