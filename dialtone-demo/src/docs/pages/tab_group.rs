use super::*;

fn sync_selected(selected: RwSignal<&'static str>, next: &str) {
    const IDS: [&str; 3] = ["overview", "people", "activity"];
    if let Some(id) = IDS.into_iter().find(|id| *id == next) {
        selected.set(id);
    }
}

fn tab_buttons() -> ViewFn {
    ViewFn::from(|| {
        view! {
            <DtTab id="overview" panel_id="overview">"Overview"</DtTab>
            <DtTab id="people" panel_id="people">"People"</DtTab>
            <DtTab id="activity" panel_id="activity">"Activity"</DtTab>
        }
    })
}

macro_rules! tab_group_preview {
    (
        $label:expr,
        $selected:expr,
        $orientation:expr,
        $size:expr,
        $kind:expr,
        $spread:expr,
        $activation:expr,
        $borderless:expr,
        $outlined:expr,
        $disabled:expr,
        $inverted:expr,
        $transition:expr,
        $tabs_class:expr,
        $tabs:expr,
        $on_change:expr,
        $tab_list:expr,
        $child:expr
    ) => {{
        let tab_list: String = $tab_list;
        let child: String = $child;
        let list_empty = tab_list.is_empty();
        let child_empty = child.is_empty();
        match (list_empty, child_empty) {
            (true, true) => view! {
                <DtTabGroup
                    label=$label
                    selected=$selected
                    orientation=$orientation
                    size=$size
                    kind=$kind
                    spread=$spread
                    activation_mode=$activation
                    borderless=$borderless
                    outlined=$outlined
                    disabled=$disabled
                    inverted=$inverted
                    show_indicator_transition=$transition
                    tabs_class=$tabs_class
                    tabs=$tabs
                    on_change=$on_change
                >
                    <DtTabPanel id="overview" tab_id="overview">
                        <p class="m-0 text-sm">"Account overview"</p>
                    </DtTabPanel>
                    <DtTabPanel id="people" tab_id="people">
                        <p class="m-0 text-sm">"People on this team"</p>
                    </DtTabPanel>
                    <DtTabPanel id="activity" tab_id="activity">
                        <p class="m-0 text-sm">"Recent activity"</p>
                    </DtTabPanel>
                </DtTabGroup>
            }
            .into_any(),
            (true, false) => view! {
                <DtTabGroup
                    label=$label
                    selected=$selected
                    orientation=$orientation
                    size=$size
                    kind=$kind
                    spread=$spread
                    activation_mode=$activation
                    borderless=$borderless
                    outlined=$outlined
                    disabled=$disabled
                    inverted=$inverted
                    show_indicator_transition=$transition
                    tabs_class=$tabs_class
                    tabs=$tabs
                    on_change=$on_change
                >
                    <p class="m-0 text-sm">{child.clone()}</p>
                </DtTabGroup>
            }
            .into_any(),
            (false, true) => view! {
                <DtTabGroup
                    label=$label
                    selected=$selected
                    orientation=$orientation
                    size=$size
                    kind=$kind
                    spread=$spread
                    activation_mode=$activation
                    borderless=$borderless
                    outlined=$outlined
                    disabled=$disabled
                    inverted=$inverted
                    show_indicator_transition=$transition
                    tabs_class=$tabs_class
                    tab_list_class=tab_list
                    tabs=$tabs
                    on_change=$on_change
                >
                    <DtTabPanel id="overview" tab_id="overview">
                        <p class="m-0 text-sm">"Account overview"</p>
                    </DtTabPanel>
                    <DtTabPanel id="people" tab_id="people">
                        <p class="m-0 text-sm">"People on this team"</p>
                    </DtTabPanel>
                    <DtTabPanel id="activity" tab_id="activity">
                        <p class="m-0 text-sm">"Recent activity"</p>
                    </DtTabPanel>
                </DtTabGroup>
            }
            .into_any(),
            (false, false) => view! {
                <DtTabGroup
                    label=$label
                    selected=$selected
                    orientation=$orientation
                    size=$size
                    kind=$kind
                    spread=$spread
                    activation_mode=$activation
                    borderless=$borderless
                    outlined=$outlined
                    disabled=$disabled
                    inverted=$inverted
                    show_indicator_transition=$transition
                    tabs_class=$tabs_class
                    tab_list_class=tab_list
                    tabs=$tabs
                    on_change=$on_change
                >
                    <p class="m-0 text-sm">{child.clone()}</p>
                </DtTabGroup>
            }
            .into_any(),
        }
    }};
}

