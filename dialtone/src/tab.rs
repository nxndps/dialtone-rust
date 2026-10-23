use leptos::context::Provider;
use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, Event, HtmlElement, KeyboardEvent, MouseEvent};

use super::button::button_classes;

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Classes for a `DtTab` button (`dialtone/v10.7.0`).
///
/// The button classes match `DtButton` for the group's kind, outlined flag, and
/// size. `d-tab--is-selected` is only the default kind, not outlined and not muted.
pub(crate) fn grouped_tab_classes(
    size: &str,
    kind: &str,
    outlined: bool,
    selected: bool,
    vertical: bool,
    disabled: bool,
    tab_class: &str,
) -> String {
    let (button_kind, importance, active) = if outlined {
        (
            if kind == "muted" { "muted" } else { "default" },
            if selected { "outlined" } else { "clear" },
            false,
        )
    } else if kind == "muted" {
        ("muted", "clear", selected)
    } else if selected {
        ("default", "clear", false)
    } else {
        ("muted", "clear", false)
    };
    let marked = selected && !outlined && kind != "muted";
    // `DtButton` sizes are xs/sm/lg/xl. Tab tokens 100–500 map onto those.
    // 300, md, and default emit no size class.
    let button_size = match size {
        "100" | "xs" => "xs",
        "200" | "sm" => "sm",
        "400" | "lg" => "lg",
        "500" | "xl" => "xl",
        _ => "",
    };
    join_class([
        button_classes(
            false,
            "default",
            false,
            button_kind,
            importance,
            button_size,
            false,
            false,
            false,
            false,
            active,
        )
        .as_str(),
        "d-tablist__item",
        tab_class,
        if vertical {
            "d-tablist__item--vertical"
        } else {
            ""
        },
        if disabled { "d-btn--disabled" } else { "" },
        if marked { "d-tab--is-selected" } else { "" },
    ])
}

/// Classes for the `role="tablist"` element.
pub(crate) fn tab_list_classes(
    size: &str,
    orientation: &str,
    spread: &str,
    inverted: bool,
    borderless: bool,
    extra: &str,
) -> String {
    if !matches!(orientation, "horizontal" | "vertical") {
        leptos::logging::warn!(
            "DtTabGroup orientation `{orientation}` is not horizontal or vertical"
        );
    }
    let size_class = match size {
        "100" | "xs" => "d-tablist--xs",
        "200" | "sm" => "d-tablist--sm",
        "400" | "lg" => "d-tablist--lg",
        "500" | "xl" => "d-tablist--xl",
        "300" | "md" | "default" | "" => "",
        _ => {
            leptos::logging::warn!(
                "DtTabGroup size `{size}` is not 100, 200, 300, 400, 500, or a t-shirt alias"
            );
            ""
        }
    };
    let spread_class = if orientation == "vertical" {
        ""
    } else {
        match spread {
            "grow" => "d-tablist--spread-grow",
            "equal" => "d-tablist--spread-equal",
            "none" | "" => "",
            _ => {
                leptos::logging::warn!("DtTabGroup spread `{spread}` is not none, grow, or equal");
                ""
            }
        }
    };
    join_class([
        "d-tablist",
        size_class,
        if orientation == "vertical" {
            "d-tablist--vertical"
        } else {
            ""
        },
        spread_class,
        if inverted { "d-tablist--inverted" } else { "" },
        if borderless {
            "d-tablist--no-border"
        } else {
            ""
        },
        extra,
    ])
}

/// Next tab index. Arrow keys wrap and do not skip disabled tabs.
pub(crate) fn next_tab_index(from: usize, direction: i32, len: usize) -> Option<usize> {
    if len == 0 {
        return None;
    }
    let len = len as i32;
    let next = (from as i32 + direction).rem_euclid(len);
    Some(next as usize)
}

/// Payload for `DtTabGroup` `change`. Vue emits the group context object.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DtTabGroupChange {
    /// Selected panel id.
    pub selected: String,
    pub disabled: bool,
    pub size: String,
    pub kind: String,
    pub outlined: bool,
    pub orientation: String,
    pub spread: String,
    /// Tab id that holds the roving tabindex. `None` follows the selected tab.
    pub focused_tab_id: Option<String>,
}

