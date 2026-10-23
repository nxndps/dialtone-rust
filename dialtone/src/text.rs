use std::sync::atomic::{AtomicBool, Ordering};

use leptos::prelude::*;

const DEFAULT_SIZE: &str = "300";

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

#[derive(Clone, Copy)]
pub(crate) struct TextStyle<'a> {
    pub kind: &'a str,
    pub variant: &'a str,
    pub size: &'a str,
    pub family: &'a str,
    pub italic: bool,
    pub tone: &'a str,
    pub align: &'a str,
    pub truncate: bool,
    pub max_lines: u32,
    pub numeric: bool,
    pub wrap: &'a str,
    pub text_box_trim: &'a str,
    pub strength: &'a str,
    pub density: &'a str,
}

fn warn_text(message: &str) {
    leptos::logging::warn!("[DtText] {message}");
}

fn kind_prefix(kind: &str) -> Option<&'static str> {
    Some(match kind {
        "headline" => "d-text-headline",
        "body" => "d-text-body",
        "label" => "d-text-label",
        "code" => "d-text-code",
        _ => return None,
    })
}

fn variant_class(variant: &str) -> Option<&'static str> {
    Some(match variant {
        "headline-3xl" => "d-text-headline--3xl",
        "headline-2xl" => "d-text-headline--2xl",
        "headline-xl" => "d-text-headline--xl",
        "headline-lg" => "d-text-headline--lg",
        "headline-md" => "d-text-headline--md",
        "headline-sm" => "d-text-headline--sm",
        "headline-xs" => "d-text-headline--xs",
        "body-lg" => "d-text-body--lg",
        "body-md" => "d-text-body--md",
        "body-sm" => "d-text-body--sm",
        "body-xs" => "d-text-body--xs",
        "label-lg" => "d-text-label--lg",
        "label-md" => "d-text-label--md",
        "label-sm" => "d-text-label--sm",
        "label-xs" => "d-text-label--xs",
        "code-lg" => "d-text-code--lg",
        "code-md" => "d-text-code--md",
        "code-sm" => "d-text-code--sm",
        "code-xs" => "d-text-code--xs",
        _ => return None,
    })
}

fn size_allowed(kind: &str, size: &str) -> bool {
    match kind {
        "headline" => matches!(
            size,
            "700"
                | "600"
                | "500"
                | "400"
                | "300"
                | "200"
                | "100"
                | "3xl"
                | "2xl"
                | "xl"
                | "lg"
                | "md"
                | "sm"
                | "xs"
        ),
        "body" | "label" | "code" => {
            matches!(
                size,
                "400" | "300" | "200" | "100" | "lg" | "md" | "sm" | "xs"
            )
        }
        _ => false,
    }
}

fn headline_only_size(size: &str) -> bool {
    matches!(size, "700" | "600" | "500" | "3xl" | "2xl" | "xl")
}

fn size_suffix(size: &str) -> &str {
    match size {
        "100" => "xs",
        "200" => "sm",
        "300" => "md",
        "400" => "lg",
        "500" => "xl",
        "600" => "2xl",
        "700" => "3xl",
        other => other,
    }
}

fn font_size_class(size: &str) -> Option<&'static str> {
    Some(match size {
        "50" => "d-text--fs-50",
        "75" => "d-text--fs-75",
        "100" => "d-text--fs-100",
        "125" => "d-text--fs-125",
        "150" => "d-text--fs-150",
        "200" => "d-text--fs-200",
        "250" => "d-text--fs-250",
        "300" => "d-text--fs-300",
        "350" => "d-text--fs-350",
        "400" => "d-text--fs-400",
        "450" => "d-text--fs-450",
        "500" => "d-text--fs-500",
        "550" => "d-text--fs-550",
        "600" => "d-text--fs-600",
        "650" => "d-text--fs-650",
        "700" => "d-text--fs-700",
        "750" => "d-text--fs-750",
        "800" => "d-text--fs-800",
        _ => return None,
    })
}

fn family_class(family: &str) -> Option<&'static str> {
    Some(match family {
        "inherit" => "d-text--ff-inherit",
        "custom" => "d-text--ff-custom",
        "sans" => "d-text--ff-sans",
        "mono" => "d-text--ff-mono",
        "expressive" => "d-text--ff-expressive",
        _ => return None,
    })
}

pub(crate) fn known_text_tone(tone: &str) -> bool {
    matches!(
        tone,
        "primary"
            | "secondary"
            | "tertiary"
            | "muted"
            | "disabled"
            | "placeholder"
            | "critical"
            | "critical-strong"
            | "positive"
            | "positive-strong"
            | "warning"
            | "info"
            | "info-strong"
            | "neutral-black"
            | "neutral-white"
    )
}

fn tone_class(tone: &str) -> Option<String> {
    known_text_tone(tone).then(|| format!("d-text--tone-{tone}"))
}

