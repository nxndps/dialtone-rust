use super::*;

fn progress_literal(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{}", value as i64)
    } else {
        format!("{value}")
    }
}

/// Docs page for `DtProgressCircle` at `dialtone/v10.7.0`.
#[component]
pub(crate) fn ProgressCirclePage() -> impl IntoView {
    provide_context(VariantSource(include_str!("progress_circle.rs")));
    let progress = RwSignal::new(66.0);
    let size = RwSignal::new("500");
    let kind = RwSignal::new("default");
    let label = RwSignal::new("Upload progress".to_string());
    let root_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let size = size.get();
                let kind = kind.get();
                let label = label.get();
                let class_name = root_class.get();
                view! {
                    <DtProgressCircle
                        aria_label=label
                        progress=progress
                        size=size
                        kind=kind
                        class=class_name
                    />
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <label class="docs-field">
                <span>{move || format!("Progress ({:.0}%)", progress.get())}</span>
                <input
                    type="range"
                    min="0"
                    max="100"
                    step="1"
                    prop:value=move || format!("{:.0}", progress.get())
                    on:input=move |event| {
                        if let Ok(next) = event_target_value(&event).parse::<f64>() {
                            progress.set(next);
                        }
                    }
                />
            </label>
            <SettingSelect label="Size" value=size options=ICON_SIZE.to_vec() />
            <SettingSelect
                label="Kind"
                value=kind
                options=vec![
                    ("default", "Default"),
                    ("brand", "Brand"),
                    ("critical", "Critical"),
                    ("positive", "Positive"),
                    ("warning", "Warning"),
                    ("info", "Info"),
                    ("ai", "AI"),
                ]
            />
            <SettingText label="Accessible name" hint="ariaLabel. Required." value=label />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "aria_label", &label.get());
        push_expr(&mut props, "progress", progress_literal(progress.get()));
        push_str(&mut props, "size", size.get());
        if kind.get() != "default" {
            push_str(&mut props, "kind", kind.get());
        }
        push_str(&mut props, "class", &root_class.get());
        view_of(&element("DtProgressCircle", &props, None))
    });
    let classes = class_panel(vec![ClassField {
        label: "Class",
        hint: ROOT_CLASS,
        value: root_class,
        off: None,
    }]);
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">
                "Values from 1 to 4 draw the same arc as 5. The accessible value stays the real number. Kind ai uses an SVG gradient."
            </p>
            <Play preview=preview settings=settings classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Progress" row=true>
                    <DtProgressCircle aria_label="0% complete" progress=0.0 />
                    <DtProgressCircle aria_label="25% complete" progress=25.0 />
                    <DtProgressCircle aria_label="50% complete" progress=50.0 />
                    <DtProgressCircle aria_label="75% complete" progress=75.0 />
                    <DtProgressCircle aria_label="100% complete" progress=100.0 />
                </Variant>
                <Variant title="Sizes" row=true>
                    <DtProgressCircle aria_label="size 100" size="100" progress=66.0 />
                    <DtProgressCircle aria_label="size 200" size="200" progress=66.0 />
                    <DtProgressCircle aria_label="size 300" size="300" progress=66.0 />
                    <DtProgressCircle aria_label="size 400" size="400" progress=66.0 />
                    <DtProgressCircle aria_label="size 500" size="500" progress=66.0 />
                    <DtProgressCircle aria_label="size 600" size="600" progress=66.0 />
                    <DtProgressCircle aria_label="size 700" size="700" progress=66.0 />
                    <DtProgressCircle aria_label="size 800" size="800" progress=66.0 />
                </Variant>
                <Variant title="Kinds" row=true>
                    <DtProgressCircle aria_label="kind default" progress=66.0 />
                    <DtProgressCircle aria_label="kind brand" kind="brand" progress=66.0 />
                    <DtProgressCircle aria_label="kind critical" kind="critical" progress=66.0 />
                    <DtProgressCircle aria_label="kind positive" kind="positive" progress=66.0 />
                    <DtProgressCircle aria_label="kind warning" kind="warning" progress=66.0 />
                    <DtProgressCircle aria_label="kind info" kind="info" progress=66.0 />
                    <DtProgressCircle aria_label="kind ai" kind="ai" progress=66.0 />
                </Variant>
            </div>
        </section>
    }
}
