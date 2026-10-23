use super::*;

const PLACEMENT_OPTIONS: &[(&str, &str)] = &[
    ("top", "Top"),
    ("top-start", "Top start"),
    ("top-end", "Top end"),
    ("bottom", "Bottom"),
    ("bottom-start", "Bottom start"),
    ("bottom-end", "Bottom end"),
    ("left", "Left"),
    ("left-start", "Left start"),
    ("left-end", "Left end"),
    ("right", "Right"),
    ("right-start", "Right start"),
    ("right-end", "Right end"),
];

const STORY_ITEMS: &str =
    "Menu item 1, Menu item 2, Another menu item 1, Menu item 3, Another menu item 2";

fn item_labels(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(str::trim)
        .filter(|label| !label.is_empty())
        .map(str::to_string)
        .collect()
}

fn menu_items(labels: Vec<String>, navigation: &'static str) -> impl IntoView {
    labels
        .into_iter()
        .map(|label| {
            view! {
                <DtListItem role="menuitem" navigation_type=navigation>{label.clone()}</DtListItem>
            }
        })
        .collect_view()
}

macro_rules! dropdown_menu {
    (
        $placement:expr, $fallback:expr, $padding:expr, $append_to:expr,
        $content_width:expr, $content_mode:expr, $navigation:expr, $transition:expr,
        $context:expr, $modal:expr, $tether:expr, $max_height:expr, $max_width:expr,
        $list_class:expr, $footer_class:expr, $list_id:expr, $anchor:expr,
        $footer:expr, $labels:expr;
        $($extra:tt)*
    ) => {
        open_one!(DtDropdown {
            placement=$placement
            fallback_placements=$fallback
            padding=$padding
            append_to=$append_to
            content_width=$content_width
            content_mode=$content_mode
            navigation_type=$navigation
            transition=$transition
            open_on_context=$context
            modal=$modal
            tether=$tether
            max_height=$max_height
            max_width=$max_width
            list_class=$list_class
            footer_class=$footer_class
            list_id=$list_id
            anchor=$anchor
            $($extra)*
        } {
            {menu_items($labels.clone(), $navigation)}
        }, $footer, footer)
    };
}

fn static_items(labels: &'static [&'static str]) -> impl IntoView {
    labels
        .iter()
        .copied()
        .map(|label| {
            view! {
                <DtListItem role="menuitem" navigation_type="arrow-keys">{label}</DtListItem>
            }
        })
        .collect_view()
}