fn align_class(align: &str) -> Option<&'static str> {
    Some(match align {
        "start" => "d-text--align-start",
        "center" => "d-text--align-center",
        "end" => "d-text--align-end",
        "justify" => "d-text--align-justify",
        _ => return None,
    })
}

fn wrap_class(wrap: &str) -> Option<&'static str> {
    Some(match wrap {
        "wrap" => "d-text--wrap-wrap",
        "nowrap" => "d-text--wrap-nowrap",
        "balance" => "d-text--wrap-balance",
        "pretty" => "d-text--wrap-pretty",
        _ => return None,
    })
}

fn trim_class(trim: &str) -> Option<&'static str> {
    Some(match trim {
        "start" => "d-text--trim-start",
        "end" => "d-text--trim-end",
        "both" => "d-text--trim-both",
        _ => return None,
    })
}

fn strength_class(strength: &str) -> Option<&'static str> {
    Some(match strength {
        "bold" => "d-text--fw-bold",
        "semibold" => "d-text--fw-semibold",
        "medium" => "d-text--fw-medium",
        "normal" => "d-text--fw-normal",
        _ => return None,
    })
}

fn density_class(density: &str) -> Option<&'static str> {
    Some(match density {
        "100" => "d-text--lh-100",
        "200" => "d-text--lh-200",
        "300" => "d-text--lh-300",
        "400" => "d-text--lh-400",
        "500" => "d-text--lh-500",
        "600" => "d-text--lh-600",
        _ => return None,
    })
}

fn modifier<'a>(
    value: &str,
    prop: &str,
    class: impl Fn(&str) -> Option<&'a str>,
) -> Option<&'a str> {
    if value.is_empty() {
        return None;
    }
    let class_name = class(value);
    if class_name.is_none() {
        warn_text(&format!("Unsupported {prop} \"{value}\"."));
    }
    class_name
}

fn composition_class(style: &TextStyle<'_>) -> Option<String> {
    if !style.variant.is_empty() {
        return match variant_class(style.variant) {
            Some(class_name) => Some(class_name.to_string()),
            None => {
                warn_text(&format!("Unsupported variant \"{}\".", style.variant));
                None
            }
        };
    }
    if style.kind.is_empty() {
        return None;
    }
    let Some(prefix) = kind_prefix(style.kind) else {
        warn_text(&format!("Unsupported kind \"{}\".", style.kind));
        return None;
    };
    let requested = if style.size.is_empty() {
        DEFAULT_SIZE
    } else {
        style.size
    };
    let resolved = if size_allowed(style.kind, requested) {
        requested
    } else {
        // Vue throws when a headline-only size is paired with another kind.
        // This port warns and falls back to the default size instead.
        if headline_only_size(requested) {
            warn_text(&format!(
                "size=\"{requested}\" is only valid for kind=\"headline\". Cannot use with kind=\"{}\". Using \"{DEFAULT_SIZE}\" instead.",
                style.kind
            ));
        } else {
            warn_text(&format!(
                "size=\"{requested}\" is not valid for kind=\"{}\". Using \"{DEFAULT_SIZE}\" instead.",
                style.kind
            ));
        }
        DEFAULT_SIZE
    };
    Some(format!("{prefix}--{}", size_suffix(resolved)))
}