/// `before-change` payload. [`DtTabBeforeChange::prevent_default`] cancels the change.
pub struct DtTabBeforeChange {
    /// Panel id the group would select.
    pub selected: String,
    cancel: RwSignal<bool>,
}

impl DtTabBeforeChange {
    /// Vue `event.preventDefault()` in a `before-change` listener.
    pub fn prevent_default(&self) {
        self.cancel.set(true);
    }
}

#[derive(Clone, Copy)]
struct TabGroupContext {
    selected: RwSignal<String>,
    disabled: Signal<bool>,
    size: &'static str,
    kind: &'static str,
    outlined: bool,
    orientation: &'static str,
    spread: &'static str,
    focused_tab_id: RwSignal<Option<String>>,
    activation_mode: &'static str,
}

struct TabItem {
    element: HtmlElement,
    panel_id: String,
    tab_id: String,
    selected: bool,
    disabled: bool,
}

fn strip_prefix<'a>(value: &'a str, prefix: &str) -> &'a str {
    value.strip_prefix(prefix).unwrap_or(value)
}

fn collect_tabs(list: &HtmlElement) -> Vec<TabItem> {
    let Ok(nodes) = list.query_selector_all("[role=\"tab\"]") else {
        return Vec::new();
    };
    let mut items = Vec::new();
    for index in 0..nodes.length() {
        let Some(node) = nodes.item(index) else {
            continue;
        };
        let Ok(element) = node.dyn_into::<HtmlElement>() else {
            continue;
        };
        let controls = element.get_attribute("aria-controls").unwrap_or_default();
        let tab_id = element.id();
        items.push(TabItem {
            panel_id: strip_prefix(&controls, "dt-panel-").to_string(),
            tab_id: strip_prefix(&tab_id, "dt-tab-").to_string(),
            selected: element.get_attribute("aria-selected").as_deref() == Some("true"),
            disabled: element.get_attribute("aria-disabled").as_deref() == Some("true"),
            element,
        });
    }
    items
}

fn focused_index(tabs: &[TabItem], focused_id: Option<&str>) -> Option<usize> {
    if tabs.is_empty() {
        return None;
    }
    let found = if let Some(id) = focused_id {
        tabs.iter().position(|tab| tab.tab_id == id)
    } else {
        tabs.iter().position(|tab| tab.selected)
    };
    Some(found.unwrap_or(0))
}

fn event_tab(event: &Event) -> Option<HtmlElement> {
    let target = event.target()?;
    let element = target.dyn_into::<Element>().ok()?;
    let found = element.closest("[role=\"tab\"]").ok()??;
    found.dyn_into::<HtmlElement>().ok()
}

fn tab_index_for(tabs: &[TabItem], element: &HtmlElement) -> Option<usize> {
    tabs.iter()
        .position(|tab| tab.element.is_same_node(Some(element)))
}

fn is_focusable(element: &Element) -> bool {
    if element.has_attribute("disabled") {
        return false;
    }
    let tag = element.tag_name().to_ascii_lowercase();
    if matches!(
        tag.as_str(),
        "a" | "button" | "input" | "select" | "textarea"
    ) {
        return true;
    }
    element
        .get_attribute("tabindex")
        .and_then(|value| value.parse::<i32>().ok())
        .is_some_and(|index| index >= 0)
}

fn emit_change(group: TabGroupContext, on_change: &Option<Callback<DtTabGroupChange>>) {
    let Some(on_change) = on_change else {
        return;
    };
    on_change.run(DtTabGroupChange {
        selected: group.selected.get_untracked(),
        disabled: group.disabled.get_untracked(),
        size: group.size.to_string(),
        kind: group.kind.to_string(),
        outlined: group.outlined,
        orientation: group.orientation.to_string(),
        spread: group.spread.to_string(),
        focused_tab_id: group.focused_tab_id.get_untracked(),
    });
}

