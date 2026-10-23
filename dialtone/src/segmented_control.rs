use std::sync::atomic::{AtomicU32, Ordering};

use leptos::context::Provider;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, KeyboardEvent};

use super::button::DtButton;
#[cfg(test)]
use super::stack::stack_classes;
use super::stack::{DtStack, StackAt};
use super::unique_id;

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn next_item_id() -> u32 {
    static NEXT: AtomicU32 = AtomicU32::new(1);
    NEXT.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone)]
struct SegmentedEntry {
    id: u32,
    value: String,
    disabled: bool,
}

#[derive(Clone, Copy)]
struct SegmentedGroup {
    value: Signal<String>,
    select: Callback<String>,
    disabled: bool,
    button_size: &'static str,
    label_class: StoredValue<String>,
    focused: RwSignal<Option<String>>,
    fallback: RwSignal<Option<String>>,
    registry: RwSignal<Vec<SegmentedEntry>>,
}

fn known_size(size: &str) -> bool {
    matches!(
        size,
        "100" | "200" | "300" | "400" | "500" | "xs" | "sm" | "md" | "lg" | "xl"
    )
}

fn size_modifier(size: &str) -> &'static str {
    match size {
        "100" | "xs" => "d-segmented-control--xs",
        "200" | "sm" => "d-segmented-control--sm",
        "300" | "md" => "d-segmented-control--md",
        "400" | "lg" => "d-segmented-control--lg",
        "500" | "xl" => "d-segmented-control--xl",
        _ => "",
    }
}

/// `DtButton` size for a segmented item. Numeric tokens map to t-shirt sizes.
pub(crate) fn segmented_button_size(size: &str) -> &'static str {
    match size {
        "100" | "xs" => "xs",
        "300" | "md" => "md",
        "400" | "lg" => "lg",
        "500" | "xl" => "xl",
        _ => "sm",
    }
}

/// Segmented classes in the Vue computed order.
pub(crate) fn segmented_classes(
    size: &str,
    show_divider: bool,
    borderless: bool,
    orientation: &str,
    spread: &str,
) -> String {
    if !known_size(size) {
        leptos::logging::warn!(
            "DtSegmentedControl size `{size}` is not 100–500 or xs, sm, md, lg, xl"
        );
    }
    if !matches!(orientation, "horizontal" | "vertical") {
        leptos::logging::warn!(
            "DtSegmentedControl orientation `{orientation}` is not horizontal or vertical"
        );
    }
    if !matches!(spread, "grow" | "evenly") {
        leptos::logging::warn!("DtSegmentedControl spread `{spread}` is not grow or evenly");
    }
    join_class([
        "d-segmented-control",
        size_modifier(size),
        if show_divider {
            ""
        } else {
            "d-segmented-control--hide-divider"
        },
        if borderless {
            "d-segmented-control--borderless"
        } else {
            ""
        },
        if orientation == "vertical" {
            "d-segmented-control--vertical"
        } else {
            ""
        },
        if spread == "evenly" && orientation != "vertical" {
            "d-segmented-control--spread-evenly"
        } else {
            ""
        },
    ])
}

/// Stack classes plus segmented classes, which is the root `class` string.
#[cfg(test)]
pub(crate) fn segmented_root_class(
    size: &str,
    show_divider: bool,
    borderless: bool,
    orientation: &str,
    spread: &str,
) -> String {
    let direction = if orientation == "vertical" {
        StackAt::all("column")
    } else {
        StackAt::all("row")
    };
    join_class([
        stack_classes(
            direction,
            StackAt::all("0"),
            Some(StackAt::all("stretch")),
            StackAt::all("start"),
        ),
        segmented_classes(size, show_divider, borderless, orientation, spread),
    ])
}

/// Next enabled index. `direction` is `1` or `-1`. Wraps. Returns `from` when every item is disabled.
///
/// Home is `next_enabled(disabled, len - 1, 1)`. End is `next_enabled(disabled, 0, -1)`.
pub(crate) fn next_enabled(disabled: &[bool], from: usize, direction: i32) -> usize {
    let len = disabled.len();
    if len == 0 {
        return from;
    }
    let len_i = len as i32;
    let from_i = from as i32;
    for step in 1..=len_i {
        let index = (from_i + step * direction).rem_euclid(len_i) as usize;
        if !disabled[index] {
            return index;
        }
    }
    from
}

