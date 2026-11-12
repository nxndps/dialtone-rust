use leptos::prelude::*;

/// Dialtone `DtSkeleton` text line kind (`text-option.type`).
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum DtSkeletonTextKind {
    #[default]
    Body,
    Heading,
}

/// Dialtone heading height (`text-option.headingHeight`).
///
/// `sm` and `200` are [`Sm`](Self::Sm). `md` and `300` are [`Md`](Self::Md).
/// `lg` and `400` are [`Lg`](Self::Lg).
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum DtSkeletonHeadingHeight {
    /// `sm` / `200`. Class `d-h-25`.
    Sm,
    /// `md` / `300`. Class `d-h-24px`.
    #[default]
    Md,
    /// `lg` / `400`. Class `d-h-50`.
    Lg,
}

/// Dialtone shape (`circle` / `square`).
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum DtSkeletonShapeKind {
    #[default]
    Circle,
    Square,
}

/// Dialtone `text-option`.
///
/// `kind` is `type`. `style` is the inline `style` string from the skeleton
/// page (`--placeholder-from-color`, `--placeholder-to-color`).
#[derive(Clone, Copy)]
pub struct DtSkeletonTextOption {
    pub kind: DtSkeletonTextKind,
    pub heading_height: DtSkeletonHeadingHeight,
    pub width: &'static str,
    pub style: &'static str,
}

impl Default for DtSkeletonTextOption {
    fn default() -> Self {
        Self {
            kind: DtSkeletonTextKind::Body,
            heading_height: DtSkeletonHeadingHeight::Md,
            width: "100%",
            style: "",
        }
    }
}

/// Dialtone `paragraph-option`.
///
/// `widths` is Vue `width` when that value is an array. `width` is Vue
/// `width` when it is one length for every row. `random_width` is
/// `randomWidth`. `min_width` and `max_width` are `minWidth` and `maxWidth`.
#[derive(Clone, Copy)]
pub struct DtSkeletonParagraphOption {
    pub rows: u8,
    pub widths: Option<&'static [&'static str]>,
    pub width: Option<&'static str>,
    pub random_width: bool,
    pub min_width: f64,
    pub max_width: f64,
}

impl Default for DtSkeletonParagraphOption {
    fn default() -> Self {
        Self {
            rows: 3,
            widths: None,
            width: None,
            random_width: false,
            min_width: 30.0,
            max_width: 100.0,
        }
    }
}

/// Vue `paragraphOption`: the object, or `true` for the defaults.
#[derive(Clone, Copy, Default)]
pub enum DtSkeletonParagraphOptionProp {
    #[default]
    Unset,
    Defaults,
    Custom(DtSkeletonParagraphOption),
}

impl From<bool> for DtSkeletonParagraphOptionProp {
    fn from(value: bool) -> Self {
        if value {
            Self::Defaults
        } else {
            Self::Unset
        }
    }
}

impl From<DtSkeletonParagraphOption> for DtSkeletonParagraphOptionProp {
    fn from(value: DtSkeletonParagraphOption) -> Self {
        Self::Custom(value)
    }
}

/// Dialtone `list-item-option`.
///
/// `shape_size` is `shapeSize`. A default list item uses paragraphs
/// `{ rows: 3, randomWidth: true }`, matching Vue.
#[derive(Clone, Copy)]
pub struct DtSkeletonListItemOption {
    pub shape: DtSkeletonShapeKind,
    pub shape_size: &'static str,
    pub paragraphs: DtSkeletonParagraphOption,
}

impl Default for DtSkeletonListItemOption {
    fn default() -> Self {
        Self {
            shape: DtSkeletonShapeKind::Circle,
            shape_size: "300",
            paragraphs: DtSkeletonParagraphOption {
                random_width: true,
                ..DtSkeletonParagraphOption::default()
            },
        }
    }
}

/// Vue `listItemOption`: the object, or `true` for the defaults.
#[derive(Clone, Copy, Default)]
pub enum DtSkeletonListItemOptionProp {
    #[default]
    Unset,
    Defaults,
    Custom(DtSkeletonListItemOption),
}

impl From<bool> for DtSkeletonListItemOptionProp {
    fn from(value: bool) -> Self {
        if value {
            Self::Defaults
        } else {
            Self::Unset
        }
    }
}

