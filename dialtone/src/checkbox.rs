use std::sync::Arc;

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use super::radio::{radio_message_state, radio_messages_view, resolved_validation_state};
use super::text::{text_classes, TextStyle};
use super::unique_id;
use super::validation::DtValidationMessage;

/// Group state read by a slotted [`DtCheckbox`](crate::DtCheckbox).
#[derive(Clone)]
pub(crate) struct CheckboxGroupContext {
    pub(crate) name: Arc<str>,
    pub(crate) disabled: bool,
    pub(crate) validation_state: Option<String>,
    pub(crate) selected: Signal<Vec<String>>,
    pub(crate) on_change: Option<Callback<Vec<String>>>,
}

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn text_style<'a>(kind: &'a str, size: &'a str, tone: &'a str, strength: &'a str) -> TextStyle<'a> {
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
        density: "",
    }
}

/// Input class from `CHECKBOX_INPUT_VALIDATION_CLASSES`. `info` is a valid
/// state and adds no class. An unknown token warns and adds no class.
pub(crate) fn checkbox_validation_class(state: &str) -> &'static str {
    match state {
        "" | "info" => "",
        "warning" => "d-checkbox--warning",
        "critical" => "d-checkbox--critical",
        "positive" => "d-checkbox--positive",
        other => {
            leptos::logging::warn!(
                "[DtCheckbox] validationState \"{other}\" is not warning, critical, positive, or info."
            );
            ""
        }
    }
}

pub(crate) fn checkbox_input_class(state: &str, input_class: &str) -> String {
    join_class([
        "d-checkbox",
        checkbox_validation_class(state),
        input_class,
    ])
}

/// `DtText` label classes plus `d-checkbox__copy d-checkbox__label`.
/// Empty `label_size` is `300`. Empty `label_strength` is `normal`.
pub(crate) fn checkbox_label_class(
    label_class: &str,
    label_size: &str,
    label_strength: &str,
    disabled: bool,
) -> String {
    let size = if label_size.is_empty() {
        "300"
    } else {
        label_size
    };
    let strength = if label_strength.is_empty() {
        "normal"
    } else {
        label_strength
    };
    let tone = if disabled { "disabled" } else { "primary" };
    let text = text_classes(&text_style("label", size, tone, strength));
    join_class([
        text.as_str(),
        label_class,
        "d-checkbox__copy d-checkbox__label",
    ])
}

pub(crate) fn checkbox_description_class(description_class: &str) -> String {
    let text = text_classes(&text_style("body", "200", "tertiary", ""));
    join_class([text.as_str(), "d-description", description_class])
}

/// Next `selectedValues` after one checkbox changes. Dialtone `setGroupValue`.
pub(crate) fn next_selected(selected: &[String], value: &str, checked: bool) -> Vec<String> {
    let mut next = selected.to_vec();
    if !checked {
        next.retain(|item| item != value);
    } else if !next.iter().any(|item| item == value) {
        next.push(value.to_string());
    }
    next
}

