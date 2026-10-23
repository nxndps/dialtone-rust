use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, KeyboardEvent, MouseEvent};

use super::loader::DtLoader;

/// Screen-reader label while `loading` is set. Dialtone `DIALTONE_LOADING` in English.
const LOADING_LABEL: &str = "loading";

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn button_size_class(size: &str) -> &'static str {
    match size {
        "100" | "xs" => "d-btn--xs",
        "200" | "sm" => "d-btn--sm",
        "400" | "lg" => "d-btn--lg",
        "500" | "xl" => "d-btn--xl",
        _ => "",
    }
}

fn button_kind_class(kind: &str) -> &'static str {
    match kind {
        "muted" => "d-btn--muted",
        "critical" => "d-btn--critical",
        "danger" => "d-btn--danger",
        "positive" => "d-btn--positive",
        "inverted" => "d-btn--inverted",
        _ => "",
    }
}

fn warn_button_combination(circle: bool, kind: &str, importance: &str) {
    if kind == "unstyled" {
        return;
    }
    let invalid = matches!(
        (circle, kind, importance),
        (true, "default", "primary")
            | (true, "critical", "outlined")
            | (true, "muted", "primary")
            | (false, "muted", "primary")
    );
    if invalid {
        let shape = if circle { "circle " } else { "" };
        leptos::logging::warn!(
            "You cannot have a {shape}button with kind: {kind} and importance: {importance} as it does not exist in our design system. See https://dialtone.dialpad.com/components/button.html for a list of available button styles"
        );
    }
}

fn button_importance_class(importance: &str) -> &'static str {
    match importance {
        "outlined" => "d-btn--outlined",
        "primary" => "d-btn--primary",
        _ => "",
    }
}

fn link_kind_class(kind: &str, inverted: bool) -> &'static str {
    if inverted {
        return match kind {
            "" | "default" => "d-link--inverted",
            "warning" => "d-link--inverted-warning",
            "danger" => "d-link--inverted-danger",
            "success" => "d-link--inverted-success",
            "muted" => "d-link--inverted-muted",
            "mention" => "d-link--inverted-mention",
            _ => "",
        };
    }
    match kind {
        "warning" => "d-link--warning",
        "danger" => "d-link--danger",
        "success" => "d-link--success",
        "muted" => "d-link--muted",
        "mention" => "d-link--mention",
        _ => "",
    }
}

fn icon_position_class(position: &str) -> &'static str {
    match position {
        "right" | "end" => "d-btn__icon--right",
        "top" | "blockStart" => "d-btn__icon--top",
        "bottom" | "blockEnd" => "d-btn__icon--bottom",
        _ => "d-btn__icon--left",
    }
}

fn button_type(type_name: &str) -> &'static str {
    match type_name {
        "submit" => "submit",
        "reset" => "reset",
        _ => "button",
    }
}

/// Icon token for `size`. Dialtone `BUTTON_ICON_SIZES`.
///
/// A Leptos slot does not receive the Vue `icon-size` slot prop. Size a
/// slotted icon with this value.
pub fn button_icon_size(size: &str) -> &'static str {
    match size {
        "100" | "200" | "xs" | "sm" => "200",
        "400" | "lg" => "400",
        "500" | "xl" => "500",
        _ => "300",
    }
}

pub(crate) fn button_classes(
    link: bool,
    link_kind: &str,
    link_inverted: bool,
    kind: &str,
    importance: &str,
    size: &str,
    circle: bool,
    loading: bool,
    icon_only: bool,
    vertical: bool,
    active: bool,
) -> String {
    let modifiers = if link {
        join_class([
            "d-link",
            link_kind_class(link_kind, link_inverted),
            button_size_class(size),
        ])
    } else if kind == "unstyled" {
        "d-btn--unstyled".to_string()
    } else {
        join_class([
            "d-btn",
            button_importance_class(importance),
            button_kind_class(kind),
            button_size_class(size),
            if circle { "d-btn--circle" } else { "" },
            if loading { "d-btn--loading" } else { "" },
            if icon_only { "d-btn--icon-only" } else { "" },
            if vertical { "d-btn--vertical" } else { "" },
            if active { "d-btn--active" } else { "" },
        ])
    };
    join_class(["base-button__button", modifiers.as_str()])
}

