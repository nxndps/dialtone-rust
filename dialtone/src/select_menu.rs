use leptos::prelude::*;

use super::text::{text_classes, TextStyle};
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

fn text_style<'a>(
    kind: &'a str,
    size: &'a str,
    tone: &'a str,
    strength: &'a str,
    density: &'a str,
) -> TextStyle<'a> {
    TextStyle {
        kind,
        variant: "",
        size,
        family: "",
        italic: false,
        tone,
        align: "",
        truncate: false,
        max_lines: 0,
        numeric: false,
        wrap: "",
        text_box_trim: "",
        strength,
        density,
    }
}

fn known_size(size: &str) -> bool {
    matches!(
        size,
        "100" | "200" | "300" | "400" | "500" | "xs" | "sm" | "md" | "lg" | "xl"
    )
}

fn select_size_class(size: &str) -> &'static str {
    match size {
        "100" | "xs" => "d-select--xs",
        "200" | "sm" => "d-select--sm",
        "400" | "lg" => "d-select--lg",
        "500" | "xl" => "d-select--xl",
        _ => "",
    }
}

/// Label size derived from the select size. `500` caps at `400`. `xl` caps at `lg`.
pub(crate) fn select_label_size<'a>(size: &'a str, label_size: &'a str) -> &'a str {
    if !label_size.is_empty() {
        return label_size;
    }
    match size {
        "500" => "400",
        "xl" => "lg",
        other => other,
    }
}

fn description_size(size: &str) -> &'static str {
    match size {
        "100" | "200" | "xs" | "sm" => "xs",
        "500" | "xl" => "md",
        _ => "sm",
    }
}

fn description_density(size: &str) -> &'static str {
    match size {
        "500" | "xl" => "300",
        _ => "",
    }
}

/// Class string for the `.d-select` wrapper.
pub(crate) fn select_wrapper_class(size: &str, extra: &str, disabled: bool) -> String {
    if !known_size(size) {
        leptos::logging::warn!(
            "DtSelectMenu size `{size}` is not 100, 200, 300, 400, 500, xs, sm, md, lg, or xl"
        );
    }
    join_class([
        "d-select",
        select_size_class(size),
        extra,
        if disabled { "d-select--disabled" } else { "" },
    ])
}

/// Class string for the native `<select>`.
/// `error` and `success` are the names `validation_state` still returns.
/// v10 classes are `critical` and `positive`.
pub(crate) fn select_input_class(state: Option<&str>) -> String {
    let state_class = match state {
        Some("error" | "critical") => "d-select__input--critical",
        Some("warning") => "d-select__input--warning",
        Some("success" | "positive") => "d-select__input--positive",
        _ => "",
    };
    join_class(["d-select__input", state_class])
}

/// `aria-invalid="true"` when the message state is `error` or `critical`.
pub(crate) fn select_aria_invalid(state: Option<&str>) -> Option<&'static str> {
    matches!(state, Some("error" | "critical")).then_some("true")
}

/// Label classes. `DtText` label, tone `secondary`, plus `d-select__label-text`.
pub(crate) fn select_label_class(
    size: &str,
    label_size: &str,
    label_strength: &str,
    label_class: &str,
) -> String {
    let size = select_label_size(size, label_size);
    let text = text_classes(&text_style("label", size, "secondary", label_strength, ""));
    join_class([text.as_str(), "d-select__label-text", label_class])
}

