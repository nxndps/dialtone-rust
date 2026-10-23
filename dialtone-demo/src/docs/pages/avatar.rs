use super::*;

const AVATAR_SIZES: &[(&str, &str)] = &[
    ("100", "100 · 16px"),
    ("150", "150 · 20px"),
    ("200", "200 · 24px"),
    ("250", "250 · 28px"),
    ("300", "300 · 32px"),
    ("400", "400 · 40px"),
    ("500", "500 · 48px"),
    ("600", "600 · 64px"),
    ("700", "700 · 96px"),
    ("800", "800 · 128px"),
    ("900", "900 · 256px"),
    ("xs", "xs · alias of 100"),
    ("sm", "sm · alias of 200"),
    ("md", "md · alias of 300"),
    ("lg", "lg · alias of 500"),
    ("xl", "xl · alias of 600"),
];

#[component]
pub(crate) fn AvatarPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("avatar.rs")));
    let full_name = RwSignal::new("Ada Lovelace".to_string());
    let size = RwSignal::new("300");
    let color = RwSignal::new("");
    let family = RwSignal::new("");
    let variant = RwSignal::new("");
    let seed = RwSignal::new("ada".to_string());
    let presence = RwSignal::new("active");
    let group = RwSignal::new("0");
    let overlay = RwSignal::new(String::new());
    let image_src = RwSignal::new(String::new());
    let image_alt = RwSignal::new("Ada Lovelace".to_string());
    let use_logo = RwSignal::new(false);
    let icon = RwSignal::new("");
    let overlay_icon = RwSignal::new("");
    let avatar_class = RwSignal::new(String::new());
    let canvas_class = RwSignal::new(String::new());
    let icon_class = RwSignal::new(String::new());
    let overlay_class = RwSignal::new(String::new());
    let icon_off = Signal::derive(move || icon.get().is_empty());
    let overlay_off =
        Signal::derive(move || overlay_icon.get().is_empty() && overlay.get().is_empty());
    let icon_only = RwSignal::new(false);
    let interactive = RwSignal::new(false);
    let clickable = RwSignal::new(false);
    let deactivated = RwSignal::new(false);
    let id = RwSignal::new(String::new());
    let icon_aria_label = RwSignal::new(String::new());
    let clicks = RwSignal::new(0u32);
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let custom = image_src.get();
                let src = if !custom.is_empty() {
                    custom
                } else if use_logo.get() {
                    "/dialtone/illustrations/dialpad-logo.svg".to_string()
                } else {
                    String::new()
                };
                let icon_name = icon.get();
                let overlay_icon_name = overlay_icon.get();
                let name = full_name.get();
                let size = size.get();
                let presence = presence.get();
                let color = color.get();
                let family = family.get();
                let variant = variant.get();
                let seed = seed.get();
                let overlay = overlay.get();
                let alt = image_alt.get();
                let is_interactive = interactive.get();
                let is_clickable = clickable.get();
                let is_icon_only = icon_only.get();
                let is_deactivated = deactivated.get();
                let id = id.get();
                let group = group.get().parse::<i32>().unwrap_or(0);
                let avatar_class_value = avatar_class.get();
                let canvas_class_value = canvas_class.get();
                let icon_class_value = if icon_name.is_empty() {
                    String::new()
                } else {
                    icon_class.get()
                };
                let overlay_class_value = if overlay_icon_name.is_empty() && overlay.is_empty() {
                    String::new()
                } else {
                    overlay_class.get()
                };
                let custom_aria = icon_aria_label.get();
                let icon_label = if !custom_aria.is_empty() {
                    custom_aria
                } else {
                    icon_label(icon_name).to_string()
                };
                let avatar = void_two!(DtAvatar {
                    id=id
                    full_name=name
                    size=size
                    presence=presence
                    color=color
                    family=family
                    variant=variant
                    seed=seed
                    overlay_text=overlay
                    image_src=src
                    image_alt=alt
                    interactive=is_interactive
                    clickable=is_clickable
                    icon_only=is_icon_only
                    deactivated=is_deactivated
                    group=group
                    avatar_class=avatar_class_value
                    canvas_class=canvas_class_value
                    icon_class=icon_class_value
                    overlay_class=overlay_class_value
                    icon_aria_label=icon_label
                    on_click=move || clicks.update(|count| *count += 1)
                }, icon_slot(icon_name), icon, icon_slot(overlay_icon_name), overlay_icon);
                view! {
                    <div class="docs-inline">
                        {avatar}
                        <Show when=move || interactive.get() || clickable.get()>
                            <span class="docs-readout">
                                {move || format!("Clicked {}", clicks.get())}
                            </span>
                        </Show>
                    </div>
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Id" value=id />
            <SettingText label="Full name" value=full_name />
            <SettingSelect label="Size" value=size options=AVATAR_SIZES.to_vec() />
            <SettingText label="Seed" value=seed />
            <SettingSelect
                label="Family"
                value=family
                options=vec![
                    ("", "From seed"),
                    ("1", "1 Red"),
                    ("2", "2 Orange"),
                    ("3", "3 Amber"),
                    ("4", "4 Yellow-green"),
                    ("5", "5 Green"),
                    ("6", "6 Teal"),
                    ("7", "7 Cyan"),
                    ("8", "8 Blue"),
                    ("9", "9 Indigo"),
                    ("10", "10 Purple"),
                    ("11", "11 Magenta"),
                    ("12", "12 Pink"),
                ]
            />
            <SettingSelect
                label="Variant"
                value=variant
                options=vec![
                    ("", "From seed"),
                    ("0", "0 Darkest"),
                    ("1", "1"),
                    ("2", "2"),
                    ("3", "3"),
                    ("4", "4"),
                    ("5", "5"),
                    ("6", "6"),
                    ("7", "7"),
                    ("8", "8"),
                    ("9", "9 Lightest"),
                ]
            />
            <SettingSelect
                label="Color code (Deprecated)"
                value=color
                options=vec![
                    ("", "Unused"),
                    ("180", "180"),
                    ("500", "500"),
                    ("540", "540"),
                    ("800", "800"),
                    ("1200", "1200"),
                ]
            />
            <SettingSelect
                label="Presence"
                value=presence
                options=vec![
                    ("", "None"),
                    ("active", "Active"),
                    ("away", "Away"),
                    ("busy", "Busy"),
                    ("dnd", "Do not disturb"),
                    ("offline", "Offline"),
                ]
            />
            <SettingSelect
                label="Group count"
                value=group
                options=vec![
                    ("0", "None"),
                    ("1", "1"),
                    ("3", "3"),
                    ("12", "12"),
                    ("24", "24"),
                    ("120", "120"),
                ]
            />
            <SettingText label="Overlay text" value=overlay />
            <SettingText label="Image URL" value=image_src />
            <SettingText label="Image alt" value=image_alt />
            <SettingText label="Icon aria label" value=icon_aria_label />
            <SettingCheck label="Dialpad logo" value=use_logo />
            <SettingCheck label="Icon only" value=icon_only />
            <SettingCheck label="Interactive" value=interactive />
            <SettingCheck label="Clickable (Deprecated)" value=clickable />
            <SettingCheck label="Deactivated" value=deactivated />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "id", &id.get());
        push_str(&mut props, "full_name", &full_name.get());
        push_str(&mut props, "size", size.get());
        push_str(&mut props, "presence", presence.get());
        push_str(&mut props, "color", color.get());
        push_str(&mut props, "family", family.get());
        push_str(&mut props, "variant", variant.get());
        push_str(&mut props, "seed", &seed.get());
        push_str(&mut props, "overlay_text", &overlay.get());
        let src = if !image_src.get().is_empty() {
            image_src.get()
        } else if use_logo.get() {
            "/dialtone/illustrations/dialpad-logo.svg".to_string()
        } else {
            String::new()
        };
        push_str(&mut props, "image_src", &src);
        if !src.is_empty() {
            push_str(&mut props, "image_alt", &image_alt.get());
        }
        push_bool(&mut props, "interactive", interactive.get());
        push_bool(&mut props, "clickable", clickable.get());
        push_bool(&mut props, "icon_only", icon_only.get());
        push_bool(&mut props, "deactivated", deactivated.get());
        let count = group.get().parse::<i32>().unwrap_or(0);
        if count > 0 {
            push_expr(&mut props, "group", count.to_string());
        }
        push_str(&mut props, "avatar_class", &avatar_class.get());
        push_str(&mut props, "canvas_class", &canvas_class.get());
        if !icon_aria_label.get().is_empty() {
            push_str(&mut props, "icon_aria_label", &icon_aria_label.get());
        } else if !icon.get().is_empty() {
            push_str(&mut props, "icon_aria_label", icon_label(icon.get()));
        }
        if !icon.get().is_empty() {
            push_str(&mut props, "icon_class", &icon_class.get());
            push_icon(&mut props, "icon", icon.get());
        }
        if !overlay_icon.get().is_empty() {
            push_icon(&mut props, "overlay_icon", overlay_icon.get());
        }
        if !overlay.get().is_empty() || !overlay_icon.get().is_empty() {
            push_str(&mut props, "overlay_class", &overlay_class.get());
        }
        view_of(&element("DtAvatar", &props, None))
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingSelect label="Icon" value=icon options=ICONS.to_vec() />
            <SettingSelect label="Overlay icon" value=overlay_icon options=ICONS.to_vec() />
        }
    });
    let classes = class_panel(vec![
        ClassField {
            label: "Class",
            hint: ROOT_CLASS,
            value: avatar_class,
            off: None,
        },
        ClassField {
            label: "Canvas class",
            hint: "Extra class on the canvas.",
            value: canvas_class,
            off: None,
        },
        ClassField {
            label: "Icon class",
            hint: "Extra class on the icon. Needs the icon slot.",
            value: icon_class,
            off: Some(icon_off),
        },
        ClassField {
            label: "Overlay class",
            hint: "Extra class on the overlay. Needs overlay text or an overlay icon.",
            value: overlay_class,
            off: Some(overlay_off),
        },
    ]);
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">
                "The avatar is a rounded square. Sizes run from 100 to 900. xs through xl still select the matching step. A group count above 1 adds a badge and keeps the size. Size 100 shows only that badge. Presence, including do not disturb, is drawn through size 500. Seed, family, and variant choose the color."
            </p>
            <Play preview=preview settings=settings slots=slots classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Icon" row=true>
                    <DtAvatar icon=ViewFn::from(|| view! { <DtIconUser /> }) />
                </Variant>
                <Variant title="Initials" row=true>
                    <DtAvatar full_name="Ada Lovelace" seed="ada" />
                    <DtAvatar full_name="Grace Hopper" family="8" variant="4" />
                    <DtAvatar full_name="Alan Turing" color="1200" />
                </Variant>
                <Variant title="Image" row=true>
                    <DtAvatar
                        image_src="/dialtone/illustrations/dialpad-logo.svg"
                        image_alt="Dialpad".to_string()
                        size="500"
                    />
                </Variant>
                <Variant title="Sizes" row=true>
                    <DtAvatar
                        full_name="Ada Lovelace"
                        seed="ada"
                        size="100"
                        image_src="/assets/images/person.png"
                        image_alt="Ada Lovelace".to_string()
                    />
                    <DtAvatar
                        full_name="Ada Lovelace"
                        seed="ada"
                        size="200"
                        image_src="/assets/images/person.png"
                        image_alt="Ada Lovelace".to_string()
                    />
                    <DtAvatar
                        full_name="Ada Lovelace"
                        seed="ada"
                        size="300"
                        image_src="/assets/images/person.png"
                        image_alt="Ada Lovelace".to_string()
                    />
                    <DtAvatar
                        full_name="Ada Lovelace"
                        seed="ada"
                        size="500"
                        image_src="/assets/images/person.png"
                        image_alt="Ada Lovelace".to_string()
                    />
                    <DtAvatar
                        full_name="Ada Lovelace"
                        seed="ada"
                        size="700"
                        image_src="/assets/images/person.png"
                        image_alt="Ada Lovelace".to_string()
                    />
                </Variant>
                <Variant title="Group" row=true>
                    <DtAvatar full_name="On call" seed="on-call" size="100" group=3 />
                    <DtAvatar full_name="On call" seed="on-call" size="300" group=24 />
                    <DtAvatar full_name="On call" seed="on-call" size="500" group=120 />
                </Variant>
                <Variant title="Presence" row=true>
                    <DtAvatar
                        full_name="Ada Lovelace"
                        seed="ada"
                        presence="active"
                        image_src="/assets/images/person.png"
                        image_alt="Ada Lovelace".to_string()
                    />
                    <DtAvatar
                        full_name="Ada Lovelace"
                        seed="ada"
                        presence="away"
                        image_src="/assets/images/person.png"
                        image_alt="Ada Lovelace".to_string()
                    />
                    <DtAvatar
                        full_name="Ada Lovelace"
                        seed="ada"
                        presence="busy"
                        image_src="/assets/images/person.png"
                        image_alt="Ada Lovelace".to_string()
                    />
                    <DtAvatar
                        full_name="Ada Lovelace"
                        seed="ada"
                        presence="dnd"
                        image_src="/assets/images/person.png"
                        image_alt="Ada Lovelace".to_string()
                    />
                    <DtAvatar
                        full_name="Ada Lovelace"
                        seed="ada"
                        presence="offline"
                        image_src="/assets/images/person.png"
                        image_alt="Ada Lovelace".to_string()
                    />
                </Variant>
                <Variant title="Overlay" row=true>
                    <DtAvatar
                        full_name="Ada Lovelace"
                        seed="ada"
                        size="400"
                        overlay_text="+3"
                        image_src="/assets/images/person.png"
                        image_alt="Ada Lovelace".to_string()
                    />
                </Variant>
                <Variant title="Interactive" row=true>
                    <DtAvatar
                        interactive=true
                        icon_aria_label="User"
                        icon=ViewFn::from(|| view! { <DtIconUser /> })
                    />
                </Variant>
                <Variant title="Deactivated" row=true>
                    <DtAvatar
                        full_name="Ada Lovelace"
                        seed="ada"
                        deactivated=true
                        image_src="/assets/images/person.png"
                        image_alt="Ada Lovelace".to_string()
                    />
                    <DtAvatar
                        deactivated=true
                        icon_only=true
                        icon=ViewFn::from(|| view! { <DtIconUser /> })
                    />
                </Variant>
            </div>
        </section>
    }
}
