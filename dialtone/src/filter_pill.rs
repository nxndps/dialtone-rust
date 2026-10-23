use leptos::context::Provider;
use leptos::prelude::*;
use web_sys::{KeyboardEvent, MouseEvent};

use super::button::{button_icon_size, DtButton};
use super::checkbox::DtCheckbox;
use super::checkbox_group::DtCheckboxGroup;
use super::dropdown::DtDropdown;
use super::icon::{DtIconChevronDown, DtIconClose};
use super::popover::DtPopover;
use super::stack::{DtStack, StackAt};
use super::tooltip::DtTooltip;
use super::unique_id;

/// One filter row. Vue `modelValue` items are `{ name, active }`.
///
/// Clear sets `active` to false. Vue deletes the property; both are inactive.
#[derive(Clone, Debug, PartialEq)]
pub struct DtFilter {
    pub name: String,
    pub active: bool,
}

/// Values the default label slot receives.
#[derive(Clone, Debug, PartialEq)]
pub struct DtFilterPillLabel {
    pub label: String,
    pub filters: Vec<DtFilter>,
    pub active_filters: Vec<String>,
    pub active_filter_list: String,
    pub active_filter_overflow: String,
}

/// Callbacks for a custom content slot. Read with `use_context`.
#[derive(Clone, Copy)]
pub struct DtFilterPillContent {
    pub apply: Callback<()>,
    pub cancel: Callback<()>,
    pub close: Callback<()>,
    pub pending: Signal<Vec<DtFilter>>,
}

const CLEAR_LABEL: &str = "Clear filter";
const READ_ONLY_TOOLTIP: &str = "Read-only filter";

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn active_names(filters: &[DtFilter]) -> Vec<String> {
    filters
        .iter()
        .filter(|filter| filter.active)
        .map(|filter| filter.name.clone())
        .collect()
}

/// First active name, or `All` when every filter is active and there is more than one.
pub(crate) fn active_filter_list(filters: &[DtFilter]) -> String {
    let active = active_names(filters);
    if active.len() == filters.len() && filters.len() > 1 {
        return "All".to_string();
    }
    active.first().cloned().unwrap_or_default()
}

/// `+N` when more than one filter is active and not every filter is active.
pub(crate) fn active_filter_overflow(filters: &[DtFilter]) -> String {
    let active_count = filters.iter().filter(|filter| filter.active).count();
    if active_count <= 1 || (active_count == filters.len() && filters.len() > 1) {
        return String::new();
    }
    format!("+{}", active_count - 1)
}

pub(crate) fn filter_pill_button_size(size: &str) -> &'static str {
    match size {
        "100" | "xs" => "xs",
        "200" | "sm" => "sm",
        "300" | "md" => "md",
        "400" | "lg" => "lg",
        "500" | "xl" => "xl",
        _ => {
            leptos::logging::warn!("DtFilterPill size `{size}` is not a button size");
            "sm"
        }
    }
}

pub(crate) fn filter_pill_clear_icon_size(size: &str) -> &'static str {
    if size == "100" || size == "xs" {
        "100"
    } else {
        button_icon_size(filter_pill_button_size(size))
    }
}

pub(crate) fn filter_button_kind(active: bool, disabled: bool) -> &'static str {
    if active && !disabled {
        "default"
    } else {
        "muted"
    }
}

fn with_active(filters: &[DtFilter], names: &[String]) -> Vec<DtFilter> {
    filters
        .iter()
        .map(|filter| DtFilter {
            name: filter.name.clone(),
            active: names.iter().any(|name| name == &filter.name),
        })
        .collect()
}

fn cleared(filters: &[DtFilter]) -> Vec<DtFilter> {
    filters
        .iter()
        .map(|filter| DtFilter {
            name: filter.name.clone(),
            active: false,
        })
        .collect()
}

fn selected_only(filters: &[DtFilter], name: &str) -> Vec<DtFilter> {
    filters
        .iter()
        .map(|filter| DtFilter {
            name: filter.name.clone(),
            active: filter.name == name,
        })
        .collect()
}

