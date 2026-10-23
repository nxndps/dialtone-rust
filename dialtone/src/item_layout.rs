use leptos::prelude::*;

const TAGS: &[&str] = &[
    "div", "section", "span", "nav", "ul", "ol", "li", "article", "header", "footer", "main",
    "aside", "p", "form",
];

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// A non-empty deprecated class wins. Blank falls through to the current prop.
fn prefer<'a>(deprecated: &'a str, current: &'a str) -> &'a str {
    if deprecated.trim().is_empty() {
        current
    } else {
        deprecated
    }
}

fn layout_class(unstyled: bool, class: &str) -> String {
    join_class([
        if unstyled {
            "d-item-layout--custom"
        } else {
            "d-item-layout"
        },
        class,
    ])
}

/// Custom class first, then the layout class. Matches the Vue class array.
fn section_class(extra: &str, base: &str) -> String {
    join_class([extra, base])
}

/// Vendored `__subtitle` rules paint. Vue's `--subtitle` modifiers have no rule.
fn subtitle_classes(extra: &str, with_title: bool) -> String {
    join_class([
        extra,
        "d-item-layout__subtitle",
        "d-item-layout--subtitle",
        with_title
            .then_some("d-item-layout__subtitle-with-title")
            .unwrap_or(""),
        with_title
            .then_some("d-item-layout--subtitle--with-title")
            .unwrap_or(""),
    ])
}

/// Vue `dynamicGridTemplateColumns`. Empty columns stay as spaces.
fn grid_template_columns(start: bool, end: bool, selected: bool) -> String {
    let start = if start { "auto" } else { "" };
    let end = if end { "auto" } else { "" };
    let selected = if selected { "auto" } else { "" };
    format!("grid-template-columns: {start} 1fr {end} {selected};")
}

