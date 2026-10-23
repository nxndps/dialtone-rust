use std::sync::Mutex;

use super::*;

fn intern(value: &str) -> &'static str {
    static CACHE: Mutex<Vec<(String, &'static str)>> = Mutex::new(Vec::new());
    let mut cache = CACHE.lock().expect("tooltip intern");
    if let Some((_, leaked)) = cache.iter().find(|(key, _)| key == value) {
        return leaked;
    }
    let leaked: &'static str = Box::leak(value.to_string().into_boxed_str());
    cache.push((value.to_string(), leaked));
    leaked
}

fn offset_literal(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{}.0", value as i64)
    } else {
        value.to_string()
    }
}

#[component]
pub(crate) fn TooltipPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("tooltip.rs")));
    let id = RwSignal::new(String::new());
    let message = RwSignal::new("Copies the schedule id".to_string());
    let placement = RwSignal::new("top");
    let fallbacks = RwSignal::new(String::new());
    let offset_text = RwSignal::new(String::new());
    let inverted = RwSignal::new(false);
    let sticky = RwSignal::new(true);
    let delay = RwSignal::new(true);
    let transition = RwSignal::new(true);
    let enabled = RwSignal::new(true);
    let rich = RwSignal::new(String::new());
    let content_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let id = id.get();
                let fallback_raw = fallbacks.get();
                let fallbacks = if fallback_raw.trim().is_empty() {
                    "auto"
                } else {
                    intern(fallback_raw.trim())
                };
                let offset = offset_text
                    .get()
                    .trim()
                    .parse::<f64>()
                    .ok()
                    .filter(|value| value.is_finite())
                    .unwrap_or(12.0);
                let content_class_value = content_class.get();
                open_one!(DtTooltip {
                    id=id
                    message=message.get()
                    placement=placement.get()
                    fallback_placements=fallbacks
                    offset=offset
                    inverted=inverted.get()
                    sticky=sticky.get()
                    delay=delay.get()
                    transition=transition.get()
                    enabled=enabled.get()
                    content_class=content_class_value
                } {
                    <DtButton importance="outlined">"Hover"</DtButton>
                }, text_slot(rich.get()), content)
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Id" value=id />
            <SettingText label="Message" value=message />
            <SettingSelect label="Placement" value=placement options=PLACEMENTS.to_vec() />
            <SettingText
                label="Fallback placements"
                hint="Comma-separated. Empty uses auto."
                value=fallbacks
            />
            <SettingText label="Offset" hint="Distance in px. Empty uses 12." value=offset_text />
            <SettingCheck label="Inverted (Deprecated)" value=inverted />
            <SettingCheck label="Sticky" value=sticky />
            <SettingCheck label="Delay" value=delay />
            <SettingCheck label="Transition" value=transition />
            <SettingCheck label="Enabled" value=enabled />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "id", &id.get());
        push_str(&mut props, "message", &message.get());
        push_str(&mut props, "placement", placement.get());
        let fallback = fallbacks.get();
        let fallback = fallback.trim();
        if !fallback.is_empty() && fallback != "auto" {
            push_str(&mut props, "fallback_placements", fallback);
        }
        if let Some(offset) = offset_text
            .get()
            .trim()
            .parse::<f64>()
            .ok()
            .filter(|value| value.is_finite())
        {
            if offset != 12.0 {
                push_expr(&mut props, "offset", offset_literal(offset));
            }
        }
        push_bool(&mut props, "inverted", inverted.get());
        if !sticky.get() {
            push_expr(&mut props, "sticky", "false");
        }
        if !delay.get() {
            push_expr(&mut props, "delay", "false");
        }
        if !transition.get() {
            push_expr(&mut props, "transition", "false");
        }
        push_flag(&mut props, "enabled", enabled.get());
        push_str(&mut props, "content_class", &content_class.get());
        if !rich.get().is_empty() {
            push_expr(
                &mut props,
                "content",
                format!(
                    "ViewFn::from(|| view! {{ <span>{}</span> }})",
                    rust_string(&rich.get())
                ),
            );
        }
        let child = element(
            "DtButton",
            &[("importance", rust_string("outlined"))],
            Some(&rust_string("Hover")),
        );
        view_of(&element("DtTooltip", &props, Some(&child)))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <Play
                preview=preview
                settings=settings
                slots=ViewFn::from(move || view! {
                    <SettingText label="Content" hint="Rich tooltip body. Replaces the message." value=rich />
                })
                classes=class_panel(vec![ClassField {
                    label: "Content class",
                    hint: "Extra class on the tooltip.",
                    value: content_class,
                    off: None,
                }])
                code=code
            />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Placements" row=true>
                    <DtTooltip message="Above" placement="top">
                        <DtButton importance="outlined" size="sm">"Top"</DtButton>
                    </DtTooltip>
                    <DtTooltip message="Below" placement="bottom">
                        <DtButton importance="outlined" size="sm">"Bottom"</DtButton>
                    </DtTooltip>
                    <DtTooltip message="Beside" placement="right">
                        <DtButton importance="clear" size="sm">"Right"</DtButton>
                    </DtTooltip>
                </Variant>
            </div>
        </section>
    }
}