impl From<DtSkeletonListItemOption> for DtSkeletonListItemOptionProp {
    fn from(value: DtSkeletonListItemOption) -> Self {
        Self::Custom(value)
    }
}

/// Dialtone `shape-option`.
///
/// `size` accepts `100`–`500`, the deprecated `sm` / `md` / `lg` tokens, or
/// a CSS length (`2rem`, `100%`).
#[derive(Clone, Copy)]
pub struct DtSkeletonShapeOption {
    pub shape: DtSkeletonShapeKind,
    pub size: &'static str,
}

impl Default for DtSkeletonShapeOption {
    fn default() -> Self {
        Self {
            shape: DtSkeletonShapeKind::Circle,
            size: "300",
        }
    }
}

/// Vue `shapeOption`: the object, or `true` for the defaults.
#[derive(Clone, Copy, Default)]
pub enum DtSkeletonShapeOptionProp {
    #[default]
    Unset,
    Defaults,
    Custom(DtSkeletonShapeOption),
}

impl From<bool> for DtSkeletonShapeOptionProp {
    fn from(value: bool) -> Self {
        if value {
            Self::Defaults
        } else {
            Self::Unset
        }
    }
}

impl From<DtSkeletonShapeOption> for DtSkeletonShapeOptionProp {
    fn from(value: DtSkeletonShapeOption) -> Self {
        Self::Custom(value)
    }
}

fn paragraph_option_value(
    prop: DtSkeletonParagraphOptionProp,
) -> Option<DtSkeletonParagraphOption> {
    match prop {
        DtSkeletonParagraphOptionProp::Unset => None,
        DtSkeletonParagraphOptionProp::Defaults => Some(DtSkeletonParagraphOption::default()),
        DtSkeletonParagraphOptionProp::Custom(option) => Some(option),
    }
}

fn list_item_option_value(prop: DtSkeletonListItemOptionProp) -> Option<DtSkeletonListItemOption> {
    match prop {
        DtSkeletonListItemOptionProp::Unset => None,
        DtSkeletonListItemOptionProp::Defaults => Some(DtSkeletonListItemOption::default()),
        DtSkeletonListItemOptionProp::Custom(option) => Some(option),
    }
}

fn shape_option_value(prop: DtSkeletonShapeOptionProp) -> Option<DtSkeletonShapeOption> {
    match prop {
        DtSkeletonShapeOptionProp::Unset => None,
        DtSkeletonShapeOptionProp::Defaults => Some(DtSkeletonShapeOption::default()),
        DtSkeletonShapeOptionProp::Custom(option) => Some(option),
    }
}

/// `SKELETON_RIPPLE_DURATION` in `SkeletonConstants.js`.
const SKELETON_RIPPLE_DURATION: f64 = 3_000_000.0;

struct SkeletonMotion {
    delay_ms: f64,
    duration_ms: i32,
}

/// Vue `skeleton.js` `skeletonStyle`, using `offset` as `skeletonOffset`.
///
/// The mixin would read `getBoundingClientRect` after mount, but that ref is
/// not reactive. The skeleton page paints `animation-delay:3000ms` for the
/// default `offset` of `1`.
fn skeleton_motion(animate: bool, animation_duration: i32, offset: f64) -> Option<SkeletonMotion> {
    if offset == -1.0 || (!animate && animation_duration == -1) {
        return None;
    }
    let delay_ms = offset * SKELETON_RIPPLE_DURATION / 1000.0;
    let duration_ms = if animation_duration == -1 {
        1000
    } else {
        animation_duration
    };
    Some(SkeletonMotion {
        delay_ms,
        duration_ms,
    })
}

fn format_css_number(value: f64) -> String {
    if value.is_finite() && value.fract() == 0.0 && value.abs() < 1e15 {
        format!("{}", value as i64)
    } else {
        format!("{value}")
    }
}

fn motion_css(motion: Option<SkeletonMotion>) -> String {
    let Some(motion) = motion else {
        return String::new();
    };
    format!(
        "animation-delay:{}ms;animation-duration:{}ms;",
        format_css_number(motion.delay_ms),
        motion.duration_ms
    )
}

