use super::*;

const SPLIT_SIZES: &[(&str, &str)] = &[
    ("100", "100"),
    ("200", "200"),
    ("300", "300"),
    ("400", "400"),
    ("500", "500"),
];

const SPLIT_ICONS: &[(&str, &str)] = &[
    ("", "None"),
    ("phone", "Phone"),
    ("plus", "Plus"),
    ("check", "Check"),
    ("bell", "Bell"),
    ("more", "More"),
];

const ICON_POSITIONS: &[(&str, &str)] = &[
    ("left", "Left"),
    ("right", "Right"),
    ("top", "Top"),
    ("bottom", "Bottom"),
    ("start", "Start"),
    ("end", "End"),
    ("blockStart", "Block start"),
    ("blockEnd", "Block end"),
];

fn split_icon(name: &str) -> Option<ViewFn> {
    Some(match name {
        "phone" => ViewFn::from(|| view! { <DtIconPhone /> }),
        "more" => ViewFn::from(|| view! { <DtIconMoreVertical /> }),
        other => return icon_slot(other),
    })
}

fn split_icon_expr(name: &str) -> Option<&'static str> {
    Some(match name {
        "phone" => "ViewFn::from(|| view! { <DtIconPhone /> })",
        "more" => "ViewFn::from(|| view! { <DtIconMoreVertical /> })",
        other => return icon_expr(other),
    })
}

fn call_menu() -> ViewFn {
    ViewFn::from(|| {
        view! {
            <li class="d-list-item" role="menuitem" tabindex="-1">"Call back"</li>
            <li class="d-list-item" role="menuitem" tabindex="-1">"Call with video"</li>
            <li class="d-list-item" role="menuitem" tabindex="-1">"Add to contacts"</li>
        }
    })
}

fn phone() -> ViewFn {
    ViewFn::from(|| view! { <DtIconPhone /> })
}

fn chevron() -> ViewFn {
    ViewFn::from(|| view! { <DtIconChevronDown /> })
}

fn more_icon() -> ViewFn {
    ViewFn::from(|| view! { <DtIconMoreVertical /> })
}

fn count_badge() -> ViewFn {
    ViewFn::from(|| view! { <DtBadge text="1" /> })
}

fn shortcut() -> ViewFn {
    ViewFn::from(|| {
        view! {
            <span class="d-keyboard-shortcut">
                <span class="d-keyboard-shortcut__item">"⌘K"</span>
            </span>
        }
    })
}

