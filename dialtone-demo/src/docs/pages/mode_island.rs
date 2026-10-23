use super::*;

const MODES: &[(&str, &str)] = &[
    ("inverted", "Inverted"),
    ("light", "Light"),
    ("dark", "Dark"),
];

const TAGS: &[(&str, &str)] = &[
    ("div", "div"),
    ("section", "section"),
    ("nav", "nav"),
    ("article", "article"),
    ("aside", "aside"),
    ("header", "header"),
    ("footer", "footer"),
    ("main", "main"),
];

fn static_mode(value: &str) -> &'static str {
    match value {
        "light" => "light",
        "dark" => "dark",
        _ => "inverted",
    }
}

fn mode_label(mode: &str) -> &'static str {
    match mode {
        "light" => "light mode",
        "dark" => "dark mode",
        _ => "inverted mode",
    }
}

fn mode_swatch(mode: &'static str, title: &'static str) -> impl IntoView {
    view! {
        <DtStack gap="100" class="d-fl1">
            <DtText r#as="h3" kind="headline" size="200">
                {title}" "
                <DtText strength="normal">"(auto)"</DtText>
            </DtText>
            <DtModeIsland mode=mode class="d-p-200 d-bar-400 d-ba d-bc-subtle">
                <DtStack gap="100">
                    <DtStack direction="row" gap="100" align="center">
                        <span class="d-fc-positive">
                            <DtIconCheckCircle size="200" />
                        </span>
                        <DtText r#as="p" kind="body" size="100">"Primary"</DtText>
                        <DtText r#as="p" kind="body" size="100" tone="muted">"Muted"</DtText>
                        <DtText r#as="p" kind="body" size="100" tone="critical">"Critical"</DtText>
                        <DtText r#as="p" kind="body" size="100">
                            <DtLink href="#link">"Link"</DtLink>
                        </DtText>
                    </DtStack>
                    <DtStack direction="row" gap="100">
                        <DtButton size="100" class="d-fl1">"Button"</DtButton>
                        <DtButton size="100" class="d-fl1" kind="critical">"Button"</DtButton>
                    </DtStack>
                </DtStack>
            </DtModeIsland>
        </DtStack>
    }
}

fn profile_card() -> impl IntoView {
    view! {
        <div class="d-w-350">
            <DtText r#as="h3" kind="headline" size="400">"Katie Rodriguez"</DtText>
            <DtStack direction="row" gap="50" align="baseline">
                <DtText r#as="p" kind="body" tone="positive" size="100">"Available"</DtText>
                <DtText r#as="p" kind="body" tone="muted" size="100">"· Working from coffee shop"</DtText>
            </DtStack>
            <DtText r#as="p" kind="body" size="200">"Chief Customer Success Officer"</DtText>
            <DtText r#as="p" kind="body" tone="muted" size="100">"6:19 am local time"</DtText>
            <DtStack direction="row" gap="100" class="d-mt-200">
                <DtButton size="100" importance="outlined" kind="muted">"Call"</DtButton>
                <DtButton size="100" importance="outlined" kind="muted">"Message"</DtButton>
                <DtButton size="100" importance="outlined" kind="muted">"Meet"</DtButton>
            </DtStack>
        </div>
    }
}

fn outlined_anchor(label: &'static str) -> ViewFn {
    ViewFn::from(move || {
        view! {
            <DtButton size="200" kind="muted" importance="outlined">{label}</DtButton>
        }
    })
}

fn menu_anchor(label: &'static str) -> ViewFn {
    ViewFn::from(move || {
        view! {
            <DtButton
                size="200"
                kind="muted"
                importance="outlined"
                end_icon=Some(ViewFn::from(|| view! { <DtIconChevronDown size="200" /> }))
            >
                {label}
            </DtButton>
        }
    })
}

fn menu_list() -> impl IntoView {
    view! {
        <DtListItem role="menuitem" navigation_type="arrow-keys">"Profile"</DtListItem>
        <DtListItem role="menuitem" navigation_type="arrow-keys">"Status"</DtListItem>
        <DtListItem role="menuitem" navigation_type="arrow-keys">"Sign out"</DtListItem>
    }
}

