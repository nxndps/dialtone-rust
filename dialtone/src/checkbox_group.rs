use std::sync::Arc;

use leptos::context::Provider;
use leptos::prelude::*;

use super::checkbox::CheckboxGroupContext;
use super::validation::{validation_messages, validation_state, DtValidationMessage};

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Dialtone Vue `DtCheckboxGroup` (`dialtone/v9.190.0`).
///
/// `selected_values` is the Vue `selectedValues` model. Slotted
/// [`DtCheckbox`](crate::DtCheckbox) items whose `value` is in that list render
/// checked. `on_change` receives the next list. `name` is required and is
/// copied onto a checkbox that does not set its own name. `messages` use
/// [`DtValidationMessage`](crate::DtValidationMessage).
#[component]
pub fn DtCheckboxGroup(
    /// Shared input name (`name`).
    #[prop(into)]
    name: String,
    /// Selected checkbox values (`selectedValues`).
    #[prop(into)]
    selected_values: Signal<Vec<String>>,
    /// Called with the next selected values.
    #[prop(optional, into)]
    on_change: Option<Callback<Vec<String>>>,
    /// Legend text. The `legend` slot replaces it.
    #[prop(optional, into)]
    legend: String,
    #[prop(optional)] disabled: bool,
    #[prop(optional, default = true)] show_messages: bool,
    #[prop(optional)] messages: Vec<DtValidationMessage>,
    /// Extra class on the legend (`legendClass`).
    #[prop(optional, into)]
    legend_class: String,
    /// Extra class on the messages (`messagesClass`).
    #[prop(optional, into)]
    messages_class: String,
    /// Accessible name when there is no legend.
    #[prop(optional, into)]
    aria_label: String,
    /// Root `data-qa` (`dataQaGroup`). Default `checkbox-group`.
    #[prop(optional, default = "checkbox-group")]
    data_qa_group: &'static str,
    /// Legend `data-qa` (`dataQaGroupLegend`). Default `checkbox-group-legend`.
    #[prop(optional, default = "checkbox-group-legend")]
    data_qa_group_legend: &'static str,
    /// Messages `data-qa` (`dataQaGroupMessages`). Default `checkbox-group-messages`.
    #[prop(optional, default = "checkbox-group-messages")]
    data_qa_group_messages: &'static str,
    /// Legend slot (`legend`). Replaces `legend`.
    #[prop(optional)]
    legend_slot: Option<ViewFn>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let state = validation_state(&messages).map(str::to_string);
    let context = CheckboxGroupContext {
        name: Arc::<str>::from(name),
        disabled,
        validation_state: state,
        selected: selected_values,
        on_change,
    };
    let legend_class_name = join_class(["d-label", legend_class.as_str()]);
    let legend_body = if let Some(slot) = legend_slot {
        Some(slot.run().into_any())
    } else if !legend.is_empty() {
        Some(legend.into_any())
    } else {
        None
    };
    let messages_view = validation_messages(
        messages,
        show_messages,
        &messages_class,
        data_qa_group_messages,
    );
    let aria_label = (!aria_label.is_empty()).then_some(aria_label);

    view! {
        <fieldset class="d-input-group__fieldset" data-qa=data_qa_group aria-label=aria_label>
            {legend_body.map(|body| view! {
                <legend class=legend_class_name data-qa=data_qa_group_legend>
                    {body}
                </legend>
            })}
            {children.map(|children| view! {
                <Provider value=context>
                    {children()}
                </Provider>
            })}
            {messages_view}
        </fieldset>
    }
}