fn heading_height_class(height: DtSkeletonHeadingHeight) -> &'static str {
    match height {
        DtSkeletonHeadingHeight::Sm => "d-h-25",
        DtSkeletonHeadingHeight::Md => "d-h-24px",
        DtSkeletonHeadingHeight::Lg => "d-h-50",
    }
}

fn shape_kind_class(shape: DtSkeletonShapeKind) -> &'static str {
    match shape {
        DtSkeletonShapeKind::Circle => "d-bar-circle",
        DtSkeletonShapeKind::Square => "d-bar-200",
    }
}

fn shape_size_px(size: &str) -> &str {
    match size {
        "100" => "16px",
        "200" | "sm" => "24px",
        "300" | "md" => "32px",
        "400" | "lg" => "48px",
        "500" => "64px",
        other => other,
    }
}

fn text_class(option: DtSkeletonTextOption, animate: bool) -> String {
    let placeholder = if animate {
        "d-skeleton-placeholder d-skeleton-placeholder--animate"
    } else {
        "d-skeleton-placeholder"
    };
    match option.kind {
        DtSkeletonTextKind::Body => format!("d-skeleton-text {placeholder}"),
        DtSkeletonTextKind::Heading => format!(
            "{} d-skeleton-text--heading {placeholder}",
            heading_height_class(option.heading_height)
        ),
    }
}

fn text_style(width: &str, style: &str, motion: Option<SkeletonMotion>) -> String {
    let mut out = format!("width:{width};{}", motion_css(motion));
    if !style.is_empty() {
        out.push_str(style);
        if !style.ends_with(';') {
            out.push(';');
        }
    }
    out
}

fn shape_class_list(option: DtSkeletonShapeOption, animate: bool, content_class: &str) -> String {
    let mut class = format!("d-skeleton-placeholder {}", shape_kind_class(option.shape));
    if animate {
        class.push_str(" d-skeleton-placeholder--animate");
    }
    if !content_class.is_empty() {
        class.push(' ');
        class.push_str(content_class.trim());
    }
    class
}

fn shape_style(size: &str, motion: Option<SkeletonMotion>) -> String {
    let size = shape_size_px(size);
    format!(
        "{}min-width:{size};max-width:{size};min-height:{size};max-height:{size};",
        motion_css(motion)
    )
}

fn random_width_percentage(min_width: f64, max_width: f64, unit: f64) -> String {
    let min = min_width.min(max_width);
    let max = min_width.max(max_width);
    let value = (unit * (max - min)).round() + min;
    format!("{}%", format_css_number(value))
}

fn paragraph_row_width(option: &DtSkeletonParagraphOption, row: u8, unit: f64) -> String {
    if option.random_width {
        return random_width_percentage(option.min_width, option.max_width, unit);
    }
    if let Some(width) = option.width {
        return width.to_string();
    }
    if let Some(widths) = option.widths {
        if let Some(width) = widths.get(row.saturating_sub(1) as usize) {
            return (*width).to_string();
        }
    }
    if row == option.rows {
        "38%".to_string()
    } else {
        "100%".to_string()
    }
}

fn unit_random() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Math::random()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0.5
    }
}

fn warn_if_many_options(paragraph: bool, list_item: bool, text: bool, shape: bool) {
    let mut names = Vec::new();
    if paragraph {
        names.push("paragraphOption");
    }
    if list_item {
        names.push("listItemOption");
    }
    if text {
        names.push("textOption");
    }
    if shape {
        names.push("shapeOption");
    }
    if names.len() >= 2 {
        leptos::logging::warn!(
            "Use only one of {} options at the same time",
            names.join(" | ")
        );
    }
}

/// Dialtone `DtSkeletonText`.
#[component]
fn DtSkeletonText(
    #[prop(optional)] option: DtSkeletonTextOption,
    #[prop(into)] width: String,
    animate: bool,
    animation_duration: i32,
    offset: f64,
    /// Set on paragraph rows (`skeleton-paragraph-row`). Empty uses the text qa.
    #[prop(optional, into)]
    data_qa: String,
) -> impl IntoView {
    let data_qa = if data_qa.is_empty() {
        if option.kind == DtSkeletonTextKind::Heading {
            "skeleton-text-heading".to_string()
        } else {
            "skeleton-text-body".to_string()
        }
    } else {
        data_qa
    };
    let class = text_class(option, animate);
    let style = text_style(
        &width,
        option.style,
        skeleton_motion(animate, animation_duration, offset),
    );
    view! {
        <div class=class data-qa=data_qa style=style></div>
    }
}