#[component]
pub(crate) fn SplitButtonPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("split_button.rs")));
    let label = RwSignal::new("Place call".to_string());
    let importance = RwSignal::new("primary");
    let kind = RwSignal::new("default");
    let size = RwSignal::new("300");
    let icon_position = RwSignal::new("left");
    let show_divider = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let start_disabled = RwSignal::new(false);
    let end_disabled = RwSignal::new(false);
    let start_loading = RwSignal::new(false);
    let start_active = RwSignal::new(false);
    let end_active = RwSignal::new(false);
    let start_tooltip = RwSignal::new(String::new());
    let end_tooltip = RwSignal::new("More calling options".to_string());
    let placement = RwSignal::new("bottom-end");
    let width = RwSignal::new(String::new());
    let start_href = RwSignal::new(String::new());
    let start_target = RwSignal::new(String::new());
    let start_rel = RwSignal::new(String::new());
    let start_aria = RwSignal::new(String::new());
    let assertive = RwSignal::new(false);
    let end_aria = RwSignal::new(String::new());
    let end_id = RwSignal::new(String::new());
    let start_icon = RwSignal::new("");
    let start_end_icon = RwSignal::new("");
    let end_icon = RwSignal::new("");
    let leading = RwSignal::new(String::new());
    let trailing = RwSignal::new(String::new());
    let end_slot = RwSignal::new(String::new());
    let dropdown = RwSignal::new(true);
    let root_class = RwSignal::new(String::new());
    let start_button_class = RwSignal::new(String::new());
    let end_button_class = RwSignal::new(String::new());
    let label_class = RwSignal::new(String::new());
    let leading_class = RwSignal::new(String::new());
    let trailing_class = RwSignal::new(String::new());

    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let label = label.get();
                let importance = importance.get();
                let kind = kind.get();
                let size = size.get();
                let icon_position = icon_position.get();
                let show_divider = show_divider.get();
                let disabled = disabled.get();
                let start_disabled = start_disabled.get();
                let end_disabled = end_disabled.get();
                let start_loading = start_loading.get();
                let start_active = start_active.get();
                let end_active = end_active.get();
                let start_tooltip = start_tooltip.get();
                let end_tooltip = end_tooltip.get();
                let placement = placement.get();
                let width = width.get();
                let start_href = start_href.get();
                let start_target = start_target.get();
                let start_rel = start_rel.get();
                let assertive = assertive.get();
                let start_aria = start_aria.get();
                let end_aria = end_aria.get();
                let end_id = end_id.get();
                let class_name = root_class.get();
                let start_class = start_button_class.get();
                let end_class = end_button_class.get();
                let label_class_name = label_class.get();
                let leading_class_name = leading_class.get();
                let trailing_class_name = trailing_class.get();
                let start_icon = split_icon(start_icon.get());
                let start_end_icon = split_icon(start_end_icon.get());
                let end_icon = split_icon(end_icon.get());
                let leading = text_slot(leading.get());
                let trailing = text_slot(trailing.get());
                let end_slot = text_slot(end_slot.get());
                let use_menu = dropdown.get() && end_slot.is_none();
                let menu = call_menu();
                let has_label = !label.is_empty();
                let button = match (use_menu, has_label) {
                    (true, true) => view! {
                        <DtSplitButton
                            importance=importance
                            kind=kind
                            size=size
                            start_icon_position=icon_position
                            show_divider=show_divider
                            disabled=disabled
                            start_disabled=start_disabled
                            end_disabled=end_disabled
                            start_loading=start_loading
                            start_active=start_active
                            end_active=end_active
                            start_tooltip_text=start_tooltip
                            end_tooltip_text=end_tooltip
                            dropdown_placement=placement
                            width=width
                            start_href=start_href
                            start_target=start_target
                            start_rel=start_rel
                            assertive_on_focus=assertive
                            start_aria_label=start_aria.clone()
                            end_aria_label=end_aria.clone()
                            end_id=end_id.clone()
                            class=class_name
                            start_button_class=start_class
                            end_button_class=end_class
                            start_label_class=label_class_name
                            start_leading_class=leading_class_name
                            start_trailing_class=trailing_class_name
                            start_icon=start_icon
                            start_end_icon=start_end_icon
                            end_icon=end_icon
                            leading=leading
                            trailing=trailing
                            end=end_slot
                            dropdown_list=menu
                        >
                            {label}
                        </DtSplitButton>
                    }.into_any(),
                    (true, false) => view! {
                        <DtSplitButton
                            importance=importance
                            kind=kind
                            size=size
                            start_icon_position=icon_position
                            show_divider=show_divider
                            disabled=disabled
                            start_disabled=start_disabled
                            end_disabled=end_disabled
                            start_loading=start_loading
                            start_active=start_active
                            end_active=end_active
                            start_tooltip_text=start_tooltip
                            end_tooltip_text=end_tooltip
                            dropdown_placement=placement
                            width=width
                            start_href=start_href
                            start_target=start_target
                            start_rel=start_rel
                            assertive_on_focus=assertive
                            start_aria_label=start_aria.clone()
                            end_aria_label=end_aria.clone()
                            end_id=end_id.clone()
                            class=class_name
                            start_button_class=start_class
                            end_button_class=end_class
                            start_label_class=label_class_name
                            start_leading_class=leading_class_name
                            start_trailing_class=trailing_class_name
                            start_icon=start_icon
                            start_end_icon=start_end_icon
                            end_icon=end_icon
                            leading=leading
                            trailing=trailing
                            end=end_slot
                            dropdown_list=menu
                        />
                    }.into_any(),
                    (false, true) => view! {
                        <DtSplitButton
                            importance=importance
                            kind=kind
                            size=size
                            start_icon_position=icon_position
                            show_divider=show_divider
                            disabled=disabled
                            start_disabled=start_disabled
                            end_disabled=end_disabled
                            start_loading=start_loading
                            start_active=start_active
                            end_active=end_active
                            start_tooltip_text=start_tooltip
                            end_tooltip_text=end_tooltip
                            dropdown_placement=placement
                            width=width
                            start_href=start_href
                            start_target=start_target
                            start_rel=start_rel
                            assertive_on_focus=assertive
                            start_aria_label=start_aria.clone()
                            end_aria_label=end_aria.clone()
                            end_id=end_id.clone()
                            class=class_name
                            start_button_class=start_class
                            end_button_class=end_class
                            start_label_class=label_class_name
                            start_leading_class=leading_class_name
                            start_trailing_class=trailing_class_name
                            start_icon=start_icon
                            start_end_icon=start_end_icon
                            end_icon=end_icon
                            leading=leading
                            trailing=trailing
                            end=end_slot
                        >
                            {label}
                        </DtSplitButton>
                    }.into_any(),
                    (false, false) => view! {
                        <DtSplitButton
                            importance=importance
                            kind=kind
                            size=size
                            start_icon_position=icon_position
                            show_divider=show_divider
                            disabled=disabled
                            start_disabled=start_disabled
                            end_disabled=end_disabled
                            start_loading=start_loading
                            start_active=start_active
                            end_active=end_active
                            start_tooltip_text=start_tooltip
                            end_tooltip_text=end_tooltip
                            dropdown_placement=placement
                            width=width
                            start_href=start_href
                            start_target=start_target
                            start_rel=start_rel
                            assertive_on_focus=assertive
                            start_aria_label=start_aria.clone()
                            end_aria_label=end_aria.clone()
                            end_id=end_id.clone()
                            class=class_name
                            start_button_class=start_class
                            end_button_class=end_class
                            start_label_class=label_class_name
                            start_leading_class=leading_class_name
                            start_trailing_class=trailing_class_name
                            start_icon=start_icon
                            start_end_icon=start_end_icon
                            end_icon=end_icon
                            leading=leading
                            trailing=trailing
                            end=end_slot
                        />
                    }.into_any(),
                };
                view! { {button} }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
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
                ]
            />
            <SettingSelect label="Size" value=size options=SPLIT_SIZES.to_vec() />
            {move || {
                let show_position = !start_icon.get().is_empty() && start_end_icon.get().is_empty();
                show_position.then(|| view! {
                    <SettingSelect
                        label="Icon position"
                        value=icon_position
                        options=ICON_POSITIONS.to_vec()
                    />
                })
            }}
            {move || {
                (importance.get() == "clear").then(|| view! {
                    <SettingCheck label="Show divider" value=show_divider />
                })
            }}
            <SettingCheck label="Disabled" value=disabled />
            <SettingCheck label="Start disabled" value=start_disabled />
            <SettingCheck label="End disabled" value=end_disabled />
            <SettingCheck label="Start loading" value=start_loading />
            <SettingCheck label="Start active" value=start_active />
            <SettingCheck label="End active" value=end_active />
            <SettingText label="Start tooltip" value=start_tooltip />
            <SettingText label="Start aria label" value=start_aria />
            <SettingCheck label="Assertive on focus" value=assertive />
            <SettingText label="End tooltip" value=end_tooltip />
            <SettingText label="End aria label" value=end_aria />
            <SettingText label="End id" value=end_id />
            <SettingSelect
                label="Dropdown placement"
                value=placement
                options=vec![
                    ("bottom-end", "Bottom end"),
                    ("bottom-start", "Bottom start"),
                    ("top-end", "Top end"),
                    ("top-start", "Top start"),
                ]
            />
            <SettingText label="Width" value=width />
            <SettingText label="Start href" value=start_href />
            <SettingText label="Start target" value=start_target />
            <SettingText label="Start rel" value=start_rel />
        }
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingText label="Default" hint="Start button label." value=label />
            <SettingSelect label="Start icon" value=start_icon options=SPLIT_ICONS.to_vec() />
            <SettingSelect
                label="Start end icon"
                value=start_end_icon
                options=SPLIT_ICONS.to_vec()
            />
            <SettingSelect label="End icon" value=end_icon options=SPLIT_ICONS.to_vec() />
            <SettingText label="Leading" hint="Before the start label." value=leading />
            <SettingText label="Trailing" hint="After the start label." value=trailing />
            <SettingText label="End" hint="Replaces the end button." value=end_slot />
            <SettingCheck label="Dropdown list" value=dropdown />
        }
    });
    let classes = class_panel(vec![
        ClassField {
            label: "Class",
            hint: "On the split root. d-split-btn, d-split-btn--no-divider.",
            value: root_class,
            off: None,
        },
        ClassField {
            label: "Start button class",
            hint: "Extra class on the start button.",
            value: start_button_class,
            off: None,
        },
        ClassField {
            label: "End button class",
            hint: "Extra class on the end button.",
            value: end_button_class,
            off: None,
        },
        ClassField {
            label: "Label class",
            hint: "Extra class on the start label.",
            value: label_class,
            off: Some(Signal::derive(move || label.get().is_empty())),
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
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "importance", importance.get());
        push_str(&mut props, "kind", kind.get());
        push_str(&mut props, "size", size.get());
        if !start_icon.get().is_empty() && start_end_icon.get().is_empty() {
            push_str(&mut props, "start_icon_position", icon_position.get());
        }
        if !show_divider.get() {
            push_flag(&mut props, "show_divider", false);
        }
        push_bool(&mut props, "disabled", disabled.get());
        push_bool(&mut props, "start_disabled", start_disabled.get());
        push_bool(&mut props, "end_disabled", end_disabled.get());
        push_bool(&mut props, "start_loading", start_loading.get());
        push_bool(&mut props, "start_active", start_active.get());
        push_bool(&mut props, "end_active", end_active.get());
        push_str(&mut props, "start_tooltip_text", &start_tooltip.get());
        if end_slot.get().is_empty() {
            push_str(&mut props, "end_tooltip_text", &end_tooltip.get());
        }
        if dropdown.get() && end_slot.get().is_empty() {
            push_str(&mut props, "dropdown_placement", placement.get());
        }
        push_str(&mut props, "width", &width.get());
        push_str(&mut props, "start_href", &start_href.get());
        push_str(&mut props, "start_target", &start_target.get());
        push_str(&mut props, "start_rel", &start_rel.get());
        push_bool(&mut props, "assertive_on_focus", assertive.get());
        push_str(&mut props, "start_aria_label", &start_aria.get());
        push_str(&mut props, "end_aria_label", &end_aria.get());
        push_str(&mut props, "end_id", &end_id.get());
        push_str(&mut props, "class", &root_class.get());
        push_str(&mut props, "start_button_class", &start_button_class.get());
        push_str(&mut props, "end_button_class", &end_button_class.get());
        push_str(&mut props, "start_label_class", &label_class.get());
        push_str(&mut props, "start_leading_class", &leading_class.get());
        push_str(&mut props, "start_trailing_class", &trailing_class.get());
        if let Some(expr) = split_icon_expr(start_icon.get()) {
            push_expr(&mut props, "start_icon", format!("Some({expr})"));
        }
        if let Some(expr) = split_icon_expr(start_end_icon.get()) {
            push_expr(&mut props, "start_end_icon", format!("Some({expr})"));
        }
        if let Some(expr) = split_icon_expr(end_icon.get()) {
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
        if !end_slot.get().is_empty() {
            push_expr(
                &mut props,
                "end",
                format!(
                    "Some(ViewFn::from(|| view! {{ <span>{}</span> }}))",
                    rust_string(&end_slot.get())
                ),
            );
        }
        if dropdown.get() && end_slot.get().is_empty() {
            push_expr(
                &mut props,
                "dropdown_list",
                "ViewFn::from(|| view! {\n    <li class=\"d-list-item\" role=\"menuitem\" tabindex=\"-1\">\"Call back\"</li>\n    <li class=\"d-list-item\" role=\"menuitem\" tabindex=\"-1\">\"Call with video\"</li>\n    <li class=\"d-list-item\" role=\"menuitem\" tabindex=\"-1\">\"Add to contacts\"</li>\n})",
            );
        }
        let body = if label.get().is_empty() {
            None
        } else {
            Some(rust_string(&label.get()))
        };
        view_of(&element("DtSplitButton", &props, body.as_deref()))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">
                "The two buttons are one control. Clear can hide the divider. A start icon with no label needs a start tooltip. The end tooltip is required unless the end slot replaces that button."
            </p>
            <Play
                preview=preview
                settings=settings
                slots=slots
                classes=classes
                code=code
            />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Base" row=true>
                    <DtSplitButton end_tooltip_text="More calling options" dropdown_list=call_menu()>"Place call"</DtSplitButton>
                    <DtSplitButton importance="outlined" end_tooltip_text="More calling options" dropdown_list=call_menu()>"Place call"</DtSplitButton>
                    <DtSplitButton importance="clear" end_tooltip_text="More calling options" dropdown_list=call_menu()>"Place call"</DtSplitButton>
                </Variant>
                <Variant title="Critical" row=true>
                    <DtSplitButton kind="critical" end_tooltip_text="More calling options" dropdown_list=call_menu()>"Place call"</DtSplitButton>
                    <DtSplitButton kind="critical" importance="outlined" end_tooltip_text="More calling options" dropdown_list=call_menu()>"Place call"</DtSplitButton>
                    <DtSplitButton kind="critical" importance="clear" end_tooltip_text="More calling options" dropdown_list=call_menu()>"Place call"</DtSplitButton>
                </Variant>
                <Variant title="Positive" row=true>
                    <DtSplitButton kind="positive" end_tooltip_text="More calling options">"Place call"</DtSplitButton>
                    <DtSplitButton kind="positive" importance="outlined" end_tooltip_text="More calling options">"Place call"</DtSplitButton>
                    <DtSplitButton kind="positive" importance="clear" end_tooltip_text="More calling options">"Place call"</DtSplitButton>
                </Variant>
                <Variant title="Muted" row=true>
                    <DtSplitButton kind="muted" importance="outlined" end_tooltip_text="More calling options" dropdown_list=call_menu()>"Place call"</DtSplitButton>
                    <DtSplitButton kind="muted" importance="clear" end_tooltip_text="More calling options" dropdown_list=call_menu()>"Place call"</DtSplitButton>
                </Variant>
                <Variant title="Disabled" row=true>
                    <DtSplitButton disabled=true end_tooltip_text="More calling options">"Place call"</DtSplitButton>
                    <DtSplitButton start_disabled=true end_tooltip_text="More calling options">"Place call"</DtSplitButton>
                    <DtSplitButton end_disabled=true end_tooltip_text="More calling options">"Place call"</DtSplitButton>
                </Variant>
                <Variant title="Active" row=true>
                    <DtSplitButton start_active=true end_tooltip_text="More calling options">"Place call"</DtSplitButton>
                    <DtSplitButton end_active=true end_tooltip_text="More calling options">"Place call"</DtSplitButton>
                </Variant>
                <Variant title="No divider" row=true>
                    <DtSplitButton importance="clear" show_divider=false end_tooltip_text="More calling options">"Place call"</DtSplitButton>
                </Variant>
                <Variant title="Sizes" row=true>
                    <DtSplitButton size="100" end_tooltip_text="More calling options">"100"</DtSplitButton>
                    <DtSplitButton size="200" end_tooltip_text="More calling options">"200"</DtSplitButton>
                    <DtSplitButton size="300" end_tooltip_text="More calling options">"300"</DtSplitButton>
                    <DtSplitButton size="400" end_tooltip_text="More calling options">"400"</DtSplitButton>
                    <DtSplitButton size="500" end_tooltip_text="More calling options">"500"</DtSplitButton>
                </Variant>
                <Variant title="Loading" row=true>
                    <DtSplitButton start_loading=true end_tooltip_text="More calling options">"Place call"</DtSplitButton>
                    <DtSplitButton importance="outlined" start_loading=true end_tooltip_text="More calling options">"Place call"</DtSplitButton>
                    <DtSplitButton importance="clear" start_loading=true end_tooltip_text="More calling options">"Place call"</DtSplitButton>
                </Variant>
                <Variant title="Navigation" row=true>
                    <DtSplitButton
                        start_href="https://dialtone.dialpad.com"
                        start_target="_blank"
                        start_rel="noopener noreferrer"
                        end_tooltip_text="More calling options"
                    >
                        "Docs"
                    </DtSplitButton>
                </Variant>
                <Variant title="Icon and label" row=true>
                    <DtSplitButton start_icon=Some(phone()) end_tooltip_text="More calling options">"Call"</DtSplitButton>
                    <DtSplitButton start_icon_position="top" start_icon=Some(phone()) end_tooltip_text="More calling options">"Call"</DtSplitButton>
                    <DtSplitButton start_icon_position="right" start_icon=Some(phone()) end_tooltip_text="More calling options">"Call"</DtSplitButton>
                    <DtSplitButton start_icon_position="bottom" start_icon=Some(phone()) end_tooltip_text="More calling options">"Call"</DtSplitButton>
                </Variant>
                <Variant title="Dual icons" row=true>
                    <DtSplitButton
                        start_icon=Some(phone())
                        start_end_icon=Some(chevron())
                        end_tooltip_text="More calling options"
                    >
                        "Call"
                    </DtSplitButton>
                </Variant>
                <Variant title="Custom end icon" row=true>
                    <DtSplitButton
                        end_icon=Some(more_icon())
                        end_tooltip_text="More calling options"
                    >
                        "Place call"
                    </DtSplitButton>
                </Variant>
                <Variant title="Icon only" row=true>
                    <DtSplitButton
                        start_tooltip_text="Place call"
                        end_tooltip_text="More calling options"
                        start_icon=Some(phone())
                    />
                </Variant>
                <Variant title="Leading and trailing" row=true>
                    <DtSplitButton
                        end_tooltip_text="More calling options"
                        leading=Some(count_badge())
                        trailing=Some(shortcut())
                    >
                        "Place call"
                    </DtSplitButton>
                </Variant>
            </div>
        </section>
    }
}