fn tab_face(
    start: Option<ViewFn>,
    end: Option<ViewFn>,
    leading: Option<ViewFn>,
    trailing: Option<ViewFn>,
    label_class: &str,
    leading_class: &str,
    trailing_class: &str,
    children: Children,
) -> impl IntoView {
    let label_class_name = join_class(["base-button__label", "d-btn__label", label_class]);
    let leading_class_name = join_class(["d-btn__leading", leading_class]);
    let trailing_class_name = join_class(["d-btn__trailing", trailing_class]);
    view! {
        {leading.map(|slot| view! {
            <span class=leading_class_name>{slot.run()}</span>
        })}
        {start.map(|slot| view! {
            <span
                data-qa="dt-button-start-icon"
                class="base-button__icon d-btn__icon d-btn__icon--left"
            >
                {slot.run()}
            </span>
        })}
        <span data-qa="dt-button-label" class=label_class_name>
            {children()}
        </span>
        {end.map(|slot| view! {
            <span
                data-qa="dt-button-end-icon"
                class="base-button__icon d-btn__icon d-btn__icon--right"
            >
                {slot.run()}
            </span>
        })}
        {trailing.map(|slot| view! {
            <span class=trailing_class_name>{slot.run()}</span>
        })}
    }
}

fn ask_before_change(
    panel_id: &str,
    on_before_change: &Option<Callback<DtTabBeforeChange>>,
) -> bool {
    let Some(on_before_change) = on_before_change else {
        return true;
    };
    let cancel = RwSignal::new(false);
    on_before_change.run(DtTabBeforeChange {
        selected: panel_id.to_string(),
        cancel,
    });
    !cancel.get_untracked()
}

