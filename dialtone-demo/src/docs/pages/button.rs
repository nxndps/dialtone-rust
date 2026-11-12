use super::*;

#[component]
pub(crate) fn ButtonPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("button.rs")));
    let label = RwSignal::new("Save".to_string());
    let importance = RwSignal::new("primary");
    let kind = RwSignal::new("default");
    let size = RwSignal::new("md");
    let loading = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    let circle = RwSignal::new(false);
    let link = RwSignal::new(false);
    let link_kind = RwSignal::new("");
    let link_inverted = RwSignal::new(false);
    let icon_position = RwSignal::new("");
    let button_type = RwSignal::new("");
    let width = RwSignal::new(String::new());
    let assertive_on_focus = RwSignal::new(false);
    let href = RwSignal::new(String::new());
    let target = RwSignal::new("");
    let rel = RwSignal::new(String::new());
    let active = RwSignal::new(false);
    let icon = RwSignal::new("");
    let start_icon = RwSignal::new("");
    let end_icon = RwSignal::new("");
    let leading = RwSignal::new(String::new());
    let trailing = RwSignal::new(String::new());
    let root_class = RwSignal::new(String::new());
    let label_class = RwSignal::new(String::new());
    let leading_class = RwSignal::new(String::new());
    let trailing_class = RwSignal::new(String::new());
    let clicks = RwSignal::new(0u32);
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let label = label.get();
                let importance = importance.get();
                let kind = kind.get();
                let size = size.get();
                let loading = loading.get();
                let disabled = disabled.get();
                let circle = circle.get();
                let link = link.get();
                let link_kind = match link_kind.get() {
                    "" => "default",
                    value => value,
                };
                let link_inverted = link_inverted.get();
                let icon_position = match icon_position.get() {
                    "" => "left",
                    value => value,
                };
                let type_ = match button_type.get() {
                    "" => "button",
                    value => value,
                };
                let width = width.get();
                let assertive_on_focus = assertive_on_focus.get();
                let href = href.get();
                let target = target.get();
                let rel = rel.get();
                let active = active.get();
                let class_name = root_class.get();
                let label_class_name = label_class.get();
                let leading_class_name = leading_class.get();
                let trailing_class_name = trailing_class.get();
                let chosen = icon.get();
                let start_icon_slot = icon_slot(start_icon.get());
                let end_icon_slot = icon_slot(end_icon.get());
                let leading_slot = text_slot(leading.get());
                let trailing_slot = text_slot(trailing.get());
                let icon_view = if !chosen.is_empty() {
                    icon_slot(chosen)
                } else if circle {
                    icon_slot("plus")
                } else {
                    None
                };
                let aria = if icon_view.is_some() {
                    icon_label(if chosen.is_empty() { "plus" } else { chosen }).to_string()
                } else {
                    String::new()
                };
                let button = open_one!(DtButton {
                    importance=importance
                    kind=kind
                    size=size
                    loading=loading
                    disabled=disabled
                    circle=circle
                    link=link
                    link_kind=link_kind
                    link_inverted=link_inverted
                    icon_position=icon_position
                    type_=type_
                    width=width
                    assertive_on_focus=assertive_on_focus
                    href=href
                    target=target
                    rel=rel
                    active=active
                    class=class_name
                    label_class=label_class_name
                    leading_class=leading_class_name
                    trailing_class=trailing_class_name
                    aria_label=aria
                    start_icon=start_icon_slot
                    end_icon=end_icon_slot
                    leading=leading_slot
                    trailing=trailing_slot
                    on_click=move || clicks.update(|count| *count += 1)
                } {
                    {label}
                }, icon_view, icon);
                view! {
                    <div class="docs-inline">
                        {button}
                        <span class="docs-readout">{move || format!("Clicked {}", clicks.get())}</span>
                    </div>
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Label" value=label />
            <SettingSelect
                label="Importance"
                value=importance
                options=vec![("primary", "Primary"), ("outlined", "Outlined"), ("clear", "Clear")]
            />
            <SettingSelect
                label="Kind"
                value=kind
                options=vec![
                    ("default", "Default"),
                    ("muted", "Muted"),
                    ("critical", "Critical"),
                    ("positive", "Positive"),
                    ("inverted", "Inverted"),
                    ("unstyled", "Unstyled"),
                ]
            />
            <SettingSelect label="Size" value=size options=TSHIRT.to_vec() />
            <SettingCheck label="Circle" value=circle />
            <SettingCheck label="Link" value=link />
            <SettingSelect
                label="Link kind"
                value=link_kind
                options=vec![
                    ("", "Default"),
                    ("default", "default"),
                    ("critical", "Critical"),
                    ("warning", "Warning"),
                    ("positive", "Positive"),
                    ("muted", "Muted"),
                    ("info", "Info"),
                    ("mention", "Mention"),
                ]
            />
            <SettingCheck label="Link inverted (Deprecated)" value=link_inverted />
            <SettingSelect
                label="Icon position (Deprecated)"
                value=icon_position
                options=vec![
                    ("", "Default"),
                    ("start", "Start"),
                    ("end", "End"),
                    ("blockStart", "Block start"),
                    ("blockEnd", "Block end"),
                    ("left", "Left"),
                    ("right", "Right"),
                    ("top", "Top"),
                    ("bottom", "Bottom"),
                ]
            />
            <SettingSelect
                label="Type"
                value=button_type
                options=vec![
                    ("", "Default"),
                    ("button", "Button"),
                    ("submit", "Submit"),
                    ("reset", "Reset"),
                ]
            />
            <SettingText label="Width" value=width />
            <SettingCheck label="Assertive on focus" value=assertive_on_focus />
            <SettingText label="Href" value=href />
            <SettingSelect
                label="Target"
                value=target
                options=vec![
                    ("", "Default"),
                    ("_self", "_self"),
                    ("_blank", "_blank"),
                    ("_parent", "_parent"),
                    ("_top", "_top"),
                ]
            />
            <SettingText label="Rel" value=rel />
            <SettingCheck label="Active" value=active />
            <SettingCheck label="Loading" value=loading />
            <SettingCheck label="Disabled" value=disabled />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "importance", importance.get());
        push_str(&mut props, "kind", kind.get());
        push_str(&mut props, "size", size.get());
        push_bool(&mut props, "loading", loading.get());
        push_bool(&mut props, "disabled", disabled.get());
        push_bool(&mut props, "circle", circle.get());
        push_bool(&mut props, "link", link.get());
        push_str(&mut props, "link_kind", link_kind.get());
        push_bool(&mut props, "link_inverted", link_inverted.get());
        push_str(&mut props, "icon_position", icon_position.get());
        push_str(&mut props, "type_", button_type.get());
        push_str(&mut props, "width", &width.get());
        push_bool(&mut props, "assertive_on_focus", assertive_on_focus.get());
        push_str(&mut props, "href", &href.get());
        push_str(&mut props, "target", target.get());
        push_str(&mut props, "rel", &rel.get());
        push_bool(&mut props, "active", active.get());
        push_str(&mut props, "class", &root_class.get());
        push_str(&mut props, "label_class", &label_class.get());
        push_str(&mut props, "leading_class", &leading_class.get());
        push_str(&mut props, "trailing_class", &trailing_class.get());
        if let Some(expr) = icon_expr(start_icon.get()) {
            push_expr(&mut props, "start_icon", format!("Some({expr})"));
        }
        if let Some(expr) = icon_expr(end_icon.get()) {
            push_expr(&mut props, "end_icon", format!("Some({expr})"));
        }
        if !leading.get().is_empty() {
            push_expr(
                &mut props,
                "leading",
                format!(
                    "Some(ViewFn::from(|| view! {{ <span>{}</span> }}))",
                    rust_string(&leading.get())
                ),
            );
        }
        if !trailing.get().is_empty() {
            push_expr(
                &mut props,
                "trailing",
                format!(
                    "Some(ViewFn::from(|| view! {{ <span>{}</span> }}))",
                    rust_string(&trailing.get())
                ),
            );
        }
        let chosen = icon.get();
        let icon_name = if !chosen.is_empty() {
            chosen
        } else if circle.get() {
            "plus"
        } else {
            ""
        };
        if !icon_name.is_empty() {
            push_str(&mut props, "aria_label", icon_label(icon_name));
            push_icon(&mut props, "icon", icon_name);
        }
        view_of(&element(
            "DtButton",
            &props,
            Some(&rust_string(&label.get())),
        ))
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingText label="Default" hint="Label rendered in the default slot." value=label />
            <SettingSelect label="Icon" value=icon options=ICONS.to_vec() />
            <SettingSelect label="Start icon" value=start_icon options=ICONS.to_vec() />
            <SettingSelect label="End icon" value=end_icon options=ICONS.to_vec() />
            <SettingText label="Leading" hint="Before the label." value=leading />
            <SettingText label="Trailing" hint="After the label." value=trailing />
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
            hint: "Extra class on the label.",
            value: label_class,
            off: None,
        },
        ClassField {
            label: "Leading class",
            hint: "Extra class on the leading wrapper.",
            value: leading_class,
            off: Some(Signal::derive(move || leading.get().is_empty())),
        },
        ClassField {
            label: "Trailing class",
            hint: "Extra class on the trailing wrapper.",
            value: trailing_class,
            off: Some(Signal::derive(move || trailing.get().is_empty())),
        },
    ]);
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"A link hides the icon slot. Circle uses the aria label as its accessible name."</p>
            <Play preview=preview settings=settings slots=slots classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Importance" row=true>
                    <DtButton>"Primary"</DtButton>
                    <DtButton importance="outlined">"Outlined"</DtButton>
                    <DtButton importance="clear">"Clear"</DtButton>
                </Variant>
                <Variant title="Kind" row=true>
                    <DtButton kind="critical" importance="outlined">"Remove"</DtButton>
                    <DtButton kind="positive" icon=ViewFn::from(|| view! { <DtIconPlus /> })>"Add"</DtButton>
                    <DtButton kind="muted" importance="outlined">"Muted"</DtButton>
                </Variant>
                <Variant title="Size" row=true>
                    <DtButton size="xs">"xs"</DtButton>
                    <DtButton size="sm">"sm"</DtButton>
                    <DtButton size="md">"md"</DtButton>
                    <DtButton size="lg">"lg"</DtButton>
                </Variant>
            </div>
        </section>
    }
}
