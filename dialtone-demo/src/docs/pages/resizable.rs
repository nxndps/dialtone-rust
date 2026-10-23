use super::*;
use dialtone::{DtResizableCollapseRule, DtResizableMessages, DtResizablePanelConfig};

const STRATEGIES: &[(&str, &str)] = &[
    ("proportional", "Proportional"),
    ("preserve-manual", "Preserve manual"),
];

const OFFSET_EDGES: &[(&str, &str)] = &[("start", "Start"), ("end", "End"), ("both", "Both")];

fn panel_configs(raw: &str) -> Vec<DtResizablePanelConfig> {
    raw.split(',')
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(DtResizablePanelConfig::new)
        .collect()
}

fn collapse_rules(raw: &str) -> Vec<DtResizableCollapseRule> {
    raw.split(',')
        .filter_map(|part| {
            let part = part.trim();
            if part.is_empty() {
                return None;
            }
            let mut bits = part.split(':');
            let panel_id = bits.next().unwrap_or("").trim().to_string();
            if panel_id.is_empty() {
                return None;
            }
            let priority = bits
                .next()
                .and_then(|value| value.trim().parse::<i32>().ok())
                .unwrap_or(0);
            let min_size = bits
                .next()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_string);
            Some(DtResizableCollapseRule {
                panel_id,
                priority,
                min_size_before_collapse: min_size,
            })
        })
        .collect()
}

fn rust_f64(value: f64) -> String {
    if value.fract() == 0.0 && value.is_finite() {
        format!("{value:.1}")
    } else {
        value.to_string()
    }
}

fn panels_expr(raw: &str) -> String {
    let items = raw
        .split(',')
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(|id| format!("DtResizablePanelConfig::new({})", rust_string(id)))
        .collect::<Vec<_>>()
        .join(", ");
    format!("vec![{items}]")
}

