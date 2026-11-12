use super::*;

fn slider_literal(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.1}")
    } else {
        format!("{value}")
    }
}

fn readout_text(value: DtSliderValue) -> String {
    match value {
        DtSliderValue::Single(value) => format!("{value:.0}"),
        DtSliderValue::Range(start, end) => format!("{start:.0}–{end:.0}"),
    }
}

#[component]
pub(crate) fn SliderPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("slider.rs")));
    let volume = RwSignal::new(DtSliderValue::Single(40.0));
    let label = RwSignal::new("Volume".to_string());
    let orientation = RwSignal::new("horizontal");
    let readout = RwSignal::new("tooltip");
    let fill = RwSignal::new("start");
    let disabled = RwSignal::new(false);
    let show_ticks = RwSignal::new(false);
    let show_marks = RwSignal::new(false);
    let prefix = RwSignal::new(String::new());
    let suffix = RwSignal::new(String::new());
    let root_class = RwSignal::new(String::new());
    let range = RwSignal::new(DtSliderValue::Range(20.0, 80.0));
    let vertical = RwSignal::new(DtSliderValue::Single(30.0));
    let stepped = RwSignal::new(DtSliderValue::Single(20.0));
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let label = label.get();
                let orientation = orientation.get();
                let readout = match readout.get() {
                    "always" => DtSliderReadout::Always,
                    "never" => DtSliderReadout::Never,
                    _ => DtSliderReadout::Tooltip,
                };
                let fill = match fill.get() {
                    "end" => DtSliderFill::End,
                    "at" => DtSliderFill::At(50.0),
                    _ => DtSliderFill::Start,
                };
                let ticks = if show_ticks.get() {
                    DtSliderTicks::EveryStep
                } else {
                    DtSliderTicks::Off
                };
                let marks = if show_marks.get() {
                    DtSliderMarks::Ends
                } else {
                    DtSliderMarks::Off
                };
                let style = if orientation == "vertical" {
                    "block-size: 160px"
                } else {
                    "inline-size: 16rem"
                };
                let prefix = prefix.get();
                let suffix = suffix.get();
                let class_name = root_class.get();
                let off = disabled.get();
                view! {
                    <div class="docs-inline">
                        <DtSlider
                            label=label
                            value=volume
                            on_input=Callback::new(move |next| volume.set(next))
                            orientation=orientation
                            readout=readout
                            fill=fill
                            ticks=ticks
                            marks=marks
                            disabled=off
                            prefix=prefix
                            suffix=suffix
                            class=class_name
                            style=style
                        />
                        <span class="docs-readout">{move || readout_text(volume.get())}</span>
                    </div>
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Label" hint="Visible name. An empty label warns." value=label />
            <SettingSelect
                label="Orientation"
                value=orientation
                options=vec![("horizontal", "Horizontal"), ("vertical", "Vertical")]
            />
            <SettingSelect
                label="Readout"
                value=readout
                options=vec![("tooltip", "Tooltip"), ("always", "Always"), ("never", "Never")]
            />
            <SettingSelect
                label="Fill"
                value=fill
                options=vec![("start", "Start"), ("end", "End"), ("at", "At 50")]
            />
            <SettingCheck label="Ticks" value=show_ticks />
            <SettingCheck label="End marks" value=show_marks />
            <SettingCheck label="Disabled" value=disabled />
            <SettingText label="Prefix" value=prefix />
            <SettingText label="Suffix" value=suffix />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "label", &label.get());
        push_expr(&mut props, "value", "volume");
        push_expr(
            &mut props,
            "on_input",
            "Callback::new(move |next| volume.set(next))",
        );
        if orientation.get() != "horizontal" {
            push_str(&mut props, "orientation", orientation.get());
            push_str(&mut props, "style", "block-size: 160px");
        }
        match readout.get() {
            "always" => push_expr(&mut props, "readout", "DtSliderReadout::Always"),
            "never" => push_expr(&mut props, "readout", "DtSliderReadout::Never"),
            _ => {}
        }
        match fill.get() {
            "end" => push_expr(&mut props, "fill", "DtSliderFill::End"),
            "at" => push_expr(&mut props, "fill", "DtSliderFill::At(50.0)"),
            _ => {}
        }
        if show_ticks.get() {
            push_expr(&mut props, "ticks", "DtSliderTicks::EveryStep");
        }
        if show_marks.get() {
            push_expr(&mut props, "marks", "DtSliderMarks::Ends");
        }
        if disabled.get() {
            push_flag(&mut props, "disabled", true);
        }
        push_str(&mut props, "prefix", &prefix.get());
        push_str(&mut props, "suffix", &suffix.get());
        push_str(&mut props, "class", &root_class.get());
        let current = match volume.get() {
            DtSliderValue::Single(value) => slider_literal(value),
            DtSliderValue::Range(start, _) => slider_literal(start),
        };
        format!(
            "let volume = RwSignal::new(DtSliderValue::Single({current}));\n{}",
            element("DtSlider", &props, None)
        )
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
                "The value is controlled. on_input reports every change, and the parent writes the signal. Fill End emits d-slider--inverted. That rule is empty in the vendored CSS. A vertical slider needs a block size."
            </p>
            <Play preview=preview settings=settings classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Range">
                    <DtSlider
                        label="Price"
                        prefix="$"
                        min_gap_steps=5.0
                        value=range
                        on_input=Callback::new(move |next| range.set(next))
                        get_value_text=|(value, index): (f64, usize)| {
                            format!("Thumb {}: {value:.0}", index + 1)
                        }
                        style="inline-size: 16rem"
                    />
                </Variant>
                <Variant title="Vertical">
                    <DtSlider
                        label="Level"
                        orientation="vertical"
                        value=vertical
                        on_input=Callback::new(move |next| vertical.set(next))
                        style="block-size: 160px"
                    />
                </Variant>
                <Variant title="Ticks and marks">
                    <DtSlider
                        label="Steps"
                        step=10.0
                        ticks=DtSliderTicks::EveryStep
                        marks=DtSliderMarks::Ends
                        value=stepped
                        on_input=Callback::new(move |next| stepped.set(next))
                        style="inline-size: 16rem"
                    />
                </Variant>
            </div>
        </section>
    }
}
