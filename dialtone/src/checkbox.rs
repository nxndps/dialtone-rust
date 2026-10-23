use std::sync::Arc;

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use super::validation::{validation_messages, DtValidationMessage};

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

fn checkbox_validation_class(state: &str) -> &'static str {
    match state {
        "error" => "d-checkbox--error",
        "warning" => "d-checkbox--warning",
        "success" => "d-checkbox--success",
        _ => "",
    }
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

/// Dialtone Vue `DtCheckbox` (`dialtone/v9.190.0`).
///
/// `checked` is the Vue `modelValue`. Inside [`DtCheckboxGroup`](crate::DtCheckboxGroup),
/// the checked state comes from `selected_values` and `value`. A click while
/// `indeterminate` reports unchecked and clears the indeterminate mark.
/// `messages` use [`DtValidationMessage`].
#[component]
pub fn DtCheckbox(
    /// Checked state (`modelValue`). Ignored for display inside a checkbox group.
    #[prop(optional)]
    checked: Option<Signal<bool>>,
    /// Called with the next checked value.
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
    #[prop(optional, into)] description: String,
    #[prop(optional)] disabled: bool,
    /// `error`, `warning`, or `success` (`validationState`). A group state wins.
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
    /// Extra class on the root (`rootClass`).
    #[prop(optional, into)]
    root_class: String,
    /// Extra class on the messages (`messagesClass`).
    #[prop(optional, into)]
    messages_class: String,
    /// Show `messages` (`showMessages`).
    #[prop(optional, default = true)]
    show_messages: bool,
    #[prop(optional)] messages: Vec<DtValidationMessage>,
    #[prop(optional, into)] aria_label: String,
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
    let has_label = children.is_some() || !label.is_empty();
    let has_description = description_slot.is_some() || !description.is_empty();
    let has_messages = show_messages && !messages.is_empty();
    let is_disabled = disabled || group.as_ref().is_some_and(|group| group.disabled);
    let input_name = if !name.is_empty() {
        name
    } else {
        group
            .as_ref()
            .map(|group| group.name.to_string())
            .unwrap_or_default()
    };
    let state = group
        .as_ref()
        .and_then(|group| group.validation_state.clone())
        .filter(|state| !state.is_empty())
        .unwrap_or_else(|| validation_state.to_string());
    let input_class_name = join_class([
        "d-checkbox",
        checkbox_validation_class(&state),
        input_class.as_str(),
    ]);
    let label_class_name = join_class([label_class.as_str(), "d-checkbox__copy d-checkbox__label"]);
    let description_class_name = join_class(["d-description", description_class.as_str()]);
    let root_class = (!root_class.is_empty()).then_some(root_class);
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
    let messages_view = validation_messages(
        messages,
        show_messages,
        &messages_class,
        "dt-checkbox-validation-messages",
    );
    let description_body = if let Some(slot) = description_slot {
        Some(slot.run().into_any())
    } else if !description.is_empty() {
        Some(description.into_any())
    } else {
        None
    };
    let label_body = if let Some(children) = children {
        Some(children().into_any())
    } else if !label.is_empty() {
        Some(label.into_any())
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
                        name=input_name
                        value=value.clone()
                        prop:checked=is_checked
                        prop:disabled=is_disabled
                        prop:indeterminate=move || local_indeterminate.get()
                        aria-label=(!aria_label.is_empty()).then_some(aria_label)
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
                {has_label.then(|| view! {
                    <div class=label_class_name.clone() data-qa="checkbox-label">
                        {label_body}
                    </div>
                })}
            </label>
            {(has_description || has_messages).then(|| view! {
                <div class="d-checkbox__messages" data-qa="checkbox-description-messages">
                    {has_description.then(|| view! {
                        <div class=description_class_name.clone() data-qa="checkbox-description">
                            {description_body}
                        </div>
                    })}
                    {messages_view}
                </div>
            })}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_class_matches_the_vue_map() {
        assert_eq!(checkbox_validation_class("error"), "d-checkbox--error");
        assert_eq!(checkbox_validation_class("warning"), "d-checkbox--warning");
        assert_eq!(checkbox_validation_class("success"), "d-checkbox--success");
        assert_eq!(checkbox_validation_class(""), "");
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