fn icon_class(kind: &str, position: &str) -> String {
    if kind == "unstyled" {
        "base-button__icon".to_string()
    } else {
        join_class([
            "base-button__icon",
            "d-btn__icon",
            icon_position_class(position),
        ])
    }
}

fn label_classes(kind: &str, extra: &str) -> String {
    if kind == "unstyled" {
        join_class(["base-button__label", extra])
    } else {
        join_class(["base-button__label", "d-btn__label", extra])
    }
}

fn live_value(assertive: bool, focused: bool, fallback: &str) -> Option<String> {
    if assertive && focused {
        Some("assertive".to_string())
    } else if fallback.is_empty() {
        None
    } else {
        Some(fallback.to_string())
    }
}

/// Dialtone Vue `DtButton`.
///
/// `kind` is `default`, `muted`, `critical`, `danger`, `positive`, `inverted`,
/// or `unstyled`. `critical` is `d-btn--critical`. `danger` stays `d-btn--danger`.
/// `importance` is `clear`, `outlined`, or `primary` (default). Size is `100`–`500`
/// or the t-shirt aliases `xs`–`xl`. `300` and `md` add no size class. `link`
/// renders link classes instead of button classes. A non-empty `href` renders
/// an `<a>`. Vue Router `to` is not ported. `start_icon` (`startIcon`) and
/// `end_icon` (`endIcon`) hide the legacy `icon` slot. `leading` and `trailing`
/// use `d-btn__leading` and `d-btn__trailing`. Size a slotted icon with
/// [`button_icon_size`].
#[component]
pub fn DtButton(
    #[prop(optional)] circle: bool,
    /// Icon slot position (`iconPosition`): `left`, `right`, `top`, `bottom`,
    /// `start`, `end`, `blockStart`, or `blockEnd`.
    #[prop(optional, default = "left")]
    icon_position: &'static str,
    /// `clear`, `outlined`, or `primary` (default).
    #[prop(optional, default = "primary")]
    importance: &'static str,
    /// Renders link styles (`link`).
    #[prop(optional)]
    link: bool,
    /// Link color (`linkKind`).
    #[prop(optional, default = "default")]
    link_kind: &'static str,
    /// Inverted link color (`linkInverted`).
    #[prop(optional)]
    link_inverted: bool,
    /// `disabled`. A signal updates the attribute after the first render.
    #[prop(optional, into)]
    disabled: MaybeProp<bool>,
    /// `button` (default), `submit`, or `reset` (`type`).
    #[prop(optional, default = "button")]
    type_: &'static str,
    /// CSS width (`width`). Empty omits the style.
    #[prop(optional, into)]
    width: String,
    /// `100`–`500`, or `xs`, `sm`, `md` (default), `lg`, `xl`.
    #[prop(optional, default = "md")]
    size: &'static str,
    /// Extra class on the label (`labelClass`).
    #[prop(optional, into)]
    label_class: String,
    /// Extra class on the leading wrapper (`leadingClass`).
    #[prop(optional, into)]
    leading_class: String,
    /// Extra class on the trailing wrapper (`trailingClass`).
    #[prop(optional, into)]
    trailing_class: String,
    #[prop(optional)] loading: bool,
    /// `default`, `muted`, `critical`, `danger`, `positive`, `inverted`, or `unstyled`.
    #[prop(optional, default = "default")]
    kind: &'static str,
    /// Sets `aria-live="assertive"` while the button is focused (`assertiveOnFocus`).
    #[prop(optional)]
    assertive_on_focus: bool,
    /// Fallback `aria-live` when the button is not in that focused state.
    #[prop(optional, into)]
    aria_live: String,
    /// `active`. A signal updates `d-btn--active` after the first render.
    #[prop(optional, into)]
    active: MaybeProp<bool>,
    /// Renders an anchor when non-empty (`href`).
    #[prop(optional, into)]
    href: String,
    #[prop(optional, into)] target: String,
    #[prop(optional, into)] rel: String,
    /// Replaces the accessible name while `loading` is set.
    #[prop(optional, into)]
    aria_label: String,
    /// `title`. Empty omits the attribute.
    #[prop(optional, into)]
    title: String,
    /// Extra classes appended to the button.
    #[prop(optional, into)]
    class: String,
    /// `data-qa`. Default `dt-button`. Split and segmented controls set their own.
    #[prop(optional, default = "dt-button")]
    data_qa: &'static str,
    /// `tabindex`. Unset omits the attribute.
    #[prop(optional, into)]
    tab_index: MaybeProp<i32>,
    /// `aria-checked`. Unset omits the attribute. Rendered as `"true"` or `"false"`.
    #[prop(optional, into)]
    aria_checked: MaybeProp<bool>,
    /// `aria-disabled="true"` when true. Unset omits it on a `<button>`.
    #[prop(optional, into)]
    aria_disabled: MaybeProp<bool>,
    /// `data-value`. Empty omits the attribute.
    #[prop(optional, into)]
    data_value: String,
    /// Element id. Empty omits the attribute.
    #[prop(optional, into)]
    id: String,
    /// ARIA role. Empty omits the attribute.
    #[prop(optional, into)]
    role: String,
    /// `aria-selected`. Unset omits the attribute.
    #[prop(optional)]
    aria_selected: Option<bool>,
    /// `aria-expanded`. Unset omits the attribute.
    #[prop(optional)]
    aria_expanded: Option<bool>,
    /// `aria-controls`. Empty omits the attribute.
    #[prop(optional, into)]
    aria_controls: String,
    /// `aria-haspopup`. Empty omits the attribute.
    #[prop(optional, default = "")]
    aria_haspopup: &'static str,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
    #[prop(optional, into)] on_focus_in: Option<Callback<()>>,
    #[prop(optional, into)] on_focus_out: Option<Callback<()>>,
    /// Icon slot (`icon`). Hidden when `link` is set, or when `start_icon` or
    /// `end_icon` is set.
    #[prop(optional)]
    icon: Option<ViewFn>,
    /// Icon at the start (`startIcon`). Pass `None` or omit it.
    #[prop(optional_no_strip)]
    start_icon: Option<ViewFn>,
    /// Icon at the end (`endIcon`). Pass `None` or omit it. With `start_icon`,
    /// `icon_position` is ignored.
    #[prop(optional_no_strip)]
    end_icon: Option<ViewFn>,
    /// Content before the label (`leading`). Pass `None` or omit it.
    #[prop(optional_no_strip)]
    leading: Option<ViewFn>,
    /// Content after the label (`trailing`). Pass `None` or omit it.
    #[prop(optional_no_strip)]
    trailing: Option<ViewFn>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    warn_button_combination(circle, kind, importance);
    let has_new_icons = start_icon.is_some() || end_icon.is_some();
    let show_legacy_icon = icon.is_some() && !has_new_icons && !link;
    let show_label = children.is_some();
    let icon_only = (has_new_icons || show_legacy_icon) && !show_label;
    let vertical = !icon_only
        && !has_new_icons
        && matches!(icon_position, "top" | "bottom" | "blockStart" | "blockEnd");
    let extra_class = StoredValue::new(class);
    let class_name = move || {
        join_class([
            button_classes(
                link,
                link_kind,
                link_inverted,
                kind,
                importance,
                size,
                circle,
                loading,
                icon_only,
                vertical,
                active.get().unwrap_or(false),
            )
            .as_str(),
            extra_class.get_value().as_str(),
        ])
    };
    let is_disabled = move || disabled.get().unwrap_or(false);
    let tab_value = move || tab_index.get().map(|index| index.to_string());
    let checked_value = move || {
        aria_checked
            .get()
            .map(|checked| if checked { "true" } else { "false" })
    };
    let data_value_attr = (!data_value.is_empty()).then_some(data_value);
    let id_attr = (!id.is_empty()).then_some(id);
    let role_attr = (!role.is_empty()).then_some(role);
    let aria_controls_attr = (!aria_controls.is_empty()).then_some(aria_controls);
    let aria_haspopup_attr = (!aria_haspopup.is_empty()).then_some(aria_haspopup);
    let title_attr = (!title.is_empty()).then_some(title);
    let aria_selected_attr = aria_selected.map(|selected| if selected { "true" } else { "false" });
    let aria_expanded_attr = aria_expanded.map(|expanded| if expanded { "true" } else { "false" });
    let icon_class_name = icon_class(kind, icon_position);
    let start_icon_class = icon_class(kind, "left");
    let end_icon_class = icon_class(kind, "right");
    let leading_class_name = join_class(["d-btn__leading", leading_class.as_str()]);
    let trailing_class_name = join_class(["d-btn__trailing", trailing_class.as_str()]);
    let label_class_name = label_classes(kind, &label_class);
    let loader_size = button_icon_size(size);
    let show_loader = loading && kind != "unstyled";
    let button_type = button_type(type_);
    let style = if width.is_empty() {
        String::new()
    } else {
        format!("width: {width}")
    };
    let button_label = if loading {
        Some(LOADING_LABEL.to_string())
    } else if aria_label.is_empty() {
        None
    } else {
        Some(aria_label)
    };
    let focused = RwSignal::new(false);
    let is_anchor = !href.is_empty();

    let content = view! {
        {show_loader.then(|| view! {
            <DtLoader class="d-btn__loader" size=loader_size aria_hidden=true />
        })}
        {leading.filter(|_| !link).map(|slot| view! {
            <span class=leading_class_name.clone()>{slot.run()}</span>
        })}
        {start_icon.filter(|_| !link).map(|slot| view! {
            <span data-qa="dt-button-start-icon" class=start_icon_class.clone()>
                {slot.run()}
            </span>
        })}
        {icon.filter(|_| show_legacy_icon).map(|slot| view! {
            <span data-qa="dt-button-icon" class=icon_class_name>
                {slot.run()}
            </span>
        })}
        {children.map(|children| view! {
            <span data-qa="dt-button-label" class=label_class_name>
                {children()}
            </span>
        })}
        {end_icon.filter(|_| !link).map(|slot| view! {
            <span data-qa="dt-button-end-icon" class=end_icon_class.clone()>
                {slot.run()}
            </span>
        })}
        {trailing.filter(|_| !link).map(|slot| view! {
            <span class=trailing_class_name.clone()>{slot.run()}</span>
        })}
    };

    if is_anchor {
        let fallback = aria_live.clone();
        let href = href.clone();
        let target = target.clone();
        let rel = rel.clone();
        view! {
            <a
                id=id_attr
                role=role_attr
                class=class_name
                style=style
                data-qa=data_qa
                data-value=data_value_attr.clone()
                aria-selected=aria_selected_attr
                aria-expanded=aria_expanded_attr
                aria-controls=aria_controls_attr
                aria-haspopup=aria_haspopup_attr
                aria-checked=checked_value
                title=title_attr.clone()
                href=move || (!is_disabled()).then_some(href.clone())
                target=(!target.is_empty()).then_some(target)
                rel=(!rel.is_empty()).then_some(rel)
                aria-label=button_label.clone()
                aria-disabled=move || {
                    if let Some(flag) = aria_disabled.get() {
                        return flag.then_some("true");
                    }
                    is_disabled().then_some("true")
                }
                tabindex=move || tab_value().or_else(|| is_disabled().then(|| "-1".to_string()))
                aria-live=move || live_value(assertive_on_focus, focused.get(), &fallback)
                on:click=move |event: MouseEvent| {
                    if is_disabled() {
                        event.prevent_default();
                        event.stop_propagation();
                        event.stop_immediate_propagation();
                        return;
                    }
                    if let Some(on_click) = on_click {
                        on_click.run(());
                    }
                }
                on:keydown=move |event: KeyboardEvent| {
                    if event.key() == " " || event.code() == "Space" {
                        event.prevent_default();
                        if !is_disabled() {
                            if let Some(target) = event.target() {
                                if let Ok(element) = target.dyn_into::<HtmlElement>() {
                                    element.click();
                                }
                            }
                        }
                    }
                }
                on:focusin=move |_| {
                    if assertive_on_focus {
                        focused.set(true);
                    }
                    if let Some(on_focus_in) = on_focus_in {
                        on_focus_in.run(());
                    }
                }
                on:focusout=move |_| {
                    focused.set(false);
                    if let Some(on_focus_out) = on_focus_out {
                        on_focus_out.run(());
                    }
                }
            >
                {content}
            </a>
        }
        .into_any()
    } else {
        let fallback = aria_live;
        view! {
            <button
                id=id_attr
                type=button_type
                role=role_attr
                class=class_name
                style=style
                data-qa=data_qa
                data-value=data_value_attr
                prop:disabled=move || is_disabled()
                tabindex=tab_value
                aria-selected=aria_selected_attr
                aria-expanded=aria_expanded_attr
                aria-controls=aria_controls_attr
                aria-haspopup=aria_haspopup_attr
                aria-checked=checked_value
                title=title_attr
                aria-disabled=move || aria_disabled.get().unwrap_or(false).then_some("true")
                aria-label=button_label
                aria-live=move || live_value(assertive_on_focus, focused.get(), &fallback)
                on:click=move |_| {
                    if let Some(on_click) = on_click {
                        on_click.run(());
                    }
                }
                on:focusin=move |_| {
                    if assertive_on_focus {
                        focused.set(true);
                    }
                    if let Some(on_focus_in) = on_focus_in {
                        on_focus_in.run(());
                    }
                }
                on:focusout=move |_| {
                    focused.set(false);
                    if let Some(on_focus_out) = on_focus_out {
                        on_focus_out.run(());
                    }
                }
            >
                {content}
            </button>
        }
        .into_any()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes_follow_the_vue_modifiers() {
        assert_eq!(
            button_classes(
                false, "default", false, "default", "primary", "md", false, false, false, false,
                false
            ),
            "base-button__button d-btn d-btn--primary"
        );
        assert_eq!(
            button_classes(false, "default", false, "danger", "outlined", "sm", true, true, true, false, true),
            "base-button__button d-btn d-btn--outlined d-btn--danger d-btn--sm d-btn--circle d-btn--loading d-btn--icon-only d-btn--active"
        );
        assert_eq!(
            button_classes(
                false, "default", false, "unstyled", "primary", "lg", true, true, true, true, true
            ),
            "base-button__button d-btn--unstyled"
        );
        assert_eq!(
            button_classes(
                true, "danger", true, "default", "primary", "lg", true, false, false, true, false
            ),
            "base-button__button d-link d-link--inverted-danger d-btn--lg"
        );
        assert_eq!(
            button_classes(
                false, "default", false, "muted", "clear", "xs", false, false, false, true, false
            ),
            "base-button__button d-btn d-btn--muted d-btn--xs d-btn--vertical"
        );
    }

    #[test]
    fn icon_size_follows_the_button_size() {
        assert_eq!(button_icon_size("xs"), "200");
        assert_eq!(button_icon_size("sm"), "200");
        assert_eq!(button_icon_size("md"), "300");
        assert_eq!(button_icon_size("lg"), "400");
        assert_eq!(button_icon_size("xl"), "500");
        assert_eq!(button_icon_size("100"), "200");
        assert_eq!(button_icon_size("200"), "200");
        assert_eq!(button_icon_size("300"), "300");
        assert_eq!(button_icon_size("400"), "400");
        assert_eq!(button_icon_size("500"), "500");
        assert_eq!(button_type("submit"), "submit");
        assert_eq!(button_type("nope"), "button");
        assert_eq!(link_kind_class("mention", false), "d-link--mention");
        assert_eq!(icon_position_class("bottom"), "d-btn__icon--bottom");
        assert_eq!(icon_position_class("start"), "d-btn__icon--left");
        assert_eq!(icon_position_class("end"), "d-btn__icon--right");
        assert_eq!(icon_position_class("blockStart"), "d-btn__icon--top");
        assert_eq!(icon_position_class("blockEnd"), "d-btn__icon--bottom");
        assert_eq!(
            button_classes(
                false, "default", false, "critical", "primary", "100", false, false, true, false,
                false
            ),
            "base-button__button d-btn d-btn--primary d-btn--critical d-btn--xs d-btn--icon-only"
        );
        assert_eq!(button_size_class("300"), "");
        assert_eq!(button_size_class("500"), "d-btn--xl");
    }
}
