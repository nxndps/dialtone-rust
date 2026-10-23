use std::time::Duration;

use leptos::prelude::*;
use web_sys::KeyboardEvent;

use super::button::{button_classes, DtButton};
use super::icon::{
    DtIconAlertCircle, DtIconAlertTriangle, DtIconBell, DtIconCheckCircle, DtIconClose, DtIconInfo,
    DtIconSparkle,
};
use super::text::{text_classes, TextStyle};
use super::unique_id;

/// Close button label. Dialtone `DIALTONE_CLOSE_BUTTON` in English.
const CLOSE_LABEL: &str = "Close";

/// `TOAST_MIN_DURATION`. A shorter `duration` warns and does not auto-close.
const TOAST_MIN_DURATION: u32 = 6000;

/// Notice icon size in `DtNoticeIcon` (`size="400"`).
const NOTICE_ICON_SIZE: &str = "400";

/// Alternate toast icon size (`size="200"`).
const ALTERNATE_ICON_SIZE: &str = "200";

/// Visible close icon size. Vue sets this explicitly, not from the button size.
const CLOSE_ICON_SIZE: &str = "200";

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn layout_name(layout: &str) -> Option<&'static str> {
    match layout {
        "alternate" => Some("alternate"),
        "default" | "" => Some("default"),
        _ => None,
    }
}

fn role_name(role: &str) -> Option<&'static str> {
    match role {
        "alert" => Some("alert"),
        "status" | "" => Some("status"),
        _ => None,
    }
}

fn kind_known(layout: &str, kind: &str) -> bool {
    let kind = if kind.is_empty() { "base" } else { kind };
    if layout == "alternate" {
        matches!(
            kind,
            "base" | "critical" | "info" | "positive" | "warning" | "gradient"
        )
    } else {
        matches!(kind, "base" | "critical" | "info" | "positive" | "warning")
    }
}

fn default_kind_class(kind: &str) -> &'static str {
    match kind {
        "critical" => "d-toast--critical",
        "info" => "d-toast--info",
        "positive" => "d-toast--positive",
        "warning" => "d-toast--warning",
        "base" => "d-toast--base",
        _ => "",
    }
}

fn alternate_kind_class(kind: &str) -> &'static str {
    match kind {
        "critical" => "d-toast-alternate--critical",
        "info" => "d-toast-alternate--info",
        "positive" => "d-toast-alternate--positive",
        "warning" => "d-toast-alternate--warning",
        "gradient" => "d-toast-alternate--gradient",
        _ => "",
    }
}

/// Root class list. Default order is `d-toast`, kind, extra class, `d-toast--important`.
/// Alternate order is `d-toast-alternate`, extra class, kind. `important` is default-layout only.
fn toast_class(layout: &str, kind: &str, extra: &str, important: bool) -> String {
    if layout == "alternate" {
        join_class(["d-toast-alternate", extra, alternate_kind_class(kind)])
    } else {
        join_class([
            "d-toast",
            default_kind_class(kind),
            extra,
            if important { "d-toast--important" } else { "" },
        ])
    }
}

fn notice_icon_class(icon_class: &str, has_title: bool) -> String {
    join_class([
        "d-notice__icon",
        icon_class,
        if has_title {
            "d-notice__icon--has-title"
        } else {
            ""
        },
    ])
}

fn alternate_icon_class(icon_class: &str) -> String {
    join_class(["d-toast-layout-alternate__icon", icon_class])
}

fn notice_content_class(content_class: &str) -> String {
    join_class(["d-notice__content", content_class])
}

fn notice_action_class(action_class: &str) -> String {
    join_class(["d-notice__actions", action_class])
}

fn text_style<'a>(kind: &'a str, size: &'a str, density: &'a str, wrap: &'a str) -> TextStyle<'a> {
    TextStyle {
        kind,
        variant: "",
        size,
        family: "",
        italic: false,
        tone: "",
        align: "",
        truncate: false,
        max_lines: 0,
        numeric: false,
        wrap,
        text_box_trim: "",
        strength: "",
        density,
    }
}

fn notice_title_class(header_class: &str) -> String {
    let text = text_classes(&text_style("headline", "300", "200", "pretty"));
    join_class([text.as_str(), "d-notice__title", header_class])
}