/// Dialtone `DtSkeletonParagraph`.
#[component]
fn DtSkeletonParagraph(
    #[prop(optional)] option: DtSkeletonParagraphOption,
    animate: bool,
    animation_duration: i32,
    offset: f64,
) -> impl IntoView {
    let widths: Vec<String> = (1..=option.rows)
        .map(|row| paragraph_row_width(&option, row, unit_random()))
        .collect();
    view! {
        <div class="d-skeleton-paragraph" data-qa="skeleton-paragraph">
            {widths.into_iter().map(|width| {
                view! {
                    <DtSkeletonText
                        option=DtSkeletonTextOption {
                            kind: DtSkeletonTextKind::Body,
                            ..DtSkeletonTextOption::default()
                        }
                        width=width
                        animate=animate
                        animation_duration=animation_duration
                        offset=offset
                        data_qa="skeleton-paragraph-row"
                    />
                }
            }).collect_view()}
        </div>
    }
}

/// Dialtone `DtSkeletonShape`.
#[component]
fn DtSkeletonShape(
    #[prop(optional)] option: DtSkeletonShapeOption,
    animate: bool,
    animation_duration: i32,
    offset: f64,
    #[prop(optional, into)] content_class: String,
) -> impl IntoView {
    let class = shape_class_list(option, animate, &content_class);
    let style = shape_style(
        option.size,
        skeleton_motion(animate, animation_duration, offset),
    );
    view! {
        <div class=class data-qa="skeleton-shape" style=style></div>
    }
}

/// Dialtone `DtSkeletonListItem`.
#[component]
fn DtSkeletonListItem(
    #[prop(optional)] option: DtSkeletonListItemOption,
    animate: bool,
    animation_duration: i32,
    offset: f64,
) -> impl IntoView {
    let single = option.paragraphs.rows == 1;
    view! {
        <div
            class="d-skeleton-list-item"
            class=("d-skeleton-list-item--single", single)
            data-qa="skeleton-list-item"
        >
            <DtSkeletonShape
                option=DtSkeletonShapeOption {
                    shape: option.shape,
                    size: option.shape_size,
                }
                animate=animate
                animation_duration=animation_duration
                offset=offset
                content_class="d-skeleton-list-item__shape"
            />
            <div class="d-skeleton-list-item__paragraph-container">
                <DtSkeletonParagraph
                    option=option.paragraphs
                    animate=animate
                    animation_duration=animation_duration
                    offset=offset
                />
            </div>
        </div>
    }
}