#[component]
pub(crate) fn DropdownPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("dropdown.rs")));
    let placement = RwSignal::new("bottom");
    let fallback = RwSignal::new("auto");
    let padding = RwSignal::new("small");
    let append_to = RwSignal::new("body");
    let content_width = RwSignal::new("");
    let content_mode = RwSignal::new("");
    let navigation = RwSignal::new("arrow-keys");
    let transition = RwSignal::new("fade");
    let context = RwSignal::new(false);
    let modal = RwSignal::new(false);
    let tether = RwSignal::new(true);
    let max_height = RwSignal::new(String::new());
    let max_width = RwSignal::new(String::new());
    let anchor_label = RwSignal::new("Click to open".to_string());
    let list_items = RwSignal::new(STORY_ITEMS.to_string());
    let list_class = RwSignal::new(String::new());
    let footer_class = RwSignal::new(String::new());
    let footer = RwSignal::new(String::new());
    let open_mode = RwSignal::new("");
    let shown = RwSignal::new(false);
    let list_id = RwSignal::new(String::new());
    let on_begin = RwSignal::new(false);
    let on_end = RwSignal::new(false);
    let begin_count = RwSignal::new(0u32);
    let end_count = RwSignal::new(0u32);
    Effect::new(move |_| match open_mode.get() {
        "true" => shown.set(true),
        "false" => shown.set(false),
        _ => {}
    });
    let on_change = Callback::new(move |next: bool| shown.set(next));
    let on_begin_cb = Callback::new(move |_: ()| begin_count.update(|count| *count += 1));
    let on_end_cb = Callback::new(move |_: ()| end_count.update(|count| *count += 1));
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let placement = placement.get();
                let fallback = fallback.get();
                let padding = padding.get();
                let append_to = append_to.get();
                let content_width = content_width.get();
                let content_mode = content_mode.get();
                let navigation = navigation.get();
                let transition = transition.get();
                let context = context.get();
                let modal = modal.get();
                let tether = tether.get();
                let max_height = max_height.get();
                let max_width = max_width.get();
                let list_class_value = list_class.get();
                let footer_class_value = footer_class.get();
                let labels = item_labels(&list_items.get());
                let anchor_text = anchor_label.get();
                let footer_view = text_slot(footer.get());
                let list_id_value = list_id.get();
                let anchor = ViewFn::from(move || {
                    let anchor_text = anchor_text.clone();
                    if context {
                        view! { <DtButton>{anchor_text}</DtButton> }.into_any()
                    } else {
                        view! { <DtButton importance="outlined">{anchor_text}</DtButton> }.into_any()
                    }
                });
                let controlled = matches!(open_mode.get(), "true" | "false");
                let begin = on_begin.get();
                let end = on_end.get();
                let menu = match (controlled, begin, end) {
                    (false, false, false) => dropdown_menu!(
                        placement, fallback, padding, append_to, content_width, content_mode,
                        navigation, transition, context, modal, tether, max_height, max_width,
                        list_class_value, footer_class_value, list_id_value, anchor, footer_view, labels;
                    ),
                    (true, false, false) => dropdown_menu!(
                        placement, fallback, padding, append_to, content_width, content_mode,
                        navigation, transition, context, modal, tether, max_height, max_width,
                        list_class_value, footer_class_value, list_id_value, anchor, footer_view, labels;
                        open=shown.into() on_open_change=on_change
                    ),
                    (false, true, false) => dropdown_menu!(
                        placement, fallback, padding, append_to, content_width, content_mode,
                        navigation, transition, context, modal, tether, max_height, max_width,
                        list_class_value, footer_class_value, list_id_value, anchor, footer_view, labels;
                        on_beginning_of_list=on_begin_cb
                    ),
                    (false, false, true) => dropdown_menu!(
                        placement, fallback, padding, append_to, content_width, content_mode,
                        navigation, transition, context, modal, tether, max_height, max_width,
                        list_class_value, footer_class_value, list_id_value, anchor, footer_view, labels;
                        on_end_of_list=on_end_cb
                    ),
                    (true, true, false) => dropdown_menu!(
                        placement, fallback, padding, append_to, content_width, content_mode,
                        navigation, transition, context, modal, tether, max_height, max_width,
                        list_class_value, footer_class_value, list_id_value, anchor, footer_view, labels;
                        open=shown.into() on_open_change=on_change on_beginning_of_list=on_begin_cb
                    ),
                    (true, false, true) => dropdown_menu!(
                        placement, fallback, padding, append_to, content_width, content_mode,
                        navigation, transition, context, modal, tether, max_height, max_width,
                        list_class_value, footer_class_value, list_id_value, anchor, footer_view, labels;
                        open=shown.into() on_open_change=on_change on_end_of_list=on_end_cb
                    ),
                    (false, true, true) => dropdown_menu!(
                        placement, fallback, padding, append_to, content_width, content_mode,
                        navigation, transition, context, modal, tether, max_height, max_width,
                        list_class_value, footer_class_value, list_id_value, anchor, footer_view, labels;
                        on_beginning_of_list=on_begin_cb on_end_of_list=on_end_cb
                    ),
                    (true, true, true) => dropdown_menu!(
                        placement, fallback, padding, append_to, content_width, content_mode,
                        navigation, transition, context, modal, tether, max_height, max_width,
                        list_class_value, footer_class_value, list_id_value, anchor, footer_view, labels;
                        open=shown.into() on_open_change=on_change on_beginning_of_list=on_begin_cb on_end_of_list=on_end_cb
                    ),
                };
                if begin || end {
                    view! {
                        <div class="docs-stack">
                            {menu}
                            <p class="docs-readout">
                                {move || format!("Beginning {begin_count}. End {end_count}.", begin_count = begin_count.get(), end_count = end_count.get())}
                            </p>
                        </div>
                    }
                    .into_any()
                } else {
                    menu
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingSelect label="Placement" value=placement options=PLACEMENT_OPTIONS.to_vec() />
            <SettingSelect
                label="Fallback"
                value=fallback
                options={
                    let mut options = vec![("auto", "Auto")];
                    options.extend(PLACEMENT_OPTIONS.iter().copied());
                    options
                }
            />
            <SettingSelect
                label="Padding"
                value=padding
                options=vec![("none", "None"), ("small", "Small"), ("large", "Large")]
            />
            <SettingSelect label="Append to" value=append_to options=APPEND.to_vec() />
            <SettingSelect
                label="Content width"
                value=content_width
                options=vec![("", "Default"), ("anchor", "Anchor")]
            />
            <SettingSelect
                label="Content mode"
                value=content_mode
                options=vec![("", "None"), ("light", "Light"), ("dark", "Dark"), ("invert", "Invert")]
            />
            <SettingSelect
                label="Navigation"
                value=navigation
                options=vec![("arrow-keys", "Arrow keys"), ("tab", "Tab"), ("none", "None")]
            />
            <SettingSelect
                label="Transition"
                value=transition
                options=vec![("fade", "Fade"), ("none", "None")]
            />
            <SettingText label="Max height" hint="CSS length. Empty leaves it unset." value=max_height />
            <SettingText label="Max width" hint="CSS length. Empty leaves it unset." value=max_width />
            <SettingSelect
                label="Open"
                value=open_mode
                options=vec![("", "Default"), ("null", "Null"), ("true", "True"), ("false", "False")]
            />
            <SettingText label="List id" hint="Empty generates an id." value=list_id />
            <SettingCheck label="Context menu" value=context />
            <SettingCheck label="Modal" value=modal />
            <SettingCheck label="Tether" value=tether />
            <SettingCheck label="Beginning of list" value=on_begin />
            <SettingCheck label="End of list" value=on_end />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "placement", placement.get());
        if fallback.get() != "auto" {
            push_str(&mut props, "fallback_placements", fallback.get());
        }
        push_str(&mut props, "padding", padding.get());
        push_str(&mut props, "append_to", append_to.get());
        push_str(&mut props, "content_width", content_width.get());
        push_str(&mut props, "content_mode", content_mode.get());
        if navigation.get() != "arrow-keys" {
            push_str(&mut props, "navigation_type", navigation.get());
        }
        if transition.get() != "fade" {
            push_str(&mut props, "transition", transition.get());
        }
        match open_mode.get() {
            "true" | "false" => {
                push_expr(&mut props, "open", "shown");
                push_expr(
                    &mut props,
                    "on_open_change",
                    "move |next: bool| shown.set(next)",
                );
            }
            "null" => push_expr(&mut props, "open", "None"),
            _ => {}
        }
        push_str(&mut props, "list_id", &list_id.get());
        push_bool(&mut props, "open_on_context", context.get());
        push_flag(&mut props, "modal", modal.get());
        if !tether.get() {
            push_flag(&mut props, "tether", false);
        }
        push_str(&mut props, "max_height", &max_height.get());
        push_str(&mut props, "max_width", &max_width.get());
        if on_begin.get() {
            push_expr(
                &mut props,
                "on_beginning_of_list",
                "Callback::new(|_| begin_count.update(|count| *count += 1))",
            );
        }
        if on_end.get() {
            push_expr(
                &mut props,
                "on_end_of_list",
                "Callback::new(|_| end_count.update(|count| *count += 1))",
            );
        }
        push_str(&mut props, "list_class", &list_class.get());
        push_str(&mut props, "footer_class", &footer_class.get());
        let anchor_text = anchor_label.get();
        let anchor_expr = if context.get() {
            format!(
                "ViewFn::from(|| view! {{ <DtButton>{}</DtButton> }})",
                rust_string(&anchor_text)
            )
        } else {
            format!(
                "ViewFn::from(|| view! {{ <DtButton importance=\"outlined\">{}</DtButton> }})",
                rust_string(&anchor_text)
            )
        };
        push_expr(&mut props, "anchor", anchor_expr);
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
        let navigation = navigation.get();
        let body = item_labels(&list_items.get())
            .into_iter()
            .map(|label| {
                element(
                    "DtListItem",
                    &[
                        ("role", rust_string("menuitem")),
                        ("navigation_type", rust_string(navigation)),
                    ],
                    Some(&rust_string(&label)),
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        view_of(&element("DtDropdown", &props, Some(&body)))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"Choosing a menu item closes the menu. A context menu keeps the primary button, opens from a right click at the pointer, and sits to the right of that point. The list is 192px wide. A left click does not open it. The menu follows the anchor while the page scrolls. Modal is off so the page stays usable."</p>
            <Play
                preview=preview
                settings=settings
                slots=ViewFn::from(move || view! {
                    <SettingText label="Anchor" hint="Anchor slot. The control that opens the menu." value=anchor_label />
                    <SettingText label="List" hint="List slot. Comma-separated menu items." value=list_items />
                    <SettingText label="Footer" hint="Footer slot. Empty hides it." value=footer />
                })
                classes=class_panel(vec![
                    ClassField {
                        label: "List class",
                        hint: "Extra class on the menu list.",
                        value: list_class,
                        off: None,
                    },
                    ClassField {
                        label: "Footer class",
                        hint: "Extra class on the footer wrapper.",
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
                <Variant title="Placements" row=true>
                    <DtDropdown
                        placement="bottom-start"
                        modal=false
                        anchor=ViewFn::from(|| view! { <DtButton importance="outlined">"bottom-start"</DtButton> })
                    >
                        {static_items(&["Menu item 1", "Menu item 2"])}
                    </DtDropdown>
                    <DtDropdown
                        placement="bottom"
                        modal=false
                        anchor=ViewFn::from(|| view! { <DtButton importance="outlined">"bottom"</DtButton> })
                    >
                        {static_items(&["Menu item 1", "Menu item 2"])}
                    </DtDropdown>
                    <DtDropdown
                        placement="bottom-end"
                        modal=false
                        anchor=ViewFn::from(|| view! { <DtButton importance="outlined">"bottom-end"</DtButton> })
                    >
                        {static_items(&["Menu item 1", "Menu item 2"])}
                    </DtDropdown>
                </Variant>
                <Variant title="Sections and headings">
                    <DtDropdown
                        modal=false
                        anchor=ViewFn::from(|| view! { <DtButton importance="outlined">"With sections"</DtButton> })
                    >
                        <li class="d-py-50 d-px-100 d-c-default d-fc-tertiary d-label--sm" aria-hidden="true">"Menu Heading A"</li>
                        <DtListItem role="menuitem" navigation_type="arrow-keys">"Menu Item 1"</DtListItem>
                        <DtDropdownSeparator />
                        <DtListItem role="menuitem" navigation_type="arrow-keys">"Menu Item 2"</DtListItem>
                        <DtDropdownSeparator />
                        <li class="d-py-50 d-px-100 d-c-default d-fc-tertiary d-label--sm" aria-hidden="true">"Menu Heading B"</li>
                        <DtListItem role="menuitem" navigation_type="arrow-keys">"Menu Item 3"</DtListItem>
                    </DtDropdown>
                </Variant>
                <Variant title="Context menu">
                    <DtDropdown
                        open_on_context=true
                        padding="none"
                        anchor=ViewFn::from(|| view! { <DtButton>"Click to open"</DtButton> })
                    >
                        {static_items(&[
                            "Menu item 1",
                            "Menu item 2",
                            "Another menu item 1",
                            "Menu item 3",
                            "Another menu item 2",
                        ])}
                    </DtDropdown>
                </Variant>
                <Variant title="Content mode" row=true>
                    <DtDropdown
                        content_mode="invert"
                        placement="bottom-start"
                        modal=false
                        anchor=ViewFn::from(|| view! { <DtButton importance="outlined" kind="muted" size="sm">"Invert"</DtButton> })
                    >
                        {static_items(&["Option 1", "Option 2", "Option 3"])}
                    </DtDropdown>
                    <DtDropdown
                        content_mode="dark"
                        modal=false
                        anchor=ViewFn::from(|| view! { <DtButton importance="outlined" size="sm">"Dark"</DtButton> })
                    >
                        {static_items(&["Option 1", "Option 2"])}
                    </DtDropdown>
                    <DtDropdown
                        content_mode="light"
                        modal=false
                        anchor=ViewFn::from(|| view! { <DtButton importance="outlined" size="sm">"Light"</DtButton> })
                    >
                        {static_items(&["Option 1", "Option 2"])}
                    </DtDropdown>
                </Variant>
            </div>
        </section>
    }
}
