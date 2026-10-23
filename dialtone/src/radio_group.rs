use std::sync::Arc;

use leptos::context::Provider;
use leptos::prelude::*;

use super::radio::{
    radio_legend_class, radio_message_state, radio_messages_view, RadioGroupContext,
};
use super::unique_id;
use super::validation::DtValidationMessage;

/// Dialtone Vue `DtRadioGroup` (`dialtone/v10.7.0`).
///
/// Extends `DtInputGroup`. `selected` is `modelValue`. Slotted
/// [`DtRadio`](crate::DtRadio) items whose `value` equals `selected` render
/// checked. `on_change` receives the next value. `name` is required and is
/// copied onto a radio that does not set its own name. `messages` use the v10
/// types `critical`, `warning`, `positive`, and `info`.
#[component]
pub fn DtRadioGroup(
    /// Shared input name (`name`).
    #[prop(into)]
    name: String,
    /// Selected radio value (`modelValue`).
    #[prop(into)]
    selected: Signal<String>,
    /// Called with the next value (`update:modelValue`).
    #[prop(optional, into)]
    on_change: Option<Callback<String>>,
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
    /// Root `data-qa` (`dataQaGroup`). Default `radio-group`.
    #[prop(optional, default = "radio-group")]
    data_qa_group: &'static str,
    /// Legend `data-qa` (`dataQaGroupLegend`). Default `radio-group-legend`.
    #[prop(optional, default = "radio-group-legend")]
    data_qa_group_legend: &'static str,
    /// Messages `data-qa` (`dataQaGroupMessages`). Default `radio-group-messages`.
    #[prop(optional, default = "radio-group-messages")]
    data_qa_group_messages: &'static str,
    /// Legend slot (`legend`). Replaces `legend`.
    #[prop(optional)]
    legend_slot: Option<ViewFn>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    if name.is_empty() {
        leptos::logging::warn!("[DtRadioGroup] name is required.");
    }
    let has_legend = legend_slot.is_some() || !legend.is_empty();
    if !has_legend && aria_label.is_empty() {
        leptos::logging::warn!(
            "It is expected that an aria-label is provided when there is no legend."
        );
    }
    let state = radio_message_state(&messages).map(str::to_string);
    let context = RadioGroupContext {
        name: Arc::<str>::from(name),
        disabled,
        validation_state: state.clone(),
        selected,
        on_change,
    };
    let legend_class_name = radio_legend_class(&legend_class);
    let legend_body = if let Some(slot) = legend_slot {
        Some(slot.run().into_any())
    } else if !legend.is_empty() {
        Some(legend.into_any())
    } else {
        None
    };
    let messages_id = unique_id("dt-radio-group-messages");
    let described_by = (show_messages && !messages.is_empty()).then(|| messages_id.clone());
    let aria_invalid = (state.as_deref() == Some("critical")).then_some("true");
    let messages_view = radio_messages_view(
        &messages,
        show_messages,
        &messages_class,
        data_qa_group_messages,
        &messages_id,
    );
    let aria_label = (!aria_label.is_empty()).then_some(aria_label);

    view! {
        <fieldset
            class="d-input-group__fieldset"
            data-qa=data_qa_group
            aria-label=aria_label
            aria-invalid=aria_invalid
            aria-describedby=described_by
        >
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