/// Dialtone Vue `DtTabGroup` (`dialtone/v10.7.0`).
///
/// `selected` is the panel id. The group keeps that value internally and emits
/// [`DtTabGroupChange`] when a tab is chosen, so the list updates before the
/// caller writes the signal back. A later change of `selected` replaces it.
/// Arrow keys move focus and wrap. Enter and Space select. `activation_mode`
/// `auto` selects when focus moves. `show_indicator_transition` is accepted
/// and does not animate. `tab_list_class` is the deprecated `tabListClass` and
/// wins over `tabs_class` when set.
#[component]
pub fn DtTabGroup(
    /// Accessible name of the tab list (`label`). Empty omits `aria-label`.
    #[prop(optional, into)]
    label: String,
    /// Selected panel id (`selected`). Empty shows no panel until a tab is chosen.
    #[prop(optional)]
    selected: Option<Signal<String>>,
    /// `change`. The argument is the group context after the selection.
    #[prop(optional, into)]
    on_change: Option<Callback<DtTabGroupChange>>,
    /// `before-change`. [`DtTabBeforeChange::prevent_default`] keeps the current panel.
    #[prop(optional, into)]
    on_before_change: Option<Callback<DtTabBeforeChange>>,
    /// Disables every tab (`disabled`).
    #[prop(optional, into)]
    disabled: MaybeProp<bool>,
    /// `d-tablist--inverted` (`inverted`). Deprecated in Vue in favor of the mode directive.
    #[prop(optional)]
    inverted: bool,
    /// `d-tablist--no-border` (`borderless`).
    #[prop(optional)]
    borderless: bool,
    /// `none`, `grow`, or `equal` (`spread`). Ignored when `orientation` is `vertical`.
    #[prop(optional, default = "none")]
    spread: &'static str,
    /// `horizontal` or `vertical` (`orientation`).
    #[prop(optional, default = "horizontal")]
    orientation: &'static str,
    /// `100` through `500`, or `xs` through `xl` (`size`). Default `300`.
    #[prop(optional, default = "300")]
    size: &'static str,
    /// Extra class on the tab list (`tabsClass`).
    #[prop(optional, into)]
    tabs_class: String,
    /// Deprecated alias of `tabs_class` (`tabListClass`).
    #[prop(optional, into)]
    tab_list_class: Option<String>,
    /// `default` or `muted` (`kind`).
    #[prop(optional, default = "default")]
    kind: &'static str,
    /// Selected tab uses outlined importance (`outlined`).
    #[prop(optional)]
    outlined: bool,
    /// `manual` or `auto` (`activationMode`). Default `manual`.
    #[prop(optional, default = "manual")]
    activation_mode: &'static str,
    /// Accepted (`showIndicatorTransition`). The indicator does not animate.
    #[prop(optional, default = true)]
    show_indicator_transition: bool,
    /// Tab buttons (`tabs` slot).
    #[prop(optional)]
    tabs: Option<ViewFn>,
    /// Panels.
    children: ChildrenFn,
) -> impl IntoView {
    let _ = show_indicator_transition;
    if !matches!(kind, "default" | "muted") {
        leptos::logging::warn!("DtTabGroup kind `{kind}` is not default or muted");
    }
    if !matches!(activation_mode, "manual" | "auto") {
        leptos::logging::warn!(
            "DtTabGroup activation_mode `{activation_mode}` is not manual or auto"
        );
    }
    let selected_prop = selected.unwrap_or_else(|| Signal::derive(|| String::new()));
    let current = RwSignal::new(selected_prop.get_untracked());
    let seen = StoredValue::new(Some(current.get_untracked()));
    let focused_tab_id = RwSignal::new(None::<String>);
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    Effect::new(move |_| {
        let next = selected_prop.get();
        if seen.get_value().as_ref() == Some(&next) {
            return;
        }
        seen.set_value(Some(next.clone()));
        focused_tab_id.set(None);
        current.set(next);
    });
    let group = TabGroupContext {
        selected: current,
        disabled,
        size,
        kind,
        outlined,
        orientation,
        spread,
        focused_tab_id,
        activation_mode,
    };
    let extra = tab_list_class.unwrap_or(tabs_class);
    let list_class = tab_list_classes(size, orientation, spread, inverted, borderless, &extra);
    let root_class = if orientation == "vertical" {
        "d-tab-neux d-tab-neux--vertical"
    } else {
        "d-tab-neux"
    };
    let aria_label = (!label.is_empty()).then_some(label);
    let list_ref = NodeRef::<html::Div>::new();
    let on_change = StoredValue::new(on_change);
    let on_before_change = StoredValue::new(on_before_change);

    let select_at = move |list: HtmlElement, index: usize, focus: bool| -> bool {
        let tabs = collect_tabs(&list);
        let Some(tab) = tabs.get(index) else {
            return false;
        };
        if tab.disabled || group.selected.get_untracked() == tab.panel_id {
            return false;
        }
        let panel_id = tab.panel_id.clone();
        if !ask_before_change(&panel_id, &on_before_change.get_value()) {
            return false;
        }
        group.focused_tab_id.set(Some(tab.tab_id.clone()));
        group.selected.set(panel_id);
        if focus {
            let _ = tab.element.focus();
        }
        emit_change(group, &on_change.get_value());
        true
    };

    let focus_at = move |list: HtmlElement, index: usize| {
        let tabs = collect_tabs(&list);
        let Some(tab) = tabs.get(index) else {
            return;
        };
        group.focused_tab_id.set(Some(tab.tab_id.clone()));
        let _ = tab.element.focus();
        if group.activation_mode == "auto" && !tab.disabled {
            group.selected.set(tab.panel_id.clone());
            emit_change(group, &on_change.get_value());
        }
    };

    let move_focus = move |list: HtmlElement, direction: i32| {
        let tabs = collect_tabs(&list);
        let Some(index) = focused_index(&tabs, group.focused_tab_id.get_untracked().as_deref())
        else {
            return;
        };
        if let Some(next) = next_tab_index(index, direction, tabs.len()) {
            focus_at(list, next);
        }
    };

    view! {
        <Provider value=group>
            <div data-qa="dt-tab-group" class=root_class>
                <div
                    node_ref=list_ref
                    role="tablist"
                    class=list_class
                    aria-label=aria_label
                    aria-orientation=orientation
                    on:keydown=move |event: KeyboardEvent| {
                        let Some(list) = list_ref.get() else { return };
                        let list: HtmlElement = list.into();
                        let key = event.key();
                        if key == "Home" {
                            event.prevent_default();
                            focus_at(list, 0);
                            return;
                        }
                        if key == "End" {
                            event.prevent_default();
                            let count = collect_tabs(&list).len();
                            if count > 0 {
                                focus_at(list, count - 1);
                            }
                            return;
                        }
                        let horizontal = group.orientation == "horizontal";
                        let vertical = group.orientation == "vertical";
                        if (key == "ArrowLeft" && horizontal) || (key == "ArrowUp" && vertical) {
                            if vertical {
                                event.prevent_default();
                            }
                            move_focus(list, -1);
                            return;
                        }
                        if (key == "ArrowRight" && horizontal) || (key == "ArrowDown" && vertical) {
                            if vertical {
                                event.prevent_default();
                            }
                            move_focus(list, 1);
                            return;
                        }
                        if key != "Enter" && key != " " && key != "Spacebar" {
                            return;
                        }
                        let tabs = collect_tabs(&list);
                        let index = event_tab(&event)
                            .and_then(|element| tab_index_for(&tabs, &element))
                            .or_else(|| {
                                focused_index(&tabs, group.focused_tab_id.get_untracked().as_deref())
                            });
                        let Some(index) = index else { return };
                        if select_at(list, index, true) {
                            event.prevent_default();
                        }
                    }
                    on:click=move |event: MouseEvent| {
                        let Some(list) = list_ref.get() else { return };
                        let list: HtmlElement = list.into();
                        let Some(element) = event_tab(&event) else { return };
                        let tabs = collect_tabs(&list);
                        let Some(index) = tab_index_for(&tabs, &element) else { return };
                        if select_at(list, index, true) {
                            event.prevent_default();
                        }
                    }
                >
                    {tabs.map(|slot| slot.run())}
                </div>
                {children()}
            </div>
        </Provider>
    }
}

