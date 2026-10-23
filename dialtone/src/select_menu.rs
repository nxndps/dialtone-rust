use leptos::prelude::*;

use super::unique_id;
use super::validation::{validation_messages, validation_state, DtValidationMessage};

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn size_class(prefix: &str, size: &str) -> &'static str {
    match (prefix, size) {
        ("label", "xs") => "d-label--xs",
        ("label", "sm") => "d-label--sm",
        ("label", "lg") => "d-label--lg",
        ("label", "xl") => "d-label--xl",
        ("description", "lg") => "d-description--lg",
        ("description", "xl") => "d-description--xl",
        ("select", "xs") => "d-select--xs",
        ("select", "sm") => "d-select--sm",
        ("select", "lg") => "d-select--lg",
        ("select", "xl") => "d-select--xl",
        _ => "",
    }
}

fn known_size(size: &str) -> bool {
    matches!(size, "xs" | "sm" | "md" | "lg" | "xl")
}

/// Class string for the `.d-select` wrapper.
pub(crate) fn select_wrapper_class(size: &str, extra: &str, disabled: bool) -> String {
    if !known_size(size) {
        leptos::logging::warn!("DtSelectMenu size `{size}` is not xs, sm, md, lg, or xl");
    }
    join_class([
        "d-select",
        size_class("select", size),
        extra,
        if disabled { "d-select--disabled" } else { "" },
    ])
}

/// Class string for the native `<select>`.
pub(crate) fn select_input_class(state: Option<&str>) -> String {
    let state_class = match state {
        Some("error") => "d-select__input--error",
        Some("warning") => "d-select__input--warning",
        Some("success") => "d-select__input--success",
        _ => "",
    };
    join_class(["d-select__input", state_class])
}

/// One `DtSelectMenu` option. `value` is the Vue `value` (`string` or `number`,
/// stored as text because a native select reports strings). `label` is required.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DtSelectOption {
    pub value: String,
    pub label: String,
}

impl DtSelectOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

/// Dialtone Vue `DtSelectMenu` (`dialtone/v9.190.0`).
///
/// `value` / `on_change` are Vue `modelValue` plus the `input`, `update:modelValue`,
/// and `change` events, which all carry the same selected string. Pass options
/// with [`DtSelectOption`] or replace them with the default slot. `messages`
/// drive the input state the way the Vue messages mixin does.
#[component]
pub fn DtSelectMenu(
    /// Selected value (`modelValue`).
    #[prop(into)]
    value: Signal<String>,
    /// Selection changed (`input`, `update:modelValue`, and `change`).
    #[prop(into)]
    on_change: Callback<String>,
    /// Options used when the default slot is absent.
    #[prop(optional)]
    options: Vec<DtSelectOption>,
    /// `xs`, `sm`, `md` (default), `lg`, or `xl`.
    #[prop(optional, default = "md")]
    size: &'static str,
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] description: String,
    /// Extra class on the label (`labelClass`).
    #[prop(optional, into)]
    label_class: String,
    /// Extra class on the description (`descriptionClass`).
    #[prop(optional, into)]
    description_class: String,
    /// Extra class on `.d-select` (`selectClass`).
    #[prop(optional, into)]
    select_class: String,
    /// Extra class on each generated option (`optionClass`).
    #[prop(optional, into)]
    option_class: String,
    /// Extra class on the root (`rootClass`).
    #[prop(optional, into)]
    root_class: String,
    /// Extra class on the messages (`messagesClass`).
    #[prop(optional, into)]
    messages_class: String,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] name: String,
    /// Show `messages` (`showMessages`).
    #[prop(optional, default = true)]
    show_messages: bool,
    #[prop(optional)] messages: Vec<DtValidationMessage>,
    /// `aria-details` used when there is no description.
    #[prop(optional, into)]
    aria_details: String,
    /// Label slot. Replaces `label`.
    #[prop(optional)]
    label_slot: Option<ViewFn>,
    /// Description slot. Replaces `description`.
    #[prop(optional)]
    description_slot: Option<ViewFn>,
    /// Option slot. Replaces `options`.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    if options.is_empty() && children.is_none() {
        leptos::logging::warn!(
            "DtSelectMenu options are expected via the options prop or the default slot"
        );
    }
    let description_id = format!("{}-description", unique_id("select"));
    let has_label = label_slot.is_some() || !label.is_empty();
    let has_description = description_slot.is_some() || !description.is_empty();
    let label_class_name = join_class(["d-label", size_class("label", size), label_class.as_str()]);
    let description_class_name = join_class([
        "d-description",
        size_class("description", size),
        description_class.as_str(),
    ]);
    let state = validation_state(&messages);
    let wrapper_class = select_wrapper_class(size, &select_class, disabled);
    let input_class = select_input_class(state);
    let root_class = (!root_class.is_empty()).then_some(root_class);
    let option_class = (!option_class.is_empty()).then_some(option_class);
    let name_attr = (!name.is_empty()).then_some(name);
    let details = if has_description {
        Some(description_id.clone())
    } else if !aria_details.is_empty() {
        Some(aria_details)
    } else {
        None
    };
    let messages_view = validation_messages(
        messages,
        show_messages,
        &messages_class,
        "dt-select-messages",
    );
    let label_body = if let Some(slot) = label_slot {
        Some(slot.run().into_any())
    } else if !label.is_empty() {
        Some(label.into_any())
    } else {
        None
    };
    let description_body = if let Some(slot) = description_slot {
        Some(slot.run().into_any())
    } else if !description.is_empty() {
        Some(description.into_any())
    } else {
        None
    };
    let option_rows = options
        .into_iter()
        .filter_map(|option| {
            if option.label.is_empty() {
                leptos::logging::warn!("DtSelectMenu option `{}` is missing a label", option.value);
                return None;
            }
            let value = option.value;
            let label = option.label;
            let class_name = option_class.clone();
            Some(view! {
                <option value=value class=class_name>{label}</option>
            })
        })
        .collect_view();
    view! {
        <div class=root_class>
            <label>
                {has_label.then(|| view! {
                    <div class=label_class_name data-qa="dt-select-label" aria-details=details.clone()>
                        {label_body}
                    </div>
                })}
                {has_description.then(|| view! {
                    <div id=description_id class=description_class_name data-qa="dt-select-description">
                        {description_body}
                    </div>
                })}
                <div class=wrapper_class data-qa="dt-select-wrapper">
                    <select
                        class=input_class
                        data-qa="dt-select"
                        name=name_attr
                        prop:disabled=disabled
                        prop:value=move || value.get()
                        on:change=move |event| {
                            on_change.run(event_target_value(&event));
                        }
                    >
                        {children
                            .map(|children| children().into_any())
                            .unwrap_or_else(|| option_rows.into_any())}
                    </select>
                </div>
            </label>
            {messages_view}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapper_and_input_classes_follow_vue() {
        assert_eq!(select_wrapper_class("md", "", false), "d-select");
        assert_eq!(
            select_wrapper_class("xs", "custom", true),
            "d-select d-select--xs custom d-select--disabled"
        );
        assert_eq!(select_input_class(None), "d-select__input");
        assert_eq!(
            select_input_class(Some("error")),
            "d-select__input d-select__input--error"
        );
        assert_eq!(
            select_input_class(Some("success")),
            "d-select__input d-select__input--success"
        );
    }
}