/// Description classes. `DtText` body, tone `tertiary`, plus `d-description`.
pub(crate) fn select_description_class(size: &str, description_class: &str) -> String {
    let text = text_classes(&text_style(
        "body",
        description_size(size),
        "tertiary",
        "",
        description_density(size),
    ));
    join_class([text.as_str(), "d-description", description_class])
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

/// Dialtone Vue `DtSelectMenu` (`dialtone/v10.7.0`).
///
/// `value` / `on_change` are Vue `modelValue` plus `update:modelValue`.
/// Pass options with [`DtSelectOption`] or replace them with the default slot.
/// `size` is `100`–`500` (default `300`). `xs`–`xl` are deprecated aliases.
/// `show_label` is `showLabel`. A hidden label is the select `aria-label`.
/// `messages` still use the shared renderer, which reports `error`, `warning`,
/// and `success`. Those map to `critical`, `warning`, and `positive`.
#[component]
pub fn DtSelectMenu(
    /// Selected value (`modelValue`).
    #[prop(into)]
    value: Signal<String>,
    /// Selection changed (`update:modelValue`).
    #[prop(into)]
    on_change: Callback<String>,
    /// Options used when the default slot is absent.
    #[prop(optional)]
    options: Vec<DtSelectOption>,
    /// `100`–`500` (`size`). Default `300`. `xs`–`xl` still work.
    #[prop(optional, default = "300")]
    size: &'static str,
    #[prop(optional, into)] label: String,
    /// Show the label (`showLabel`). Default true.
    #[prop(optional, default = true)]
    show_label: bool,
    /// Accessible name when the label is absent (`aria-label`).
    #[prop(optional, into)]
    aria_label: String,
    #[prop(optional, into)] description: String,
    /// Overrides the derived label size (`labelSize`).
    #[prop(optional, default = "")]
    label_size: &'static str,
    /// Label weight (`labelStrength`).
    #[prop(optional, default = "")]
    label_strength: &'static str,
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
    let has_label = label_slot.is_some() || !label.is_empty();
    if !has_label && aria_label.is_empty() {
        leptos::logging::warn!(
            "[Dialtone] A label is required for accessibility. Provide a label prop and use show-label=\"false\" to hide it visually."
        );
    }
    let description_id = format!("{}-description", unique_id("select"));
    let has_description = description_slot.is_some() || !description.is_empty();
    let label_class_name = select_label_class(size, label_size, label_strength, &label_class);
    let description_class_name = select_description_class(size, &description_class);
    let state = validation_state(&messages);
    let wrapper_class = select_wrapper_class(size, &select_class, disabled);
    let input_class = select_input_class(state);
    let invalid = select_aria_invalid(state);
    let select_aria_label = if !aria_label.is_empty() {
        Some(aria_label)
    } else if !show_label && !label.is_empty() {
        Some(label.clone())
    } else {
        None
    };
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
    let show_label_node = show_label && has_label;
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
                {show_label_node.then(|| view! {
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
                        aria-label=select_aria_label
                        aria-invalid=invalid
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
        assert_eq!(select_wrapper_class("300", "", false), "d-select");
        assert_eq!(select_wrapper_class("md", "", false), "d-select");
        assert_eq!(
            select_wrapper_class("100", "custom", true),
            "d-select d-select--xs custom d-select--disabled"
        );
        assert_eq!(
            select_wrapper_class("xs", "custom", true),
            "d-select d-select--xs custom d-select--disabled"
        );
        assert_eq!(
            select_wrapper_class("500", "", false),
            "d-select d-select--xl"
        );
        assert_eq!(select_input_class(None), "d-select__input");
        assert_eq!(
            select_input_class(Some("error")),
            "d-select__input d-select__input--critical"
        );
        assert_eq!(
            select_input_class(Some("critical")),
            "d-select__input d-select__input--critical"
        );
        assert_eq!(
            select_input_class(Some("success")),
            "d-select__input d-select__input--positive"
        );
        assert_eq!(
            select_input_class(Some("positive")),
            "d-select__input d-select__input--positive"
        );
        assert_eq!(
            select_input_class(Some("warning")),
            "d-select__input d-select__input--warning"
        );
        assert_eq!(select_aria_invalid(Some("error")), Some("true"));
        assert_eq!(select_aria_invalid(Some("critical")), Some("true"));
        assert_eq!(select_aria_invalid(Some("success")), None);
        assert_eq!(select_aria_invalid(None), None);
    }

    #[test]
    fn label_and_description_classes_follow_dt_text() {
        assert_eq!(select_label_size("500", ""), "400");
        assert_eq!(select_label_size("xl", ""), "lg");
        assert_eq!(select_label_size("300", ""), "300");
        assert_eq!(select_label_size("300", "100"), "100");
        assert_eq!(
            select_label_class("300", "", "", ""),
            "d-text d-text-label--md d-text--tone-secondary d-select__label-text"
        );
        assert_eq!(
            select_label_class("500", "", "bold", "custom"),
            "d-text d-text-label--lg d-text--tone-secondary d-text--fw-bold d-select__label-text custom"
        );
        assert_eq!(
            select_label_class("xl", "", "", ""),
            "d-text d-text-label--lg d-text--tone-secondary d-select__label-text"
        );
        assert_eq!(
            select_description_class("300", ""),
            "d-text d-text-body--sm d-text--tone-tertiary d-description"
        );
        assert_eq!(
            select_description_class("100", "custom"),
            "d-text d-text-body--xs d-text--tone-tertiary d-description custom"
        );
        assert_eq!(
            select_description_class("500", ""),
            "d-text d-text-body--md d-text--tone-tertiary d-text--lh-300 d-description"
        );
        assert_eq!(
            select_description_class("lg", ""),
            "d-text d-text-body--sm d-text--tone-tertiary d-description"
        );
    }
}
