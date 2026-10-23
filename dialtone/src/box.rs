use leptos::prelude::*;

/// Spacing tokens for padding (`DT_BOX_SPACING_VALUES`).
const SPACING: &[&str] = &[
    "0", "1", "25", "50", "75", "100", "125", "150", "175", "200", "250", "300", "350", "400",
    "450", "500", "525", "550", "600", "650", "700", "750", "800",
];

const SURFACES: &[&str] = &[
    "primary",
    "secondary",
    "moderate",
    "bold",
    "strong",
    "contrast",
    "backdrop",
    "overlay",
    "brand",
    "info",
    "positive",
    "warning",
    "critical",
    "brand-subtle",
    "brand-strong",
    "info-subtle",
    "info-strong",
    "positive-subtle",
    "positive-strong",
    "warning-subtle",
    "warning-strong",
    "critical-subtle",
    "critical-strong",
    "primary-opaque",
    "secondary-opaque",
    "moderate-opaque",
    "bold-opaque",
    "strong-opaque",
    "contrast-opaque",
    "brand-opaque",
    "brand-subtle-opaque",
    "info-opaque",
    "info-subtle-opaque",
    "positive-opaque",
    "positive-subtle-opaque",
    "warning-opaque",
    "warning-subtle-opaque",
    "critical-opaque",
    "critical-subtle-opaque",
];

const BORDER_COLORS: &[&str] = &[
    "transparent",
    "subtle",
    "default",
    "moderate",
    "bold",
    "accent",
    "focus",
    "brand",
    "positive",
    "warning",
    "critical",
    "info",
    "brand-subtle",
    "brand-strong",
    "positive-subtle",
    "positive-strong",
    "warning-subtle",
    "warning-strong",
    "critical-subtle",
    "critical-strong",
    "info-subtle",
    "info-strong",
];

const BORDER_WIDTHS: &[&str] = &["0", "50", "100", "150", "200", "300", "400"];

const RADII: &[&str] = &[
    "0", "100", "200", "300", "350", "400", "450", "500", "550", "600", "pill", "circle",
];

const SHADOWS: &[&str] = &["small", "medium", "large", "extra-large", "card"];

const POSITIONS: &[&str] = &["static", "relative", "absolute", "fixed", "sticky"];

const Z_INDEXES: &[&str] = &[
    "hide",
    "base",
    "base1",
    "selected",
    "active",
    "navigation",
    "navigation-fixed",
    "dropdown",
    "popover",
    "tooltip",
    "drawer",
    "modal",
    "modal-element",
    "notification",
];

const LAYOUT: &[&str] = &[
    "0", "1px", "2px", "8px", "25", "20px", "24px", "50", "75", "100", "125", "150", "175", "200",
    "250", "300", "350", "400", "450", "500", "550", "600", "650", "700", "750", "800", "850",
    "900", "950", "1000", "1050", "1100", "1150", "1200", "1250", "1300", "1350", "1400", "1450",
    "1500", "1550", "1600", "10p", "20p", "25p", "30p", "33p", "40p", "50p", "60p", "66p", "70p",
    "75p", "80p", "90p", "95p", "100p",
];

const OVERFLOW: &[&str] = &["hidden", "scroll", "auto", "clip", "visible"];

const SCROLLBAR: &[&str] = &["leave", "scroll", "move", "always"];

const AS: &[&str] = &[
    "div", "span", "section", "article", "aside", "main", "header", "footer", "nav", "ul", "ol",
    "li", "fieldset", "form", "figure",
];

const SCROLLBAR_INCOMPATIBLE: &[&str] = &["ul", "ol", "span", "fieldset"];

const INSET_PERCENTS: &[&str] = &["50p", "100p", "n50p", "n100p"];

/// Class inputs in Vue `boxClasses` order. Empty strings omit that modifier.
#[derive(Clone, Copy)]
pub(crate) struct BoxStyle<'a> {
    pub padding: &'a str,
    pub padding_inline: &'a str,
    pub padding_inline_start: &'a str,
    pub padding_inline_end: &'a str,
    pub padding_block: &'a str,
    pub padding_block_start: &'a str,
    pub padding_block_end: &'a str,
    pub surface: &'a str,
    pub border_color: &'a str,
    pub border_width: &'a str,
    pub border_width_inline: &'a str,
    pub border_width_inline_start: &'a str,
    pub border_width_inline_end: &'a str,
    pub border_width_block: &'a str,
    pub border_width_block_start: &'a str,
    pub border_width_block_end: &'a str,
    pub border_radius: &'a str,
    pub shadow: &'a str,
    pub overflow: &'a str,
    pub inline_size: &'a str,
    pub block_size: &'a str,
    pub min_inline_size: &'a str,
    pub max_inline_size: &'a str,
    pub min_block_size: &'a str,
    pub max_block_size: &'a str,
    pub position: &'a str,
    pub inset: &'a str,
    pub inset_block: &'a str,
    pub inset_inline: &'a str,
    pub inset_block_start: &'a str,
    pub inset_block_end: &'a str,
    pub inset_inline_start: &'a str,
    pub inset_inline_end: &'a str,
    pub z_index: &'a str,
    pub extra_class: &'a str,
}

