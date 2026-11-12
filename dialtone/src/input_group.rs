use leptos::prelude::*;

use super::radio::{radio_legend_class, radio_message_state, radio_messages_view};
use super::unique_id;
use super::validation::DtValidationMessage;

/// Dialtone Vue `DtInputGroup` (`dialtone/v10.7.0`).
///
/// The root is `fieldset.d-input-group__fieldset`. The legend is `DtText` label,
/// tone secondary, plus `d-input-group__legend-text`. The `legend` slot replaces
/// `legend`. `messages` use the v10 types `critical`, `warning`, `positive`, and
/// `info`. An empty kind means `critical`. `aria-invalid` is `"true"` only for
/// `critical`. `aria-describedby` points at the messages container when
/// `show_messages` is set and `messages` is not empty.
///
/// Vue requires `name` and provides `value` to slotted inputs. This port warns
/// when `name` is empty. It does not provide that value. Slotted [`DtInput`](crate::DtInput)
/// stays controlled by its own `value`. `disabled` from the mixin is not copied
/// onto those inputs. `legendChildProps` and `messagesChildProps` are not ported.
#[component]
pub fn DtInputGroup(
    /// Group name (`name`). Vue requires it. Empty warns.
    #[prop(optional, into)]
    name: String,
    /// Legend text. The `legend` slot replaces it.
    #[prop(optional, into)]
    legend: String,
    /// Extra class on the fieldset (`$attrs.class`).
    #[prop(optional, into)]
    class: String,
    #[prop(optional, default = true)] show_messages: bool,
    #[prop(optional)] messages: Vec<DtValidationMessage>,
    /// Extra class on the legend (`legendClass`).
    #[prop(optional, into)]
    legend_class: String,
    /// Extra class on the messages (`messagesClass`).
    #[prop(optional, into)]
    messages_class: String,
    /// Accessible name when there is no legend (`aria-label`).
    #[prop(optional, into)]
    aria_label: String,
    /// Root `data-qa` (`dataQaGroup`). Default `input-group`.
    #[prop(optional, default = "input-group")]
    data_qa_group: &'static str,
    /// Legend `data-qa` (`dataQaGroupLegend`). Default `input-group-legend`.
    #[prop(optional, default = "input-group-legend")]
    data_qa_group_legend: &'static str,
    /// Messages `data-qa` (`dataQaGroupMessages`). Default `input-group-messages`.
    #[prop(optional, default = "input-group-messages")]
    data_qa_group_messages: &'static str,
    /// Legend slot (`legend`). Replaces `legend`.
    #[prop(optional)]
    legend_slot: Option<ViewFn>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    if name.is_empty() {
        leptos::logging::warn!("[DtInputGroup] name is required.");
    }
    let _ = name;
    let has_legend = legend_slot.is_some() || !legend.is_empty();
    if !has_legend && aria_label.is_empty() {
        leptos::logging::warn!(
            "It is expected that an aria-label is provided when there is no legend."
        );
    }
    let state = radio_message_state(&messages);
    let legend_class_name = radio_legend_class(&legend_class);
    let legend_body = if let Some(slot) = legend_slot {
        Some(slot.run().into_any())
    } else if !legend.is_empty() {
        Some(legend.into_any())
    } else {
        None
    };
    let messages_id = unique_id("dt-input-group-messages");
    let described_by = (show_messages && !messages.is_empty()).then(|| messages_id.clone());
    let aria_invalid = (state == Some("critical")).then_some("true");
    let messages_view = radio_messages_view(
        &messages,
        show_messages,
        &messages_class,
        data_qa_group_messages,
        &messages_id,
    );
    let class_name = join_class(["d-input-group__fieldset", class.as_str()]);
    let aria_label = (!aria_label.is_empty()).then_some(aria_label);

    view! {
        <fieldset
            class=class_name
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
            {children.map(|children| children())}
            {messages_view}
        </fieldset>
    }
}

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}
