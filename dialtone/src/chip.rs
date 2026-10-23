use leptos::prelude::*;
use web_sys::KeyboardEvent;

use super::button::DtButton;
use super::icon::DtIconClose;
use super::unique_id;

/// `DIALTONE_CLOSE_BUTTON` in English.
const CLOSE_LABEL: &str = "Click to close";

/// `CHIP_ICON_SIZES`. Every token is `200`.
const CLOSE_ICON_SIZE: &str = "200";

const VALID_SIZES: &str = "100, 200, 300, xs, sm, md";

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Label modifier, then close-button modifier. `CHIP_SIZE_MODIFIERS` and
/// `CHIP_CLOSE_BUTTON_SIZE_MODIFIERS`. An unknown token warns and both are empty.
fn size_modifiers(size: &str) -> (&'static str, &'static str) {
    match size {
        "100" | "xs" => ("d-chip__label--xs", "d-chip__close--xs"),
        "200" | "sm" => ("d-chip__label--sm", "d-chip__close--sm"),
        "300" | "md" => ("", ""),
        _ => {
            leptos::logging::warn!(
                "[DtChip] Invalid prop value: \"{size}\". Valid values: {VALID_SIZES}"
            );
            ("", "")
        }
    }
}

/// Vue `chipClasses`. `grouped_chip` is `$attrs['grouped-chip']`.
fn label_classes(
    grouped_chip: bool,
    size_class: &str,
    label_class: &str,
    disabled: bool,
) -> String {
    join_class([
        if grouped_chip {
            "d-chip"
        } else {
            "d-chip__label"
        },
        size_class,
        label_class,
        if disabled { "d-chip--disabled" } else { "" },
    ])
}

/// Vue `chipCloseButtonClasses`.
fn close_classes(size_class: &str, disabled: bool) -> String {
    join_class([
        "d-chip__close",
        size_class,
        if disabled {
            "d-chip__close--disabled"
        } else {
            ""
        },
    ])
}

/// Delete emits `close` when the close button is shown, and does not emit `keydown`.
fn key_action(code: &str, show_close: bool) -> &'static str {
    if code.eq_ignore_ascii_case("delete") {
        if show_close {
            "close"
        } else {
            "none"
        }
    } else {
        "keydown"
    }
}

fn handle_keydown(
    event: &KeyboardEvent,
    show_close: bool,
    on_close: Option<Callback<()>>,
    on_keydown: Option<Callback<()>>,
) {
    match key_action(&event.code(), show_close) {
        "close" => {
            if let Some(on_close) = on_close {
                on_close.run(());
            }
        }
        "keydown" => {
            if let Some(on_keydown) = on_keydown {
                on_keydown.run(());
            }
        }
        _ => {}
    }
}

#[allow(clippy::too_many_arguments)]
fn chip_shell(
    interactive: bool,
    id: String,
    class_name: String,
    aria_label: Option<String>,
    labelled_by: Option<String>,
    disabled: bool,
    show_close: bool,
    on_click: Option<Callback<()>>,
    on_keydown: Option<Callback<()>>,
    on_keyup: Option<Callback<()>>,
    on_close: Option<Callback<()>>,
    face: impl IntoView + 'static,
) -> AnyView {
    let aria_disabled = disabled.then(|| "true".to_string());
    let tab_index = disabled.then(|| "-1".to_string());
    if interactive {
        view! {
            <button
                id=id
                type="button"
                class=class_name
                aria-label=aria_label
                aria-labelledby=labelled_by
                aria-disabled=aria_disabled
                tabindex=tab_index
                data-qa="dt-chip"
                on:click=move |_| {
                    if let Some(on_click) = on_click {
                        on_click.run(());
                    }
                }
                on:keydown=move |event: KeyboardEvent| {
                    handle_keydown(&event, show_close, on_close, on_keydown);
                }
                on:keyup=move |_| {
                    if let Some(on_keyup) = on_keyup {
                        on_keyup.run(());
                    }
                }
            >
                {face}
            </button>
        }
        .into_any()
    } else {
        view! {
            <span
                id=id
                class=class_name
                aria-label=aria_label
                aria-labelledby=labelled_by
                aria-disabled=aria_disabled
                tabindex=tab_index
                data-qa="dt-chip"
                on:keydown=move |event: KeyboardEvent| {
                    handle_keydown(&event, show_close, on_close, on_keydown);
                }
                on:keyup=move |_| {
                    if let Some(on_keyup) = on_keyup {
                        on_keyup.run(());
                    }
                }
            >
                {face}
            </span>
        }
        .into_any()
    }
}