fn is_spacing(value: &str) -> bool {
    SPACING.contains(&value)
}

fn is_negative_spacing(value: &str) -> bool {
    let Some(rest) = value.strip_prefix('n') else {
        return false;
    };
    rest != "0" && is_spacing(rest)
}

fn is_inset(value: &str) -> bool {
    is_spacing(value) || is_negative_spacing(value)
}

fn is_inset_side(value: &str) -> bool {
    is_inset(value) || INSET_PERCENTS.contains(&value)
}

fn listed(allowed: &[&str]) -> String {
    allowed.join(", ")
}

fn inset_list(side: bool) -> String {
    let values: Vec<&str> = SPACING.to_vec();
    let negatives: Vec<String> = SPACING
        .iter()
        .filter(|value| **value != "0")
        .map(|value| format!("n{value}"))
        .collect();
    let mut owned: Vec<&str> = values;
    let negative_refs: Vec<&str> = negatives.iter().map(String::as_str).collect();
    owned.extend(negative_refs.iter().copied());
    if side {
        owned.extend(INSET_PERCENTS);
    }
    owned.join(", ")
}

fn warn_value(value: &str, valid: &str) {
    leptos::logging::warn!("[DtBox] Invalid prop value: \"{value}\". Valid values: {valid}");
}

fn push_listed(out: &mut Vec<String>, prefix: &str, value: &str, allowed: &[&str]) {
    if value.is_empty() {
        return;
    }
    if !allowed.contains(&value) {
        warn_value(value, &listed(allowed));
        return;
    }
    out.push(format!("{prefix}-{value}"));
}

fn push_inset(out: &mut Vec<String>, prefix: &str, value: &str, side: bool) {
    if value.is_empty() {
        return;
    }
    let ok = if side {
        is_inset_side(value)
    } else {
        is_inset(value)
    };
    if !ok {
        warn_value(value, &inset_list(side));
        return;
    }
    out.push(format!("{prefix}-{value}"));
}

/// Vue `boxClasses`, with an invalid token omitted after the Vue warning.
pub(crate) fn box_classes(style: &BoxStyle<'_>) -> String {
    let mut out = vec!["d-box".to_string()];
    push_listed(&mut out, "d-box--p", style.padding, SPACING);
    push_listed(&mut out, "d-box--pi", style.padding_inline, SPACING);
    push_listed(&mut out, "d-box--pis", style.padding_inline_start, SPACING);
    push_listed(&mut out, "d-box--pie", style.padding_inline_end, SPACING);
    push_listed(&mut out, "d-box--pbl", style.padding_block, SPACING);
    push_listed(&mut out, "d-box--pbs", style.padding_block_start, SPACING);
    push_listed(&mut out, "d-box--pbe", style.padding_block_end, SPACING);
    push_listed(&mut out, "d-box--surface", style.surface, SURFACES);
    push_listed(&mut out, "d-box--bc", style.border_color, BORDER_COLORS);
    push_listed(&mut out, "d-box--bw", style.border_width, BORDER_WIDTHS);
    push_listed(
        &mut out,
        "d-box--bwi",
        style.border_width_inline,
        BORDER_WIDTHS,
    );
    push_listed(
        &mut out,
        "d-box--bwis",
        style.border_width_inline_start,
        BORDER_WIDTHS,
    );
    push_listed(
        &mut out,
        "d-box--bwie",
        style.border_width_inline_end,
        BORDER_WIDTHS,
    );
    push_listed(
        &mut out,
        "d-box--bwbl",
        style.border_width_block,
        BORDER_WIDTHS,
    );
    push_listed(
        &mut out,
        "d-box--bwbs",
        style.border_width_block_start,
        BORDER_WIDTHS,
    );
    push_listed(
        &mut out,
        "d-box--bwbe",
        style.border_width_block_end,
        BORDER_WIDTHS,
    );
    push_listed(&mut out, "d-box--br", style.border_radius, RADII);
    push_listed(&mut out, "d-box--shadow", style.shadow, SHADOWS);
    push_listed(&mut out, "d-box--of", style.overflow, OVERFLOW);
    push_listed(&mut out, "d-box--is", style.inline_size, LAYOUT);
    push_listed(&mut out, "d-box--bls", style.block_size, LAYOUT);
    push_listed(&mut out, "d-box--min-is", style.min_inline_size, LAYOUT);
    push_listed(&mut out, "d-box--max-is", style.max_inline_size, LAYOUT);
    push_listed(&mut out, "d-box--min-bls", style.min_block_size, LAYOUT);
    push_listed(&mut out, "d-box--max-bls", style.max_block_size, LAYOUT);
    push_listed(&mut out, "d-box--ps", style.position, POSITIONS);
    push_inset(&mut out, "d-box--inset", style.inset, false);
    push_inset(&mut out, "d-box--inset-block", style.inset_block, false);
    push_inset(&mut out, "d-box--inset-inline", style.inset_inline, false);
    push_inset(&mut out, "d-box--ibs", style.inset_block_start, true);
    push_inset(&mut out, "d-box--ibe", style.inset_block_end, true);
    push_inset(&mut out, "d-box--iis", style.inset_inline_start, true);
    push_inset(&mut out, "d-box--iie", style.inset_inline_end, true);
    push_listed(&mut out, "d-box--zi", style.z_index, Z_INDEXES);
    if !style.extra_class.is_empty() {
        out.push(style.extra_class.to_string());
    }
    out.join(" ")
}