/// Dialtone Vue `DtSkeleton`.
///
/// Set only one of `paragraph_option`, `list_item_option`, `shape_option`, or
/// `text_option`. With none set, renders a single body text line. `true` on
/// the object-or-boolean options uses that option's defaults.
///
/// `animation_duration` is `animationDuration`. `offset` is the ripple offset.
/// `animate` wins over an `animate` key on an option.
#[component]
pub fn DtSkeleton(
    /// Descriptive label for the content (`ariaLabel`).
    #[prop(optional, into)]
    aria_label: String,
    /// Throb the placeholder. Default true. Wins over `option.animate`.
    #[prop(optional, default = true)]
    animate: bool,
    /// Animation length in ms (`animationDuration`). `-1` is infinite and
    /// paints `1000ms`.
    #[prop(optional, default = -1)]
    animation_duration: i32,
    /// Ripple offset (`offset`). Default `1`, which paints a `3000ms` delay.
    /// `-1` omits the animation style.
    #[prop(optional, default = 1.0)]
    offset: f64,
    /// Extra class on the root. `DtEmoji` passes `d-icon` and the size class.
    #[prop(optional, into)]
    class: String,
    /// Several text lines (`paragraphOption`). Pass `true` for the defaults.
    #[prop(optional, into)]
    paragraph_option: DtSkeletonParagraphOptionProp,
    /// Avatar plus wrapping text (`listItemOption`). Pass `true` for the defaults.
    #[prop(optional, into)]
    list_item_option: DtSkeletonListItemOptionProp,
    /// A circle or square (`shapeOption`). Pass `true` for the defaults.
    #[prop(optional, into)]
    shape_option: DtSkeletonShapeOptionProp,
    /// One text line (`textOption`).
    #[prop(optional, into)]
    text_option: Option<DtSkeletonTextOption>,
) -> impl IntoView {
    let paragraph_option = paragraph_option_value(paragraph_option);
    let list_item_option = list_item_option_value(list_item_option);
    let shape_option = shape_option_value(shape_option);
    warn_if_many_options(
        paragraph_option.is_some(),
        list_item_option.is_some(),
        text_option.is_some(),
        shape_option.is_some(),
    );

    // Text, paragraph, and list lines are a percentage of this element. A
    // centered flex parent would otherwise shrink the element to 0.
    let fills = shape_option.is_none();
    let root_class = match (fills, class.is_empty()) {
        (true, true) => "d-w100p".to_string(),
        (true, false) => format!("d-w100p {class}"),
        (false, true) => String::new(),
        (false, false) => class,
    };
    let root_class = (!root_class.is_empty()).then_some(root_class);
    let body = if let Some(option) = list_item_option {
        view! {
            <DtSkeletonListItem
                option=option
                animate=animate
                animation_duration=animation_duration
                offset=offset
            />
        }
        .into_any()
    } else if let Some(option) = shape_option {
        view! {
            <DtSkeletonShape
                option=option
                animate=animate
                animation_duration=animation_duration
                offset=offset
            />
        }
        .into_any()
    } else if let Some(option) = paragraph_option {
        view! {
            <DtSkeletonParagraph
                option=option
                animate=animate
                animation_duration=animation_duration
                offset=offset
            />
        }
        .into_any()
    } else {
        let option = text_option.unwrap_or_default();
        let width = option.width.to_string();
        view! {
            <DtSkeletonText
                option=option
                width=width
                animate=animate
                animation_duration=animation_duration
                offset=offset
            />
        }
        .into_any()
    };

    view! {
        <div
            class=root_class
            aria-busy="true"
            role="status"
            attr:aria-label=aria_label
            data-qa="dt-skeleton"
        >
            {body}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_matches_the_docs_page() {
        let motion = skeleton_motion(true, -1, 1.0).unwrap();
        assert_eq!(motion.delay_ms, 3000.0);
        assert_eq!(motion.duration_ms, 1000);
        assert_eq!(
            motion_css(Some(motion)),
            "animation-delay:3000ms;animation-duration:1000ms;"
        );
    }

    #[test]
    fn still_skeleton_omits_the_animation_style() {
        assert!(skeleton_motion(false, -1, 1.0).is_none());
        assert!(skeleton_motion(true, 400, -1.0).is_none());
    }

    #[test]
    fn finite_duration_keeps_the_ripple_delay() {
        let motion = skeleton_motion(true, 2500, 1.0).unwrap();
        assert_eq!(motion.delay_ms, 3000.0);
        assert_eq!(motion.duration_ms, 2500);
        let paused = skeleton_motion(false, 2500, 2.0).unwrap();
        assert_eq!(paused.delay_ms, 6000.0);
        assert_eq!(paused.duration_ms, 2500);
    }

    #[test]
    fn text_style_appends_placeholder_colors() {
        assert_eq!(
            text_style("100%", "", skeleton_motion(true, -1, 1.0)),
            "width:100%;animation-delay:3000ms;animation-duration:1000ms;"
        );
        assert_eq!(
            text_style(
                "100%",
                "--placeholder-from-color: var(--dt-color-blue-400); --placeholder-to-color: var(--dt-color-blue-200);",
                skeleton_motion(true, -1, 1.0),
            ),
            "width:100%;animation-delay:3000ms;animation-duration:1000ms;--placeholder-from-color: var(--dt-color-blue-400); --placeholder-to-color: var(--dt-color-blue-200);"
        );
        assert_eq!(
            text_style(
                "100%",
                "--placeholder-from-color: var(--dt-color-blue-400)",
                skeleton_motion(false, -1, 1.0),
            ),
            "width:100%;--placeholder-from-color: var(--dt-color-blue-400);"
        );
    }

    #[test]
    fn heading_and_shape_classes_follow_v10() {
        assert_eq!(heading_height_class(DtSkeletonHeadingHeight::Sm), "d-h-25");
        assert_eq!(
            heading_height_class(DtSkeletonHeadingHeight::Md),
            "d-h-24px"
        );
        assert_eq!(heading_height_class(DtSkeletonHeadingHeight::Lg), "d-h-50");
        let heading = DtSkeletonTextOption {
            kind: DtSkeletonTextKind::Heading,
            heading_height: DtSkeletonHeadingHeight::Sm,
            width: "160px",
            style: "",
        };
        assert_eq!(
            text_class(heading, false),
            "d-h-25 d-skeleton-text--heading d-skeleton-placeholder"
        );
        assert_eq!(
            text_class(DtSkeletonTextOption::default(), true),
            "d-skeleton-text d-skeleton-placeholder d-skeleton-placeholder--animate"
        );
        assert_eq!(shape_size_px("100"), "16px");
        assert_eq!(shape_size_px("200"), "24px");
        assert_eq!(shape_size_px("sm"), "24px");
        assert_eq!(shape_size_px("300"), "32px");
        assert_eq!(shape_size_px("md"), "32px");
        assert_eq!(shape_size_px("lg"), "48px");
        assert_eq!(shape_size_px("500"), "64px");
        assert_eq!(shape_size_px("2rem"), "2rem");
        assert_eq!(shape_size_px("3.6rem"), "3.6rem");
        let circle = DtSkeletonShapeOption {
            shape: DtSkeletonShapeKind::Circle,
            size: "sm",
        };
        assert_eq!(
            shape_class_list(circle, false, ""),
            "d-skeleton-placeholder d-bar-circle"
        );
        assert_eq!(
            shape_style("sm", skeleton_motion(true, -1, 1.0)),
            "animation-delay:3000ms;animation-duration:1000ms;min-width:24px;max-width:24px;min-height:24px;max-height:24px;"
        );
        let square = DtSkeletonShapeOption {
            shape: DtSkeletonShapeKind::Square,
            size: "lg",
        };
        assert_eq!(
            shape_class_list(square, true, "d-skeleton-list-item__shape"),
            "d-skeleton-placeholder d-bar-200 d-skeleton-placeholder--animate d-skeleton-list-item__shape"
        );
    }

    #[test]
    fn paragraph_widths_follow_vue() {
        let base = DtSkeletonParagraphOption::default();
        assert_eq!(paragraph_row_width(&base, 1, 0.0), "100%");
        assert_eq!(paragraph_row_width(&base, 3, 0.0), "38%");
        let string_width = DtSkeletonParagraphOption {
            width: Some("12rem"),
            ..base
        };
        assert_eq!(paragraph_row_width(&string_width, 2, 0.0), "12rem");
        let array_width = DtSkeletonParagraphOption {
            rows: 4,
            widths: Some(&["120px", "311px", "371px", "279px"]),
            ..DtSkeletonParagraphOption::default()
        };
        assert_eq!(paragraph_row_width(&array_width, 1, 0.0), "120px");
        assert_eq!(paragraph_row_width(&array_width, 4, 0.0), "279px");
        let random = DtSkeletonParagraphOption {
            random_width: true,
            ..DtSkeletonParagraphOption::default()
        };
        assert_eq!(paragraph_row_width(&random, 1, 0.0), "30%");
        assert_eq!(paragraph_row_width(&random, 2, 1.0), "100%");
        assert_eq!(random_width_percentage(100.0, 30.0, 0.0), "30%");
    }

    #[test]
    fn boolean_shorthand_uses_vue_defaults() {
        assert!(paragraph_option_value(true.into()).unwrap().rows == 3);
        assert!(!paragraph_option_value(true.into()).unwrap().random_width);
        assert!(paragraph_option_value(false.into()).is_none());
        let list = list_item_option_value(true.into()).unwrap();
        assert_eq!(list.shape_size, "300");
        assert!(list.paragraphs.random_width);
        assert_eq!(list.paragraphs.rows, 3);
        let shape = shape_option_value(true.into()).unwrap();
        assert_eq!(shape.size, "300");
        assert!(matches!(shape.shape, DtSkeletonShapeKind::Circle));
    }
}