/// Arrow, Home, and End target. `None` when the key does not navigate this orientation.
pub(crate) fn navigation_index(
    key: &str,
    horizontal: bool,
    current: usize,
    disabled: &[bool],
) -> Option<usize> {
    if disabled.is_empty() {
        return None;
    }
    let next_key = if horizontal {
        "ArrowRight"
    } else {
        "ArrowDown"
    };
    let prev_key = if horizontal { "ArrowLeft" } else { "ArrowUp" };
    let len = disabled.len();
    match key {
        key if key == next_key => Some(next_enabled(disabled, current, 1)),
        key if key == prev_key => Some(next_enabled(disabled, current, -1)),
        "Home" => Some(next_enabled(disabled, len - 1, 1)),
        "End" => Some(next_enabled(disabled, 0, -1)),
        _ => None,
    }
}

/// Value that receives tabindex 0 when the selected item is missing or disabled.
pub(crate) fn tab_fallback(selected: &str, values: &[String], disabled: &[bool]) -> Option<String> {
    let selected_enabled = values
        .iter()
        .zip(disabled)
        .any(|(value, disabled)| value == selected && !disabled);
    if selected_enabled {
        None
    } else {
        values
            .iter()
            .zip(disabled)
            .find(|(_, disabled)| !**disabled)
            .map(|(value, _)| value.clone())
    }
}

/// Tab index for one item. The selected enabled item, or the fallback, is `0`.
pub(crate) fn item_tab_index(selected: bool, disabled: bool, fallback: bool) -> i32 {
    if !disabled && (selected || fallback) {
        0
    } else {
        -1
    }
}

fn radio_index(event: &KeyboardEvent, items: &[Element]) -> Option<usize> {
    let target = event.target()?;
    let mut current: Element = target.dyn_into().ok()?;
    loop {
        if current.get_attribute("role").as_deref() == Some("radio") {
            return items
                .iter()
                .position(|item| item.is_same_node(Some(current.as_ref())));
        }
        current = current.parent_element()?;
    }
}

fn on_segment_key(
    event: KeyboardEvent,
    horizontal: bool,
    activation_auto: bool,
    select: Callback<String>,
    focused: RwSignal<Option<String>>,
) {
    let Some(current) = event.current_target() else {
        return;
    };
    let Ok(root) = current.dyn_into::<Element>() else {
        return;
    };
    let Ok(nodes) = root.query_selector_all("[role=\"radio\"]") else {
        return;
    };
    let mut items = Vec::new();
    for index in 0..nodes.length() {
        if let Some(node) = nodes.item(index) {
            if let Ok(element) = node.dyn_into::<Element>() {
                items.push(element);
            }
        }
    }
    if items.is_empty() {
        return;
    }
    let disabled: Vec<bool> = items
        .iter()
        .map(|item| item.get_attribute("aria-disabled").as_deref() == Some("true"))
        .collect();
    let focused_index = radio_index(&event, &items)
        .or_else(|| {
            let value = focused.get_untracked()?;
            items.iter().position(|item| {
                item.get_attribute("data-value").as_deref() == Some(value.as_str())
            })
        })
        .or_else(|| {
            items
                .iter()
                .position(|item| item.get_attribute("aria-checked").as_deref() == Some("true"))
        })
        .unwrap_or(0);
    let key = event.key();
    if key == "Enter" || key == " " {
        event.prevent_default();
        event.stop_propagation();
        if !disabled[focused_index] {
            if let Some(value) = items[focused_index].get_attribute("data-value") {
                select.run(value);
            }
        }
        return;
    }
    let Some(next) = navigation_index(&key, horizontal, focused_index, &disabled) else {
        return;
    };
    event.prevent_default();
    event.stop_propagation();
    if let Ok(element) = items[next].clone().dyn_into::<HtmlElement>() {
        let _ = element.focus();
    }
    if activation_auto && !disabled[next] {
        if let Some(value) = items[next].get_attribute("data-value") {
            select.run(value);
        }
    }
}

