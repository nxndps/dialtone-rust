use super::*;

const PILL_PLACEMENTS: &[(&str, &str)] = &[
    ("top", "Top"),
    ("top-start", "Top start"),
    ("top-end", "Top end"),
    ("right", "Right"),
    ("right-start", "Right start"),
    ("right-end", "Right end"),
    ("bottom", "Bottom"),
    ("bottom-start", "Bottom start"),
    ("bottom-end", "Bottom end"),
    ("left", "Left"),
    ("left-start", "Left start"),
    ("left-end", "Left end"),
];

fn or_static<'a>(value: &'a str, default: &'a str) -> &'a str {
    if value.is_empty() {
        default
    } else {
        value
    }
}

const PILL_ICONS: &[(&str, &str)] = &[
    ("", "None"),
    ("user", "User"),
    ("plus", "Plus"),
    ("check", "Check"),
    ("info", "Info"),
    ("bell", "Bell"),
];

fn channels(active: &str) -> Vec<DtFilter> {
    ["Voice", "SMS", "Meetings"]
        .into_iter()
        .map(|name| DtFilter {
            name: name.to_string(),
            active: name == active,
        })
        .collect()
}

#[component]
pub(crate) fn FilterPillPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("filter_pill.rs")));
    let filters = RwSignal::new(channels("Voice"));
    let menu = RwSignal::new(channels("SMS"));
    let label = RwSignal::new("Channel".to_string());
    let size = RwSignal::new("200");
    let use_dropdown = RwSignal::new(false);
    let defer_selection = RwSignal::new(false);
    let show_clear = RwSignal::new(true);
    let start_tooltip = RwSignal::new(String::new());
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);
    let end_tooltip = RwSignal::new(String::new());
    let popover_append = RwSignal::new("");
    let popover_fallback = RwSignal::new("");
    let popover_max_height = RwSignal::new(String::new());
    let popover_max_width = RwSignal::new(String::new());
    let popover_padding = RwSignal::new("");
    let popover_placement = RwSignal::new("");
    let content_mode = RwSignal::new("");
    let icon = RwSignal::new("");
    let label_slot = RwSignal::new(String::new());
    let content_slot = RwSignal::new(String::new());
    let header_slot = RwSignal::new(String::new());
    let footer_slot = RwSignal::new(String::new());
    let label_class = RwSignal::new(String::new());
    let popover_content_class = RwSignal::new(String::new());
    let popover_header_class = RwSignal::new(String::new());
    let popover_footer_class = RwSignal::new(String::new());
    let popover_dialog_class = RwSignal::new(String::new());
    let dropdown_list_class = RwSignal::new(String::new());
    let popover_off = Signal::derive(move || use_dropdown.get());
    let dropdown_off = Signal::derive(move || !use_dropdown.get());
    let header_off = Signal::derive(move || use_dropdown.get() || header_slot.get().is_empty());
    let footer_off = Signal::derive(move || {
        use_dropdown.get() || (footer_slot.get().is_empty() && !defer_selection.get())
    });
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let label = label.get();
                let size = size.get();
                let use_dropdown = use_dropdown.get();
                let defer_selection = defer_selection.get();
                let show_clear = show_clear.get();
                let start_icon = icon_slot(icon.get());
                let label_view = text_slot(label_slot.get());
                let content_view = text_slot(content_slot.get());
                let header_view = text_slot(header_slot.get());
                let footer_view = text_slot(footer_slot.get());
                let label_class = label_class.get();
                let popover_content_class = popover_content_class.get();
                let popover_header_class = popover_header_class.get();
                let popover_footer_class = popover_footer_class.get();
                let popover_dialog_class = popover_dialog_class.get();
                let dropdown_list_class = dropdown_list_class.get();
                let start_tooltip = start_tooltip.get();
                let disabled = disabled.get();
                let read_only = read_only.get();
                let end_tooltip = end_tooltip.get();
                let popover_append_to = or_static(popover_append.get(), "body");
                let popover_fallback_placements = or_static(popover_fallback.get(), "auto");
                let popover_max_height = popover_max_height.get();
                let popover_max_width = popover_max_width.get();
                let popover_padding = or_static(popover_padding.get(), "large");
                let popover_placement = or_static(popover_placement.get(), "bottom-start");
                let content_mode = content_mode.get();
                view! {
                    <DtFilterPill
                        filters=filters.into()
                        on_change=move |next| filters.set(next)
                        use_dropdown=use_dropdown
                        defer_selection=defer_selection
                        show_clear=show_clear
                        label=label
                        start_tooltip_text=start_tooltip
                        disabled=disabled
                        read_only=read_only
                        end_tooltip_text=end_tooltip
                        popover_append_to=popover_append_to
                        popover_fallback_placements=popover_fallback_placements
                        popover_max_height=popover_max_height
                        popover_max_width=popover_max_width
                        popover_padding=popover_padding
                        popover_placement=popover_placement
                        content_mode=content_mode
                        size=size
                        start_icon=start_icon
                        label_slot=label_view
                        content=content_view
                        header=header_view
                        footer=footer_view
                        label_class=label_class
                        popover_content_class=popover_content_class
                        popover_header_class=popover_header_class
                        popover_footer_class=popover_footer_class
                        popover_dialog_class=popover_dialog_class
                        dropdown_list_class=dropdown_list_class
                    />
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Label" value=label />
            <SettingSelect
                label="Size"
                value=size
                options=vec![
                    ("100", "100"),
                    ("200", "200"),
                    ("300", "300"),
                    ("400", "400"),
                    ("500", "500"),
                ]
            />
            <SettingCheck label="Dropdown" value=use_dropdown />
            <SettingCheck label="Defer selection" value=defer_selection />
            <SettingCheck label="Clear" value=show_clear />
            <SettingText label="Start tooltip" value=start_tooltip />
            <SettingCheck label="Disabled" value=disabled />
            <SettingCheck label="Read only" value=read_only />
            <SettingText label="End tooltip" value=end_tooltip />
            <SettingSelect
                label="Popover append to"
                value=popover_append
                options=vec![
                    ("", "Default"),
                    ("body", "Body"),
                    ("parent", "Parent"),
                    ("root", "Root"),
                    ("HTMLElement", "HTMLElement"),
                ]
            />
            <SettingSelect
                label="Popover fallback"
                value=popover_fallback
                options={
                    let mut options = vec![("", "Default")];
                    options.extend(PILL_PLACEMENTS.iter().copied());
                    options
                }
            />
            <SettingText
                label="Popover max height"
                hint="CSS length. Empty leaves it unset."
                value=popover_max_height
            />
            <SettingText
                label="Popover max width"
                hint="CSS length. Empty leaves it unset."
                value=popover_max_width
            />
            <SettingSelect
                label="Popover padding"
                value=popover_padding
                options=vec![
                    ("", "Default"),
                    ("none", "None"),
                    ("small", "Small"),
                    ("medium", "Medium"),
                    ("large", "Large"),
                ]
            />
            <SettingSelect
                label="Popover placement"
                value=popover_placement
                options={
                    let mut options = vec![("", "Default")];
                    options.extend(PILL_PLACEMENTS.iter().copied());
                    options
                }
            />
            <SettingSelect
                label="Content mode"
                value=content_mode
                options=vec![("", "None"), ("light", "Light"), ("dark", "Dark"), ("invert", "Invert")]
            />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_expr(&mut props, "filters", "filters.into()");
        push_expr(
            &mut props,
            "on_change",
            "move |next: Vec<DtFilter>| filters.set(next)",
        );
        push_str(&mut props, "label", &label.get());
        push_str(&mut props, "size", size.get());
        push_bool(&mut props, "use_dropdown", use_dropdown.get());
        push_bool(&mut props, "defer_selection", defer_selection.get());
        if !show_clear.get() {
            props.push(("show_clear", "false".to_string()));
        }
        push_str(&mut props, "start_tooltip_text", &start_tooltip.get());
        push_bool(&mut props, "disabled", disabled.get());
        push_bool(&mut props, "read_only", read_only.get());
        push_str(&mut props, "end_tooltip_text", &end_tooltip.get());
        push_str(&mut props, "popover_append_to", popover_append.get());
        push_str(
            &mut props,
            "popover_fallback_placements",
            popover_fallback.get(),
        );
        push_str(&mut props, "popover_max_height", &popover_max_height.get());
        push_str(&mut props, "popover_max_width", &popover_max_width.get());
        push_str(&mut props, "popover_padding", popover_padding.get());
        push_str(&mut props, "popover_placement", popover_placement.get());
        push_str(&mut props, "content_mode", content_mode.get());
        push_icon(&mut props, "start_icon", icon.get());
        push_span_slot(&mut props, "label_slot", &label_slot.get());
        push_span_slot(&mut props, "content", &content_slot.get());
        push_span_slot(&mut props, "header", &header_slot.get());
        push_span_slot(&mut props, "footer", &footer_slot.get());
        push_str(&mut props, "label_class", &label_class.get());
        push_str(
            &mut props,
            "popover_content_class",
            &popover_content_class.get(),
        );
        push_str(
            &mut props,
            "popover_header_class",
            &popover_header_class.get(),
        );
        push_str(
            &mut props,
            "popover_footer_class",
            &popover_footer_class.get(),
        );
        push_str(
            &mut props,
            "popover_dialog_class",
            &popover_dialog_class.get(),
        );
        push_str(
            &mut props,
            "dropdown_list_class",
            &dropdown_list_class.get(),
        );
        view_of(&element("DtFilterPill", &props, None))
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingSelect label="Start icon" value=icon options=PILL_ICONS.to_vec() />
            <SettingText
                label="Default"
                hint="Default slot. Replaces the label. Empty uses the label prop."
                value=label_slot
            />
            <SettingText
                label="Content"
                hint="Replaces the checkbox list or the dropdown rows."
                value=content_slot
            />
            <SettingText
                label="Header"
                hint="Popover header. A dropdown does not show it."
                value=header_slot
            />
            <SettingText
                label="Footer"
                hint="Popover footer. Replaces the Apply row. A dropdown does not show it."
                value=footer_slot
            />
        }
    });
    let classes = class_panel(vec![
        ClassField {
            label: "Label class",
            hint: "Extra class on the label.",
            value: label_class,
            off: None,
        },
        ClassField {
            label: "Popover content class",
            hint: "Extra class on the popover content.",
            value: popover_content_class,
            off: Some(popover_off),
        },
        ClassField {
            label: "Popover header class",
            hint: "Extra class on the popover header.",
            value: popover_header_class,
            off: Some(header_off),
        },
        ClassField {
            label: "Popover footer class",
            hint: "Extra class on the popover footer.",
            value: popover_footer_class,
            off: Some(footer_off),
        },
        ClassField {
            label: "Popover dialog class",
            hint: "Extra class on the popover dialog.",
            value: popover_dialog_class,
            off: Some(popover_off),
        },
        ClassField {
            label: "Dropdown list class",
            hint: "Extra class on the dropdown list.",
            value: dropdown_list_class,
            off: Some(dropdown_off),
        },
    ]);
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"filters is RwSignal of Voice, SMS, and Meetings, with Voice active. on_change writes that signal. Defer selection holds checkbox edits until Apply. A dropdown ignores it. The default slot replaces the label. Content replaces the list."</p>
            <Play stack=true preview=preview settings=settings slots=slots classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Dropdown">
                    <DtFilterPill
                        filters=menu.into()
                        use_dropdown=true
                        label="Channel"
                        on_change=move |next| menu.set(next)
                    />
                </Variant>
            </div>
        </section>
    }
}
