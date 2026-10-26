use leptos::prelude::*;

#[cfg(test)]
use super::button::button_classes;
use super::button::DtButton;
use super::dropdown::DtDropdown;
use super::icon::DtIconChevronDown;
use super::tooltip::DtTooltip;
use super::unique_id;

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Icon token for the end chevron. Dialtone `SPLIT_BUTTON_ICON_SIZES`.
///
/// `100`/`200`/`xs`/`sm` are `100`. `300`/`400`/`md`/`lg` are `200`.
/// `500`/`xl` is `300`. Size a start icon with
/// [`button_icon_size`](super::button::button_icon_size).
pub fn split_button_icon_size(size: &str) -> &'static str {
    match size {
        "100" | "200" | "xs" | "sm" => "100",
        "500" | "xl" => "300",
        _ => "200",
    }
}

fn split_size_suffix(size: &str) -> &'static str {
    match size {
        "100" | "xs" => "xs",
        "200" | "sm" => "sm",
        "400" | "lg" => "lg",
        "500" | "xl" => "xl",
        _ => "md",
    }
}

/// Root class. `class` is Vue `$attrs.class` and follows `d-split-btn`.
/// `no_divider` adds `d-split-btn--no-divider`.
pub(crate) fn split_root_class(root_class: &str, class: &str, no_divider: bool) -> String {
    join_class([
        root_class,
        "d-split-btn",
        if no_divider {
            "d-split-btn--no-divider"
        } else {
            ""
        },
        class,
    ])
}

/// Side class. The size suffix is the t-shirt token the CSS already defines.
pub(crate) fn split_side_class(side: &str, size: &str, extra: &str) -> String {
    let suffix = split_size_suffix(size);
    join_class([
        format!("d-split-btn__{side} d-split-btn__{side}--{suffix}"),
        extra.to_string(),
    ])
}

/// Full button class for one side, in `DtButton` order plus the split modifier.
#[cfg(test)]
pub(crate) fn split_button_class(
    side: &str,
    size: &str,
    kind: &str,
    importance: &str,
    loading: bool,
    icon_only: bool,
    vertical: bool,
    active: bool,
    extra: &str,
) -> String {
    join_class([
        button_classes(
            false, "default", false, kind, importance, size, false, loading, icon_only, vertical,
            active,
        ),
        split_side_class(side, size, extra),
    ])
}

fn warn_token(label: &str, value: &str, allowed: &[&str]) {
    if !allowed.contains(&value) {
        leptos::logging::warn!(
            "DtSplitButton {label} `{value}` is not {}",
            allowed.join(", ")
        );
    }
}

fn prefer_text(deprecated: &str, current: &str) -> String {
    if deprecated.trim().is_empty() {
        current.to_string()
    } else {
        deprecated.to_string()
    }
}

fn prefer_static(deprecated: &'static str, current: &'static str) -> &'static str {
    if deprecated.is_empty() {
        current
    } else {
        deprecated
    }
}

fn prefer_bool(deprecated: Option<bool>, current: bool) -> bool {
    deprecated.unwrap_or(current)
}

fn with_tooltip(text: String, inverted: bool, button: AnyView) -> AnyView {
    if text.trim().is_empty() {
        button
    } else {
        view! {
            <DtTooltip message=text inverted=inverted flatten=true>
                {button}
            </DtTooltip>
        }
        .into_any()
    }
}