/// Dialtone Vue `DtSegmentedControl` (staging `a46ac063`).
///
/// Absent from tag `dialtone/v9.190.0`. `d-segmented-control*` is in the
/// vendored Dialtone 10 CSS. `show_indicator_transition` is accepted and does not animate.
///
/// The root is [`DtStack`] with `data-qa="dt-segmented-control"`. `value` /
/// `on_change` are Vue `modelValue`, `update:modelValue`, and `change`.
/// `on_before_change` runs first; return `true` to cancel (Vue `preventDefault`).
/// User selection emits `on_change`. Programmatic signal updates do not.
#[component]
pub fn DtSegmentedControl(
    /// Selected value (`modelValue`).
    #[prop(into)]
    value: Signal<String>,
    /// User selection (`update:modelValue` and `change`).
    #[prop(into)]
    on_change: Callback<String>,
    /// Return `true` to cancel the selection (`before-change` + `preventDefault`).
    #[prop(optional, into)]
    on_before_change: Option<Callback<String, bool>>,
    /// Element id. Generated when empty.
    #[prop(optional, into)]
    id: String,
    /// Accessible name (`ariaLabel`).
    #[prop(optional, into)]
    aria_label: String,
    /// `horizontal` (default) or `vertical` (`orientation`).
    #[prop(optional, default = "horizontal")]
    orientation: &'static str,
    /// `100`–`500`, or `xs`–`xl`. Default `200` (`size`).
    #[prop(optional, default = "200")]
    size: &'static str,
    /// `auto` (default) selects on arrow keys. `manual` only moves focus (`activationMode`).
    #[prop(optional, default = "auto")]
    activation_mode: &'static str,
    #[prop(optional)] disabled: bool,
    /// Show dividers (`showDivider`). Default true.
    #[prop(optional, default = true)]
    show_divider: bool,
    #[prop(optional)] borderless: bool,
    /// `grow` (default) or `evenly` (`spread`). `evenly` applies on a horizontal control.
    #[prop(optional, default = "grow")]
    spread: &'static str,
    /// Default label class for items (`labelClass`).
    #[prop(optional, into)]
    label_class: String,
    /// Accepted. The indicator animation waits until segmented CSS is vendored (`showIndicatorTransition`).
    #[prop(optional, default = true)]
    show_indicator_transition: bool,
    children: Children,
) -> impl IntoView {
    let _ = show_indicator_transition;
    if !matches!(activation_mode, "auto" | "manual") {
        leptos::logging::warn!(
            "DtSegmentedControl activation_mode `{activation_mode}` is not auto or manual"
        );
    }
    let id = if id.is_empty() {
        unique_id("dt-segmented-control")
    } else {
        id
    };
    let direction = if orientation == "vertical" {
        StackAt::all("column")
    } else {
        StackAt::all("row")
    };
    let class_name = segmented_classes(size, show_divider, borderless, orientation, spread);
    let button_size = segmented_button_size(size);
    let registry = RwSignal::new(Vec::<SegmentedEntry>::new());
    let fallback = RwSignal::new(None::<String>);
    let focused = RwSignal::new(None::<String>);
    let select = Callback::new(move |next: String| {
        if disabled {
            return;
        }
        if next == value.get_untracked() {
            return;
        }
        if let Some(before) = on_before_change {
            if before.run(next.clone()) {
                return;
            }
        }
        on_change.run(next);
    });
    Effect::new(move |_| {
        let items = registry.get();
        let selected = value.get();
        let values: Vec<String> = items.iter().map(|item| item.value.clone()).collect();
        let flags: Vec<bool> = items.iter().map(|item| item.disabled).collect();
        fallback.set(tab_fallback(&selected, &values, &flags));
    });
    let group = SegmentedGroup {
        value,
        select,
        disabled,
        button_size,
        label_class: StoredValue::new(label_class),
        focused,
        fallback,
        registry,
    };
    let horizontal = orientation != "vertical";
    let activation_auto = activation_mode == "auto";

    view! {
        <Provider value=group>
            <DtStack
                id=id
                direction=direction
                align=StackAt::all("stretch")
                class=class_name
                role="radiogroup"
                aria_label=aria_label
                aria_orientation=orientation.to_string()
                data_qa="dt-segmented-control"
                on_keydown=Callback::new(move |event: KeyboardEvent| {
                    on_segment_key(event, horizontal, activation_auto, select, focused);
                })
            >
                {children()}
            </DtStack>
        </Provider>
    }
}