/// Dialtone Vue `DtCheckbox` (`dialtone/v10.7.0`).
///
/// `checked` is the boolean `modelValue`. `on_change` is `update:modelValue`
/// and reports that boolean. Inside [`DtCheckboxGroup`](crate::DtCheckboxGroup),
/// the checked state follows `selected_values`. A click while `indeterminate`
/// reports unchecked, clears the indeterminate mark, and sets the DOM
/// `checked` property to false. `show_label` is `showLabel`. `label_size` is
/// `labelSize`. `label_strength` is `labelStrength`. `validation_state` is
/// `validationState`. `class` is `$attrs.class` on the root div.
///
/// A non-empty `aria_label` wins over the hidden-label fallback. Vue binds
/// `$attrs` after the `showLabel` aria-label.
#[component]
pub fn DtCheckbox(
    /// Checked state (`modelValue`). Ignored for display inside a checkbox group.
    #[prop(optional)]
    checked: Option<Signal<bool>>,
    /// Called with the next checked value (`update:modelValue`).
    #[prop(optional, into)]
    on_change: Option<Callback<bool>>,
    /// Clears on click. The next click reports unchecked.
    #[prop(optional)]
    indeterminate: bool,
    /// Input value (`value`). This is the group token inside a checkbox group.
    #[prop(optional, into)]
    value: String,
    #[prop(optional, into)] name: String,
    /// Label text. The default slot replaces it.
    #[prop(optional, into)]
    label: String,
    /// Show the label (`showLabel`). False sets `aria-label` from `label`.
    #[prop(optional, default = true)]
    show_label: bool,
    /// Label size (`labelSize`). Empty uses `300`.
    #[prop(optional, default = "")]
    label_size: &'static str,
    /// Label weight (`labelStrength`). Empty uses `normal`.
    #[prop(optional, default = "")]
    label_strength: &'static str,
    #[prop(optional, into)] description: String,
    #[prop(optional)] disabled: bool,
    /// `warning`, `critical`, `positive`, or `info` (`validationState`).
    /// A group state wins. `info` adds no input class.
    #[prop(optional, default = "")]
    validation_state: &'static str,
    /// Extra class on the input (`inputClass`).
    #[prop(optional, into)]
    input_class: String,
    /// Extra class on the label (`labelClass`).
    #[prop(optional, into)]
    label_class: String,
    /// Extra class on the description (`descriptionClass`).
    #[prop(optional, into)]
    description_class: String,
    /// Extra class on the messages (`messagesClass`).
    #[prop(optional, into)]
    messages_class: String,
    /// Extra class on the root (`$attrs.class`).
    #[prop(optional, into)]
    class: String,
    /// Show `messages` (`showMessages`).
    #[prop(optional, default = true)]
    show_messages: bool,
    #[prop(optional)] messages: Vec<DtValidationMessage>,
    /// Accessible name. Wins over the hidden-label fallback.
    #[prop(optional, into)]
    aria_label: String,
    #[prop(optional, into)] on_focus_in: Option<Callback<()>>,
    #[prop(optional, into)] on_focus_out: Option<Callback<()>>,
    /// Description slot (`description`). Replaces `description`.
    #[prop(optional)]
    description_slot: Option<ViewFn>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let checked = checked.unwrap_or_else(|| Signal::derive(|| false));
    let group = use_context::<CheckboxGroupContext>();
    let local_indeterminate = RwSignal::new(indeterminate);
    let has_label_content = children.is_some() || !label.is_empty();
    if !has_label_content && aria_label.is_empty() {
        leptos::logging::warn!(
            "[Dialtone] A label is required for accessibility. Provide a label prop and use show-label=\"false\" to hide it visually."
        );
    }
    if let Some(group) = group.as_ref() {
        if !name.is_empty() && group.name.as_ref() != name.as_str() {
            leptos::logging::warn!(
                "Component is being used inside a Group. Did you mean to override the name prop value ({}) with ({})? It is recommended to only set name at the Group level.",
                group.name,
                name
            );
        }
    }
    let is_disabled = disabled || group.as_ref().is_some_and(|group| group.disabled);
    let input_name = if !name.is_empty() {
        name
    } else {
        group
            .as_ref()
            .map(|group| group.name.to_string())
            .unwrap_or_default()
    };
    let state = resolved_validation_state(
        group
            .as_ref()
            .and_then(|group| group.validation_state.as_deref()),
        validation_state,
    );
    let input_class_name = checkbox_input_class(&state, &input_class);
    let label_class_name =
        checkbox_label_class(&label_class, label_size, label_strength, is_disabled);
    let description_class_name = checkbox_description_class(&description_class);
    let root_class = (!class.is_empty()).then_some(class);
    let name_attr = (!input_name.is_empty()).then_some(input_name);
    let input_label = if !aria_label.is_empty() {
        Some(aria_label)
    } else if !show_label && !label.is_empty() {
        Some(label.clone())
    } else {
        None
    };
    let value_for_check = value.clone();
    let group_for_check = group.clone();
    let is_checked = move || {
        if let Some(group) = group_for_check.clone() {
            let value_for_check = value_for_check.clone();
            group
                .selected
                .with(|items| items.iter().any(|item| item == &value_for_check))
        } else {
            checked.get()
        }
    };
    let messages_id = unique_id("dt-checkbox-messages");
    let described_by = (show_messages && !messages.is_empty()).then(|| messages_id.clone());
    let aria_invalid = (radio_message_state(&messages) == Some("critical")).then_some("true");
    let messages_view = radio_messages_view(
        &messages,
        show_messages,
        &messages_class,
        "dt-checkbox-validation-messages",
        &messages_id,
    );
    let has_description = description_slot.is_some() || !description.is_empty();
    let description_body = if let Some(slot) = description_slot {
        Some(slot.run().into_any())
    } else if !description.is_empty() {
        Some(description.into_any())
    } else {
        None
    };
    let label_body = if show_label {
        if let Some(children) = children {
            Some(children().into_any())
        } else if !label.is_empty() {
            Some(label.into_any())
        } else {
            None
        }
    } else {
        None
    };

    let value_for_change = value.clone();
    let group_for_change = group.clone();
    view! {
        <div class=root_class>
            <label class="d-checkbox-group" class=("d-checkbox-group--disabled", is_disabled)>
                <div class="d-checkbox__input">
                    <input
                        type="checkbox"
                        class=input_class_name
                        name=name_attr
                        value=value.clone()
                        prop:checked=is_checked
                        prop:disabled=is_disabled
                        prop:indeterminate=move || local_indeterminate.get()
                        aria-label=input_label
                        aria-invalid=aria_invalid
                        aria-describedby=described_by
                        on:change=move |event| {
                            let mut next = event_target_checked(&event);
                            if local_indeterminate.get_untracked() {
                                next = false;
                                local_indeterminate.set(false);
                                if let Some(target) = event.target() {
                                    if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                                        input.set_checked(false);
                                    }
                                }
                            }
                            if let Some(group) = group_for_change.clone() {
                                let values = next_selected(
                                    &group.selected.get_untracked(),
                                    &value_for_change,
                                    next,
                                );
                                if let Some(on_change) = group.on_change {
                                    on_change.run(values);
                                }
                            }
                            if let Some(on_change) = on_change {
                                on_change.run(next);
                            }
                        }
                        on:focusin=move |_| {
                            if let Some(on_focus_in) = on_focus_in {
                                on_focus_in.run(());
                            }
                        }
                        on:focusout=move |_| {
                            if let Some(on_focus_out) = on_focus_out {
                                on_focus_out.run(());
                            }
                        }
                    />
                </div>
                {label_body.map(|body| view! {
                    <div class=label_class_name.clone() data-qa="checkbox-label">
                        {body}
                    </div>
                })}
            </label>
            <div class="d-checkbox__messages" data-qa="checkbox-description-messages">
                {has_description.then(|| view! {
                    <div class=description_class_name.clone() data-qa="checkbox-description">
                        {description_body}
                    </div>
                })}
                {messages_view}
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_class_matches_the_vue_map() {
        assert_eq!(checkbox_validation_class("warning"), "d-checkbox--warning");
        assert_eq!(checkbox_validation_class("critical"), "d-checkbox--critical");
        assert_eq!(checkbox_validation_class("positive"), "d-checkbox--positive");
        assert_eq!(checkbox_validation_class("info"), "");
        assert_eq!(checkbox_validation_class(""), "");
        assert_eq!(checkbox_validation_class("error"), "");
        assert_eq!(checkbox_validation_class("success"), "");
    }

    #[test]
    fn input_class_joins_in_vue_order() {
        assert_eq!(checkbox_input_class("", ""), "d-checkbox");
        assert_eq!(
            checkbox_input_class("critical", "my-custom-class"),
            "d-checkbox d-checkbox--critical my-custom-class"
        );
    }

    #[test]
    fn label_and_description_classes_follow_dt_text() {
        assert_eq!(
            checkbox_label_class("", "", "", false),
            "d-text d-text-label--md d-text--tone-primary d-text--fw-normal d-checkbox__copy d-checkbox__label"
        );
        assert_eq!(
            checkbox_label_class("my-custom-class", "400", "bold", true),
            "d-text d-text-label--lg d-text--tone-disabled d-text--fw-bold my-custom-class d-checkbox__copy d-checkbox__label"
        );
        assert_eq!(
            checkbox_description_class("my-custom-class"),
            "d-text d-text-body--sm d-text--tone-tertiary d-description my-custom-class"
        );
    }

    #[test]
    fn group_selection_adds_and_removes_the_value() {
        let selected = vec!["a".to_string(), "b".to_string()];
        assert_eq!(
            next_selected(&selected, "c", true),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
        assert_eq!(next_selected(&selected, "a", true), selected);
        assert_eq!(next_selected(&selected, "a", false), vec!["b".to_string()]);
        assert_eq!(
            next_selected(&["a".into(), "a".into()], "a", false),
            Vec::<String>::new()
        );
    }
}