/// Dialtone Vue `DtChip`.
///
/// `size` is `100`, `200`, or `300` (default). `xs`, `sm`, and `md` are the
/// deprecated aliases. `show_close` is `showClose` and defaults to true.
/// `on_close` is `close`. `on_click` is `click` and runs only when `interactive`.
/// `content_class` is `contentClass`. `label_class` is `labelClass`.
/// `icon_class` is `iconClass`. `avatar_class` is `avatarClass`.
/// `grouped_chip` is the fallthrough attribute `grouped-chip`.
/// The icon slot wins over the avatar slot.
#[component]
pub fn DtChip(
    /// `100`, `200`, `300`, or the deprecated `xs`, `sm`, `md`. Default `300`.
    #[prop(optional, default = "300")]
    size: &'static str,
    /// Shows the close button (`showClose`). Default true.
    #[prop(optional, default = true)]
    show_close: bool,
    /// Renders the label as a button.
    #[prop(optional)]
    interactive: bool,
    #[prop(optional)] disabled: bool,
    /// Element id. Empty generates one.
    #[prop(optional, into)]
    id: String,
    /// Accessible name (`ariaLabel`). Empty uses `aria-labelledby` on the text.
    #[prop(optional, into)]
    aria_label: String,
    /// Extra class on the root `span.d-chip` (Vue `$attrs.class`).
    #[prop(optional, into)]
    class: String,
    /// Extra class on the label (`labelClass`).
    #[prop(optional, into)]
    label_class: String,
    /// Extra class on the text (`contentClass`).
    #[prop(optional, into)]
    content_class: String,
    /// Extra class on the icon wrapper (`iconClass`).
    #[prop(optional, into)]
    icon_class: String,
    /// Extra class on the avatar wrapper (`avatarClass`).
    #[prop(optional, into)]
    avatar_class: String,
    /// `$attrs['grouped-chip']`. The label class is `d-chip` instead of `d-chip__label`.
    #[prop(optional)]
    grouped_chip: bool,
    #[prop(optional, into)] on_close: Option<Callback<()>>,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
    #[prop(optional, into)] on_keydown: Option<Callback<()>>,
    #[prop(optional, into)] on_keyup: Option<Callback<()>>,
    /// Icon slot. Hides the avatar slot.
    #[prop(optional)]
    icon: Option<ViewFn>,
    /// Avatar slot. Hidden when `icon` is set.
    #[prop(optional)]
    avatar: Option<ViewFn>,
    children: Children,
) -> impl IntoView {
    let id = if id.is_empty() {
        unique_id("dt-chip")
    } else {
        id
    };
    let content_id = format!("{id}-content");
    let labelled_by = aria_label.is_empty().then(|| content_id.clone());
    let aria_label = (!aria_label.is_empty()).then_some(aria_label);
    let (label_size, close_size) = size_modifiers(size);
    let root_class = join_class(["d-chip", class.as_str()]);
    let label_class_name = label_classes(grouped_chip, label_size, &label_class, disabled);
    let close_class = close_classes(close_size, disabled);
    let text_class = join_class(["d-chip__text", content_class.as_str()]);
    let icon_class_name = join_class(["d-chip__icon", icon_class.as_str()]);
    let avatar_class_name = join_class(["d-chip__avatar", avatar_class.as_str()]);
    let show_avatar = icon.is_none();
    let face = view! {
        {icon.map(|slot| {
            view! {
                <span class=icon_class_name data-qa="dt-chip-icon">{slot.run()}</span>
            }
        })}
        {show_avatar.then(|| avatar.map(|slot| {
            view! {
                <span class=avatar_class_name data-qa="dt-chip-avatar">{slot.run()}</span>
            }
        }))}
        <span id=content_id class=text_class data-qa="dt-chip-label">
            {children()}
        </span>
    };
    let shell = chip_shell(
        interactive,
        id,
        label_class_name,
        aria_label,
        labelled_by,
        disabled,
        show_close,
        on_click,
        on_keydown,
        on_keyup,
        on_close,
        face,
    );
    view! {
        <span class=root_class>
            {shell}
            {show_close.then(|| {
                view! {
                    <DtButton
                        class=close_class
                        data_qa="dt-chip-close"
                        aria_label=CLOSE_LABEL
                        title=CLOSE_LABEL
                        aria_disabled=disabled.then_some(true)
                        tab_index=disabled.then_some(-1)
                        on_click=move || {
                            if let Some(on_close) = on_close {
                                on_close.run(());
                            }
                        }
                        start_icon=Some(ViewFn::from(|| view! { <DtIconClose size=CLOSE_ICON_SIZE /> }))
                    />
                }
            })}
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_size_omits_the_modifier() {
        let (label, close) = size_modifiers("300");
        assert_eq!(label_classes(false, label, "", false), "d-chip__label");
        assert_eq!(close_classes(close, false), "d-chip__close");
        let (label, close) = size_modifiers("md");
        assert_eq!((label, close), ("", ""));
    }

    #[test]
    fn numeric_and_tshirt_sizes_share_modifiers() {
        for (size, label_mod, close_mod) in [
            ("100", "d-chip__label--xs", "d-chip__close--xs"),
            ("xs", "d-chip__label--xs", "d-chip__close--xs"),
            ("200", "d-chip__label--sm", "d-chip__close--sm"),
            ("sm", "d-chip__label--sm", "d-chip__close--sm"),
        ] {
            assert_eq!(size_modifiers(size), (label_mod, close_mod));
        }
    }

    #[test]
    fn label_class_order_matches_vue() {
        let (label, close) = size_modifiers("100");
        assert_eq!(
            label_classes(false, label, "extra", true),
            "d-chip__label d-chip__label--xs extra d-chip--disabled"
        );
        assert_eq!(
            close_classes(close, true),
            "d-chip__close d-chip__close--xs d-chip__close--disabled"
        );
    }

    #[test]
    fn grouped_chip_uses_the_root_class_on_the_label() {
        let (label, _) = size_modifiers("200");
        assert_eq!(
            label_classes(true, label, "", false),
            "d-chip d-chip__label--sm"
        );
    }

    #[test]
    fn unknown_size_warns_and_omits_the_modifier() {
        assert_eq!(size_modifiers("lg"), ("", ""));
    }

    #[test]
    fn delete_closes_only_while_the_close_button_is_shown() {
        assert_eq!(key_action("DELETE", true), "close");
        assert_eq!(key_action("delete", false), "none");
        assert_eq!(key_action("Enter", true), "keydown");
    }
}
