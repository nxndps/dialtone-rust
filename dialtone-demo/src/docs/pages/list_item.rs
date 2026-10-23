use super::*;

const WRAPPER_CLASS: &str = "Extra classes on the layout. Applies when type is default.";
const CUSTOM_BODY: &str = r#"<DtStack direction="row" align="start" justify="between" gap="500" class="d-py-100 d-px-150 d-pie-100">
    <DtStack align="baseline" direction="row" gap="400">
        <DtText kind="body" tone="muted" size="100">"10:00"</DtText>
        <DtText size="300" density="300" kind="body" tone="secondary">
            "Custom list item example lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua"
        </DtText>
    </DtStack>
    <DtStack direction="row">
        <DtButton size="100" kind="muted" importance="clear" title="share" start_icon=Some(ViewFn::from(|| view! { <DtIconShare size="200" /> })) />
        <DtButton size="100" kind="muted" importance="clear" title="star" start_icon=Some(ViewFn::from(|| view! { <DtIconStar size="200" /> })) />
        <DtButton size="100" kind="muted" importance="clear" title="more" start_icon=Some(ViewFn::from(|| view! { <DtIconMoreVertical size="200" /> })) />
    </DtStack>
</DtStack>"#;

fn custom_body() -> impl IntoView {
    view! {
        <DtStack
            direction="row"
            align="start"
            justify="between"
            gap="500"
            class="d-py-100 d-px-150 d-pie-100"
        >
            <DtStack align="baseline" direction="row" gap="400">
                <DtText kind="body" tone="muted" size="100">"10:00"</DtText>
                <DtText size="300" density="300" kind="body" tone="secondary">
                    "Custom list item example lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua"
                </DtText>
            </DtStack>
            <DtStack direction="row">
                <DtButton
                    size="100"
                    kind="muted"
                    importance="clear"
                    title="share"
                    start_icon=Some(ViewFn::from(|| view! { <DtIconShare size="200" /> }))
                />
                <DtButton
                    size="100"
                    kind="muted"
                    importance="clear"
                    title="star"
                    start_icon=Some(ViewFn::from(|| view! { <DtIconStar size="200" /> }))
                />
                <DtButton
                    size="100"
                    kind="muted"
                    importance="clear"
                    title="more"
                    start_icon=Some(ViewFn::from(|| view! { <DtIconMoreVertical size="200" /> }))
                />
            </DtStack>
        </DtStack>
    }
}

fn sample_row(selected: bool, navigation: &'static str) -> impl IntoView {
    view! {
        <ul>
            <DtListItem
                navigation_type=navigation
                selected=selected
                start=ViewFn::from(|| view! { <DtIconCheck size="300" /> })
                subtitle=ViewFn::from(|| view! { <span>"Description"</span> })
                block_end=ViewFn::from(|| view! { <DtBadge text="Label" /> })
                end=ViewFn::from(|| view! { <DtIconExternalLink size="300" /> })
            >
                "Default List Item"
            </DtListItem>
        </ul>
    }
}

macro_rules! preview_item {
    (
        $id:expr,
        $role:expr,
        $element:expr,
        $navigation:expr,
        $selected:expr,
        $root_class:expr,
        $wrapper_class:expr,
        $highlight_id:expr,
        $title:expr
        $(, $slot_name:ident = $slot_value:expr)* $(,)?
    ) => {
        view! {
            <ul>
                <DtListItem
                    id=$id
                    role=$role
                    element_type=$element
                    navigation_type=$navigation
                    selected=$selected
                    class=$root_class
                    wrapper_class=$wrapper_class
                    highlight_id=$highlight_id
                    $($slot_name=$slot_value )*
                >
                    {$title}
                </DtListItem>
            </ul>
        }
        .into_any()
    };
}