/// Dialtone Vue `DtTabPanel` (`dialtone/v10.7.0`).
///
/// The panel stays in the document. An unselected panel uses `display: none`
/// and `aria-hidden="true"`. `tabindex` is `0` unless the first element child
/// is focusable, in which case it is `-1`.
#[component]
pub fn DtTabPanel(
    /// Panel id (`id`). Rendered as `dt-panel-{id}`.
    #[prop(into)]
    id: String,
    /// Associated tab id (`tabId`). `aria-labelledby` is `dt-tab-{tabId}`.
    #[prop(into)]
    tab_id: String,
    /// Hides the panel even when it is selected (`hidden`).
    #[prop(optional)]
    hidden: bool,
    /// Extra class (`tabPanelClass`).
    #[prop(optional, into)]
    tab_panel_class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let group = use_context::<TabGroupContext>();
    if group.is_none() {
        leptos::logging::warn!("DtTabPanel is outside DtTabGroup");
    }
    let panel_id = id.clone();
    let hide = Signal::derive(move || {
        hidden || group.is_none_or(|group| group.selected.get() != panel_id)
    });
    let tab_index = RwSignal::new("0");
    let panel_ref = NodeRef::<html::Div>::new();
    Effect::new(move |_| {
        let Some(panel) = panel_ref.get() else {
            return;
        };
        if panel
            .first_element_child()
            .is_some_and(|child| is_focusable(&child))
        {
            tab_index.set("-1");
        }
    });
    let dom_id = format!("dt-panel-{id}");
    let labelled_by = format!("dt-tab-{tab_id}");
    let class_name = join_class([tab_panel_class.as_str()]);
    view! {
        <div
            node_ref=panel_ref
            id=dom_id
            role="tabpanel"
            class=class_name
            data-qa="dt-tab-panel"
            tabindex=move || tab_index.get()
            aria-labelledby=labelled_by
            aria-hidden=move || if hide.get() { "true" } else { "false" }
            style:display=move || hide.get().then_some("none")
        >
            {children()}
        </div>
    }
}

