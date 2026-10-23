use leptos::prelude::*;

use super::unique_id;

/// On, off, or the docs `modelValue` `"mixed"`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ToggleState {
    Off,
    On,
    Mixed,
}

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn state_of(checked: bool, indeterminate: bool) -> ToggleState {
    if indeterminate {
        ToggleState::Mixed
    } else if checked {
        ToggleState::On
    } else {
        ToggleState::Off
    }
}

/// `200` and `sm` use `d-toggle--small`. `300` and `md` add no modifier.
/// The current docs site uses `200` and `300`. Tag `dialtone/v9.190.0` uses `sm` and `md`.
/// Both map to the same vendored class.
pub(crate) fn toggle_size_class(size: &str) -> &'static str {
    match size {
        "200" | "sm" => "d-toggle--small",
        "300" | "md" => "",
        _ => {
            leptos::logging::warn!("DtToggle size `{size}` is not 200, 300, sm, or md");
            ""
        }
    }
}

/// Button classes in the Vue computed order.
pub(crate) fn toggle_button_classes(size: &str, state: ToggleState, disabled: bool) -> String {
    join_class([
        "d-toggle",
        toggle_size_class(size),
        (state == ToggleState::On)
            .then_some("d-toggle--checked")
            .unwrap_or(""),
        disabled.then_some("d-toggle--disabled").unwrap_or(""),
        (state == ToggleState::Mixed)
            .then_some("d-toggle--indeterminate")
            .unwrap_or(""),
    ])
}

/// Mixed uses `checkbox`. On and off use `switch`.
pub(crate) fn toggle_role(state: ToggleState) -> &'static str {
    match state {
        ToggleState::Mixed => "checkbox",
        ToggleState::On | ToggleState::Off => "switch",
    }
}

/// `aria-checked` from `modelValue.toString()`.
pub(crate) fn toggle_aria_checked(state: ToggleState) -> &'static str {
    match state {
        ToggleState::On => "true",
        ToggleState::Off => "false",
        ToggleState::Mixed => "mixed",
    }
}

/// Next boolean after a click. `"mixed"` is truthy in Vue, so `!'mixed'` is `false`.
pub(crate) fn next_toggle_checked(state: ToggleState) -> bool {
    match state {
        ToggleState::Off => true,
        ToggleState::On | ToggleState::Mixed => false,
    }
}