fn rules_expr(rules: &[DtResizableCollapseRule]) -> String {
    let items = rules
        .iter()
        .map(|rule| {
            let min_size = match &rule.min_size_before_collapse {
                Some(size) => format!("Some({}.to_string())", rust_string(size)),
                None => "None".to_string(),
            };
            format!(
                "DtResizableCollapseRule {{ panel_id: {}.to_string(), priority: {}, min_size_before_collapse: {min_size} }}",
                rust_string(&rule.panel_id),
                rule.priority
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("vec![{items}]")
}

fn messages_expr(messages: &DtResizableMessages) -> String {
    format!(
        "DtResizableMessages {{ resize_announcement: {}.to_string(), collapse_announcement: {}.to_string(), expand_announcement: {}.to_string(), reset_announcement: {}.to_string(), aria_value_text: {}.to_string(), handle_aria_label: {}.to_string() }}",
        rust_string(&messages.resize_announcement),
        rust_string(&messages.collapse_announcement),
        rust_string(&messages.expand_announcement),
        rust_string(&messages.reset_announcement),
        rust_string(&messages.aria_value_text),
        rust_string(&messages.handle_aria_label),
    )
}

#[component]
pub(crate) fn ResizablePage() -> impl IntoView {
    provide_context(VariantSource(include_str!("resizable.rs")));
    let direction = RwSignal::new("row");
    let direction_signal = Signal::derive(move || direction.get().to_string());
    let storage_key = RwSignal::new(String::new());
    let panels_text = RwSignal::new(String::new());
    let strategy = RwSignal::new("proportional");
    let rules_text = RwSignal::new(String::new());
    let offset_element = RwSignal::new(String::new());
    let offset_amount = RwSignal::new(String::new());
    let offset_direction = RwSignal::new("start");
    let resize_announcement = RwSignal::new(String::new());
    let collapse_announcement = RwSignal::new(String::new());
    let expand_announcement = RwSignal::new(String::new());
    let reset_announcement = RwSignal::new(String::new());
    let aria_value_text = RwSignal::new(String::new());
    let handle_aria_label = RwSignal::new(String::new());
    let default_slot = RwSignal::new(String::new());
    let root_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            <div class="w-full" style="block-size:240px">
                {move || {
                    let class_name = root_class.get();
                    let storage_key = storage_key.get();
                    let panels = panel_configs(&panels_text.get());
                    let strategy = strategy.get();
                    let rules = collapse_rules(&rules_text.get());
                    let offset_element = offset_element.get();
                    let amount = offset_amount.get().trim().parse::<f64>().unwrap_or(0.0);
                    let edge = offset_direction.get();
                    let messages = DtResizableMessages {
                        resize_announcement: resize_announcement.get(),
                        collapse_announcement: collapse_announcement.get(),
                        expand_announcement: expand_announcement.get(),
                        reset_announcement: reset_announcement.get(),
                        aria_value_text: aria_value_text.get(),
                        handle_aria_label: handle_aria_label.get(),
                    };
                    let custom = default_slot.get();
                    view! {
                        <DtResizable
                            direction=direction_signal
                            storage_key=storage_key
                            class=class_name
                            panels=panels
                            space_allocation_strategy=strategy
                            collapse_rules=rules
                            offset_element=offset_element
                            offset_amount=amount
                            offset_direction=edge
                            messages=messages
                        >
                            {if custom.is_empty() {
                                vec![
                                    view! {
                                        <DtResizablePanel id="one">
                                            <p class="m-0 p-4 text-sm">"One"</p>
                                        </DtResizablePanel>
                                    }.into_any(),
                                    view! {
                                        <DtResizableHandle before_panel_id="one" after_panel_id="two" />
                                    }.into_any(),
                                    view! {
                                        <DtResizablePanel id="two">
                                            <p class="m-0 p-4 text-sm">"Two"</p>
                                        </DtResizablePanel>
                                    }.into_any(),
                                ].collect_view()
                            } else {
                                vec![custom.into_any()].collect_view()
                            }}
                        </DtResizable>
                    }
                }}
            </div>
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingSelect
                label="Direction"
                value=direction
                options=vec![("row", "Row"), ("column", "Column")]
            />
            <SettingText label="Storage key" hint="Empty does not persist sizes." value=storage_key />
            <SettingText label="Panels" hint="Comma-separated ids. Accepted and not applied." value=panels_text />
            <SettingSelect label="Space allocation" value=strategy options=STRATEGIES.to_vec() />
            <SettingText
                label="Collapse rules"
                hint="id:priority or id:priority:minSize. Comma-separated."
                value=rules_text
            />
            <SettingText label="Offset element" hint="CSS selector. Empty uses none." value=offset_element />
            <SettingText
                label="Offset amount"
                hint="Pixels. Wins over Offset element when greater than 0. Empty uses none."
                value=offset_amount
            />
            <SettingSelect label="Offset direction" value=offset_direction options=OFFSET_EDGES.to_vec() />
            <SettingText label="Resize announcement" hint="Messages. Empty keeps the English default." value=resize_announcement />
            <SettingText label="Collapse announcement" value=collapse_announcement />
            <SettingText label="Expand announcement" value=expand_announcement />
            <SettingText label="Reset announcement" value=reset_announcement />
            <SettingText label="Aria value text" value=aria_value_text />
            <SettingText label="Handle label" value=handle_aria_label />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "direction", direction.get());
        push_str(&mut props, "storage_key", &storage_key.get());
        push_str(&mut props, "class", &root_class.get());
        let panels = panels_text.get();
        if !panels.trim().is_empty() {
            push_expr(&mut props, "panels", panels_expr(&panels));
        }
        if strategy.get() != "proportional" {
            push_str(&mut props, "space_allocation_strategy", strategy.get());
        }
        let rules = collapse_rules(&rules_text.get());
        if !rules.is_empty() {
            push_expr(&mut props, "collapse_rules", rules_expr(&rules));
        }
        push_str(&mut props, "offset_element", &offset_element.get());
        if let Ok(amount) = offset_amount.get().trim().parse::<f64>() {
            push_expr(&mut props, "offset_amount", rust_f64(amount));
        }
        if offset_direction.get() != "start" {
            push_str(&mut props, "offset_direction", offset_direction.get());
        }
        let messages = DtResizableMessages {
            resize_announcement: resize_announcement.get(),
            collapse_announcement: collapse_announcement.get(),
            expand_announcement: expand_announcement.get(),
            reset_announcement: reset_announcement.get(),
            aria_value_text: aria_value_text.get(),
            handle_aria_label: handle_aria_label.get(),
        };
        if !messages.resize_announcement.is_empty()
            || !messages.collapse_announcement.is_empty()
            || !messages.expand_announcement.is_empty()
            || !messages.reset_announcement.is_empty()
            || !messages.aria_value_text.is_empty()
            || !messages.handle_aria_label.is_empty()
        {
            push_expr(&mut props, "messages", messages_expr(&messages));
        }
        let custom = default_slot.get();
        let body = if custom.is_empty() {
            let one = element(
                "DtResizablePanel",
                &[("id", rust_string("one"))],
                Some("\"One\""),
            );
            let handle = element(
                "DtResizableHandle",
                &[
                    ("before_panel_id", rust_string("one")),
                    ("after_panel_id", rust_string("two")),
                ],
                None,
            );
            let two = element(
                "DtResizablePanel",
                &[("id", rust_string("two"))],
                Some("\"Two\""),
            );
            [one, handle, two].join("\n")
        } else {
            rust_string(&custom)
        };
        view_of(&element("DtResizable", &props, Some(&body)))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">
                "Drag the handle. Arrow keys move a focused handle by 8px, Shift by 24px, and Ctrl or Meta by 1px. Changing direction keeps the panels mounted."
            </p>
            <Play
                stack=true
                preview=preview
                settings=settings
                slots=ViewFn::from(move || view! {
                    <SettingText label="Default" hint="Default slot. Empty keeps the two panels." value=default_slot />
                })
                classes=class_panel(vec![ClassField {
                    label: "Class",
                    hint: ROOT_CLASS,
                    value: root_class,
                    off: None,
                }])
                code=code
            />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Column">
                    <ColumnPanels />
                </Variant>
                <Variant title="Three panels">
                    <ThreePanels />
                </Variant>
                <Variant title="Minimum size">
                    <MinimumPanels />
                </Variant>
                <Variant title="Collapsible">
                    <CollapsiblePanels />
                </Variant>
            </div>
        </section>
    }
}

#[component]
fn ColumnPanels() -> impl IntoView {
    view! {
        <div class="w-full" style="block-size:240px">
            <DtResizable direction="column">
                <DtResizablePanel id="top">
                    <p class="m-0 p-4 text-sm">"Top"</p>
                </DtResizablePanel>
                <DtResizableHandle before_panel_id="top" after_panel_id="bottom" />
                <DtResizablePanel id="bottom">
                    <p class="m-0 p-4 text-sm">"Bottom"</p>
                </DtResizablePanel>
            </DtResizable>
        </div>
    }
}

#[component]
fn ThreePanels() -> impl IntoView {
    view! {
        <div class="w-full" style="block-size:240px">
            <DtResizable>
                <DtResizablePanel id="start">
                    <p class="m-0 p-4 text-sm">"Start"</p>
                </DtResizablePanel>
                <DtResizableHandle before_panel_id="start" after_panel_id="middle" />
                <DtResizablePanel id="middle">
                    <p class="m-0 p-4 text-sm">"Middle"</p>
                </DtResizablePanel>
                <DtResizableHandle before_panel_id="middle" after_panel_id="end" />
                <DtResizablePanel id="end">
                    <p class="m-0 p-4 text-sm">"End"</p>
                </DtResizablePanel>
            </DtResizable>
        </div>
    }
}

#[component]
fn MinimumPanels() -> impl IntoView {
    view! {
        <div class="w-full" style="block-size:240px">
            <DtResizable>
                <DtResizablePanel id="narrow" user_min_size="200">
                    <p class="m-0 p-4 text-sm">"Stops at layout 200"</p>
                </DtResizablePanel>
                <DtResizableHandle before_panel_id="narrow" after_panel_id="wide" />
                <DtResizablePanel id="wide">
                    <p class="m-0 p-4 text-sm">"Wide"</p>
                </DtResizablePanel>
            </DtResizable>
        </div>
    }
}

#[component]
fn CollapsiblePanels() -> impl IntoView {
    let collapsed = RwSignal::new(false);
    view! {
        <div class="w-full" style="block-size:240px">
            <DtResizable>
                <DtResizablePanel id="side" initial_size="25p" collapsible=true collapsed=collapsed>
                    <p class="m-0 p-4 text-sm">"Side"</p>
                </DtResizablePanel>
                <DtResizableHandle before_panel_id="side" after_panel_id="main" />
                <DtResizablePanel id="main">
                    <p class="m-0 p-4 text-sm">"Main"</p>
                </DtResizablePanel>
            </DtResizable>
        </div>
        <DtButton
            class="mt-2"
            size="sm"
            importance="outlined"
            on_click=move || collapsed.update(|value| *value = !*value)
        >
            "Toggle side"
        </DtButton>
    }
}