fn element_tag(value: &'static str) -> &'static str {
    if TAGS.contains(&value) {
        return value;
    }
    leptos::logging::warn!("DtItemLayout `as` value `{value}` is not supported; rendering a div");
    "div"
}

/// Dialtone Vue `DtItemLayout` (`dialtone/v10.7.0`).
///
/// The root is the `as` element (`div` by default). An unknown tag warns and
/// renders `div`. `unstyled` uses `d-item-layout--custom` and sets
/// `grid-template-columns` from the start, end, and selected slots. Styled
/// uses `d-item-layout`. `class` is appended on the root.
///
/// `start_class` is `startClass`. A non-empty `left_class` (`leftClass`) wins.
/// `content_class` is `contentClass`. `title_class` is `titleClass`.
/// `subtitle_class` is `subtitleClass`. `block_end_class` is `blockEndClass`.
/// A non-empty `bottom_class` (`bottomClass`) wins. `end_class` is `endClass`.
/// A non-empty `right_class` (`rightClass`) wins. `selected_class` is
/// `selectedClass`. A custom section class comes first.
///
/// `start` wins over `left`. `end` wins over `right`. `block_end` wins over
/// `bottom`. The default slot is the title. The subtitle emits the vendored
/// `d-item-layout__subtitle` classes and Vue's `d-item-layout--subtitle`
/// modifiers. The with-title classes are included whenever the subtitle slot
/// renders, because the default slot is always present.
#[component]
pub fn DtItemLayout(
    /// Root element (`as`).
    #[prop(optional, default = "div")]
    r#as: &'static str,
    /// Drop the styled flex row (`unstyled`).
    #[prop(optional)]
    unstyled: bool,
    /// Extra class appended on the root.
    #[prop(optional, into)]
    class: String,
    /// Extra class on the start section (`startClass`).
    #[prop(optional, into)]
    start_class: String,
    /// Deprecated. Use `start_class` (`leftClass`). A non-empty value wins.
    #[prop(optional, into)]
    left_class: String,
    /// Extra class on the content section (`contentClass`).
    #[prop(optional, into)]
    content_class: String,
    /// Extra class on the title (`titleClass`).
    #[prop(optional, into)]
    title_class: String,
    /// Extra class on the subtitle (`subtitleClass`).
    #[prop(optional, into)]
    subtitle_class: String,
    /// Extra class on the block-end row (`blockEndClass`).
    #[prop(optional, into)]
    block_end_class: String,
    /// Deprecated. Use `block_end_class` (`bottomClass`). A non-empty value wins.
    #[prop(optional, into)]
    bottom_class: String,
    /// Extra class on the end section (`endClass`).
    #[prop(optional, into)]
    end_class: String,
    /// Deprecated. Use `end_class` (`rightClass`). A non-empty value wins.
    #[prop(optional, into)]
    right_class: String,
    /// Extra class on the selected section (`selectedClass`).
    #[prop(optional, into)]
    selected_class: String,
    /// Start slot. Wins over `left`.
    #[prop(optional)]
    start: Option<ViewFn>,
    /// Deprecated. Use `start`.
    #[prop(optional)]
    left: Option<ViewFn>,
    /// Subtitle slot.
    #[prop(optional)]
    subtitle: Option<ViewFn>,
    /// Block-end slot. Wins over `bottom`.
    #[prop(optional)]
    block_end: Option<ViewFn>,
    /// Deprecated. Use `block_end`.
    #[prop(optional)]
    bottom: Option<ViewFn>,
    /// End slot. Wins over `right`.
    #[prop(optional)]
    end: Option<ViewFn>,
    /// Deprecated. Use `end`.
    #[prop(optional)]
    right: Option<ViewFn>,
    /// Selected slot.
    #[prop(optional)]
    selected: Option<ViewFn>,
    children: Children,
) -> impl IntoView {
    let tag = element_tag(r#as);
    let start_slot = start.or(left);
    let end_slot = end.or(right);
    let block_end_slot = block_end.or(bottom);
    let has_start = start_slot.is_some();
    let has_end = end_slot.is_some();
    let has_selected = selected.is_some();
    let root_class = layout_class(unstyled, &class);
    let grid_style = unstyled.then(|| grid_template_columns(has_start, has_end, has_selected));
    let start_section_class =
        section_class(prefer(&left_class, &start_class), "d-item-layout__left");
    let content_section_class = section_class(&content_class, "d-item-layout__content");
    let title_section_class = section_class(&title_class, "d-item-layout__title");
    let subtitle_section_class = subtitle_classes(&subtitle_class, true);
    let block_end_section_class = section_class(
        prefer(&bottom_class, &block_end_class),
        "d-item-layout__bottom",
    );
    let end_section_class = section_class(prefer(&right_class, &end_class), "d-item-layout__right");
    let selected_section_class = section_class(&selected_class, "d-item-layout__selected");
    let title = children();
    let body = view! {
        {start_slot.map(|slot| view! {
            <section class=start_section_class data-qa="dt-item-layout-left-wrapper">
                {slot.run()}
            </section>
        })}
        <section class=content_section_class data-qa="dt-item-layout-content-wrapper">
            <div class=title_section_class data-qa="dt-item-layout-title-wrapper">
                {title}
            </div>
            {subtitle.map(|slot| view! {
                <div class=subtitle_section_class data-qa="dt-item-layout-subtitle-wrapper">
                    {slot.run()}
                </div>
            })}
            {block_end_slot.map(|slot| view! {
                <div class=block_end_section_class data-qa="dt-item-layout-bottom-wrapper">
                    {slot.run()}
                </div>
            })}
        </section>
        {end_slot.map(|slot| view! {
            <section class=end_section_class data-qa="dt-item-layout-right-wrapper">
                {slot.run()}
            </section>
        })}
        {selected.map(|slot| view! {
            <section class=selected_section_class data-qa="dt-item-layout-selected-wrapper">
                {slot.run()}
            </section>
        })}
    }
    .into_any();

    macro_rules! layout_el {
        ($tag:ident) => {
            view! {
                <$tag class=root_class style=grid_style data-qa="dt-item-layout">
                    {body}
                </$tag>
            }
            .into_any()
        };
    }

    match tag {
        "section" => layout_el!(section),
        "span" => layout_el!(span),
        "nav" => layout_el!(nav),
        "ul" => layout_el!(ul),
        "ol" => layout_el!(ol),
        "li" => layout_el!(li),
        "article" => layout_el!(article),
        "header" => layout_el!(header),
        "footer" => layout_el!(footer),
        "main" => layout_el!(main),
        "aside" => layout_el!(aside),
        "p" => layout_el!(p),
        "form" => layout_el!(form),
        _ => layout_el!(div),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deprecated_class_wins_only_when_it_has_text() {
        assert_eq!(prefer("legacy", "next"), "legacy");
        assert_eq!(prefer("", "next"), "next");
        assert_eq!(prefer("   ", "next"), "next");
        assert_eq!(prefer("", ""), "");
    }

    #[test]
    fn grid_keeps_empty_columns() {
        assert_eq!(
            grid_template_columns(true, true, true),
            "grid-template-columns: auto 1fr auto auto;"
        );
        assert_eq!(
            grid_template_columns(true, false, true),
            "grid-template-columns: auto 1fr  auto;"
        );
        assert_eq!(
            grid_template_columns(false, false, false),
            "grid-template-columns:  1fr  ;"
        );
    }

    #[test]
    fn section_class_puts_the_custom_class_first() {
        assert_eq!(
            section_class("extra", "d-item-layout__left"),
            "extra d-item-layout__left"
        );
        assert_eq!(
            section_class("  ", "d-item-layout__content"),
            "d-item-layout__content"
        );
    }

    #[test]
    fn subtitle_emits_both_class_families() {
        assert_eq!(
            subtitle_classes("mine", true),
            "mine d-item-layout__subtitle d-item-layout--subtitle d-item-layout__subtitle-with-title d-item-layout--subtitle--with-title"
        );
        assert_eq!(
            subtitle_classes("", false),
            "d-item-layout__subtitle d-item-layout--subtitle"
        );
    }

    #[test]
    fn root_class_is_appended() {
        assert_eq!(layout_class(false, "mine"), "d-item-layout mine");
        assert_eq!(layout_class(true, ""), "d-item-layout--custom");
        assert_eq!(layout_class(true, "mine"), "d-item-layout--custom mine");
    }
}
