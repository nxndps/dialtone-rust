use super::*;

const AS: &[(&str, &str)] = &[
    ("", "Default"),
    ("span", "span"),
    ("p", "p"),
    ("h1", "h1"),
    ("h2", "h2"),
    ("h3", "h3"),
    ("h4", "h4"),
    ("h5", "h5"),
    ("h6", "h6"),
    ("label", "label"),
    ("div", "div"),
    ("component", "component"),
];

const KINDS: &[(&str, &str)] = &[
    ("", "Default"),
    ("headline", "Headline"),
    ("body", "Body"),
    ("label", "Label"),
    ("code", "Code"),
];

const SIZES: &[(&str, &str)] = &[
    ("", "Default"),
    ("50", "50"),
    ("75", "75"),
    ("100", "100"),
    ("125", "125"),
    ("150", "150"),
    ("200", "200"),
    ("250", "250"),
    ("300", "300"),
    ("350", "350"),
    ("400", "400"),
    ("450", "450"),
    ("500", "500"),
];

const FAMILIES: &[(&str, &str)] = &[
    ("", "Default"),
    ("inherit", "Inherit"),
    ("custom", "Custom"),
    ("sans", "Sans"),
    ("mono", "Mono"),
    ("expressive", "Expressive"),
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

const ALIGNS: &[(&str, &str)] = &[
    ("", "Default"),
    ("start", "Start"),
    ("center", "Center"),
    ("end", "End"),
    ("justify", "Justify"),
];

const WRAPS: &[(&str, &str)] = &[
    ("", "Default"),
    ("wrap", "Wrap"),
    ("nowrap", "Nowrap"),
    ("balance", "Balance"),
    ("pretty", "Pretty"),
];

const TRIMS: &[(&str, &str)] = &[
    ("", "Default"),
    ("start", "Start"),
    ("end", "End"),
    ("both", "Both"),
];

const STRENGTHS: &[(&str, &str)] = &[
    ("", "Default"),
    ("bold", "Bold"),
    ("semibold", "Semibold"),
    ("medium", "Medium"),
    ("normal", "Normal"),
];

const DENSITIES: &[(&str, &str)] = &[
    ("", "Default"),
    ("100", "100"),
    ("200", "200"),
    ("300", "300"),
    ("400", "400"),
    ("500", "500"),
    ("600", "600"),
];

fn line_count(value: &str) -> u32 {
    value.trim().parse::<u32>().unwrap_or(0)
}

#[component]
pub(crate) fn TextPage() -> impl IntoView {
    let tag = RwSignal::new("");
    let kind = RwSignal::new("");
    let variant = RwSignal::new("body-md");
    let size = RwSignal::new("");
    let family = RwSignal::new("");
    let italic = RwSignal::new(false);
    let tone = RwSignal::new("");
    let align = RwSignal::new("");
    let truncate = RwSignal::new(false);
    let max_lines = RwSignal::new(String::new());
    let numeric = RwSignal::new(false);
    let wrap = RwSignal::new("");
    let text_box_trim = RwSignal::new("");
    let strength = RwSignal::new("");
    let density = RwSignal::new("");
    let copy = RwSignal::new("The primary rotation changed this morning.".to_string());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let tag = if tag.get().is_empty() { "span" } else { tag.get() };
                let kind = kind.get();
                let variant = variant.get();
                let size = size.get();
                let family = family.get();
                let italic = italic.get();
                let tone = tone.get();
                let align = align.get();
                let truncate = truncate.get();
                let lines = line_count(&max_lines.get());
                let numeric = numeric.get();
                let wrap = wrap.get();
                let trim = text_box_trim.get();
                let strength = strength.get();
                let density = density.get();
                let copy = copy.get();
                view! {
                    <DtText
                        r#as=tag
                        kind=kind
                        variant=variant
                        size=size
                        family=family
                        italic=italic
                        tone=tone
                        align=align
                        truncate=truncate
                        max_lines=lines
                        numeric=numeric
                        wrap=wrap
                        text_box_trim=trim
                        strength=strength
                        density=density
                    >
                        {copy}
                    </DtText>
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingSelect label="As" value=tag options=AS.to_vec() />
            <SettingSelect label="Kind (Deprecated)" value=kind options=KINDS.to_vec() />
            <SettingSelect
                label="Variant"
                value=variant
                options=vec![
                    ("", "Default"),
                    ("headline-lg", "Headline lg"),
                    ("body-md", "Body md"),
                    ("label-sm", "Label sm"),
                ]
            />
            <SettingSelect label="Size" value=size options=SIZES.to_vec() />
            <SettingSelect label="Family" value=family options=FAMILIES.to_vec() />
            <SettingCheck label="Italic" value=italic />
            <SettingSelect label="Tone" value=tone options=TONES.to_vec() />
            <SettingSelect label="Align" value=align options=ALIGNS.to_vec() />
            <SettingCheck label="Truncate" value=truncate />
            <SettingText label="Max lines" hint="Line clamp. Empty is unset." value=max_lines />
            <SettingCheck label="Numeric" value=numeric />
            <SettingSelect label="Wrap" value=wrap options=WRAPS.to_vec() />
            <SettingSelect label="Text box trim" value=text_box_trim options=TRIMS.to_vec() />
            <SettingSelect label="Strength" value=strength options=STRENGTHS.to_vec() />
            <SettingSelect label="Density" value=density options=DENSITIES.to_vec() />
            <SettingText label="Text" value=copy />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        if !tag.get().is_empty() {
            push_str(&mut props, "r#as", tag.get());
        }
        if !kind.get().is_empty() {
            push_str(&mut props, "kind", kind.get());
        }
        if !variant.get().is_empty() {
            push_str(&mut props, "variant", variant.get());
        }
        if !size.get().is_empty() {
            push_str(&mut props, "size", size.get());
        }
        if !family.get().is_empty() {
            push_str(&mut props, "family", family.get());
        }
        push_bool(&mut props, "italic", italic.get());
        if !tone.get().is_empty() {
            push_str(&mut props, "tone", tone.get());
        }
        if !align.get().is_empty() {
            push_str(&mut props, "align", align.get());
        }
        push_bool(&mut props, "truncate", truncate.get());
        let lines = line_count(&max_lines.get());
        if lines > 0 {
            push_expr(&mut props, "max_lines", lines.to_string());
        }
        push_bool(&mut props, "numeric", numeric.get());
        if !wrap.get().is_empty() {
            push_str(&mut props, "wrap", wrap.get());
        }
        if !text_box_trim.get().is_empty() {
            push_str(&mut props, "text_box_trim", text_box_trim.get());
        }
        if !strength.get().is_empty() {
            push_str(&mut props, "strength", strength.get());
        }
        if !density.get().is_empty() {
            push_str(&mut props, "density", density.get());
        }
        view_of(&element("DtText", &props, Some(&rust_string(&copy.get()))))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <Play stack=true preview=preview settings=settings code=code />
        </section>
    }
}