/// Dialtone Vue `DtFilterPill` (staging `a46ac063`).
///
/// The component is not in tag `dialtone/v9.190.0`. `d-filter-pill*` is in the
/// vendored Dialtone 10 CSS. Dropdown rows stay `li.d-list-item`. That caller was
/// not migrated to `DtListItem`. `filters` is the Vue `modelValue`. `defer_selection` keeps
/// checkbox edits in [`DtFilterPillContent::pending`] until Apply.
#[component]
pub fn DtFilterPill(
    /// Filters (`modelValue`).
    filters: Signal<Vec<DtFilter>>,
    /// Next filter list (`update:modelValue`).
    #[prop(optional, into)]
    on_change: Option<Callback<Vec<DtFilter>>>,
    /// Use a menu instead of checkboxes (`useDropdown`).
    #[prop(optional)]
    use_dropdown: bool,
    /// Hold checkbox edits until Apply (`deferSelection`). Ignored with `use_dropdown`.
    #[prop(optional)]
    defer_selection: bool,
    /// Tooltip on the pill (`startTooltipText`).
    #[prop(optional, into)]
    start_tooltip_text: String,
    #[prop(optional)] disabled: bool,
    /// Block interaction without the disabled style (`readOnly`).
    #[prop(optional)]
    read_only: bool,
    /// Show the clear button while a filter is active (`showClear`).
    #[prop(optional, default = true)]
    show_clear: bool,
    #[prop(optional, into)] label: String,
    /// Clear-button tooltip and accessible name (`endTooltipText`).
    #[prop(optional, into)]
    end_tooltip_text: String,
    /// `body` or `parent` (`popoverAppendTo`).
    #[prop(optional, default = "body")]
    popover_append_to: &'static str,
    /// Comma-separated placements (`popoverFallbackPlacements`).
    #[prop(optional, default = "auto")]
    popover_fallback_placements: &'static str,
    /// CSS max-height (`popoverMaxHeight`).
    #[prop(optional, into)]
    popover_max_height: String,
    /// CSS max-width (`popoverMaxWidth`).
    #[prop(optional, into)]
    popover_max_width: String,
    /// `none`, `small`, `medium`, or `large` (`popoverPadding`).
    #[prop(optional, default = "large")]
    popover_padding: &'static str,
    /// Popover placement (`popoverPlacement`).
    #[prop(optional, default = "bottom-start")]
    popover_placement: &'static str,
    /// Extra class on the label (`labelClass`).
    #[prop(optional, into)]
    label_class: String,
    /// Extra class on the popover content (`popoverContentClass`).
    #[prop(optional, into)]
    popover_content_class: String,
    /// Extra class on the popover header (`popoverHeaderClass`).
    #[prop(optional, into)]
    popover_header_class: String,
    /// Extra class on the popover footer (`popoverFooterClass`).
    #[prop(optional, into)]
    popover_footer_class: String,
    /// Extra class on the popover dialog (`popoverDialogClass`).
    #[prop(optional, into)]
    popover_dialog_class: String,
    /// Extra class on the dropdown list (`dropdownListClass`).
    #[prop(optional, into)]
    dropdown_list_class: String,
    /// `light`, `dark`, or `invert` (`contentMode`).
    #[prop(optional, default = "")]
    content_mode: &'static str,
    /// `100` through `500`, or `xs` through `xl` (`size`). Default `200`.
    #[prop(optional, default = "200")]
    size: &'static str,
    /// Open or closed (`open`).
    #[prop(optional, into)]
    on_open: Option<Callback<bool>>,
    /// Clear button (`clear`).
    #[prop(optional, into)]
    on_clear: Option<Callback<()>>,
    /// Apply in deferred mode (`apply`).
    #[prop(optional, into)]
    on_apply: Option<Callback<()>>,
    /// Icon before the label (`startIcon`). Pass `None` or `Some(ViewFn)`.
    #[prop(optional_no_strip)]
    start_icon: Option<ViewFn>,
    /// Replaces the label text. [`DtFilterPillLabel`] is in context.
    /// Pass `None` or `Some(ViewFn)`.
    #[prop(optional_no_strip)]
    label_slot: Option<ViewFn>,
    /// Replaces the checkbox list or dropdown rows. Pass `None` or `Some(ViewFn)`.
    #[prop(optional_no_strip)]
    content: Option<ViewFn>,
    /// Popover header (`headerContent`). Pass `None` or `Some(ViewFn)`.
    #[prop(optional_no_strip)]
    header: Option<ViewFn>,
    /// Popover footer (`footerContent`). Replaces the Apply row.
    /// Pass `None` or `Some(ViewFn)`.
    #[prop(optional_no_strip)]
    footer: Option<ViewFn>,
) -> impl IntoView {
    if popover_append_to != "body" && popover_append_to != "parent" {
        leptos::logging::warn!(
            "DtFilterPill popover_append_to `{popover_append_to}` is not body or parent"
        );
    }
    if filters.get_untracked().is_empty() && content.is_none() {
        leptos::logging::warn!(
            "DtFilterPill: Please provide content through the v-model or the \"content\" slot."
        );
    }
    if defer_selection && use_dropdown {
        leptos::logging::warn!(
            "DtFilterPill defer_selection has no effect when use_dropdown is true."
        );
    }
    let local = RwSignal::new(filters.get_untracked());
    Effect::new(move |_| {
        local.set(filters.get());
    });
    let open = RwSignal::new(false);
    let pending = RwSignal::new(Vec::<DtFilter>::new());
    let was_open = StoredValue::new(false);
    let on_open = StoredValue::new(on_open);
    let on_change = StoredValue::new(on_change);
    let on_clear = StoredValue::new(on_clear);
    let on_apply = StoredValue::new(on_apply);
    Effect::new(move |_| {
        let now = open.get();
        if now == was_open.get_value() {
            return;
        }
        was_open.set_value(now);
        if now && defer_selection && !use_dropdown {
            pending.set(local.get_untracked());
        }
        if !now {
            pending.set(Vec::new());
        }
        if let Some(callback) = on_open.get_value() {
            callback.run(now);
        }
    });

    let publish = move |next: Vec<DtFilter>| {
        local.set(next.clone());
        if let Some(callback) = on_change.get_value() {
            callback.run(next);
        }
    };
    let set_open = move |next: bool| {
        open.set(next);
    };
    let apply_selection = Callback::new(move |_| {
        if !defer_selection {
            return;
        }
        let next = pending.get_untracked();
        if next.is_empty() {
            return;
        }
        publish(next);
        if let Some(callback) = on_apply.get_value() {
            callback.run(());
        }
        set_open(false);
    });
    let cancel_selection = Callback::new(move |_| set_open(false));
    let close_overlay = Callback::new(move |_| set_open(false));
    let dialog_id = StoredValue::new(unique_id("dt-filter-pill-dialog"));
    let button_size = filter_pill_button_size(size);
    let icon_size = button_icon_size(button_size);
    let clear_icon = filter_pill_clear_icon_size(size);
    let group_name = if label.is_empty() {
        "filter-pill".to_string()
    } else {
        label.clone()
    };
    let start_tip = if read_only {
        if start_tooltip_text.is_empty() {
            READ_ONLY_TOOLTIP.to_string()
        } else {
            start_tooltip_text.clone()
        }
    } else {
        start_tooltip_text.clone()
    };
    let clear_label = if end_tooltip_text.is_empty() {
        CLEAR_LABEL.to_string()
    } else {
        end_tooltip_text.clone()
    };
    let clear_title = if end_tooltip_text.is_empty() {
        clear_label.clone()
    } else {
        String::new()
    };

    let label_for_context = StoredValue::new(label.clone());
    let label_class = StoredValue::new(label_class);
    let label_slot = StoredValue::new(label_slot);
    let start_icon = StoredValue::new(start_icon);
    let start_tip = StoredValue::new(start_tip);
    let clear_label = StoredValue::new(clear_label);
    let clear_title = StoredValue::new(clear_title);
    let end_tooltip_text = StoredValue::new(end_tooltip_text);
    let label_view = move || {
        // The button closure already tracks `local`. This read runs while the
        // view is built, which is outside a subscriber.
        let filters_now = local.get_untracked();
        let label_for_context = label_for_context.get_value();
        let label_class = label_class.get_value();
        let label_slot = label_slot.get_value();
        let state = DtFilterPillLabel {
            label: label_for_context,
            active_filters: active_names(&filters_now),
            active_filter_list: active_filter_list(&filters_now),
            active_filter_overflow: active_filter_overflow(&filters_now),
            filters: filters_now,
        };
        let class_name = join_class(["d-filter-pill__label", label_class.as_str()]);
        let default_label = {
            let list = state.active_filter_list.clone();
            let overflow = state.active_filter_overflow.clone();
            let overflow_title = if state.active_filters.len() > 1 {
                format!("plus {} others", state.active_filters.len() - 1)
            } else {
                String::new()
            };
            let text = state.label.clone();
            let list_present = !list.is_empty();
            let overflow_present = !overflow.is_empty();
            view! {
                <span class="d-filter-pill__label-start" title=text.clone()>{text.clone()}</span>
                <Show when=move || list_present>
                    <span class="d-filter-pill__label-end" title=list.clone()>{list.clone()}</span>
                </Show>
                <Show when=move || overflow_present>
                    <span class="d-filter-pill__label-end-overflow" title=overflow_title.clone()>
                        {overflow.clone()}
                    </span>
                </Show>
            }
            .into_any()
        };
        let body = if let Some(slot) = label_slot {
            view! {
                <Provider value=state>
                    {slot.run()}
                </Provider>
            }
            .into_any()
        } else {
            default_label
        };
        view! { <span class=class_name>{body}</span> }
    };

    let anchor = ViewFn::from(move || {
        let haspopup = if use_dropdown { "menu" } else { "dialog" };
        let tip = start_tip.get_value();
        let button = view! {
            <span
                on:click=move |event: MouseEvent| {
                    event.stop_propagation();
                    if !read_only && !disabled {
                        set_open(true);
                    }
                }
                on:keydown=move |event: KeyboardEvent| {
                    let key = event.key();
                    if key == "ArrowUp" || key == "ArrowDown" {
                        event.prevent_default();
                        if !read_only && !disabled {
                            set_open(true);
                        }
                    }
                }
            >
                {move || {
                    let filters_now = local.get();
                    let active = !active_filter_list(&filters_now).is_empty();
                    let kind = filter_button_kind(active, disabled);
                    let selected = active.then_some("d-filter-pill--selected").unwrap_or("");
                    let class_name = join_class(["d-filter-pill__primary", selected]);
                    view! {
                        <DtButton
                            active=active
                            aria_disabled=read_only.then_some(true)
                            aria_expanded=open.get()
                            aria_controls=dialog_id.get_value()
                            aria_haspopup=haspopup
                            class=class_name
                            disabled=disabled
                            kind=kind
                            size=button_size
                            data_qa="dt-filter-pill__button"
                            importance="outlined"
                        >
                            {start_icon.get_value().map(|slot| slot.run())}
                            {label_view()}
                            {(!read_only).then(|| view! {
                                <span class="d-filter-pill__icon" data-qa="dt-filter-pill__icon">
                                    <DtIconChevronDown size=icon_size />
                                </span>
                            })}
                        </DtButton>
                    }
                }}
            </span>
        }
        .into_any();
        if tip.is_empty() {
            button
        } else {
            view! { <DtTooltip message=tip>{button}</DtTooltip> }.into_any()
        }
    });

    let content_body = {
        let content_slot = content.clone();
        ViewFn::from(move || {
            let actions = DtFilterPillContent {
                apply: apply_selection,
                cancel: cancel_selection,
                close: close_overlay,
                pending: pending.into(),
            };
            if let Some(slot) = content_slot.clone() {
                return view! {
                    <Provider value=actions>{slot.run()}</Provider>
                }
                .into_any();
            }
            if use_dropdown {
                let filters_now = local.get();
                return filters_now
                    .iter()
                    .map(|filter| {
                        let name = filter.name.clone();
                        let selected = filter.active;
                        let name_click = name.clone();
                        view! {
                            <li
                                role="menuitem"
                                class="d-list-item"
                                tabindex="0"
                                aria-selected=if selected { "true" } else { "false" }
                                on:click=move |_| {
                                    publish(selected_only(&local.get_untracked(), &name_click));
                                    set_open(false);
                                }
                            >
                                {name}
                            </li>
                        }
                    })
                    .collect_view()
                    .into_any();
            }
            let selected = Signal::derive(move || {
                let source = if defer_selection && open.get() {
                    pending.get()
                } else {
                    local.get()
                };
                active_names(&source)
            });
            let aria = label.clone();
            let name = group_name.clone();
            view! {
                <Show when=move || !local.get().is_empty()>
                    <DtCheckboxGroup
                        name=name.clone()
                        selected_values=selected
                        aria_label=aria.clone()
                        on_change=Callback::new(move |names: Vec<String>| {
                            if defer_selection && !use_dropdown {
                                pending.set(with_active(&pending.get_untracked(), &names));
                            } else {
                                publish(with_active(&local.get_untracked(), &names));
                            }
                        })
                    >
                        {move || {
                            let source = if defer_selection && open.get() {
                                pending.get()
                            } else {
                                local.get()
                            };
                            source
                                .into_iter()
                                .map(|filter| {
                                    let name = filter.name.clone();
                                    view! { <DtCheckbox label=name.clone() value=name /> }
                                })
                                .collect_view()
                        }}
                    </DtCheckboxGroup>
                </Show>
            }
            .into_any()
        })
    };

    let footer_view = if !use_dropdown {
        if let Some(slot) = footer.clone() {
            Some(slot)
        } else if defer_selection {
            Some(ViewFn::from(move || {
                view! {
                    <DtStack
                        direction=StackAt::all("row")
                        gap=StackAt::all("400")
                        justify=StackAt::all("end")
                        data_qa="dt-filter-pill__deferred-footer"
                    >
                        <DtButton
                            importance="clear"
                            kind="muted"
                            size="sm"
                            data_qa="dt-filter-pill__cancel-button"
                            on_click=cancel_selection
                        >
                            "Cancel"
                        </DtButton>
                        <DtButton
                            importance="primary"
                            size="sm"
                            data_qa="dt-filter-pill__apply-button"
                            on_click=apply_selection
                        >
                            "Apply"
                        </DtButton>
                    </DtStack>
                }
            }))
        } else {
            None
        }
    } else {
        None
    };

    let content_body = StoredValue::new(content_body);
    let root_class = join_class([
        "d-filter-pill",
        read_only
            .then_some("d-filter-pill--read-only")
            .unwrap_or(""),
    ]);
    let overlay = if use_dropdown {
        view! {
            <DtDropdown
                open=Signal::derive(move || open.get())
                on_open_change=Callback::new(move |next| set_open(next))
                placement=popover_placement
                fallback_placements=popover_fallback_placements
                append_to=popover_append_to
                modal=true
                max_height=popover_max_height.clone()
                max_width=popover_max_width.clone()
                list_class=dropdown_list_class.clone()
                content_mode=content_mode
                anchor=anchor
            >
                {move || content_body.get_value().run()}
            </DtDropdown>
        }
        .into_any()
    } else {
        let popover_id = dialog_id.get_value();
        view! {
            <DtPopover
                id=popover_id
                open=Signal::derive(move || open.get())
                on_open_change=Callback::new(move |next| set_open(next))
                placement=popover_placement
                fallback_placements=popover_fallback_placements
                append_to=popover_append_to
                modal=true
                padding=popover_padding
                max_height=popover_max_height
                max_width=popover_max_width
                dialog_class=popover_dialog_class
                content_class=popover_content_class
                header_class=popover_header_class
                footer_class=popover_footer_class
                content_mode=content_mode
                anchor=anchor
                nostrip:header=header
                nostrip:footer=footer_view
            >
                {move || content_body.get_value().run()}
            </DtPopover>
        }
        .into_any()
    };

    let show_clear_button =
        move || !read_only && show_clear && !active_filter_list(&local.get()).is_empty();

    view! {
        <div class=root_class data-qa="dt-filter-pill">
            {overlay}
            <Show when=show_clear_button>
                {move || {
                    let filters_now = local.get();
                    let active = !active_filter_list(&filters_now).is_empty();
                    let kind = filter_button_kind(active, disabled);
                    let class_name = join_class([
                        "d-filter-pill__clear",
                        active.then_some("d-filter-pill__clear--selected").unwrap_or(""),
                    ]);
                    let label = clear_label.get_value();
                    let title = clear_title.get_value();
                    let end_tip = end_tooltip_text.get_value();
                    let button = view! {
                        <DtButton
                            active=active
                            aria_label=label
                            title=title
                            class=class_name
                            disabled=disabled
                            kind=kind
                            size=button_size
                            data_qa="dt-filter-pill__clear-button"
                            importance="outlined"
                            on_click=Callback::new(move |_| {
                                publish(cleared(&local.get_untracked()));
                                if let Some(callback) = on_clear.get_value() {
                                    callback.run(());
                                }
                            })
                            icon=ViewFn::from(move || view! { <DtIconClose size=clear_icon /> })
                        />
                    }
                    .into_any();
                    if end_tip.is_empty() {
                        button
                    } else {
                        view! { <DtTooltip message=end_tip>{button}</DtTooltip> }.into_any()
                    }
                }}
            </Show>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn filters(items: &[(&str, bool)]) -> Vec<DtFilter> {
        items
            .iter()
            .map(|(name, active)| DtFilter {
                name: (*name).to_string(),
                active: *active,
            })
            .collect()
    }

    #[test]
    fn label_summary_matches_vue() {
        let none = filters(&[("A", false), ("B", false)]);
        assert_eq!(active_filter_list(&none), "");
        assert_eq!(active_filter_overflow(&none), "");
        let one = filters(&[("Team A", true), ("Team B", false), ("Team C", false)]);
        assert_eq!(active_filter_list(&one), "Team A");
        assert_eq!(active_filter_overflow(&one), "");
        let two = filters(&[("Team A", true), ("Team B", true), ("Team C", false)]);
        assert_eq!(active_filter_list(&two), "Team A");
        assert_eq!(active_filter_overflow(&two), "+1");
        let all = filters(&[("Team A", true), ("Team B", true)]);
        assert_eq!(active_filter_list(&all), "All");
        assert_eq!(active_filter_overflow(&all), "");
    }

    #[test]
    fn size_and_kind_follow_the_button_map() {
        assert_eq!(filter_pill_button_size("200"), "sm");
        assert_eq!(filter_pill_button_size("100"), "xs");
        assert_eq!(filter_pill_clear_icon_size("100"), "100");
        assert_eq!(filter_pill_clear_icon_size("200"), "200");
        assert_eq!(filter_button_kind(true, false), "default");
        assert_eq!(filter_button_kind(true, true), "muted");
        assert_eq!(filter_button_kind(false, false), "muted");
    }

    #[test]
    fn clear_and_single_select_update_active() {
        let source = filters(&[("A", true), ("B", true)]);
        assert_eq!(cleared(&source), filters(&[("A", false), ("B", false)]));
        assert_eq!(
            selected_only(&source, "B"),
            filters(&[("A", false), ("B", true)])
        );
        assert_eq!(
            with_active(&source, &["A".to_string()]),
            filters(&[("A", true), ("B", false)])
        );
    }
}