fn notice_message_class() -> String {
    let text = text_classes(&text_style("body", "200", "", "pretty"));
    join_class([text.as_str(), "d-notice__message"])
}

fn sr_only_close_class() -> String {
    let button = button_classes(
        false, "default", false, "default", "primary", "md", false, false, true, false, false,
    );
    join_class([button.as_str(), "d-vi-visible-sr"])
}

/// `duration` that starts the auto-close timer. Below 6000 ms returns `None`.
fn valid_duration(duration: Option<u32>) -> Option<u32> {
    duration.filter(|ms| *ms >= TOAST_MIN_DURATION)
}

fn has_default_icon(alternate: bool, kind: &str) -> bool {
    if alternate {
        matches!(
            kind,
            "info" | "positive" | "critical" | "warning" | "base" | "gradient"
        )
    } else {
        matches!(kind, "info" | "positive" | "warning" | "critical" | "base")
    }
}

fn default_icon(alternate: bool, kind: &str) -> Option<AnyView> {
    let size = if alternate {
        ALTERNATE_ICON_SIZE
    } else {
        NOTICE_ICON_SIZE
    };
    Some(match (alternate, kind) {
        (false, "info") => view! { <DtIconInfo size=size /> }.into_any(),
        (false, "positive") => view! { <DtIconCheckCircle size=size /> }.into_any(),
        (false, "warning") => view! { <DtIconAlertTriangle size=size /> }.into_any(),
        (false, "critical") => view! { <DtIconAlertCircle size=size /> }.into_any(),
        (false, "base") => view! { <DtIconBell size=size /> }.into_any(),
        (true, "info" | "positive" | "critical") => view! { <DtIconInfo size=size /> }.into_any(),
        (true, "warning") => view! { <DtIconAlertTriangle size=size /> }.into_any(),
        (true, "base") => view! { <DtIconBell size=size /> }.into_any(),
        (true, "gradient") => view! { <DtIconSparkle size=size /> }.into_any(),
        _ => return None,
    })
}

#[cfg(target_arch = "wasm32")]
fn active_html_element() -> Option<web_sys::HtmlElement> {
    use wasm_bindgen::JsCast;
    web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.active_element())
        .and_then(|element| element.dyn_into().ok())
}

fn notice_actions(
    action_class_name: String,
    show_action: bool,
    action: Option<ViewFn>,
    show_close: bool,
    close_size: &'static str,
    dismiss: Callback<()>,
) -> AnyView {
    let sr_class = sr_only_close_class();
    view! {
        <div class=action_class_name data-qa="notice-content-actions">
            {if show_action {
                if let Some(slot) = action {
                    slot.run().into_any()
                } else {
                    ().into_any()
                }
            } else {
                ().into_any()
            }}
            {if show_close {
                view! {
                    <DtButton
                        importance="clear"
                        kind="muted"
                        size=close_size
                        data_qa="dt-notice-action-close-button"
                        aria_label=CLOSE_LABEL
                        title=CLOSE_LABEL
                        icon=ViewFn::from(|| view! { <DtIconClose size=CLOSE_ICON_SIZE /> })
                        on_click=dismiss
                    />
                }
                .into_any()
            } else {
                view! {
                    <button
                        type="button"
                        id="sr-only-close-button"
                        class=sr_class
                        data-qa="dt-sr-only-close-button"
                        aria-label=CLOSE_LABEL
                        title=CLOSE_LABEL
                        on:click=move |_| dismiss.run(())
                        on:keydown=move |event: KeyboardEvent| {
                            if event.key() == " " || event.code() == "Space" {
                                event.prevent_default();
                                event.stop_propagation();
                                dismiss.run(());
                            }
                        }
                    >
                        <span data-qa="dt-button-icon" class="base-button__icon d-btn__icon d-btn__icon--left">
                            <DtIconClose />
                        </span>
                    </button>
                }
                .into_any()
            }}
        </div>
    }
    .into_any()
}

fn notice_title(
    header_id: String,
    title_class: String,
    header_text: String,
    header: Option<ViewFn>,
) -> AnyView {
    view! {
        <p id=header_id class=title_class data-qa="notice-content-title">
            {if let Some(header) = header {
                header.run().into_any()
            } else {
                header_text.into_any()
            }}
        </p>
    }
    .into_any()
}