#[component]
pub(crate) fn TabGroupPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("tab_group.rs")));
    let selected = RwSignal::new("overview");
    let selected_id = Signal::derive(move || selected.get().to_string());
    let label = RwSignal::new("Sections".to_string());
    let orientation = RwSignal::new("horizontal");
    let size = RwSignal::new("300");
    let kind = RwSignal::new("default");
    let spread = RwSignal::new("none");
    let activation = RwSignal::new("manual");
    let borderless = RwSignal::new(false);
    let outlined = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    let inverted = RwSignal::new(false);
    let show_transition = RwSignal::new(true);
    let tabs_slot = RwSignal::new(String::new());
    let child_slot = RwSignal::new(String::new());
    let tabs_class = RwSignal::new(String::new());
    let tab_list_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let label = label.get();
                let orientation = orientation.get();
                let size = size.get();
                let kind = kind.get();
                let spread = spread.get();
                let activation = activation.get();
                let borderless = borderless.get();
                let outlined = outlined.get();
                let disabled = disabled.get();
                let inverted = inverted.get();
                let show_transition = show_transition.get();
                let tabs_class = tabs_class.get();
                let tab_list = tab_list_class.get();
                let tabs_text = tabs_slot.get();
                let child_text = child_slot.get();
                let tabs = if tabs_text.is_empty() {
                    tab_buttons()
                } else {
                    let text = tabs_text;
                    ViewFn::from(move || {
                        let text = text.clone();
                        view! { <span>{text}</span> }
                    })
                };
                tab_group_preview!(
                    label,
                    selected_id,
                    orientation,
                    size,
                    kind,
                    spread,
                    activation,
                    borderless,
                    outlined,
                    disabled,
                    inverted,
                    show_transition,
                    tabs_class,
                    tabs,
                    move |next: DtTabGroupChange| sync_selected(selected, &next.selected),
                    tab_list,
                    child_text
                )
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingSelect
                label="Selected"
                value=selected
                options=vec![
                    ("overview", "Overview"),
                    ("people", "People"),
                    ("activity", "Activity"),
                ]
            />
            <SettingSelect
                label="Orientation"
                value=orientation
                options=vec![("horizontal", "Horizontal"), ("vertical", "Vertical")]
            />
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
            <SettingSelect
                label="Kind"
                value=kind
                options=vec![("default", "Default"), ("muted", "Muted")]
            />
            <SettingSelect
                label="Spread"
                value=spread
                options=vec![("none", "None"), ("grow", "Grow"), ("equal", "Equal")]
            />
            <SettingSelect
                label="Activation"
                value=activation
                options=vec![("manual", "Manual"), ("auto", "Auto")]
            />
            <SettingText label="Label" value=label />
            <SettingCheck label="Borderless" value=borderless />
            <SettingCheck label="Outlined" value=outlined />
            <SettingCheck label="Disabled" value=disabled />
            <SettingCheck label="Inverted (Deprecated)" value=inverted />
            <SettingCheck label="Show indicator transition" value=show_transition />
        }
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingText label="Tabs" hint="Replaces the three tabs." value=tabs_slot />
            <SettingText label="Default" hint="Replaces the three panels." value=child_slot />
        }
    });
    let classes = class_panel(vec![
        ClassField {
            label: "Tabs class",
            hint: "Extra class on the tab list.",
            value: tabs_class,
            off: None,
        },
        ClassField {
            label: "Tab list class",
            hint: "Alias of tabs class. A set value wins.",
            value: tab_list_class,
            off: None,
        },
    ]);
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "label", &label.get());
        push_expr(&mut props, "selected", "selected");
        push_str(&mut props, "orientation", orientation.get());
        push_str(&mut props, "size", size.get());
        push_str(&mut props, "kind", kind.get());
        push_str(&mut props, "spread", spread.get());
        push_str(&mut props, "activation_mode", activation.get());
        push_bool(&mut props, "borderless", borderless.get());
        push_bool(&mut props, "outlined", outlined.get());
        push_bool(&mut props, "disabled", disabled.get());
        push_bool(&mut props, "inverted", inverted.get());
        if !show_transition.get() {
            push_expr(&mut props, "show_indicator_transition", "false");
        }
        push_str(&mut props, "tabs_class", &tabs_class.get());
        push_str(&mut props, "tab_list_class", &tab_list_class.get());
        push_expr(
            &mut props,
            "on_change",
            "move |next: DtTabGroupChange| sync_selected(selected, &next.selected)",
        );
        let tab = |id: &str, label: &str| {
            element(
                "DtTab",
                &[("id", rust_string(id)), ("panel_id", rust_string(id))],
                Some(&rust_string(label)),
            )
        };
        let tabs_body = [
            tab("overview", "Overview"),
            tab("people", "People"),
            tab("activity", "Activity"),
        ]
        .join("\n");
        if tabs_slot.get().is_empty() {
            push_expr(
                &mut props,
                "tabs",
                format!("ViewFn::from(|| view! {{ {tabs_body} }})"),
            );
        } else {
            push_span_slot(&mut props, "tabs", &tabs_slot.get());
        }
        let panel = |id: &str, body: &str| {
            element(
                "DtTabPanel",
                &[("id", rust_string(id)), ("tab_id", rust_string(id))],
                Some(&rust_string(body)),
            )
        };
        let panels = if child_slot.get().is_empty() {
            [
                panel("overview", "Account overview"),
                panel("people", "People on this team"),
                panel("activity", "Recent activity"),
            ]
            .join("\n")
        } else {
            format!(
                "<p class=\"m-0 text-sm\">{}</p>",
                rust_string(&child_slot.get())
            )
        };
        view_of(&element("DtTabGroup", &props, Some(&panels)))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">
                "The selection indicator does not animate. A panel tabindex is 0 unless its first element child is focusable. Vertical orientation ignores spread."
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
                <Variant title="Vertical">
                    <FixedGroup orientation="vertical" kind="default" outlined=false />
                </Variant>
                <Variant title="Muted">
                    <FixedGroup orientation="horizontal" kind="muted" outlined=false />
                </Variant>
                <Variant title="Outlined">
                    <FixedGroup orientation="horizontal" kind="default" outlined=true />
                </Variant>
            </div>
        </section>
    }
}

#[component]
fn FixedGroup(orientation: &'static str, kind: &'static str, outlined: bool) -> impl IntoView {
    let selected = RwSignal::new("overview");
    let selected_id = Signal::derive(move || selected.get().to_string());
    view! {
        <DtTabGroup
            label="Sections"
            orientation=orientation
            kind=kind
            outlined=outlined
            selected=selected_id
            tabs=tab_buttons()
            on_change=move |next: DtTabGroupChange| sync_selected(selected, &next.selected)
        >
            <DtTabPanel id="overview" tab_id="overview">
                <p class="m-0 text-sm">"Account overview"</p>
            </DtTabPanel>
            <DtTabPanel id="people" tab_id="people">
                <p class="m-0 text-sm">"People on this team"</p>
            </DtTabPanel>
            <DtTabPanel id="activity" tab_id="activity">
                <p class="m-0 text-sm">"Recent activity"</p>
            </DtTabPanel>
        </DtTabGroup>
    }
}