/// `scrollbar` `true` is mode `leave`. Empty is off. An invalid value warns and is off.
pub(crate) fn scrollbar_mode(value: &str) -> Option<&'static str> {
    if value.is_empty() {
        return None;
    }
    if value == "true" {
        return Some("leave");
    }
    if let Some(mode) = SCROLLBAR.iter().copied().find(|item| *item == value) {
        return Some(mode);
    }
    let mut valid = vec!["true".to_string()];
    valid.extend(SCROLLBAR.iter().map(|item| (*item).to_string()));
    warn_value(value, &valid.join(", "));
    None
}

pub(crate) fn box_tag(value: &'static str) -> &'static str {
    if AS.contains(&value) {
        return value;
    }
    warn_value(value, &listed(AS));
    "div"
}

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Dialtone Vue `DtBox` (`dialtone/v10.7.0`).
///
/// The root is the `as` element (`div` by default), `data-qa="dt-box"`.
/// `border_color` (`borderColor`) defaults to `default`, so `d-box--bc-default`
/// is present unless that value is invalid. An invalid token warns and is
/// omitted. `scrollbar` `true` uses mode `leave` and inserts
/// `div.d-box__scrollbar-content`. `v-dt-scrollbar` is not ported. The wrapper
/// scrolls with the vendored `overflow: auto` rule. `class`, `id`, and
/// `aria_label` are `$attrs` on the root.
#[component]
pub fn DtBox(
    /// Element (`as`). Default `div`.
    #[prop(optional, default = "div")]
    r#as: &'static str,
    /// `leave`, `scroll`, `move`, `always`, or `true` (`scrollbar`). `true` is `leave`.
    #[prop(optional, default = "")]
    scrollbar: &'static str,
    /// Extra class on the scrollbar content (`scrollbarContentClass`).
    #[prop(optional, into)]
    scrollbar_content_class: String,
    /// Background (`surface`).
    #[prop(optional, default = "")]
    surface: &'static str,
    /// Border color (`borderColor`). Default `default`.
    #[prop(optional, default = "default")]
    border_color: &'static str,
    /// Radius (`borderRadius`).
    #[prop(optional, default = "")]
    border_radius: &'static str,
    /// Width on every side (`borderWidth`).
    #[prop(optional, default = "")]
    border_width: &'static str,
    /// Block-axis width (`borderWidthBlock`).
    #[prop(optional, default = "")]
    border_width_block: &'static str,
    /// Block-end width (`borderWidthBlockEnd`).
    #[prop(optional, default = "")]
    border_width_block_end: &'static str,
    /// Block-start width (`borderWidthBlockStart`).
    #[prop(optional, default = "")]
    border_width_block_start: &'static str,
    /// Inline-axis width (`borderWidthInline`).
    #[prop(optional, default = "")]
    border_width_inline: &'static str,
    /// Inline-end width (`borderWidthInlineEnd`).
    #[prop(optional, default = "")]
    border_width_inline_end: &'static str,
    /// Inline-start width (`borderWidthInlineStart`).
    #[prop(optional, default = "")]
    border_width_inline_start: &'static str,
    /// Padding on every side (`padding`).
    #[prop(optional, default = "")]
    padding: &'static str,
    /// Block-axis padding (`paddingBlock`).
    #[prop(optional, default = "")]
    padding_block: &'static str,
    /// Block-end padding (`paddingBlockEnd`).
    #[prop(optional, default = "")]
    padding_block_end: &'static str,
    /// Block-start padding (`paddingBlockStart`).
    #[prop(optional, default = "")]
    padding_block_start: &'static str,
    /// Inline-axis padding (`paddingInline`).
    #[prop(optional, default = "")]
    padding_inline: &'static str,
    /// Inline-end padding (`paddingInlineEnd`).
    #[prop(optional, default = "")]
    padding_inline_end: &'static str,
    /// Inline-start padding (`paddingInlineStart`).
    #[prop(optional, default = "")]
    padding_inline_start: &'static str,
    /// Block size (`blockSize`).
    #[prop(optional, default = "")]
    block_size: &'static str,
    /// Inline size (`inlineSize`).
    #[prop(optional, default = "")]
    inline_size: &'static str,
    /// Max block size (`maxBlockSize`).
    #[prop(optional, default = "")]
    max_block_size: &'static str,
    /// Min block size (`minBlockSize`).
    #[prop(optional, default = "")]
    min_block_size: &'static str,
    /// Max inline size (`maxInlineSize`).
    #[prop(optional, default = "")]
    max_inline_size: &'static str,
    /// Min inline size (`minInlineSize`).
    #[prop(optional, default = "")]
    min_inline_size: &'static str,
    /// Shadow (`shadow`).
    #[prop(optional, default = "")]
    shadow: &'static str,
    /// CSS position (`position`).
    #[prop(optional, default = "")]
    position: &'static str,
    /// Inset on every side (`inset`).
    #[prop(optional, default = "")]
    inset: &'static str,
    /// Block-axis inset (`insetBlock`).
    #[prop(optional, default = "")]
    inset_block: &'static str,
    /// Block-start inset (`insetBlockStart`).
    #[prop(optional, default = "")]
    inset_block_start: &'static str,
    /// Block-end inset (`insetBlockEnd`).
    #[prop(optional, default = "")]
    inset_block_end: &'static str,
    /// Inline-axis inset (`insetInline`).
    #[prop(optional, default = "")]
    inset_inline: &'static str,
    /// Inline-start inset (`insetInlineStart`).
    #[prop(optional, default = "")]
    inset_inline_start: &'static str,
    /// Inline-end inset (`insetInlineEnd`).
    #[prop(optional, default = "")]
    inset_inline_end: &'static str,
    /// Z-index layer (`zIndex`).
    #[prop(optional, default = "")]
    z_index: &'static str,
    /// Overflow (`overflow`).
    #[prop(optional, default = "")]
    overflow: &'static str,
    /// `$attrs.class` on the root.
    #[prop(optional, into)]
    class: String,
    /// `$attrs.id` on the root. Empty omits it.
    #[prop(optional, into)]
    id: String,
    /// `$attrs['aria-label']`. Empty omits it.
    #[prop(optional, into)]
    aria_label: String,
    /// Default slot.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let tag = box_tag(r#as);
    let mode = scrollbar_mode(scrollbar);
    if mode.is_some() && SCROLLBAR_INCOMPATIBLE.contains(&tag) {
        leptos::logging::warn!(
            "[DtBox] scrollbar prop inserts a <div> wrapper which breaks semantic child structure for <{tag}>. Consider applying scrollbar on a parent DtBox container instead."
        );
    }
    let class = box_classes(&BoxStyle {
        padding,
        padding_inline,
        padding_inline_start,
        padding_inline_end,
        padding_block,
        padding_block_start,
        padding_block_end,
        surface,
        border_color,
        border_width,
        border_width_inline,
        border_width_inline_start,
        border_width_inline_end,
        border_width_block,
        border_width_block_start,
        border_width_block_end,
        border_radius,
        shadow,
        overflow,
        inline_size,
        block_size,
        min_inline_size,
        max_inline_size,
        min_block_size,
        max_block_size,
        position,
        inset,
        inset_block,
        inset_inline,
        inset_block_start,
        inset_block_end,
        inset_inline_start,
        inset_inline_end,
        z_index,
        extra_class: &class,
    });
    let id = (!id.is_empty()).then_some(id);
    let aria_label = (!aria_label.is_empty()).then_some(aria_label);
    let body = if let Some(children) = children {
        children().into_any()
    } else {
        view! {}.into_any()
    };
    let body = if mode.is_some() {
        let content_class =
            join_class(["d-box__scrollbar-content", scrollbar_content_class.as_str()]);
        view! {
            <div data-qa="dt-box-scrollbar-content" class=content_class>
                {body}
            </div>
        }
        .into_any()
    } else {
        body
    };

    macro_rules! tag {
        ($name:ident) => {
            view! {
                <$name class=class.clone() data-qa="dt-box" id=id.clone() aria-label=aria_label.clone()>
                    {body}
                </$name>
            }
            .into_any()
        };
    }

    match tag {
        "span" => tag!(span),
        "section" => tag!(section),
        "article" => tag!(article),
        "aside" => tag!(aside),
        "main" => tag!(main),
        "header" => tag!(header),
        "footer" => tag!(footer),
        "nav" => tag!(nav),
        "ul" => tag!(ul),
        "ol" => tag!(ol),
        "li" => tag!(li),
        "fieldset" => tag!(fieldset),
        "form" => tag!(form),
        "figure" => tag!(figure),
        _ => tag!(div),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BOX_STYLE_DEFAULT: BoxStyle<'static> = BoxStyle {
        padding: "",
        padding_inline: "",
        padding_inline_start: "",
        padding_inline_end: "",
        padding_block: "",
        padding_block_start: "",
        padding_block_end: "",
        surface: "",
        border_color: "default",
        border_width: "",
        border_width_inline: "",
        border_width_inline_start: "",
        border_width_inline_end: "",
        border_width_block: "",
        border_width_block_start: "",
        border_width_block_end: "",
        border_radius: "",
        shadow: "",
        overflow: "",
        inline_size: "",
        block_size: "",
        min_inline_size: "",
        max_inline_size: "",
        min_block_size: "",
        max_block_size: "",
        position: "",
        inset: "",
        inset_block: "",
        inset_inline: "",
        inset_block_start: "",
        inset_block_end: "",
        inset_inline_start: "",
        inset_inline_end: "",
        z_index: "",
        extra_class: "",
    };

    #[test]
    fn default_is_the_base_class_and_the_default_border_color() {
        assert_eq!(box_classes(&BOX_STYLE_DEFAULT), "d-box d-box--bc-default");
    }

    #[test]
    fn classes_follow_the_vue_computed_order() {
        let style = BoxStyle {
            padding: "200",
            padding_inline: "100",
            padding_block_start: "0",
            surface: "primary",
            border_color: "subtle",
            border_width: "100",
            border_radius: "300",
            shadow: "card",
            inline_size: "100p",
            block_size: "500",
            position: "sticky",
            inset_block_start: "50p",
            z_index: "navigation",
            overflow: "hidden",
            extra_class: "d-ps-sticky",
            ..BOX_STYLE_DEFAULT
        };
        assert_eq!(
            box_classes(&style),
            "d-box d-box--p-200 d-box--pi-100 d-box--pbs-0 d-box--surface-primary d-box--bc-subtle d-box--bw-100 d-box--br-300 d-box--shadow-card d-box--of-hidden d-box--is-100p d-box--bls-500 d-box--ps-sticky d-box--ibs-50p d-box--zi-navigation d-ps-sticky"
        );
    }

    #[test]
    fn invalid_layout_token_is_omitted() {
        let style = BoxStyle {
            inline_size: "225",
            ..BOX_STYLE_DEFAULT
        };
        assert_eq!(box_classes(&style), "d-box d-box--bc-default");
    }

    #[test]
    fn shorthand_inset_rejects_a_percentage() {
        let style = BoxStyle {
            inset: "50p",
            inset_inline_end: "n100p",
            ..BOX_STYLE_DEFAULT
        };
        assert_eq!(
            box_classes(&style),
            "d-box d-box--bc-default d-box--iie-n100p"
        );
    }

    #[test]
    fn negative_zero_is_not_an_inset() {
        let style = BoxStyle {
            inset: "n0",
            inset_inline: "n150",
            ..BOX_STYLE_DEFAULT
        };
        assert_eq!(
            box_classes(&style),
            "d-box d-box--bc-default d-box--inset-inline-n150"
        );
    }

    #[test]
    fn scrollbar_true_is_leave_and_invalid_is_off() {
        assert_eq!(scrollbar_mode(""), None);
        assert_eq!(scrollbar_mode("true"), Some("leave"));
        assert_eq!(scrollbar_mode("always"), Some("always"));
        assert_eq!(scrollbar_mode("nope"), None);
    }

    #[test]
    fn unknown_element_falls_back_to_div() {
        assert_eq!(box_tag("section"), "section");
        assert_eq!(box_tag("table"), "div");
    }
}
