use super::*;

const CHIP_SIZES: &[(&str, &str)] = &[
    ("100", "100"),
    ("200", "200"),
    ("300", "300"),
    ("xs", "xs · alias of 100"),
    ("sm", "sm · alias of 200"),
    ("md", "md · alias of 300"),
];

const CHIP_ICONS: &[(&str, &str)] = &[("", "None"), ("phone", "Phone")];

const CHIP_AVATARS: &[(&str, &str)] = &[("", "None"), ("photo", "Photo"), ("initials", "Initials")];

fn chip_icon(name: &str) -> Option<ViewFn> {
    match name {
        "phone" => Some(ViewFn::from(|| view! { <DtIconPhone size="200" /> })),
        _ => None,
    }
}

fn chip_avatar(kind: &str) -> Option<ViewFn> {
    match kind {
        "photo" => Some(ViewFn::from(|| {
            view! {
                <DtAvatar
                    size="200"
                    image_src="/assets/images/person.png"
                    image_alt="Jaqueline Nackos".to_string()
                    full_name="Jaqueline Nackos"
                />
            }
        })),
        "initials" => Some(ViewFn::from(|| {
            view! { <DtAvatar size="200" full_name="Jaqueline Nackos" /> }
        })),
        _ => None,
    }
}

#[component]
pub(crate) fn ChipPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("chip.rs")));
    let label = RwSignal::new("Chip".to_string());
    let id = RwSignal::new(String::new());
    let aria_label = RwSignal::new(String::new());
    let size = RwSignal::new("300");
    let interactive = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    let show_close = RwSignal::new(true);
    let icon = RwSignal::new("");
    let avatar = RwSignal::new("");
    let removed = RwSignal::new(false);
    let clicks = RwSignal::new(0u32);
    let root_class = RwSignal::new(String::new());
    let label_class = RwSignal::new(String::new());
    let content_class = RwSignal::new(String::new());
    let icon_class = RwSignal::new(String::new());
    let avatar_class = RwSignal::new(String::new());
    let icon_off = Signal::derive(move || icon.get() != "phone");
    let avatar_off = Signal::derive(move || avatar.get().is_empty());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let label = label.get();
                let size = size.get();
                let interactive = interactive.get();
                let disabled = disabled.get();
                let show_close = show_close.get();
                let root_class = root_class.get();
                let label_class = label_class.get();
                let content_class = content_class.get();
                let icon_class = icon_class.get();
                let avatar_class = avatar_class.get();
                let id = id.get();
                let aria_label = aria_label.get();
                if removed.get() {
                    view! {
                        <DtButton size="sm" importance="outlined" on_click=move || removed.set(false)>
                            "Reset chip"
                        </DtButton>
                    }.into_any()
                } else {
                    let chip = open_two!(DtChip {
                        id=id
                        aria_label=aria_label
                        size=size
                        interactive=interactive
                        disabled=disabled
                        show_close=show_close
                        class=root_class
                        label_class=label_class
                        content_class=content_class
                        icon_class=icon_class
                        avatar_class=avatar_class
                        on_close=move || removed.set(true)
                        on_click=move || clicks.update(|count| *count += 1)
                    } {
                        {label}
                    }, chip_icon(icon.get()), icon, chip_avatar(avatar.get()), avatar);
                    view! {
                        <div class="docs-inline">
                            {chip}
                            <Show when=move || interactive>
                                <span class="docs-readout">
                                    {move || format!("Clicked {}", clicks.get())}
                                </span>
                            </Show>
                        </div>
                    }.into_any()
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Id" value=id />
            <SettingText label="Aria label" value=aria_label />
            <SettingSelect label="Size" value=size options=CHIP_SIZES.to_vec() />
            <SettingCheck label="Interactive" value=interactive />
            <SettingCheck label="Disabled" value=disabled />
            <SettingCheck label="Show close" value=show_close />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "id", &id.get());
        push_str(&mut props, "aria_label", &aria_label.get());
        if size.get() != "300" {
            push_str(&mut props, "size", size.get());
        }
        push_bool(&mut props, "interactive", interactive.get());
        push_bool(&mut props, "disabled", disabled.get());
        if !show_close.get() {
            props.push(("show_close", "false".to_string()));
        }
        push_str(&mut props, "class", &root_class.get());
        push_str(&mut props, "label_class", &label_class.get());
        push_str(&mut props, "content_class", &content_class.get());
        if icon.get() == "phone" {
            push_expr(
                &mut props,
                "icon",
                "ViewFn::from(|| view! { <DtIconPhone size=\"200\" /> })",
            );
            push_str(&mut props, "icon_class", &icon_class.get());
        }
        if !avatar.get().is_empty() {
            let expr = if avatar.get() == "photo" {
                "ViewFn::from(|| view! { <DtAvatar size=\"200\" image_src=\"/assets/images/person.png\" image_alt=\"Jaqueline Nackos\".to_string() full_name=\"Jaqueline Nackos\" /> })"
            } else {
                "ViewFn::from(|| view! { <DtAvatar size=\"200\" full_name=\"Jaqueline Nackos\" /> })"
            };
            push_expr(&mut props, "avatar", expr);
            push_str(&mut props, "avatar_class", &avatar_class.get());
        }
        if show_close.get() {
            push_expr(&mut props, "on_close", "move || { /* dismiss */ }");
        }
        if interactive.get() {
            push_expr(&mut props, "on_click", "move || { /* click */ }");
        }
        view_of(&element("DtChip", &props, Some(&rust_string(&label.get()))))
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingText label="Default" hint="Chip label." value=label />
            <SettingSelect label="Icon" value=icon options=CHIP_ICONS.to_vec() />
            <SettingSelect label="Avatar" value=avatar options=CHIP_AVATARS.to_vec() />
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
            label: "Label class",
            hint: "Extra class on the chip label.",
            value: label_class,
            off: None,
        },
        ClassField {
            label: "Content class",
            hint: "Extra class on the text. Use d-w-150 with a truncating child.",
            value: content_class,
            off: None,
        },
        ClassField {
            label: "Icon class",
            hint: "Extra class on the icon wrapper.",
            value: icon_class,
            off: Some(icon_off),
        },
        ClassField {
            label: "Avatar class",
            hint: "Extra class on the avatar wrapper.",
            value: avatar_class,
            off: Some(avatar_off),
        },
    ]);
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"Close is on by default. Delete on the chip runs the same on_close. Interactive turns the label into a button. Disabled sets aria-disabled and blocks pointer events. The icon slot wins over the avatar slot. xs, sm, and md are aliases of 100, 200, and 300."</p>
            <Play preview=preview settings=settings slots=slots classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Sizes" row=true>
                    <DtChip size="100">"100"</DtChip>
                    <DtChip size="200">"200"</DtChip>
                    <DtChip size="300">"300"</DtChip>
                </Variant>
                <Variant title="Interactive">
                    <DtChip interactive=true>"Chip"</DtChip>
                </Variant>
                <Variant title="Disabled">
                    <DtChip disabled=true>"Chip"</DtChip>
                </Variant>
                <Variant title="Without close button">
                    <DtChip show_close=false>"Chip"</DtChip>
                </Variant>
                <Variant title="With icon">
                    <DtChip show_close=false icon=chip_icon("phone").unwrap()>"Chip"</DtChip>
                </Variant>
                <Variant title="With icon and close button">
                    <DtChip icon=chip_icon("phone").unwrap()>"Chip"</DtChip>
                </Variant>
                <Variant title="With avatar" row=true>
                    <DtChip avatar=chip_avatar("photo").unwrap()>"Chip"</DtChip>
                    <DtChip avatar=chip_avatar("initials").unwrap()>"Chip"</DtChip>
                </Variant>
                <Variant title="Truncated">
                    <DtChip content_class="d-w-150">
                        <DtText truncate=true>"Chip loooooong name here"</DtText>
                    </DtChip>
                </Variant>
            </div>
        </section>
    }
}
