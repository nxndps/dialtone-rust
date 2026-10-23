use super::*;

const SIZES: &[(&str, &str)] = &[
    ("", "Default"),
    ("100", "100"),
    ("200", "200"),
    ("300", "300"),
    ("400", "400"),
    ("500", "500"),
];

const ACTIVATION: &[(&str, &str)] = &[("", "Default"), ("auto", "Auto"), ("manual", "Manual")];

const SPREAD: &[(&str, &str)] = &[("", "Default"), ("grow", "Grow"), ("evenly", "Evenly")];

fn sample_items() -> String {
    [
        element(
            "DtSegmentedControlItem",
            &[("value", rust_string("day"))],
            Some(&rust_string("Day")),
        ),
        element(
            "DtSegmentedControlItem",
            &[("value", rust_string("week"))],
            Some(&rust_string("Week")),
        ),
        element(
            "DtSegmentedControlItem",
            &[("value", rust_string("month"))],
            Some(&rust_string("Month")),
        ),
    ]
    .join("\n")
}

#[component]
pub(crate) fn SegmentedControlPage() -> impl IntoView {
    let value = RwSignal::new("week".to_string());
    let orientation = RwSignal::new("horizontal");
    let id = RwSignal::new(String::new());
    let aria_label = RwSignal::new("Range".to_string());
    let size = RwSignal::new("");
    let activation = RwSignal::new("");
    let disabled = RwSignal::new(false);
    let show_divider = RwSignal::new(true);
    let borderless = RwSignal::new(false);
    let spread = RwSignal::new("");
    let show_transition = RwSignal::new(true);
    let label_class = RwSignal::new(String::new());
    let default_slot = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let orientation = orientation.get();
                let size = if size.get().is_empty() { "200" } else { size.get() };
                let activation = if activation.get().is_empty() {
                    "auto"
                } else {
                    activation.get()
                };
                let spread = if spread.get().is_empty() { "grow" } else { spread.get() };
                let id = id.get();
                let aria_label = aria_label.get();
                let disabled = disabled.get();
                let show_divider = show_divider.get();
                let borderless = borderless.get();
                let show_transition = show_transition.get();
                let label_class = label_class.get();
                let custom = default_slot.get();
                if custom.is_empty() {
                    view! {
                        <DtSegmentedControl
                            id=id.clone()
                            aria_label=aria_label.clone()
                            orientation=orientation
                            size=size
                            activation_mode=activation
                            disabled=disabled
                            show_divider=show_divider
                            borderless=borderless
                            spread=spread
                            label_class=label_class.clone()
                            show_indicator_transition=show_transition
                            value=value
                            on_change=move |next| value.set(next)
                        >
                            <DtSegmentedControlItem value="day">"Day"</DtSegmentedControlItem>
                            <DtSegmentedControlItem value="week">"Week"</DtSegmentedControlItem>
                            <DtSegmentedControlItem value="month">"Month"</DtSegmentedControlItem>
                        </DtSegmentedControl>
                    }
                    .into_any()
                } else {
                    view! {
                        <DtSegmentedControl
                            id=id
                            aria_label=aria_label
                            orientation=orientation
                            size=size
                            activation_mode=activation
                            disabled=disabled
                            show_divider=show_divider
                            borderless=borderless
                            spread=spread
                            label_class=label_class
                            show_indicator_transition=show_transition
                            value=value
                            on_change=move |next| value.set(next)
                        >
                            <DtSegmentedControlItem value="custom">{custom}</DtSegmentedControlItem>
                        </DtSegmentedControl>
                    }
                    .into_any()
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Value" value=value />
            <SettingText label="Id" value=id />
            <SettingText label="Aria label" value=aria_label />
            <SettingSelect
                label="Orientation"
                value=orientation
                options=vec![("horizontal", "Horizontal"), ("vertical", "Vertical")]
            />
            <SettingSelect label="Size" value=size options=SIZES.to_vec() />
            <SettingSelect label="Activation" value=activation options=ACTIVATION.to_vec() />
            <SettingSelect label="Spread" value=spread options=SPREAD.to_vec() />
            <SettingCheck label="Disabled" value=disabled />
            <SettingCheck label="Show divider" value=show_divider />
            <SettingCheck label="Borderless" value=borderless />
            <SettingCheck label="Show indicator transition" value=show_transition />
        }
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingText
                label="Default"
                hint="Replaces the items. Empty keeps Day, Week, and Month."
                value=default_slot
            />
        }
    });
    let classes = class_panel(vec![ClassField {
        label: "Label class",
        hint: "Extra class on item labels.",
        value: label_class,
        off: None,
    }]);
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "aria_label", &aria_label.get());
        push_str(&mut props, "orientation", orientation.get());
        if !size.get().is_empty() {
            push_str(&mut props, "size", size.get());
        }
        if !activation.get().is_empty() {
            push_str(&mut props, "activation_mode", activation.get());
        }
        push_bool(&mut props, "disabled", disabled.get());
        if !show_divider.get() {
            push_expr(&mut props, "show_divider", "false");
        }
        push_bool(&mut props, "borderless", borderless.get());
        if !spread.get().is_empty() {
            push_str(&mut props, "spread", spread.get());
        }
        push_str(&mut props, "label_class", &label_class.get());
        if !show_transition.get() {
            push_expr(&mut props, "show_indicator_transition", "false");
        }
        push_str(&mut props, "id", &id.get());
        push_expr(&mut props, "value", "value");
        push_expr(&mut props, "on_change", "move |next| value.set(next)");
        let body = if default_slot.get().is_empty() {
            sample_items()
        } else {
            element(
                "DtSegmentedControlItem",
                &[("value", rust_string("custom"))],
                Some(&rust_string(&default_slot.get())),
            )
        };
        view_of(&element("DtSegmentedControl", &props, Some(&body)))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"The indicator does not animate until segmented CSS is vendored."</p>
            <Play preview=preview settings=settings slots=slots classes=classes code=code />
        </section>
    }
}