/// Dialtone Vue `DtSplitButton` (`dialtone/v10.7.0`).
///
/// The root is `span.d-split-btn`. `class` is `$attrs.class` on that root.
/// `show_divider` false adds `d-split-btn--no-divider` only when `importance`
/// is `clear`. The sides are `d-split-btn__start` and `d-split-btn__end`.
/// The size suffix is the t-shirt token (`100` is `--xs`) so icon-only padding
/// matches the vendored CSS. Deprecated `alpha*` and `omega*` values win when
/// they are set. `on_alpha_click` fires with `on_start_click`. `on_omega_click`
/// fires with `on_end_click`. Tooltip and dropdown wrappers are
/// `display: contents` so the two buttons are the flex items. Vue Router
/// `startTo` is not ported. `start_href` renders the start side as an anchor.
#[component]
pub fn DtSplitButton(
    /// `100`–`500` (default `300`), or `xs`, `sm`, `md`, `lg`, `xl`.
    #[prop(optional, default = "300")]
    size: &'static str,
    /// `clear`, `outlined`, or `primary` (default).
    #[prop(optional, default = "primary")]
    importance: &'static str,
    /// `default`, `muted`, `critical`, `positive`, `inverted`, or `unstyled`.
    /// `danger` still emits `d-btn--danger`.
    #[prop(optional, default = "default")]
    kind: &'static str,
    /// Disables both sides (`disabled`).
    #[prop(optional)]
    disabled: bool,
    /// Disables the start button (`startDisabled`).
    #[prop(optional)]
    start_disabled: bool,
    /// @deprecated Use `start_disabled` (`alphaDisabled`). A set value wins.
    #[prop(optional)]
    alpha_disabled: Option<bool>,
    /// Disables the end button (`endDisabled`).
    #[prop(optional)]
    end_disabled: bool,
    /// @deprecated Use `end_disabled` (`omegaDisabled`). A set value wins.
    #[prop(optional)]
    omega_disabled: Option<bool>,
    /// Start loading spinner (`startLoading`).
    #[prop(optional)]
    start_loading: bool,
    /// @deprecated Use `start_loading` (`alphaLoading`). A set value wins.
    #[prop(optional)]
    alpha_loading: Option<bool>,
    /// Start active style (`startActive`).
    #[prop(optional)]
    start_active: bool,
    /// @deprecated Use `start_active` (`alphaActive`). A set value wins.
    #[prop(optional)]
    alpha_active: Option<bool>,
    /// End active style (`endActive`). Ignored while `dropdown_list` is open.
    #[prop(optional)]
    end_active: bool,
    /// @deprecated Use `end_active` (`omegaActive`). A set value wins.
    #[prop(optional)]
    omega_active: Option<bool>,
    /// Start icon position (`startIconPosition`).
    #[prop(optional, default = "left")]
    start_icon_position: &'static str,
    /// @deprecated Use `start_icon_position` (`alphaIconPosition`).
    #[prop(optional, default = "")]
    alpha_icon_position: &'static str,
    /// Extra class on the start button (`startButtonClass`).
    #[prop(optional, into)]
    start_button_class: String,
    /// Extra class on the end button (`endButtonClass`).
    #[prop(optional, into)]
    end_button_class: String,
    /// Extra class on the start label (`startLabelClass`).
    #[prop(optional, into)]
    start_label_class: String,
    /// @deprecated Use `start_label_class` (`alphaLabelClass`).
    #[prop(optional, into)]
    alpha_label_class: String,
    /// Extra class on the start leading wrapper (`startLeadingClass`).
    #[prop(optional, into)]
    start_leading_class: String,
    /// @deprecated Use `start_leading_class` (`alphaLeadingClass`).
    #[prop(optional, into)]
    alpha_leading_class: String,
    /// Extra class on the start trailing wrapper (`startTrailingClass`).
    #[prop(optional, into)]
    start_trailing_class: String,
    /// @deprecated Use `start_trailing_class` (`alphaTrailingClass`).
    #[prop(optional, into)]
    alpha_trailing_class: String,
    /// `aria-live="assertive"` while the start button is focused (`assertiveOnFocus`).
    #[prop(optional)]
    assertive_on_focus: bool,
    /// Start accessible name (`startAriaLabel`).
    #[prop(optional, into)]
    start_aria_label: String,
    /// @deprecated Use `start_aria_label` (`alphaAriaLabel`).
    #[prop(optional, into)]
    alpha_aria_label: String,
    /// End accessible name (`endAriaLabel`).
    #[prop(optional, into)]
    end_aria_label: String,
    /// @deprecated Use `end_aria_label` (`omegaAriaLabel`).
    #[prop(optional, into)]
    omega_aria_label: String,
    /// End element id (`endId`). Generated when empty.
    #[prop(optional, into)]
    end_id: String,
    /// @deprecated Use `end_id` (`omegaId`).
    #[prop(optional, into)]
    omega_id: String,
    /// Start tooltip (`startTooltipText`). Required when the start button is icon-only.
    #[prop(optional, into)]
    start_tooltip_text: String,
    /// @deprecated Use `start_tooltip_text` (`alphaTooltipText`).
    #[prop(optional, into)]
    alpha_tooltip_text: String,
    /// End tooltip (`endTooltipText`). Required unless `end` replaces that side.
    #[prop(optional, into)]
    end_tooltip_text: String,
    /// @deprecated Use `end_tooltip_text` (`omegaTooltipText`).
    #[prop(optional, into)]
    omega_tooltip_text: String,
    /// Dropdown placement (`dropdownPlacement`).
    #[prop(optional, default = "bottom-end")]
    dropdown_placement: &'static str,
    /// CSS width of the root (`width`). Empty omits the style.
    #[prop(optional, into)]
    width: String,
    /// Divider between the sides (`showDivider`). Only `clear` can hide it.
    #[prop(optional, default = true)]
    show_divider: bool,
    /// Start anchor href (`startHref`). Vue Router `startTo` is not ported.
    #[prop(optional, into)]
    start_href: String,
    /// Start anchor target (`startTarget`).
    #[prop(optional, into)]
    start_target: String,
    /// Start anchor rel (`startRel`).
    #[prop(optional, into)]
    start_rel: String,
    /// Extra class before `d-split-btn`.
    #[prop(optional, into)]
    root_class: String,
    /// Extra class on the root (Vue `$attrs.class`).
    #[prop(optional, into)]
    class: String,
    /// `data-qa` on the start button. Default `dt-split-button-start`.
    #[prop(optional, default = "dt-split-button-start")]
    start_data_qa: &'static str,
    /// `data-qa` on the end button. Default `dt-split-button-end`.
    #[prop(optional, default = "dt-split-button-end")]
    end_data_qa: &'static str,
    /// Start click (`start-clicked`).
    #[prop(optional, into)]
    on_start_click: Option<Callback<()>>,
    /// @deprecated Also fired on the start click (`alpha-clicked`).
    #[prop(optional, into)]
    on_alpha_click: Option<Callback<()>>,
    /// End click (`end-clicked`).
    #[prop(optional, into)]
    on_end_click: Option<Callback<()>>,
    /// @deprecated Also fired on the end click (`omega-clicked`).
    #[prop(optional, into)]
    on_omega_click: Option<Callback<()>>,
    /// Start icon (`startIcon`).
    #[prop(optional_no_strip)]
    start_icon: Option<ViewFn>,
    /// @deprecated Use `start_icon` (`alphaIcon`).
    #[prop(optional_no_strip)]
    alpha_icon: Option<ViewFn>,
    /// Second icon on the start button (`startEndIcon`). Enables the dual-icon path.
    #[prop(optional_no_strip)]
    start_end_icon: Option<ViewFn>,
    /// End icon (`endIcon`). The default is [`DtIconChevronDown`].
    #[prop(optional_no_strip)]
    end_icon: Option<ViewFn>,
    /// @deprecated Use `end_icon` (`omegaIcon`).
    #[prop(optional_no_strip)]
    omega_icon: Option<ViewFn>,
    /// Content before the start label (`leading`).
    #[prop(optional_no_strip)]
    leading: Option<ViewFn>,
    /// Content after the start label (`trailing`).
    #[prop(optional_no_strip)]
    trailing: Option<ViewFn>,
    /// Replaces the end button (`end`).
    #[prop(optional_no_strip)]
    end: Option<ViewFn>,
    /// @deprecated Use `end` (`omega`).
    #[prop(optional_no_strip)]
    omega: Option<ViewFn>,
    /// Dropdown list (`dropdownList`). The end button opens it.
    #[prop(optional)]
    dropdown_list: Option<ViewFn>,
    /// Start label. Absent with a start icon makes an icon-only start button.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let sizes = [
        "100", "200", "300", "400", "500", "xs", "sm", "md", "lg", "xl",
    ];
    warn_token("size", size, &sizes);
    warn_token("importance", importance, &["clear", "outlined", "primary"]);
    warn_token(
        "kind",
        kind,
        &[
            "default", "muted", "critical", "danger", "positive", "inverted", "unstyled",
        ],
    );
    let icon_position = prefer_static(alpha_icon_position, start_icon_position);
    warn_token(
        "start_icon_position",
        icon_position,
        &[
            "start",
            "end",
            "blockStart",
            "blockEnd",
            "left",
            "right",
            "top",
            "bottom",
        ],
    );
    if !show_divider && importance != "clear" {
        leptos::logging::warn!(
            "show-divider prop set to false has no effect when importance is not \"clear\". The divider is always shown for \"outlined\" and \"primary\" importances."
        );
    }

    let start_icon_slot = start_icon.or(alpha_icon);
    let end_icon_slot = end_icon.or(omega_icon);
    let end_replacement = end.or(omega);
    let has_start_icon = start_icon_slot.is_some() || start_end_icon.is_some();
    let start_tip = prefer_text(&alpha_tooltip_text, &start_tooltip_text);
    let end_tip = prefer_text(&omega_tooltip_text, &end_tooltip_text);
    if children.is_none() && has_start_icon && start_tip.trim().is_empty() {
        leptos::logging::warn!(
            "start-tooltip-text prop must be set if start button has an icon only"
        );
    }
    if end_replacement.is_none() && end_tip.trim().is_empty() {
        leptos::logging::warn!("end-tooltip-text prop is required as it is an icon-only button");
    }

    let (legacy_icon, dual_start, dual_end) = if start_end_icon.is_some() {
        (None, start_icon_slot, start_end_icon)
    } else {
        (start_icon_slot, None, None)
    };
    let start_is_disabled = disabled || prefer_bool(alpha_disabled, start_disabled);
    let end_is_disabled = disabled || prefer_bool(omega_disabled, end_disabled);
    let start_is_loading = prefer_bool(alpha_loading, start_loading);
    let start_is_active = prefer_bool(alpha_active, start_active);
    let end_is_active = prefer_bool(omega_active, end_active);
    let label_class = prefer_text(&alpha_label_class, &start_label_class);
    let leading_class = prefer_text(&alpha_leading_class, &start_leading_class);
    let trailing_class = prefer_text(&alpha_trailing_class, &start_trailing_class);
    let start_name = prefer_text(&alpha_aria_label, &start_aria_label);
    let end_name = prefer_text(&omega_aria_label, &end_aria_label);
    let end_element_id = {
        let chosen = prefer_text(&omega_id, &end_id);
        if chosen.is_empty() {
            unique_id("dt-split-button-end")
        } else {
            chosen
        }
    };
    let no_divider = !show_divider && importance == "clear";
    let root = split_root_class(&root_class, &class, no_divider);
    let style = (!width.is_empty()).then(|| format!("width: {width}"));
    let start_class = split_side_class("start", size, &start_button_class);
    let end_class = StoredValue::new(split_side_class("end", size, &end_button_class));
    let inverted = kind == "inverted";
    let icon_size = split_button_icon_size(size);
    let end_icon_stored = StoredValue::new(end_icon_slot);
    let end_tip_stored = StoredValue::new(end_tip);
    let end_name = StoredValue::new(end_name);
    let end_id = StoredValue::new(end_element_id);
    let on_start = Callback::new(move |_| {
        if let Some(callback) = on_start_click {
            callback.run(());
        }
        if let Some(callback) = on_alpha_click {
            callback.run(());
        }
    });
    let on_end = Callback::new(move |_| {
        if let Some(callback) = on_end_click {
            callback.run(());
        }
        if let Some(callback) = on_omega_click {
            callback.run(());
        }
    });

    let start_button = match (legacy_icon, children) {
        (Some(icon), Some(children)) => view! {
            <DtButton
                data_qa=start_data_qa
                class=start_class.clone()
                kind=kind
                importance=importance
                size=size
                icon_position=icon_position
                label_class=label_class.clone()
                leading_class=leading_class.clone()
                trailing_class=trailing_class.clone()
                disabled=start_is_disabled
                loading=start_is_loading
                active=start_is_active
                assertive_on_focus=assertive_on_focus
                aria_label=start_name.clone()
                href=start_href.clone()
                target=start_target.clone()
                rel=start_rel.clone()
                icon=icon
                start_icon=dual_start
                end_icon=dual_end
                leading=leading
                trailing=trailing
                on_click=on_start
            >
                {children()}
            </DtButton>
        }
        .into_any(),
        (None, Some(children)) => view! {
            <DtButton
                data_qa=start_data_qa
                class=start_class.clone()
                kind=kind
                importance=importance
                size=size
                icon_position=icon_position
                label_class=label_class.clone()
                leading_class=leading_class.clone()
                trailing_class=trailing_class.clone()
                disabled=start_is_disabled
                loading=start_is_loading
                active=start_is_active
                assertive_on_focus=assertive_on_focus
                aria_label=start_name.clone()
                href=start_href.clone()
                target=start_target.clone()
                rel=start_rel.clone()
                start_icon=dual_start
                end_icon=dual_end
                leading=leading
                trailing=trailing
                on_click=on_start
            >
                {children()}
            </DtButton>
        }
        .into_any(),
        (Some(icon), None) => view! {
            <DtButton
                data_qa=start_data_qa
                class=start_class.clone()
                kind=kind
                importance=importance
                size=size
                icon_position=icon_position
                label_class=label_class
                leading_class=leading_class
                trailing_class=trailing_class
                disabled=start_is_disabled
                loading=start_is_loading
                active=start_is_active
                assertive_on_focus=assertive_on_focus
                aria_label=start_name
                href=start_href
                target=start_target
                rel=start_rel
                icon=icon
                start_icon=dual_start
                end_icon=dual_end
                leading=leading
                trailing=trailing
                on_click=on_start
            />
        }
        .into_any(),
        (None, None) => view! {
            <DtButton
                data_qa=start_data_qa
                class=start_class
                kind=kind
                importance=importance
                size=size
                icon_position=icon_position
                label_class=label_class
                leading_class=leading_class
                trailing_class=trailing_class
                disabled=start_is_disabled
                loading=start_is_loading
                active=start_is_active
                assertive_on_focus=assertive_on_focus
                aria_label=start_name
                href=start_href
                target=start_target
                rel=start_rel
                start_icon=dual_start
                end_icon=dual_end
                leading=leading
                trailing=trailing
                on_click=on_start
            />
        }
        .into_any(),
    };
    let start = with_tooltip(start_tip, inverted, start_button);

    let end_button = move |active_flag: MaybeProp<bool>, menu: bool| -> AnyView {
        let icon_slot = end_icon_stored.get_value();
        let popup = if menu { "menu" } else { "" };
        let button = view! {
            <DtButton
                data_qa=end_data_qa
                id=end_id.get_value()
                class=end_class.get_value()
                kind=kind
                importance=importance
                size=size
                disabled=end_is_disabled
                active=active_flag
                aria_label=end_name.get_value()
                aria_haspopup=popup
                icon=ViewFn::from(move || {
                    if let Some(slot) = icon_slot.clone() {
                        slot.run()
                    } else {
                        view! { <DtIconChevronDown size=icon_size /> }.into_any()
                    }
                })
                on_click=on_end
            />
        }
        .into_any();
        with_tooltip(end_tip_stored.get_value(), inverted, button)
    };

    let end_view = if let Some(slot) = end_replacement {
        slot.run().into_any()
    } else if let Some(list) = dropdown_list {
        let list = StoredValue::new(list);
        let dropdown_open = RwSignal::new(false);
        view! {
            <DtDropdown
                placement=dropdown_placement
                flatten_anchor=true
                on_opened=Callback::new(move |open: bool| dropdown_open.set(open))
                anchor=ViewFn::from(move || {
                    end_button(Signal::derive(move || dropdown_open.get()).into(), true)
                })
            >
                {list.get_value().run()}
            </DtDropdown>
        }
        .into_any()
    } else {
        end_button(end_is_active.into(), false)
    };

    view! {
        <span data-qa="dt-split-button" class=root style=style>{start}{end_view}</span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_class_puts_the_split_class_then_the_attr_class() {
        assert_eq!(split_root_class("", "", false), "d-split-btn");
        assert_eq!(split_root_class("wide", "", false), "wide d-split-btn");
        assert_eq!(
            split_root_class("", "extra", true),
            "d-split-btn d-split-btn--no-divider extra"
        );
    }

    #[test]
    fn size_suffix_uses_the_tshirt_token_the_css_defines() {
        assert_eq!(split_size_suffix("100"), "xs");
        assert_eq!(split_size_suffix("xs"), "xs");
        assert_eq!(split_size_suffix("200"), "sm");
        assert_eq!(split_size_suffix("300"), "md");
        assert_eq!(split_size_suffix("md"), "md");
        assert_eq!(split_size_suffix("400"), "lg");
        assert_eq!(split_size_suffix("500"), "xl");
        assert_eq!(
            split_side_class("end", "100", ""),
            "d-split-btn__end d-split-btn__end--xs"
        );
        assert_eq!(
            split_side_class("start", "300", "extra"),
            "d-split-btn__start d-split-btn__start--md extra"
        );
    }

    #[test]
    fn end_icon_sizes_follow_split_button_constants() {
        assert_eq!(split_button_icon_size("100"), "100");
        assert_eq!(split_button_icon_size("200"), "100");
        assert_eq!(split_button_icon_size("xs"), "100");
        assert_eq!(split_button_icon_size("sm"), "100");
        assert_eq!(split_button_icon_size("300"), "200");
        assert_eq!(split_button_icon_size("400"), "200");
        assert_eq!(split_button_icon_size("md"), "200");
        assert_eq!(split_button_icon_size("lg"), "200");
        assert_eq!(split_button_icon_size("500"), "300");
        assert_eq!(split_button_icon_size("xl"), "300");
    }

    #[test]
    fn deprecated_values_win_when_they_are_set() {
        assert_eq!(prefer_text("", "start"), "start");
        assert_eq!(prefer_text("alpha", "start"), "alpha");
        assert_eq!(prefer_static("", "left"), "left");
        assert_eq!(prefer_static("top", "left"), "top");
        assert!(!prefer_bool(None, false));
        assert!(prefer_bool(None, true));
        assert!(!prefer_bool(Some(false), true));
        assert!(prefer_bool(Some(true), false));
    }

    #[test]
    fn side_classes_follow_the_button_then_the_split_modifier() {
        assert_eq!(
            split_button_class(
                "start", "300", "default", "primary", false, false, false, false, ""
            ),
            "base-button__button d-btn d-btn--primary d-split-btn__start d-split-btn__start--md"
        );
        assert_eq!(
            split_button_class(
                "end", "lg", "danger", "primary", false, true, false, true, ""
            ),
            "base-button__button d-btn d-btn--primary d-btn--danger d-btn--lg d-btn--icon-only d-btn--active d-split-btn__end d-split-btn__end--lg"
        );
        assert_eq!(
            split_button_class(
                "start", "100", "critical", "outlined", true, false, false, false, "extra"
            ),
            "base-button__button d-btn d-btn--outlined d-btn--critical d-btn--xs d-btn--loading d-split-btn__start d-split-btn__start--xs extra"
        );
    }
}