fn surface_caption(text: &'static str) -> impl IntoView {
    view! {
        <DtStack gap="200">
            <DtText r#as="p" kind="code" size="100" tone="tertiary">{text}</DtText>
            <div>
                <DtButton>"Button"</DtButton>
            </div>
        </DtStack>
    }
}

#[component]
pub(crate) fn ModeIslandPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("mode_island.rs")));
    let mode = RwSignal::new("inverted");
    let tag = RwSignal::new("div");
    let slot_text = RwSignal::new("Primary surface".to_string());
    let root_class = RwSignal::new("d-p-200 d-bar-400".to_string());
    let dynamic_mode = RwSignal::new("inverted".to_string());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let mode = mode.get();
                let tag = tag.get();
                let class_name = root_class.get();
                let label = slot_text.get();
                view! {
                    <DtModeIsland r#as=tag mode=mode class=class_name>
                        {(!label.is_empty()).then(|| {
                            let label = label.clone();
                            view! { <DtText r#as="p" tone="positive">{label}</DtText> }
                        })}
                        <DtButton size="100">"Button"</DtButton>
                    </DtModeIsland>
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingSelect label="Mode" value=mode options=MODES.to_vec() />
            <SettingSelect label="As" value=tag options=TAGS.to_vec() />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        if mode.get() != "inverted" {
            push_str(&mut props, "mode", mode.get());
        }
        if tag.get() != "div" {
            push_str(&mut props, "r#as", tag.get());
        }
        push_str(&mut props, "class", &root_class.get());
        let label = slot_text.get();
        let text = (!label.is_empty()).then(|| {
            format!(
                "<DtText r#as=\"p\" tone=\"positive\">{}</DtText>",
                rust_string(&label)
            )
        });
        let child = match text {
            Some(text) => format!("{text}\n<DtButton size=\"100\">\"Button\"</DtButton>"),
            None => "<DtButton size=\"100\">\"Button\"</DtButton>".to_string(),
        };
        view_of(&element("DtModeIsland", &props, Some(&child)))
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingText
                label="Default"
                hint="Slot for main content. Empty hides the text."
                value=slot_text
            />
        }
    });
    let classes = class_panel(vec![ClassField {
        label: "Class",
        hint: ROOT_CLASS,
        value: root_class,
        off: None,
    }]);
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">
                "The default slot is the text inside the island. Inverted follows a parent island, then the nearest data-dt-mode ancestor, then the document element. Light and dark stay fixed. The examples use DtModeIsland. v-dt-mode is not ported. Positioned content uses content_mode."
            </p>
            <Play preview=preview settings=settings slots=slots classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Color modes">
                    <DtStack direction="row" gap="200" align="stretch" class="d-fw-wrap">
                        {mode_swatch("inverted", "Inverted")}
                        {mode_swatch("dark", "Dark")}
                        {mode_swatch("light", "Light")}
                    </DtStack>
                </Variant>
                <Variant title="Inverting" row=true>
                    <DtButton>"Button"</DtButton>
                    <DtModeIsland class="d-p-200 d-bar-400">
                        <DtButton>"Button"</DtButton>
                    </DtModeIsland>
                </Variant>
                <Variant title="Dynamic mode">
                    <DtStack gap="200">
                        <DtSegmentedControl
                            aria_label="Mode"
                            size="100"
                            value=dynamic_mode
                            on_change=move |next| dynamic_mode.set(next)
                        >
                            <DtSegmentedControlItem value="inverted">"Invert"</DtSegmentedControlItem>
                            <DtSegmentedControlItem
                                value="light"
                                start_icon=ViewFn::from(|| view! { <DtIconSun size="200" /> })
                            >
                                "Light"
                            </DtSegmentedControlItem>
                            <DtSegmentedControlItem
                                value="dark"
                                start_icon=ViewFn::from(|| view! { <DtIconMoon size="200" /> })
                            >
                                "Dark"
                            </DtSegmentedControlItem>
                        </DtSegmentedControl>
                        {move || {
                            let mode = static_mode(&dynamic_mode.get());
                            let label = mode_label(mode);
                            view! {
                                <DtModeIsland mode=mode class="d-p-200 d-bar-400 d-w100p">
                                    <DtText r#as="p" tone="positive" align="center">{label}</DtText>
                                </DtModeIsland>
                            }
                        }}
                    </DtStack>
                </Variant>
                <Variant title="Nesting">
                    <DtModeIsland mode="light" class="d-p-200 d-bar-400 d-bgc-secondary d-ba">
                        <DtStack gap="200">
                            <DtText r#as="p" tone="positive" text_box_trim="both">"Explicit Light"</DtText>
                            <DtModeIsland class="d-p-200 d-bar-400 d-bgc-secondary">
                                <DtStack gap="200">
                                    <DtText r#as="p" tone="positive" text_box_trim="both">"Inverted (Dark)"</DtText>
                                    <DtModeIsland class="d-p-200 d-bar-300 d-bgc-secondary">
                                        <DtText r#as="p" tone="positive" text_box_trim="both">"Inverted again (Light)"</DtText>
                                    </DtModeIsland>
                                </DtStack>
                            </DtModeIsland>
                        </DtStack>
                    </DtModeIsland>
                </Variant>
                <Variant title="Custom background">
                    <DtStack gap="200">
                        <DtModeIsland class="d-p-200 d-bar-400 d-w100p">
                            {surface_caption("Component: automatic primary surface, inverted mode")}
                        </DtModeIsland>
                        <DtModeIsland mode="dark" class="d-p-200 d-bar-400 d-w100p d-bgc-critical">
                            {surface_caption("Explicit critical background, dark mode")}
                        </DtModeIsland>
                        <DtModeIsland mode="light" class="d-p-200 d-bar-400 d-w100p d-bgc-critical">
                            {surface_caption("Explicit critical background, light mode")}
                        </DtModeIsland>
                    </DtStack>
                </Variant>
                <Variant title="Callbar">
                    <DtModeIsland class="d-ba d-bc-subtle d-bgc-secondary d-p-100 d-pis-125 d-py-100 d-bar-450 d-bs-md d-w100p">
                        <DtStack direction="row" gap="400" align="center" class="d-fw-wrap">
                            <DtStack direction="row" gap="100" align="center">
                                <DtAvatar full_name="TA" seed="ted-anderson" size="400" />
                                <DtStack gap="50">
                                    <DtText kind="label" size="300" density="200">"Ted Anderson"</DtText>
                                    <DtStack direction="row" gap="50" align="baseline">
                                        <DtText kind="body" size="100" tone="tertiary" wrap="nowrap" numeric=true>"(913) 555-6745"</DtText>
                                        <DtText kind="body" size="100" tone="muted">"·"</DtText>
                                        <DtText kind="body" size="100" tone="tertiary" numeric=true>"21:18"</DtText>
                                    </DtStack>
                                </DtStack>
                            </DtStack>
                            <DtStack class="d-fl1" direction="row" gap="50" justify="center">
                                <DtButton size="100" kind="critical" class="d-px-100">"Unmute"</DtButton>
                                <DtButton size="100" kind="muted" importance="clear" class="d-px-100">"Record"</DtButton>
                                <DtButton size="100" kind="muted" importance="clear" class="d-px-100">"Keypad"</DtButton>
                                <DtButton
                                    size="100"
                                    kind="muted"
                                    importance="clear"
                                    class="d-px-100"
                                    icon_position="blockStart"
                                    icon=ViewFn::from(|| view! { <DtIconUser /> })
                                >
                                    "Add"
                                </DtButton>
                                <DtButton
                                    size="100"
                                    kind="muted"
                                    importance="clear"
                                    class="d-px-100"
                                    icon_position="blockStart"
                                    icon=ViewFn::from(|| view! { <DtIconMoreHorizontal size="300" /> })
                                >
                                    "More"
                                </DtButton>
                            </DtStack>
                            <DtButton
                                class="d-p-150"
                                size="400"
                                kind="critical"
                                aria_label="Hang up"
                                icon=ViewFn::from(|| view! { <DtIconPhone size="500" /> })
                            />
                        </DtStack>
                    </DtModeIsland>
                    <div class="d-mbs-100">
                        <DtText r#as="p" kind="label" size="200" tone="muted">"* Not real, still just an example"</DtText>
                    </div>
                </Variant>
                <Variant title="Positioned components">
                    <DtStack gap="200">
                        <DtStack gap="100">
                            <DtText r#as="h4" kind="headline" size="300">"Hovercard"</DtText>
                            <DtStack direction="row" gap="100" class="d-fw-wrap">
                                <DtHovercard placement="top-start" enter_delay=0 leave_delay=0 anchor=outlined_anchor("Default")>
                                    {profile_card()}
                                </DtHovercard>
                                <DtHovercard placement="top-start" content_mode="invert" enter_delay=0 leave_delay=0 anchor=outlined_anchor("Inverted")>
                                    {profile_card()}
                                </DtHovercard>
                                <DtHovercard placement="top-start" content_mode="light" enter_delay=0 leave_delay=0 anchor=outlined_anchor("Light")>
                                    {profile_card()}
                                </DtHovercard>
                                <DtHovercard placement="top-start" content_mode="dark" enter_delay=0 leave_delay=0 anchor=outlined_anchor("Dark")>
                                    {profile_card()}
                                </DtHovercard>
                            </DtStack>
                        </DtStack>
                        <DtStack gap="100">
                            <DtText r#as="h4" kind="headline" size="300">"Popover"</DtText>
                            <DtStack direction="row" gap="100" class="d-fw-wrap">
                                <DtPopover
                                    placement="top-start"
                                    modal=false
                                    dialog_class="d-w-350"
                                    anchor=outlined_anchor("Default")
                                >
                                    <DtText r#as="p">"This is just a default Popover, and does not use Mode Island."</DtText>
                                </DtPopover>
                                <DtPopover
                                    placement="top-start"
                                    content_mode="invert"
                                    modal=false
                                    dialog_class="d-w-350"
                                    anchor=outlined_anchor("Inverted")
                                >
                                    <DtText r#as="p">
                                        "This Popover's content is in the "
                                        <DtText strength="bold">"inverted"</DtText>
                                        " mode."
                                    </DtText>
                                </DtPopover>
                                <DtPopover
                                    placement="top-start"
                                    content_mode="light"
                                    modal=false
                                    dialog_class="d-w-350"
                                    anchor=outlined_anchor("Light")
                                >
                                    <DtText r#as="p">
                                        "This Popover's content is in explicit "
                                        <DtText strength="bold">"light"</DtText>
                                        " mode."
                                    </DtText>
                                </DtPopover>
                                <DtPopover
                                    placement="top-start"
                                    content_mode="dark"
                                    modal=false
                                    dialog_class="d-w-350"
                                    anchor=outlined_anchor("Dark")
                                >
                                    <DtText r#as="p">
                                        "This Popover's content is in explicit "
                                        <DtText strength="bold">"dark"</DtText>
                                        " mode."
                                    </DtText>
                                </DtPopover>
                            </DtStack>
                        </DtStack>
                        <DtStack gap="100">
                            <DtText r#as="h4" kind="headline" size="300">"Dropdown"</DtText>
                            <DtStack direction="row" gap="100" class="d-fw-wrap">
                                <DtDropdown placement="bottom-start" modal=false anchor=menu_anchor("Default")>
                                    {menu_list()}
                                </DtDropdown>
                                <DtDropdown placement="bottom-start" content_mode="invert" modal=false anchor=menu_anchor("Inverted")>
                                    {menu_list()}
                                </DtDropdown>
                                <DtDropdown placement="bottom-start" content_mode="light" modal=false anchor=menu_anchor("Light")>
                                    {menu_list()}
                                </DtDropdown>
                                <DtDropdown placement="bottom-start" content_mode="dark" modal=false anchor=menu_anchor("Dark")>
                                    {menu_list()}
                                </DtDropdown>
                            </DtStack>
                        </DtStack>
                    </DtStack>
                </Variant>
                <Variant title="Elements" row=true>
                    <DtModeIsland r#as="section" class="d-p-200 d-bar-400">
                        <DtText r#as="p">"section"</DtText>
                    </DtModeIsland>
                    <DtModeIsland r#as="article" class="d-p-200 d-bar-400">
                        <DtText r#as="p">"article"</DtText>
                    </DtModeIsland>
                    <DtModeIsland r#as="nav" class="d-p-200 d-bar-400">
                        <DtText r#as="p">"nav"</DtText>
                    </DtModeIsland>
                </Variant>
            </div>
        </section>
    }
}