fn font_size_modifier(style: &TextStyle<'_>) -> Option<&'static str> {
    if style.size.is_empty() {
        return None;
    }
    if style.variant.is_empty() && style.kind.is_empty() {
        warn_text(
            "size must be paired with variant or legacy kind. Prefer variant for typography composition; use size only as a variant override.",
        );
        return None;
    }
    // Legacy kind consumes `size` as the composition step, not a font-size override.
    if style.variant.is_empty() {
        return None;
    }
    match font_size_class(style.size) {
        Some(class_name) => Some(class_name),
        None => {
            warn_text(&format!(
                "Unsupported size \"{}\". Use a numeric font-size token.",
                style.size
            ));
            None
        }
    }
}

fn uses_headline_visual(style: &TextStyle<'_>) -> bool {
    if !style.variant.is_empty() {
        style.variant.starts_with("headline-")
    } else {
        style.kind == "headline"
    }
}

/// Dialtone `DtText` class list. Staging `Text.vue` (`a46ac063`).
pub(crate) fn text_classes(style: &TextStyle<'_>) -> String {
    let composition = composition_class(style).unwrap_or_default();
    let font_size = font_size_modifier(style).unwrap_or("");
    let family = modifier(style.family, "family", family_class).unwrap_or("");
    let align = modifier(style.align, "align", align_class).unwrap_or("");
    let tone = if style.tone.is_empty() {
        String::new()
    } else {
        match tone_class(style.tone) {
            Some(class_name) => class_name,
            None => {
                warn_text(&format!("Unsupported tone \"{}\".", style.tone));
                String::new()
            }
        }
    };
    let wrap = modifier(style.wrap, "wrap", wrap_class).unwrap_or("");
    let trim = modifier(style.text_box_trim, "textBoxTrim", trim_class).unwrap_or("");
    let strength = modifier(style.strength, "strength", strength_class).unwrap_or("");
    let density = modifier(style.density, "density", density_class).unwrap_or("");
    join_class([
        "d-text",
        composition.as_str(),
        font_size,
        family,
        if style.italic { "d-text--italic" } else { "" },
        align,
        tone.as_str(),
        if style.truncate { "d-truncate" } else { "" },
        if style.numeric { "d-text--numeric" } else { "" },
        if style.max_lines > 0 {
            "d-text--clamp"
        } else {
            ""
        },
        wrap,
        trim,
        strength,
        density,
    ])
}

pub(crate) fn text_clamp_style(max_lines: u32) -> Option<String> {
    (max_lines > 0).then(|| format!("--dt-text-line-clamp: {max_lines}"))
}

fn note_headline_element(style: &TextStyle<'_>, tag: &str) {
    if !uses_headline_visual(style) {
        return;
    }
    if matches!(tag, "h1" | "h2" | "h3" | "h4" | "h5" | "h6") {
        return;
    }
    static NOTED: AtomicBool = AtomicBool::new(false);
    if NOTED.swap(true, Ordering::Relaxed) {
        return;
    }
    leptos::logging::log!(
        "[DtText] A headline text style is used without a semantic heading element. Consider using as=\"h1|h2|h3|h4|h5|h6\" for better accessibility."
    );
}

fn text_tag(tag: &str, class_name: String, style_attr: Option<String>, body: AnyView) -> AnyView {
    macro_rules! tag {
        ($el:ident) => {
            view! {
                <$el data-qa="dt-text" class=class_name style=style_attr>
                    {body}
                </$el>
            }
            .into_any()
        };
    }
    match tag {
        "span" => tag!(span),
        "p" => tag!(p),
        "h1" => tag!(h1),
        "h2" => tag!(h2),
        "h3" => tag!(h3),
        "h4" => tag!(h4),
        "h5" => tag!(h5),
        "h6" => tag!(h6),
        "label" => tag!(label),
        "div" => tag!(div),
        other => {
            warn_text(&format!("Unsupported as \"{other}\". Rendering a span."));
            tag!(span)
        }
    }
}

/// Dialtone Vue `DtText`.
///
/// Source is staging `Text.vue` (`a46ac063`). Tag `dialtone/v9.190.0` has no
/// `DtText`. `d-text*` is in the vendored Dialtone 10 CSS.
///
/// `as` is the element: `span` (default), `p`, `h1`–`h6`, `label`, or `div`.
/// `variant` is the typography composition. `kind` is the deprecated
/// composition and is ignored when `variant` is set. `size` is a font-size
/// token (`50`–`800`) when `variant` is set, or the legacy kind size when
/// `variant` is empty. An empty `size` with `kind` uses `"300"`.
///
/// Vue throws when a headline-only size (`700`, `600`, `500`, `3xl`, `2xl`,
/// `xl`) is used with a non-headline `kind`. This port warns and uses `"300"`.
/// `max_lines` is `maxLines`. `0` leaves the clamp unset. `text_box_trim` is
/// `textBoxTrim`.
#[component]
pub fn DtText(
    /// Element (`as`). Default `span`.
    #[prop(optional, default = "span")]
    r#as: &'static str,
    /// Deprecated composition (`kind`): `headline`, `body`, `label`, or `code`.
    #[prop(optional, default = "")]
    kind: &'static str,
    /// Typography composition (`variant`).
    #[prop(optional, default = "")]
    variant: &'static str,
    /// Font-size token, or the legacy kind size (`size`).
    #[prop(optional, default = "")]
    size: &'static str,
    /// Font family (`family`): `inherit`, `custom`, `sans`, `mono`, or `expressive`.
    #[prop(optional, default = "")]
    family: &'static str,
    #[prop(optional)] italic: bool,
    /// Foreground tone (`tone`).
    #[prop(optional, default = "")]
    tone: &'static str,
    /// Alignment (`align`): `start`, `center`, `end`, or `justify`.
    #[prop(optional, default = "")]
    align: &'static str,
    #[prop(optional)] truncate: bool,
    /// Line clamp (`maxLines`). `0` is unset.
    #[prop(optional, default = 0)]
    max_lines: u32,
    #[prop(optional)] numeric: bool,
    /// Wrapping (`wrap`): `wrap`, `nowrap`, `balance`, or `pretty`.
    #[prop(optional, default = "")]
    wrap: &'static str,
    /// Text-box trim (`textBoxTrim`): `start`, `end`, or `both`.
    #[prop(optional, default = "")]
    text_box_trim: &'static str,
    /// Font weight (`strength`): `bold`, `semibold`, `medium`, or `normal`.
    #[prop(optional, default = "")]
    strength: &'static str,
    /// Line height (`density`): `100`–`600`.
    #[prop(optional, default = "")]
    density: &'static str,
    children: Children,
) -> impl IntoView {
    let style = TextStyle {
        kind,
        variant,
        size,
        family,
        italic,
        tone,
        align,
        truncate,
        max_lines,
        numeric,
        wrap,
        text_box_trim,
        strength,
        density,
    };
    note_headline_element(&style, r#as);
    let class_name = text_classes(&style);
    let style_attr = text_clamp_style(max_lines);
    text_tag(r#as, class_name, style_attr, children().into_any())
}

#[cfg(test)]
mod tests {
    use super::*;

    const PLAIN: TextStyle<'static> = TextStyle {
        kind: "",
        variant: "",
        size: "",
        family: "",
        italic: false,
        tone: "",
        align: "",
        truncate: false,
        max_lines: 0,
        numeric: false,
        wrap: "",
        text_box_trim: "",
        strength: "",
        density: "",
    };

    #[test]
    fn variant_headline_uses_the_composition_class() {
        let style = TextStyle {
            variant: "headline-xl",
            ..PLAIN
        };
        assert_eq!(text_classes(&style), "d-text d-text-headline--xl");
    }

    #[test]
    fn legacy_kind_maps_numeric_size_and_defaults_to_300() {
        assert_eq!(
            text_classes(&TextStyle {
                kind: "body",
                size: "300",
                ..PLAIN
            }),
            "d-text d-text-body--md"
        );
        assert_eq!(
            text_classes(&TextStyle {
                kind: "body",
                ..PLAIN
            }),
            "d-text d-text-body--md"
        );
        assert_eq!(
            text_classes(&TextStyle {
                kind: "headline",
                size: "700",
                ..PLAIN
            }),
            "d-text d-text-headline--3xl"
        );
        assert_eq!(
            text_classes(&TextStyle {
                kind: "headline",
                size: "3xl",
                ..PLAIN
            }),
            "d-text d-text-headline--3xl"
        );
        assert_eq!(
            text_classes(&TextStyle {
                kind: "label",
                size: "xs",
                ..PLAIN
            }),
            "d-text d-text-label--xs"
        );
        assert_eq!(
            text_classes(&TextStyle {
                kind: "code",
                size: "100",
                ..PLAIN
            }),
            "d-text d-text-code--xs"
        );
    }

    #[test]
    fn headline_only_size_on_body_falls_back_to_md() {
        assert_eq!(
            text_classes(&TextStyle {
                kind: "body",
                size: "700",
                ..PLAIN
            }),
            "d-text d-text-body--md"
        );
        assert_eq!(
            text_classes(&TextStyle {
                kind: "body",
                size: "50",
                ..PLAIN
            }),
            "d-text d-text-body--md"
        );
    }

    #[test]
    fn variant_wins_and_size_becomes_a_font_size_override() {
        assert_eq!(
            text_classes(&TextStyle {
                kind: "headline",
                variant: "body-md",
                size: "200",
                ..PLAIN
            }),
            "d-text d-text-body--md d-text--fs-200"
        );
    }

    #[test]
    fn size_without_variant_or_kind_adds_no_font_class() {
        assert_eq!(
            text_classes(&TextStyle {
                size: "200",
                ..PLAIN
            }),
            "d-text"
        );
    }

    #[test]
    fn modifiers_follow_the_vue_order() {
        assert_eq!(
            text_classes(&TextStyle {
                variant: "body-md",
                size: "200",
                family: "mono",
                italic: true,
                tone: "muted",
                align: "center",
                truncate: true,
                max_lines: 2,
                numeric: true,
                wrap: "balance",
                text_box_trim: "both",
                strength: "semibold",
                density: "400",
                ..PLAIN
            }),
            "d-text d-text-body--md d-text--fs-200 d-text--ff-mono d-text--italic d-text--align-center d-text--tone-muted d-truncate d-text--numeric d-text--clamp d-text--wrap-balance d-text--trim-both d-text--fw-semibold d-text--lh-400"
        );
    }

    #[test]
    fn unknown_tokens_are_omitted() {
        assert_eq!(
            text_classes(&TextStyle {
                variant: "display",
                family: "comic",
                tone: "brand",
                ..PLAIN
            }),
            "d-text"
        );
    }

    #[test]
    fn clamp_style_is_set_only_when_max_lines_is_positive() {
        assert_eq!(text_clamp_style(0), None);
        assert_eq!(
            text_clamp_style(3),
            Some("--dt-text-line-clamp: 3".to_string())
        );
    }
}
