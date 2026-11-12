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
        "positive" => "d-btn--positive",
        "inverted" => "d-btn--inverted",
        _ => "",
    }
}

fn warn_button_kind(kind: &str) {
    if matches!(
        kind,
        "default" | "muted" | "critical" | "positive" | "inverted" | "unstyled" | ""
    ) {
        return;
    }
    leptos::logging::warn!(
        "DtButton kind `{kind}` is not default, muted, critical, positive, inverted, or unstyled."
    );
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

/// Dialtone `getLinkKindModifier` at `dialtone/v10.7.0`.
fn link_kind_class(kind: &str, inverted: bool) -> &'static str {
    if inverted {
        return match kind {
            "" | "default" => "d-link--inverted",
            "critical" => "d-link--inverted-critical",
            "warning" => "d-link--inverted-warning",
            "positive" => "d-link--inverted-positive",
            "info" => "d-link--inverted-info",
            "muted" => "d-link--inverted-muted",
            "mention" => "d-link--inverted-mention",
            _ => "",
        };
    }
    match kind {
        "critical" => "d-link--critical",
        "warning" => "d-link--warning",
        "positive" => "d-link--positive",
        "info" => "d-link--info",
        "muted" => "d-link--muted",
        "mention" => "d-link--mention",
        _ => "",
    }
}

fn warn_link_kind(kind: &str) {
    if matches!(
        kind,
        "" | "default" | "critical" | "warning" | "positive" | "info" | "muted" | "mention"
    ) {
        return;
    }
    leptos::logging::warn!(
        "DtButton link_kind `{kind}` is not default, critical, warning, positive, info, muted, or mention."
    );
}

fn known_icon_position(position: &str) -> bool {
    matches!(
        position,
        "start" | "end" | "blockStart" | "blockEnd" | "left" | "right" | "top" | "bottom"
    )
}

/// `d-link--no-underline` when a link button turns the underline off.
fn link_underline_class(link: bool, underline: bool) -> &'static str {
    if link && !underline {
        "d-link--no-underline"
    } else {
        ""
    }
}

fn show_legacy_icon(has_icon: bool, has_new: bool, link: bool) -> bool {
    has_icon && !has_new && !link
}

fn button_icon_only(has_new: bool, legacy: bool, has_label: bool) -> bool {
    (has_new || legacy) && !has_label
}