#[component]
pub(crate) fn ListItemPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("list_item.rs")));
    let kind = RwSignal::new("default");
    let role = RwSignal::new("listitem");
    let navigation = RwSignal::new("tab");
    let tag = RwSignal::new("li");
    let selected = RwSignal::new(false);
    let highlighted = RwSignal::new(false);
    let title = RwSignal::new("Default List Item".to_string());
    let subtitle = RwSignal::new("Description".to_string());
    let show_start = RwSignal::new(true);
    let show_bottom = RwSignal::new(true);
    let show_end = RwSignal::new(true);
    let root_class = RwSignal::new(String::new());
    let wrapper_class = RwSignal::new(String::new());
    let id = RwSignal::new("dt-list-item-preview".to_string());
    let highlight_id = Signal::derive(move || {
        if highlighted.get() {
            id.get()
        } else {
            String::new()
        }
    });
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let kind = kind.get();
                let role = role.get();
                let navigation = navigation.get();
                let element = tag.get();
                let selected = selected.get();
                let root_class = root_class.get();
                let wrapper_class = wrapper_class.get();
                let title = title.get();
                let id_value = id.get();
                if kind != "default" {
                    return view! {
                        <ul>
                            <DtListItem
                                id=id_value
                                r#type="custom"
                                role=role
                                element_type=element
                                navigation_type=navigation
                                selected=selected
                                class=root_class
                                highlight_id=highlight_id
                            >
                                {custom_body()}
                            </DtListItem>
                        </ul>
                    }
                    .into_any();
                }
                let start = show_start
                    .get()
                    .then(|| ViewFn::from(|| view! { <DtIconCheck size="300" /> }));
                let subtitle_text = subtitle.get();
                let subtitle_slot = (!subtitle_text.is_empty()).then(|| {
                    ViewFn::from(move || {
                        let subtitle_text = subtitle_text.clone();
                        view! { <span>{subtitle_text}</span> }
                    })
                });
                let bottom = show_bottom
                    .get()
                    .then(|| ViewFn::from(|| view! { <DtBadge text="Label" /> }));
                let end = show_end
                    .get()
                    .then(|| ViewFn::from(|| view! { <DtIconExternalLink size="300" /> }));
                match (start, subtitle_slot, bottom, end) {
                    (Some(start), Some(subtitle), Some(bottom), Some(end)) => preview_item!(
                        id_value.clone(), role, element, navigation, selected, root_class, wrapper_class, highlight_id, title,
                        start = start, subtitle = subtitle, block_end = bottom, end = end
                    ),
                    (Some(start), Some(subtitle), Some(bottom), None) => preview_item!(
                        id_value.clone(), role, element, navigation, selected, root_class, wrapper_class, highlight_id, title,
                        start = start, subtitle = subtitle, block_end = bottom
                    ),
                    (Some(start), Some(subtitle), None, Some(end)) => preview_item!(
                        id_value.clone(), role, element, navigation, selected, root_class, wrapper_class, highlight_id, title,
                        start = start, subtitle = subtitle, end = end
                    ),
                    (Some(start), Some(subtitle), None, None) => preview_item!(
                        id_value.clone(), role, element, navigation, selected, root_class, wrapper_class, highlight_id, title,
                        start = start, subtitle = subtitle
                    ),
                    (Some(start), None, Some(bottom), Some(end)) => preview_item!(
                        id_value.clone(), role, element, navigation, selected, root_class, wrapper_class, highlight_id, title,
                        start = start, block_end = bottom, end = end
                    ),
                    (Some(start), None, Some(bottom), None) => preview_item!(
                        id_value.clone(), role, element, navigation, selected, root_class, wrapper_class, highlight_id, title,
                        start = start, block_end = bottom
                    ),
                    (Some(start), None, None, Some(end)) => preview_item!(
                        id_value.clone(), role, element, navigation, selected, root_class, wrapper_class, highlight_id, title,
                        start = start, end = end
                    ),
                    (Some(start), None, None, None) => preview_item!(
                        id_value.clone(), role, element, navigation, selected, root_class, wrapper_class, highlight_id, title,
                        start = start
                    ),
                    (None, Some(subtitle), Some(bottom), Some(end)) => preview_item!(
                        id_value.clone(), role, element, navigation, selected, root_class, wrapper_class, highlight_id, title,
                        subtitle = subtitle, block_end = bottom, end = end
                    ),
                    (None, Some(subtitle), Some(bottom), None) => preview_item!(
                        id_value.clone(), role, element, navigation, selected, root_class, wrapper_class, highlight_id, title,
                        subtitle = subtitle, block_end = bottom
                    ),
                    (None, Some(subtitle), None, Some(end)) => preview_item!(
                        id_value.clone(), role, element, navigation, selected, root_class, wrapper_class, highlight_id, title,
                        subtitle = subtitle, end = end
                    ),
                    (None, Some(subtitle), None, None) => preview_item!(
                        id_value.clone(), role, element, navigation, selected, root_class, wrapper_class, highlight_id, title,
                        subtitle = subtitle
                    ),
                    (None, None, Some(bottom), Some(end)) => preview_item!(
                        id_value.clone(), role, element, navigation, selected, root_class, wrapper_class, highlight_id, title,
                        block_end = bottom, end = end
                    ),
                    (None, None, Some(bottom), None) => preview_item!(
                        id_value.clone(), role, element, navigation, selected, root_class, wrapper_class, highlight_id, title,
                        block_end = bottom
                    ),
                    (None, None, None, Some(end)) => preview_item!(
                        id_value.clone(), role, element, navigation, selected, root_class, wrapper_class, highlight_id, title,
                        end = end
                    ),
                    (None, None, None, None) => preview_item!(
                        id_value.clone(), role, element, navigation, selected, root_class, wrapper_class, highlight_id, title
                    ),
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingSelect
                label="Type"
                value=kind
                options=vec![("default", "Default"), ("custom", "Custom")]
            />
            <SettingSelect
                label="Role"
                value=role
                options=vec![("listitem", "List item"), ("menuitem", "Menu item"), ("option", "Option")]
            />
            <SettingSelect
                label="Navigation"
                value=navigation
                options=vec![("tab", "Tab"), ("arrow-keys", "Arrow keys"), ("none", "None")]
            />
            <SettingSelect
                label="Element"
                value=tag
                options=vec![("li", "li"), ("div", "div")]
            />
            <SettingText label="Id" hint="Empty generates one. Highlight uses this id." value=id />
            <SettingCheck label="Selected" value=selected />
            <SettingCheck label="Highlighted" value=highlighted />
        }
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingText label="Title" value=title />
            <SettingText label="Subtitle" hint="Subtitle slot. Empty hides it." value=subtitle />
            <SettingCheck label="Start icon" value=show_start />
            <SettingCheck label="Label badge" value=show_bottom />
            <SettingCheck label="End icon" value=show_end />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        let item_kind = kind.get();
        if item_kind != "default" {
            push_str(&mut props, "type", item_kind);
        }
        let item_role = role.get();
        if item_role != "listitem" {
            push_str(&mut props, "role", item_role);
        }
        let item_element = tag.get();
        if item_element != "li" {
            push_str(&mut props, "element_type", item_element);
        }
        let item_navigation = navigation.get();
        if item_navigation != "none" {
            push_str(&mut props, "navigation_type", item_navigation);
        }
        push_bool(&mut props, "selected", selected.get());
        push_str(&mut props, "id", &id.get());
        push_str(&mut props, "class", &root_class.get());
        if item_kind == "default" {
            push_str(&mut props, "wrapper_class", &wrapper_class.get());
        }
        if highlighted.get() && !id.get().is_empty() {
            push_expr(
                &mut props,
                "highlight_id",
                format!("Signal::derive(|| {}.to_string())", rust_string(&id.get())),
            );
        }
        let child = if item_kind != "default" {
            CUSTOM_BODY.to_string()
        } else {
            if show_start.get() {
                push_expr(
                    &mut props,
                    "start",
                    "ViewFn::from(|| view! { <DtIconCheck size=\"300\" /> })",
                );
            }
            if !subtitle.get().is_empty() {
                push_expr(
                    &mut props,
                    "subtitle",
                    format!(
                        "ViewFn::from(|| view! {{ <span>{}</span> }})",
                        rust_string(&subtitle.get())
                    ),
                );
            }
            if show_bottom.get() {
                push_expr(
                    &mut props,
                    "block_end",
                    "ViewFn::from(|| view! { <DtBadge text=\"Label\" /> })",
                );
            }
            if show_end.get() {
                push_expr(
                    &mut props,
                    "end",
                    "ViewFn::from(|| view! { <DtIconExternalLink size=\"300\" /> })",
                );
            }
            rust_string(&title.get())
        };
        view_of(&element("DtListItem", &props, Some(&child)))
    });
    let wrapper_off = Signal::derive(move || kind.get() != "default");
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <Play
                stack=true
                preview=preview
                settings=settings
                slots=slots
                classes=class_panel(vec![
                    ClassField {
                        label: "Class",
                        hint: ROOT_CLASS,
                        value: root_class,
                        off: None,
                    },
                    ClassField {
                        label: "Wrapper class",
                        hint: WRAPPER_CLASS,
                        value: wrapper_class,
                        off: Some(wrapper_off),
                    },
                ])
                code=code
            />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Custom">
                    <ul>
                        <DtListItem navigation_type="tab" r#type="custom">
                            {custom_body()}
                        </DtListItem>
                    </ul>
                </Variant>
                <Variant title="Selected">
                    {sample_row(true, "tab")}
                </Variant>
                <Variant title="Static">
                    {sample_row(false, "none")}
                </Variant>
            </div>
        </section>
    }
}