/// Dialtone Vue `DtTab` (`dialtone/v10.7.0`).
///
/// The button is native: `data-qa="dt-tab"`, `id` `dt-tab-{id}`, and
/// `aria-controls` `dt-panel-{panelId}`. Inside [`DtTabGroup`](DtTabGroup),
/// `selected` sets the group's panel id on mount (`groupContext.selected = panelId`).
/// The visible selection is the group's panel id. Outside a group, `selected`
/// is this tab's own state, with size `300` and kind `default`. `disabled`
/// sets `aria-disabled` and `d-btn--disabled`. It does not
/// set the native disabled attribute. `start_icon` is `startIcon` and wins over
/// the deprecated `icon` slot. `end_icon` is `endIcon`.
#[component]
pub fn DtTab(
    /// Tab id (`id`). Rendered as `dt-tab-{id}`.
    #[prop(into)]
    id: String,
    /// Associated panel id (`panelId`). `aria-controls` is `dt-panel-{panelId}`.
    #[prop(into)]
    panel_id: String,
    /// Accessible name (`label`). Empty omits `aria-label`.
    #[prop(optional, into)]
    label: String,
    /// Outside a group, the selected state. Inside a group, `true` selects this
    /// panel when the tab mounts.
    #[prop(optional)]
    selected: bool,
    #[prop(optional)] disabled: bool,
    /// Extra class on the tab (`tabClass`).
    #[prop(optional, into)]
    tab_class: String,
    /// Extra class on the label (`labelClass`).
    #[prop(optional, into)]
    label_class: String,
    /// Extra class on the leading wrapper (`leadingClass`).
    #[prop(optional, into)]
    leading_class: String,
    /// Extra class on the trailing wrapper (`trailingClass`).
    #[prop(optional, into)]
    trailing_class: String,
    /// Icon at the start (`startIcon`). Wins over `icon`.
    #[prop(optional)]
    start_icon: Option<ViewFn>,
    /// Deprecated start icon (`icon`). Used when `start_icon` is absent.
    #[prop(optional)]
    icon: Option<ViewFn>,
    /// Icon at the end (`endIcon`).
    #[prop(optional)]
    end_icon: Option<ViewFn>,
    /// Content before the label (`leading`).
    #[prop(optional)]
    leading: Option<ViewFn>,
    /// Content after the label (`trailing`).
    #[prop(optional)]
    trailing: Option<ViewFn>,
    /// Native click (`click`).
    #[prop(optional, into)]
    on_click: Option<Callback<()>>,
    /// Native focus (`focus`).
    #[prop(optional, into)]
    on_focus: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let group = use_context::<TabGroupContext>();
    let start = start_icon.or(icon);
    let id_attr = format!("dt-tab-{id}");
    let controls = format!("dt-panel-{panel_id}");
    let aria_label = (!label.is_empty()).then_some(label);
    let id_grouped = id_attr.clone();
    let controls_grouped = controls.clone();
    let label_grouped = aria_label.clone();
    match group {
        None => {
            let class_name = grouped_tab_classes(
                "300", "default", false, selected, false, disabled, &tab_class,
            );
            let aria_selected = if selected { "true" } else { "false" };
            let tabindex = if selected { "0" } else { "-1" };
            let aria_disabled = disabled.then_some("true");
            let face = tab_face(
                start,
                end_icon,
                leading,
                trailing,
                &label_class,
                &leading_class,
                &trailing_class,
                children,
            );
            view! {
                <button
                    id=id_attr
                    type="button"
                    role="tab"
                    class=class_name
                    data-qa="dt-tab"
                    aria-selected=aria_selected
                    aria-controls=controls
                    aria-label=aria_label
                    aria-disabled=aria_disabled
                    tabindex=tabindex
                    on:click=move |_| {
                        if let Some(on_click) = on_click {
                            on_click.run(());
                        }
                    }
                    on:focus=move |_| {
                        if let Some(on_focus) = on_focus {
                            on_focus.run(());
                        }
                    }
                >
                    {face}
                </button>
            }
            .into_any()
        }
        Some(group) => {
            let panel_for_mount = panel_id.clone();
            let button_ref = NodeRef::<html::Button>::new();
            let applied = StoredValue::new(false);
            Effect::new(move |_| {
                if button_ref.get().is_some() && selected && !applied.get_value() {
                    applied.set_value(true);
                    group.selected.set(panel_for_mount.clone());
                }
            });
            let id_for_cleanup = id.clone();
            on_cleanup(move || {
                if group.focused_tab_id.get_untracked().as_deref() == Some(id_for_cleanup.as_str())
                {
                    group.focused_tab_id.set(None);
                }
            });
            let extra = tab_class;
            let panel_for_class = panel_id.clone();
            let class_name = move || {
                grouped_tab_classes(
                    group.size,
                    group.kind,
                    group.outlined,
                    group.selected.get() == panel_for_class,
                    group.orientation == "vertical",
                    group.disabled.get() || disabled,
                    &extra,
                )
            };
            let panel_for_aria = panel_id.clone();
            let aria_selected = move || {
                if group.selected.get() == panel_for_aria {
                    "true"
                } else {
                    "false"
                }
            };
            let id_for_tab = id.clone();
            let panel_for_tab = panel_id;
            let tabindex = move || {
                let is_target = match group.focused_tab_id.get() {
                    Some(focused) => focused == id_for_tab,
                    None => group.selected.get() == panel_for_tab,
                };
                if is_target {
                    "0"
                } else {
                    "-1"
                }
            };
            let aria_disabled = move || (group.disabled.get() || disabled).then_some("true");
            let id_for_focus = id;
            let face = tab_face(
                start,
                end_icon,
                leading,
                trailing,
                &label_class,
                &leading_class,
                &trailing_class,
                children,
            );
            view! {
                <button
                    node_ref=button_ref
                    id=id_grouped
                    type="button"
                    role="tab"
                    class=class_name
                    data-qa="dt-tab"
                    aria-selected=aria_selected
                    aria-controls=controls_grouped
                    aria-label=label_grouped
                    aria-disabled=aria_disabled
                    tabindex=tabindex
                    on:click=move |_| {
                        if let Some(on_click) = &on_click {
                            on_click.run(());
                        }
                    }
                    on:focus=move |_| {
                        group.focused_tab_id.set(Some(id_for_focus.clone()));
                        if let Some(on_focus) = &on_focus {
                            on_focus.run(());
                        }
                    }
                >
                    {face}
                </button>
            }
            .into_any()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orphan_tab_uses_the_v10_button() {
        assert_eq!(
            grouped_tab_classes("300", "default", false, false, false, false, ""),
            "base-button__button d-btn d-btn--muted d-tablist__item"
        );
        assert_eq!(
            grouped_tab_classes("300", "default", false, true, false, true, "extra"),
            "base-button__button d-btn d-tablist__item extra d-btn--disabled d-tab--is-selected"
        );
    }

    #[test]
    fn grouped_classes_follow_kind_outline_and_size() {
        assert_eq!(
            grouped_tab_classes("300", "default", false, true, false, false, ""),
            "base-button__button d-btn d-tablist__item d-tab--is-selected"
        );
        assert_eq!(
            grouped_tab_classes("300", "default", false, false, false, false, ""),
            "base-button__button d-btn d-btn--muted d-tablist__item"
        );
        assert_eq!(
            grouped_tab_classes("md", "muted", false, true, false, false, ""),
            "base-button__button d-btn d-btn--muted d-btn--active d-tablist__item"
        );
        assert_eq!(
            grouped_tab_classes("300", "default", true, true, false, false, ""),
            "base-button__button d-btn d-btn--outlined d-tablist__item"
        );
        assert_eq!(
            grouped_tab_classes("300", "default", true, false, false, false, ""),
            "base-button__button d-btn d-tablist__item"
        );
        assert_eq!(
            grouped_tab_classes("sm", "default", false, true, true, true, "extra"),
            "base-button__button d-btn d-btn--sm d-tablist__item extra d-tablist__item--vertical d-btn--disabled d-tab--is-selected"
        );
        assert_eq!(
            grouped_tab_classes("200", "default", false, false, false, false, ""),
            "base-button__button d-btn d-btn--muted d-btn--sm d-tablist__item"
        );
    }

    #[test]
    fn tab_list_classes_follow_size_spread_and_orientation() {
        assert_eq!(
            tab_list_classes("sm", "horizontal", "none", false, false, ""),
            "d-tablist d-tablist--sm"
        );
        assert_eq!(
            tab_list_classes("200", "horizontal", "none", false, false, ""),
            "d-tablist d-tablist--sm"
        );
        assert_eq!(
            tab_list_classes("300", "horizontal", "grow", true, true, "extra"),
            "d-tablist d-tablist--spread-grow d-tablist--inverted d-tablist--no-border extra"
        );
        assert_eq!(
            tab_list_classes("300", "vertical", "grow", false, false, ""),
            "d-tablist d-tablist--vertical"
        );
        assert_eq!(
            tab_list_classes("xl", "horizontal", "equal", false, false, ""),
            "d-tablist d-tablist--xl d-tablist--spread-equal"
        );
    }

    #[test]
    fn arrow_indexes_wrap() {
        assert_eq!(next_tab_index(0, -1, 3), Some(2));
        assert_eq!(next_tab_index(2, 1, 3), Some(0));
        assert_eq!(next_tab_index(0, 1, 3), Some(1));
        assert_eq!(next_tab_index(1, -1, 3), Some(0));
        assert_eq!(next_tab_index(0, 1, 0), None);
    }
}
