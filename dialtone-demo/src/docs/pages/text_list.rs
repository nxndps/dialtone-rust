use super::*;

const MARKERS: &[(&str, &str)] = &[
    ("", "Default"),
    ("disc", "Disc"),
    ("circle", "Circle"),
    ("square", "Square"),
    ("decimal", "Decimal"),
    ("lower-alpha", "Lower alpha"),
    ("upper-alpha", "Upper alpha"),
    ("lower-roman", "Lower roman"),
    ("upper-roman", "Upper roman"),
    ("none", "None"),
];

const TONES: &[(&str, &str)] = &[
    ("", "Default"),
    ("primary", "Primary"),
    ("secondary", "Secondary"),
    ("tertiary", "Tertiary"),
    ("muted", "Muted"),
    ("disabled", "Disabled"),
    ("placeholder", "Placeholder"),
    ("critical", "Critical"),
    ("critical-strong", "Critical strong"),
    ("positive", "Positive"),
    ("positive-strong", "Positive strong"),
    ("warning", "Warning"),
    ("info", "Info"),
];

const GAPS: &[(&str, &str)] = &[
    ("", "Default"),
    ("0", "0"),
    ("1", "1"),
    ("25", "25"),
    ("50", "50"),
    ("75", "75"),
    ("100", "100"),
    ("125", "125"),
    ("150", "150"),
    ("175", "175"),
    ("200", "200"),
    ("250", "250"),
    ("300", "300"),
];

fn list_start(value: &str) -> Option<i32> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        trimmed.parse().ok()
    }
}

fn sample_items() -> String {
    ["Voice", "Meetings", "SMS"]
        .into_iter()
        .map(|label| element("DtTextListItem", &[], Some(&rust_string(label))))
        .collect::<Vec<_>>()
        .join("\n")
}

#[component]
pub(crate) fn TextListPage() -> impl IntoView {
    let kind = RwSignal::new("unordered");
    let marker = RwSignal::new("");
    let marker_tone = RwSignal::new("");
    let gap = RwSignal::new("");
    let start = RwSignal::new(String::new());
    let reversed = RwSignal::new(false);
    let default_slot = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let kind = kind.get();
                let marker = marker.get();
                let marker_tone = marker_tone.get();
                let gap = gap.get();
                let reversed = reversed.get();
                let custom = default_slot.get();
                let start = list_start(&start.get());
                let custom_empty = custom.is_empty();
                match (start, custom_empty) {
                    (Some(start), true) => view! {
                        <DtTextList
                            r#type=kind
                            marker=marker
                            marker_tone=marker_tone
                            gap=gap
                            start=start
                            reversed=reversed
                        >
                            <DtTextListItem>"Voice"</DtTextListItem>
                            <DtTextListItem>"Meetings"</DtTextListItem>
                            <DtTextListItem>"SMS"</DtTextListItem>
                        </DtTextList>
                    }
                    .into_any(),
                    (None, true) => view! {
                        <DtTextList
                            r#type=kind
                            marker=marker
                            marker_tone=marker_tone
                            gap=gap
                            reversed=reversed
                        >
                            <DtTextListItem>"Voice"</DtTextListItem>
                            <DtTextListItem>"Meetings"</DtTextListItem>
                            <DtTextListItem>"SMS"</DtTextListItem>
                        </DtTextList>
                    }
                    .into_any(),
                    (Some(start), false) => view! {
                        <DtTextList
                            r#type=kind
                            marker=marker
                            marker_tone=marker_tone
                            gap=gap
                            start=start
                            reversed=reversed
                        >
                            <DtTextListItem>{custom}</DtTextListItem>
                        </DtTextList>
                    }
                    .into_any(),
                    (None, false) => view! {
                        <DtTextList
                            r#type=kind
                            marker=marker
                            marker_tone=marker_tone
                            gap=gap
                            reversed=reversed
                        >
                            <DtTextListItem>{custom}</DtTextListItem>
                        </DtTextList>
                    }
                    .into_any(),
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingSelect
                label="Type"
                value=kind
                options=vec![("unordered", "Unordered"), ("ordered", "Ordered")]
            />
            <SettingSelect label="Marker" value=marker options=MARKERS.to_vec() />
            <SettingSelect label="Marker tone" value=marker_tone options=TONES.to_vec() />
            <SettingSelect label="Gap" value=gap options=GAPS.to_vec() />
            <SettingText label="Start" hint="Ordered lists only." value=start />
            <SettingCheck label="Reversed" value=reversed />
        }
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingText
                label="Default"
                hint="Replaces the sample items. Empty keeps Voice, Meetings, and SMS."
                value=default_slot
            />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "type", kind.get());
        if !marker.get().is_empty() {
            push_str(&mut props, "marker", marker.get());
        }
        if !marker_tone.get().is_empty() {
            push_str(&mut props, "marker_tone", marker_tone.get());
        }
        if !gap.get().is_empty() {
            push_str(&mut props, "gap", gap.get());
        }
        if let Some(start) = list_start(&start.get()) {
            push_expr(&mut props, "start", start.to_string());
        }
        push_bool(&mut props, "reversed", reversed.get());
        let body = if default_slot.get().is_empty() {
            sample_items()
        } else {
            element(
                "DtTextListItem",
                &[],
                Some(&rust_string(&default_slot.get())),
            )
        };
        view_of(&element("DtTextList", &props, Some(&body)))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <Play preview=preview settings=settings slots=slots code=code />
        </section>
    }
}