/// Dialtone Vue `DtToggle`.
///
/// The [toggle page](https://dialtone.dialpad.com/components/toggle.html) is a
/// native `button`. `checked` is `modelValue` `true` or `false`.
/// `indeterminate` is `modelValue` `"mixed"`: role `checkbox`,
/// `aria-checked="mixed"`, and `d-toggle--indeterminate`. A click on mixed
/// reports `false`. `size` accepts `200` or `sm`, and `300` or `md`.
/// `show_icon` is `showIcon`. `show_label` is `showLabel`. `toggle_on_click`
/// is `toggleOnClick`: the click still reports the next value, and the knob
/// moves even when `checked` stays put. Enter and Space activate the button.
/// The default slot is the label (`data-qa="toggle-label"`). `class` lands on
/// `.d-toggle-wrapper`. `label_class` is `labelClass`.
#[component]
pub fn DtToggle(
    /// On or off (`modelValue`). Ignored while `indeterminate` is set.
    #[prop(into)]
    checked: Signal<bool>,
    /// Next checked value (`update:modelValue`). Mixed reports `false`.
    #[prop(into)]
    on_change: Callback<bool>,
    /// `"mixed"` (`modelValue`).
    #[prop(optional, into)]
    indeterminate: MaybeProp<bool>,
    /// Accessible name when the label slot is absent or hidden.
    #[prop(optional, into)]
    aria_label: String,
    #[prop(optional)] disabled: bool,
    /// `200`/`sm` or `300`/`md` (`size`). Default `md`.
    #[prop(optional, default = "md")]
    size: &'static str,
    /// Draw the check or cross inside the knob (`showIcon`).
    #[prop(optional, default = true)]
    show_icon: bool,
    /// Render the label slot (`showLabel`).
    #[prop(optional, default = true)]
    show_label: bool,
    /// Move the knob on click when `checked` does not change (`toggleOnClick`).
    #[prop(optional, default = true)]
    toggle_on_click: bool,
    /// Button id. Generated when empty.
    #[prop(optional, into)]
    id: String,
    /// Extra class on the label (`labelClass`).
    #[prop(optional, into)]
    label_class: String,
    /// Extra class on `.d-toggle-wrapper`.
    #[prop(optional, into)]
    class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let has_label = show_label && children.is_some();
    if !has_label && aria_label.is_empty() {
        leptos::logging::warn!(
            "DtToggle: A label is required for accessibility. Provide a label and use show_label=false to hide it visually."
        );
    }
    let input_id = if id.is_empty() {
        unique_id("dt-toggle")
    } else {
        id
    };
    let wrapper_class = join_class(["d-toggle-wrapper", class.as_str()]);
    let label_class_attr = (!label_class.is_empty()).then_some(label_class);
    let aria_label_attr = (!aria_label.is_empty()).then_some(aria_label);
    let initial = state_of(
        checked.get_untracked(),
        indeterminate.get_untracked().unwrap_or(false),
    );
    let shown_override = RwSignal::new(None::<ToggleState>);
    let synced = StoredValue::new(initial);
    Effect::new(move |_| {
        let next = state_of(checked.get(), indeterminate.get().unwrap_or(false));
        if next != synced.get_value() {
            synced.set_value(next);
            shown_override.set(None);
        }
    });
    let shown = move || {
        let from_props = state_of(checked.get(), indeterminate.get().unwrap_or(false));
        shown_override.get().unwrap_or(from_props)
    };
    let disabled_attr = disabled.then_some(true);
    let aria_disabled = if disabled { "true" } else { "false" };

    view! {
        <div class=wrapper_class>
            {has_label.then(|| children.map(|children| {
                view! {
                    <label
                        class=label_class_attr.clone()
                        for=input_id.clone()
                        data-qa="toggle-label"
                    >
                        {children()}
                    </label>
                }
            }))}
            <button
                id=input_id
                type="button"
                role=move || toggle_role(shown())
                class=move || toggle_button_classes(size, shown(), disabled)
                disabled=disabled_attr
                aria-disabled=aria_disabled
                aria-checked=move || toggle_aria_checked(shown())
                aria-label=aria_label_attr.clone()
                on:click=move |_| {
                    if disabled {
                        return;
                    }
                    let before = state_of(
                        checked.get_untracked(),
                        indeterminate.get_untracked().unwrap_or(false),
                    );
                    let current = shown_override.get_untracked().unwrap_or(before);
                    let next = next_toggle_checked(current);
                    on_change.run(next);
                    if !toggle_on_click {
                        return;
                    }
                    let after = state_of(
                        checked.get_untracked(),
                        indeterminate.get_untracked().unwrap_or(false),
                    );
                    if after == before {
                        shown_override.set(Some(state_of(next, false)));
                    } else {
                        shown_override.set(None);
                    }
                }
            >
                {show_icon.then(|| view! { <span class="d-toggle__inner"></span> })}
            </button>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_and_state_classes_follow_the_docs() {
        assert_eq!(toggle_size_class("200"), "d-toggle--small");
        assert_eq!(toggle_size_class("sm"), "d-toggle--small");
        assert_eq!(toggle_size_class("300"), "");
        assert_eq!(toggle_size_class("md"), "");
        assert_eq!(
            toggle_button_classes("md", ToggleState::Off, false),
            "d-toggle"
        );
        assert_eq!(
            toggle_button_classes("200", ToggleState::On, false),
            "d-toggle d-toggle--small d-toggle--checked"
        );
        assert_eq!(
            toggle_button_classes("sm", ToggleState::Mixed, true),
            "d-toggle d-toggle--small d-toggle--disabled d-toggle--indeterminate"
        );
        assert_eq!(
            toggle_button_classes("nope", ToggleState::Off, false),
            "d-toggle"
        );
    }

    #[test]
    fn mixed_uses_checkbox_and_click_turns_off() {
        assert_eq!(toggle_role(ToggleState::Off), "switch");
        assert_eq!(toggle_role(ToggleState::On), "switch");
        assert_eq!(toggle_role(ToggleState::Mixed), "checkbox");
        assert_eq!(toggle_aria_checked(ToggleState::Off), "false");
        assert_eq!(toggle_aria_checked(ToggleState::On), "true");
        assert_eq!(toggle_aria_checked(ToggleState::Mixed), "mixed");
        assert!(next_toggle_checked(ToggleState::Off));
        assert!(!next_toggle_checked(ToggleState::On));
        assert!(!next_toggle_checked(ToggleState::Mixed));
    }
}