/// Block-start or block-end icons stack when a label is present.
/// Legacy `top` and `bottom` do the same when no new icon slot is set.
fn button_vertical(icon_only: bool, has_block: bool, has_new: bool, position: &str) -> bool {
    if icon_only {
        return false;
    }
    if has_block {
        return true;
    }
    !has_new && matches!(position, "top" | "bottom")
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

fn icon_span_class(kind: &str, position: &str, extra: &str) -> String {
    if kind == "unstyled" {
        join_class(["base-button__icon", extra])
    } else {
        join_class([
            "base-button__icon",
            "d-btn__icon",
            icon_position_class(position),
            extra,
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

/// Dialtone Vue `DtButton` (`dialtone/v10.7.0`).
///
/// `kind` is `default`, `muted`, `critical`, `positive`, `inverted`, or
/// `unstyled`. `critical` is `d-btn--critical`. `danger` warns and omits the
/// kind class. `importance` is `clear`, `outlined`, or `primary` (default).
/// Size is `100`–`500` (default `300`) or the t-shirt aliases `xs`–`xl`.
/// `300` and `md` add no size class. `link` renders link classes instead of
/// button classes. Link kinds are `default`, `critical`, `warning`, `positive`,
/// `muted`, `info`, and `mention`. `link_underline` (`linkUnderline`) defaults
/// to true. Deprecated `underline` wins when it is set. False adds
/// `d-link--no-underline`. A non-empty `href` renders an `<a>`. Vue Router `to`
/// is not ported. `start_icon`, `end_icon`, `block_start_icon`, and
/// `block_end_icon` hide the legacy `icon` slot. A block icon with a label adds
/// `d-btn--vertical`. Icon-only and vertical are exclusive. `leading` and
/// `trailing` use `d-btn__leading` and `d-btn__trailing`. The loading label is
/// `DIALTONE_LOADING` in en-US, "loading". Size a slotted icon with
/// [`button_icon_size`]. A Leptos slot does not receive the Vue `icon-size` prop.
#[component]
pub fn DtButton(
    #[prop(optional)] circle: bool,
    /// Deprecated icon slot position (`iconPosition`). `start` (default), `end`,
    /// `blockStart`, `blockEnd`, `left`, `right`, `top`, or `bottom`.
    #[prop(optional, default = "start")]
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
    /// Inverted link color (`linkInverted`). Deprecated. Prefer `v-dt-mode`.
    #[prop(optional)]
    link_inverted: bool,
    /// Underline on a link button (`linkUnderline`). Default true.
    #[prop(optional, default = true)]
    link_underline: bool,
    /// Deprecated underline (`underline`). When set, it wins over `link_underline`.
    #[prop(optional)]
    underline: Option<bool>,
    /// `disabled`. A signal updates the attribute after the first render.
    #[prop(optional, into)]
    disabled: MaybeProp<bool>,
    /// `button` (default), `submit`, or `reset` (`type`).
    #[prop(optional, default = "button")]
    type_: &'static str,
    /// CSS width (`width`). Empty omits the style.
    #[prop(optional, into)]
    width: String,
    /// `100`–`500` (default `300`), or `xs`, `sm`, `md`, `lg`, `xl`.
    #[prop(optional, default = "300")]
    size: &'static str,
    /// Extra class on the label (`labelClass`).
    #[prop(optional, into)]
    label_class: String,
    /// Extra class on the legacy icon (`iconClass`).
    #[prop(optional, into)]
    icon_class: String,
    /// Extra class on the start icon (`startIconClass`).
    #[prop(optional, into)]
    start_icon_class: String,
    /// Extra class on the end icon (`endIconClass`).
    #[prop(optional, into)]
    end_icon_class: String,
    /// Extra class on the block-start icon (`blockStartIconClass`).
    #[prop(optional, into)]
    block_start_icon_class: String,
    /// Extra class on the block-end icon (`blockEndIconClass`).
    #[prop(optional, into)]
    block_end_icon_class: String,
    /// Extra class on the leading wrapper (`leadingClass`).
    #[prop(optional, into)]
    leading_class: String,
    /// Extra class on the trailing wrapper (`trailingClass`).
    #[prop(optional, into)]
    trailing_class: String,
    #[prop(optional)] loading: bool,
    /// `default`, `muted`, `critical`, `positive`, `inverted`, or `unstyled`.
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
    /// Accessible name that follows a signal. A non-empty value wins over `aria_label`.
    #[prop(optional, into)]
    aria_label_signal: MaybeProp<String>,
    /// `aria-keyshortcuts`. Empty omits the attribute.
    #[prop(optional, into)]
    aria_keyshortcuts: String,
    /// `title`. Empty omits the attribute.
    #[prop(optional, into)]
    title: String,
    /// Extra classes appended to the button.
    #[prop(optional, into)]
    class: String,
    /// Classes that follow a signal. Appended after `class`.
    #[prop(optional, into)]
    class_signal: MaybeProp<String>,
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
    /// `aria-expanded`. Unset omits the attribute. A signal updates it.
    #[prop(optional, into)]
    aria_expanded: MaybeProp<bool>,
    /// `aria-controls`. Empty omits the attribute.
    #[prop(optional, into)]
    aria_controls: String,
    /// `aria-haspopup`. Empty omits the attribute.
    #[prop(optional, default = "")]
    aria_haspopup: &'static str,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
    #[prop(optional, into)] on_focus_in: Option<Callback<()>>,
    #[prop(optional, into)] on_focus_out: Option<Callback<()>>,
    /// Deprecated icon slot (`icon`). Hidden when `link` is set, or when a new
    /// icon slot is set.
    #[prop(optional)]
    icon: Option<ViewFn>,
    /// Icon above the label (`blockStartIcon`). Pass `None` or omit it.
    #[prop(optional_no_strip)]
    block_start_icon: Option<ViewFn>,
    /// Icon at the start (`startIcon`). Pass `None` or omit it.
    #[prop(optional_no_strip)]
    start_icon: Option<ViewFn>,
    /// Icon at the end (`endIcon`). Pass `None` or omit it.
    #[prop(optional_no_strip)]
    end_icon: Option<ViewFn>,
    /// Icon below the label (`blockEndIcon`). Pass `None` or omit it.
    #[prop(optional_no_strip)]
    block_end_icon: Option<ViewFn>,
    /// Content before the label (`leading`). Pass `None` or omit it.
    #[prop(optional_no_strip)]
    leading: Option<ViewFn>,
    /// Content after the label (`trailing`). Pass `None` or omit it.
    #[prop(optional_no_strip)]
    trailing: Option<ViewFn>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    warn_button_combination(circle, kind, importance);
    warn_button_kind(kind);
    if circle && link {
        leptos::logging::warn!("You cannot enable circle and link at the same time");
    }
    if link {
        warn_link_kind(link_kind);
    }
    if !known_icon_position(icon_position) {
        leptos::logging::warn!(
            "DtButton icon_position `{icon_position}` is not start, end, blockStart, blockEnd, left, right, top, or bottom."
        );
    }
    let has_block = block_start_icon.is_some() || block_end_icon.is_some();
    let has_new_icons =
        has_block || start_icon.is_some() || end_icon.is_some();
    let legacy = show_legacy_icon(icon.is_some(), has_new_icons, link);
    let show_label = children.is_some();
    let icon_only = button_icon_only(has_new_icons, legacy, show_label);
    let vertical = button_vertical(icon_only, has_block, has_new_icons, icon_position);
    let resolved_underline = underline.unwrap_or(link_underline);
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
            link_underline_class(link, resolved_underline),
            extra_class.get_value().as_str(),
            class_signal.get().unwrap_or_default().as_str(),
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
    let shortcut_attr = (!aria_keyshortcuts.is_empty()).then_some(aria_keyshortcuts);
    let aria_selected_attr = aria_selected.map(|selected| if selected { "true" } else { "false" });
    let expanded_value = move || {
        aria_expanded
            .get()
            .map(|expanded| if expanded { "true" } else { "false" })
    };
    let legacy_icon_class = icon_span_class(kind, icon_position, &icon_class);
    let start_class = icon_span_class(kind, "left", &start_icon_class);
    let end_class = icon_span_class(kind, "right", &end_icon_class);
    let block_start_class = icon_span_class(kind, "top", &block_start_icon_class);
    let block_end_class = icon_span_class(kind, "bottom", &block_end_icon_class);
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
    let static_label = StoredValue::new(aria_label);
    let read_label = move || -> Option<String> {
        if loading {
            return Some(LOADING_LABEL.to_string());
        }
        if let Some(text) = aria_label_signal.get() {
            if !text.is_empty() {
                return Some(text);
            }
        }
        let text = static_label.get_value();
        (!text.is_empty()).then_some(text)
    };
    let focused = RwSignal::new(false);
    let is_anchor = !href.is_empty();

    let content = view! {
        {show_loader.then(|| view! {
            <DtLoader class="d-btn__loader" size=loader_size aria_hidden=true />
        })}
        {leading.map(|slot| view! {
            <span class=leading_class_name.clone()>{slot.run()}</span>
        })}
        {block_start_icon.map(|slot| view! {
            <span data-qa="dt-button-block-start-icon" class=block_start_class.clone()>
                {slot.run()}
            </span>
        })}
        {start_icon.map(|slot| view! {
            <span data-qa="dt-button-start-icon" class=start_class.clone()>
                {slot.run()}
            </span>
        })}
        {icon.filter(|_| legacy).map(|slot| view! {
            <span data-qa="dt-button-icon" class=legacy_icon_class>
                {slot.run()}
            </span>
        })}
        {children.map(|children| view! {
            <span data-qa="dt-button-label" class=label_class_name>
                {children()}
            </span>
        })}
        {end_icon.map(|slot| view! {
            <span data-qa="dt-button-end-icon" class=end_class.clone()>
                {slot.run()}
            </span>
        })}
        {block_end_icon.map(|slot| view! {
            <span data-qa="dt-button-block-end-icon" class=block_end_class.clone()>
                {slot.run()}
            </span>
        })}
        {trailing.map(|slot| view! {
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
                aria-expanded=expanded_value
                aria-controls=aria_controls_attr
                aria-haspopup=aria_haspopup_attr
                aria-keyshortcuts=shortcut_attr.clone()
                aria-checked=checked_value
                title=title_attr.clone()
                href=move || (!is_disabled()).then_some(href.clone())
                target=(!target.is_empty()).then_some(target)
                rel=(!rel.is_empty()).then_some(rel)
                aria-label=read_label
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
                aria-expanded=expanded_value
                aria-controls=aria_controls_attr
                aria-haspopup=aria_haspopup_attr
                aria-keyshortcuts=shortcut_attr
                aria-checked=checked_value
                title=title_attr
                aria-disabled=move || aria_disabled.get().unwrap_or(false).then_some("true")
                aria-label=read_label
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
            button_classes(
                false, "default", false, "positive", "outlined", "sm", true, true, true, false, true
            ),
            "base-button__button d-btn d-btn--outlined d-btn--positive d-btn--sm d-btn--circle d-btn--loading d-btn--icon-only d-btn--active"
        );
        assert_eq!(button_kind_class("danger"), "");
        assert_eq!(button_kind_class("critical"), "d-btn--critical");
        assert_eq!(
            button_classes(
                false, "default", false, "unstyled", "primary", "lg", true, true, true, true, true
            ),
            "base-button__button d-btn--unstyled"
        );
        assert_eq!(
            button_classes(
                true, "critical", true, "default", "primary", "lg", true, false, false, true, false
            ),
            "base-button__button d-link d-link--inverted-critical d-btn--lg"
        );
        assert_eq!(link_kind_class("positive", false), "d-link--positive");
        assert_eq!(link_kind_class("info", false), "d-link--info");
        assert_eq!(link_kind_class("danger", true), "");
        assert_eq!(link_underline_class(true, false), "d-link--no-underline");
        assert_eq!(link_underline_class(true, true), "");
        assert_eq!(link_underline_class(false, false), "");
        assert!(button_vertical(false, true, true, "start"));
        assert!(!button_vertical(true, true, true, "start"));
        assert!(button_vertical(false, false, false, "top"));
        assert!(!button_vertical(false, false, false, "blockStart"));
        assert!(show_legacy_icon(true, false, false));
        assert!(!show_legacy_icon(true, true, false));
        assert!(!show_legacy_icon(true, false, true));
        assert!(button_icon_only(true, false, false));
        assert!(!button_icon_only(true, false, true));
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