/// Dialtone Vue `DtSegmentedControlItem` (staging `a46ac063`).
///
/// Renders inside [`DtSegmentedControl`]. `label` is the accessible name.
/// Visible text is the default slot. An item outside the group still renders.
#[component]
pub fn DtSegmentedControlItem(
    /// Selection value matched against the group (`value`).
    #[prop(into)]
    value: String,
    /// Accessible name (`label`). Visible text is the default slot.
    #[prop(optional, into)]
    label: String,
    #[prop(optional)] disabled: bool,
    /// Overrides the group `labelClass` when non-empty (`labelClass`).
    #[prop(optional, into)]
    label_class: String,
    /// Item click. Also selects the item when it is enabled.
    #[prop(optional, into)]
    on_click: Option<Callback<()>>,
    /// Item focus (`focus`).
    #[prop(optional, into)]
    on_focus: Option<Callback<()>>,
    /// Icon at the start of the label (`startIcon`).
    #[prop(optional)]
    start_icon: Option<ViewFn>,
    /// Icon at the end of the label (`endIcon`).
    #[prop(optional)]
    end_icon: Option<ViewFn>,
    /// Content before the label (`leading`).
    #[prop(optional)]
    leading: Option<ViewFn>,
    /// Content after the label (`trailing`).
    #[prop(optional)]
    trailing: Option<ViewFn>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let group = use_context::<SegmentedGroup>();
    if group.is_none() {
        leptos::logging::warn!("DtSegmentedControlItem rendered outside DtSegmentedControl");
    }
    let item_id = next_item_id();
    let item_value = value;
    if let Some(group) = group {
        let registry = group.registry;
        let registered = item_value.clone();
        Effect::new(move |_| {
            let disabled_now = group.disabled || disabled;
            registry.update(|items| {
                if let Some(entry) = items.iter_mut().find(|entry| entry.id == item_id) {
                    entry.disabled = disabled_now;
                    entry.value.clone_from(&registered);
                } else {
                    items.push(SegmentedEntry {
                        id: item_id,
                        value: registered.clone(),
                        disabled: disabled_now,
                    });
                }
            });
        });
        on_cleanup(move || {
            registry.update(|items| items.retain(|entry| entry.id != item_id));
        });
    }

    let selected = Signal::derive({
        let item_value = item_value.clone();
        move || group.is_some_and(|group| group.value.get() == item_value)
    });
    let is_disabled = Signal::derive(move || disabled || group.is_some_and(|group| group.disabled));
    let tab_index = Signal::derive({
        let item_value = item_value.clone();
        move || {
            let Some(group) = group else {
                return item_tab_index(false, disabled, false);
            };
            let fallback = group
                .fallback
                .get()
                .as_deref()
                .is_some_and(|value| value == item_value);
            item_tab_index(
                group.value.get() == item_value,
                group.disabled || disabled,
                fallback,
            )
        }
    });
    let button_size = group.map(|group| group.button_size).unwrap_or("sm");
    let group_label = group
        .map(|group| group.label_class.get_value())
        .unwrap_or_default();
    let label_extra = if label_class.is_empty() {
        group_label
    } else {
        label_class
    };
    let label_class_name = join_class(["d-segmented-control__item-label", label_extra.as_str()]);
    let icon_position = if start_icon.is_some() {
        "left"
    } else if end_icon.is_some() {
        "right"
    } else {
        "left"
    };
    let show_end_in_label = start_icon.is_some() && end_icon.is_some();
    let icon = if start_icon.is_some() {
        start_icon
    } else {
        end_icon.clone()
    };
    let has_label =
        children.is_some() || leading.is_some() || trailing.is_some() || show_end_in_label;
    let label_body = if has_label {
        Some(
            view! {
                {leading.map(|slot| slot.run())}
                {children.map(|children| children())}
                {show_end_in_label.then(|| end_icon.map(|slot| slot.run()))}
                {trailing.map(|slot| slot.run())}
            }
            .into_any(),
        )
    } else {
        None
    };
    let click = Callback::new({
        let item_value = item_value.clone();
        move |_| {
            if let Some(callback) = on_click {
                callback.run(());
            }
            if let Some(group) = group {
                if !(group.disabled || disabled) {
                    group.select.run(item_value.clone());
                }
            }
        }
    });
    let focus = Callback::new({
        let item_value = item_value.clone();
        move |_| {
            if let Some(callback) = on_focus {
                callback.run(());
            }
            if let Some(group) = group {
                group.focused.set(Some(item_value.clone()));
            }
        }
    });

    macro_rules! item_button {
        (icon = $icon:expr, label = $label:expr) => {
            view! {
                <DtButton
                    class="d-segmented-control__item"
                    kind="muted"
                    importance="clear"
                    size=button_size
                    role="radio"
                    data_qa="dt-segmented-control-item"
                    data_value=item_value.clone()
                    aria_label=label.clone()
                    label_class=label_class_name.clone()
                    icon_position=icon_position
                    icon=$icon
                    disabled=is_disabled
                    active=selected
                    aria_checked=selected
                    aria_disabled=is_disabled
                    tab_index=tab_index
                    on_click=click
                    on_focus_in=focus
                >
                    {$label}
                </DtButton>
            }
            .into_any()
        };
        (label = $label:expr) => {
            view! {
                <DtButton
                    class="d-segmented-control__item"
                    kind="muted"
                    importance="clear"
                    size=button_size
                    role="radio"
                    data_qa="dt-segmented-control-item"
                    data_value=item_value.clone()
                    aria_label=label.clone()
                    label_class=label_class_name.clone()
                    icon_position=icon_position
                    disabled=is_disabled
                    active=selected
                    aria_checked=selected
                    aria_disabled=is_disabled
                    tab_index=tab_index
                    on_click=click
                    on_focus_in=focus
                >
                    {$label}
                </DtButton>
            }
            .into_any()
        };
        (icon = $icon:expr) => {
            view! {
                <DtButton
                    class="d-segmented-control__item"
                    kind="muted"
                    importance="clear"
                    size=button_size
                    role="radio"
                    data_qa="dt-segmented-control-item"
                    data_value=item_value.clone()
                    aria_label=label.clone()
                    label_class=label_class_name.clone()
                    icon_position=icon_position
                    icon=$icon
                    disabled=is_disabled
                    active=selected
                    aria_checked=selected
                    aria_disabled=is_disabled
                    tab_index=tab_index
                    on_click=click
                    on_focus_in=focus
                />
            }
            .into_any()
        };
        () => {
            view! {
                <DtButton
                    class="d-segmented-control__item"
                    kind="muted"
                    importance="clear"
                    size=button_size
                    role="radio"
                    data_qa="dt-segmented-control-item"
                    data_value=item_value.clone()
                    aria_label=label.clone()
                    label_class=label_class_name.clone()
                    icon_position=icon_position
                    disabled=is_disabled
                    active=selected
                    aria_checked=selected
                    aria_disabled=is_disabled
                    tab_index=tab_index
                    on_click=click
                    on_focus_in=focus
                />
            }
            .into_any()
        };
    }
    match (icon, label_body) {
        (Some(icon), Some(label_body)) => item_button!(icon = icon, label = label_body),
        (None, Some(label_body)) => item_button!(label = label_body),
        (Some(icon), None) => item_button!(icon = icon),
        (None, None) => item_button!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_root_class_is_a_row_stack_plus_the_small_control() {
        assert_eq!(
            segmented_root_class("200", true, false, "horizontal", "grow"),
            "d-stack d-stack--row d-stack--gap-0 d-stack--align-stretch d-stack--justify-start d-segmented-control d-segmented-control--sm"
        );
    }

    #[test]
    fn vertical_borderless_control_hides_the_even_spread() {
        assert_eq!(
            segmented_root_class("500", false, true, "vertical", "evenly"),
            "d-stack d-stack--gap-0 d-stack--align-stretch d-stack--justify-start d-segmented-control d-segmented-control--xl d-segmented-control--hide-divider d-segmented-control--borderless d-segmented-control--vertical"
        );
        assert_eq!(
            segmented_classes("xs", true, false, "horizontal", "evenly"),
            "d-segmented-control d-segmented-control--xs d-segmented-control--spread-evenly"
        );
        assert_eq!(
            segmented_classes("nope", true, false, "horizontal", "grow"),
            "d-segmented-control"
        );
    }

    #[test]
    fn button_sizes_map_numeric_tokens() {
        assert_eq!(segmented_button_size("200"), "sm");
        assert_eq!(segmented_button_size("100"), "xs");
        assert_eq!(segmented_button_size("lg"), "lg");
        assert_eq!(segmented_button_size("500"), "xl");
    }

    #[test]
    fn keyboard_navigation_skips_disabled_items_and_wraps() {
        let disabled = [false, true, false];
        assert_eq!(next_enabled(&disabled, 0, 1), 2);
        assert_eq!(next_enabled(&disabled, 2, 1), 0);
        assert_eq!(next_enabled(&disabled, 0, -1), 2);
        assert_eq!(navigation_index("Home", true, 1, &disabled), Some(0));
        assert_eq!(navigation_index("End", true, 1, &disabled), Some(2));
        assert_eq!(navigation_index("ArrowDown", true, 0, &disabled), None);
        assert_eq!(navigation_index("ArrowDown", false, 0, &disabled), Some(2));
        assert_eq!(next_enabled(&[true, true], 0, 1), 0);
        assert_eq!(next_enabled(&[], 3, 1), 3);
    }

    #[test]
    fn tab_fallback_matches_the_radiogroup_rules() {
        let values = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(tab_fallback("b", &values, &[false, false, false]), None);
        assert_eq!(
            tab_fallback("a", &values, &[true, false, false]).as_deref(),
            Some("b")
        );
        assert_eq!(
            tab_fallback("missing", &values, &[false, false, false]).as_deref(),
            Some("a")
        );
        assert_eq!(tab_fallback("a", &values, &[true, true, true]), None);
        assert_eq!(item_tab_index(true, false, false), 0);
        assert_eq!(item_tab_index(false, false, true), 0);
        assert_eq!(item_tab_index(true, true, false), -1);
        assert_eq!(item_tab_index(false, false, false), -1);
    }
}