fn notice_message(
    content_id: String,
    message_class: String,
    message: String,
    children: Option<ChildrenFn>,
) -> AnyView {
    view! {
        <p id=content_id class=message_class data-qa="notice-content-message">
            {if let Some(children) = children {
                children().into_any()
            } else {
                message.into_any()
            }}
        </p>
    }
    .into_any()
}

/// Dialtone Vue `DtToast` (`dialtone/v10.7.0`).
///
/// `open` is controlled. `on_open` is Vue `update:open`. `on_close` is `close`.
/// `kind` is `base`, `critical`, `info`, `positive`, or `warning`. `gradient` is
/// valid when `layout` is `alternate`. `duration` below 6000 ms warns and does
/// not auto-close. An empty duration stays until close. The default slot
/// replaces `message`. The alternate layout does not render the action slot.
/// `d-toast-wrapper` is not part of this component.
#[component]
pub fn DtToast(
    /// Header element id (`headerId`). Empty generates one.
    #[prop(optional, into)]
    header_id: String,
    /// Message element id (`contentId`). Empty generates one.
    #[prop(optional, into)]
    content_id: String,
    /// Header text (`headerText`). Empty omits the title unless `header` is set.
    #[prop(optional, into)]
    header_text: String,
    /// Message text (`message`). The default slot replaces it.
    #[prop(optional, into)]
    message: String,
    /// `status` (default) or `alert` (`role`).
    #[prop(optional, default = "status")]
    role: &'static str,
    /// `base` (default), `critical`, `info`, `positive`, `warning`, or `gradient`.
    #[prop(optional, default = "base")]
    kind: &'static str,
    /// Visually dominant default layout (`important`).
    #[prop(optional)]
    important: bool,
    /// Controlled visibility (`open`).
    #[prop(into)]
    open: Signal<bool>,
    /// Vue `update:open`.
    #[prop(optional, into)]
    on_open: Option<Callback<bool>>,
    /// Vue `close`. Also emits `update:open` false.
    #[prop(optional, into)]
    on_close: Option<Callback<()>>,
    /// Shows the close button (`showClose`).
    #[prop(optional, default = true)]
    show_close: bool,
    /// Shows the icon (`showIcon`).
    #[prop(optional, default = true)]
    show_icon: bool,
    /// Shows the action slot (`showAction`). Ignored on the alternate layout.
    #[prop(optional, default = true)]
    show_action: bool,
    /// Auto-close delay in ms (`duration`). Empty stays open. Minimum 6000.
    #[prop(optional)]
    duration: Option<u32>,
    /// `default` or `alternate` (`layout`).
    #[prop(optional, default = "default")]
    layout: &'static str,
    /// Extra class on the icon wrapper (`iconClass`).
    #[prop(optional, into)]
    icon_class: String,
    /// Extra class on the title (`headerClass`).
    #[prop(optional, into)]
    header_class: String,
    /// Extra class on `.d-notice__content` (`contentClass`).
    #[prop(optional, into)]
    content_class: String,
    /// Extra class on `.d-notice__actions` (`actionClass`).
    #[prop(optional, into)]
    action_class: String,
    /// Extra class on the toast root. Vue `$attrs.class`.
    #[prop(optional, into)]
    class: String,
    /// Custom icon slot (`icon`).
    #[prop(optional)]
    icon: Option<ViewFn>,
    /// Header slot (`header`). Replaces `headerText`.
    #[prop(optional)]
    header: Option<ViewFn>,
    /// Action slot (`action`). Omitted on the alternate layout.
    #[prop(optional)]
    action: Option<ViewFn>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let layout = match layout_name(layout) {
        Some(layout) => layout,
        None => {
            leptos::logging::warn!("[DtToast] Unknown layout \"{layout}\". Using \"default\".");
            "default"
        }
    };
    let kind = if kind.is_empty() { "base" } else { kind };
    if !kind_known(layout, kind) {
        leptos::logging::warn!("[DtToast] Unknown kind \"{kind}\" for layout \"{layout}\".");
    }
    let role = match role_name(role) {
        Some(role) => role,
        None => {
            leptos::logging::warn!("[DtToast] Unknown role \"{role}\". Using \"status\".");
            "status"
        }
    };
    if let Some(ms) = duration {
        if ms < TOAST_MIN_DURATION {
            leptos::logging::warn!(
                "[DtToast] duration {ms} is below {TOAST_MIN_DURATION}. The toast stays open."
            );
        }
    }
    let alternate = layout == "alternate";
    let header_id = if header_id.is_empty() {
        unique_id("dt-toast-header")
    } else {
        header_id
    };
    let content_id = if content_id.is_empty() {
        unique_id("dt-toast-content")
    } else {
        content_id
    };
    let class_name = toast_class(layout, kind, &class, important);
    let has_title = !header_text.is_empty() || header.is_some();
    let icon_class_name = if alternate {
        alternate_icon_class(&icon_class)
    } else {
        notice_icon_class(&icon_class, has_title)
    };
    let content_class_name = notice_content_class(&content_class);
    let title_class_name = notice_title_class(&header_class);
    let message_class_name = notice_message_class();
    let action_class_name = notice_action_class(&action_class);
    let show_icon_node = show_icon && (icon.is_some() || has_default_icon(alternate, kind));
    let close_size = if alternate { "xs" } else { "sm" };
    let render_action = show_action && !alternate;

    let shown = RwSignal::new(open.get_untracked());
    let generation = StoredValue::new(0u32);
    on_cleanup(move || {
        generation.try_update_value(|value| *value = value.wrapping_add(1));
    });
    Effect::new(move |_| {
        let is_open = open.get();
        shown.set(is_open);
        generation.try_update_value(|value| *value = value.wrapping_add(1));
        let Some(ticket) = generation.try_get_value() else {
            return;
        };
        if !is_open {
            return;
        }
        let Some(ms) = valid_duration(duration) else {
            return;
        };
        leptos::leptos_dom::helpers::set_timeout(
            move || {
                if generation.try_get_value() != Some(ticket) {
                    return;
                }
                shown.set(false);
                if let Some(callback) = on_open {
                    callback.run(false);
                }
            },
            Duration::from_millis(u64::from(ms)),
        );
    });
    Effect::new(move |_| {
        if !(shown.get() && show_close) {
            return;
        }
        #[cfg(target_arch = "wasm32")]
        {
            let previous = active_html_element();
            on_cleanup(move || {
                if let Some(element) = previous {
                    let _ = element.focus();
                }
            });
        }
    });

    let dismiss = Callback::new(move |_| {
        generation.try_update_value(|value| *value = value.wrapping_add(1));
        shown.set(false);
        if let Some(callback) = on_close {
            callback.run(());
        }
        if let Some(callback) = on_open {
            callback.run(false);
        }
    });

    let icon_slot = icon;
    let header_slot = header;
    let action_slot = action;
    let body = children;
    view! {
        <Show when=move || shown.get()>
            {if alternate {
                let icon_slot = icon_slot.clone();
                let header_slot = header_slot.clone();
                let message = message.clone();
                let body = body.clone();
                view! {
                    <div class=class_name.clone() data-qa="dt-toast" aria-hidden="false">
                        <div class="d-toast-alternate__dialog">
                            <div class="d-toast-alternate__header">
                                {show_icon_node.then(|| {
                                    let icon_slot = icon_slot.clone();
                                    view! {
                                        <div class=icon_class_name.clone() aria-hidden="true">
                                            {if let Some(slot) = icon_slot {
                                                slot.run().into_any()
                                            } else {
                                                default_icon(true, kind).unwrap_or_else(|| ().into_any())
                                            }}
                                        </div>
                                    }
                                })}
                                <div
                                    class=content_class_name.clone()
                                    role=role
                                    data-qa="notice-content"
                                >
                                    {has_title.then(|| notice_title(
                                        header_id.clone(),
                                        title_class_name.clone(),
                                        header_text.clone(),
                                        header_slot.clone(),
                                    ))}
                                </div>
                                {notice_actions(
                                    action_class_name.clone(),
                                    false,
                                    None,
                                    show_close,
                                    close_size,
                                    dismiss,
                                )}
                            </div>
                            <div class="d-toast-alternate__content">
                                {if let Some(body) = body.clone() {
                                    body().into_any()
                                } else if !message.is_empty() {
                                    message.into_any()
                                } else {
                                    ().into_any()
                                }}
                            </div>
                        </div>
                    </div>
                }
                .into_any()
            } else {
                view! {
                    <div class=class_name.clone() data-qa="dt-toast" aria-hidden="false">
                        <div class="d-toast__dialog">
                            {show_icon_node.then(|| {
                                let icon_slot = icon_slot.clone();
                                view! {
                                    <div class=icon_class_name.clone() aria-hidden="true">
                                        {if let Some(slot) = icon_slot {
                                            slot.run().into_any()
                                        } else {
                                            default_icon(false, kind).unwrap_or_else(|| ().into_any())
                                        }}
                                    </div>
                                }
                            })}
                            <div
                                class=content_class_name.clone()
                                role=role
                                data-qa="notice-content"
                            >
                                {has_title.then(|| notice_title(
                                    header_id.clone(),
                                    title_class_name.clone(),
                                    header_text.clone(),
                                    header_slot.clone(),
                                ))}
                                {(body.is_some() || !message.is_empty()).then(|| notice_message(
                                    content_id.clone(),
                                    message_class_name.clone(),
                                    message.clone(),
                                    body.clone(),
                                ))}
                            </div>
                            {notice_actions(
                                action_class_name.clone(),
                                render_action,
                                action_slot.clone(),
                                show_close,
                                close_size,
                                dismiss,
                            )}
                        </div>
                    </div>
                }
                .into_any()
            }}
        </Show>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_classes_match_vue_order() {
        assert_eq!(
            toast_class("default", "base", "", false),
            "d-toast d-toast--base"
        );
        assert_eq!(
            toast_class("default", "critical", "", true),
            "d-toast d-toast--critical d-toast--important"
        );
        assert_eq!(
            toast_class("default", "info", "extra", true),
            "d-toast d-toast--info extra d-toast--important"
        );
        assert_eq!(
            toast_class("alternate", "base", "", true),
            "d-toast-alternate"
        );
        assert_eq!(
            toast_class("alternate", "gradient", "extra", false),
            "d-toast-alternate extra d-toast-alternate--gradient"
        );
        assert_eq!(
            toast_class("alternate", "positive", "", false),
            "d-toast-alternate d-toast-alternate--positive"
        );
        assert_eq!(toast_class("default", "gradient", "", false), "d-toast");
        assert_eq!(toast_class("default", "success", "", false), "d-toast");
    }

    #[test]
    fn notice_classes_join_in_vue_order() {
        assert_eq!(
            notice_icon_class("extra", true),
            "d-notice__icon extra d-notice__icon--has-title"
        );
        assert_eq!(notice_icon_class("", false), "d-notice__icon");
        assert_eq!(
            alternate_icon_class("extra"),
            "d-toast-layout-alternate__icon extra"
        );
        assert_eq!(notice_content_class("extra"), "d-notice__content extra");
        assert_eq!(notice_action_class("extra"), "d-notice__actions extra");
        assert_eq!(
            notice_title_class("extra"),
            "d-text d-text-headline--md d-text--wrap-pretty d-text--lh-200 d-notice__title extra"
        );
        assert_eq!(
            notice_message_class(),
            "d-text d-text-body--sm d-text--wrap-pretty d-notice__message"
        );
        assert_eq!(
            sr_only_close_class(),
            "base-button__button d-btn d-btn--primary d-btn--icon-only d-vi-visible-sr"
        );
    }

    #[test]
    fn duration_requires_the_vue_minimum() {
        assert_eq!(valid_duration(Some(6000)), Some(6000));
        assert_eq!(valid_duration(Some(5999)), None);
        assert_eq!(valid_duration(None), None);
    }

    #[test]
    fn kind_layout_and_role_tokens() {
        assert!(kind_known("default", ""));
        assert!(kind_known("default", "positive"));
        assert!(!kind_known("default", "gradient"));
        assert!(kind_known("alternate", "gradient"));
        assert!(!kind_known("alternate", "success"));
        assert_eq!(layout_name("alternate"), Some("alternate"));
        assert_eq!(layout_name(""), Some("default"));
        assert_eq!(layout_name("side"), None);
        assert_eq!(role_name("alert"), Some("alert"));
        assert_eq!(role_name(""), Some("status"));
        assert_eq!(role_name("dialog"), None);
    }
}
