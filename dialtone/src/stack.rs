use leptos::prelude::*;

const DIRECTIONS: &[&str] = &["default", "column", "row", "row-reverse", "column-reverse"];
const GAPS: &[&str] = &[
    "0", "1", "25", "50", "75", "100", "125", "150", "175", "200", "250", "300", "350", "400",
    "450", "500", "525", "550", "600", "650", "700", "750", "800",
];
const ALIGNS: &[&str] = &["start", "center", "end", "stretch", "baseline"];
const JUSTIFIES: &[&str] = &[
    "start",
    "center",
    "end",
    "space-around",
    "space-between",
    "space-evenly",
    "around",
    "between",
    "evenly",
];
const BREAKPOINTS: &[&str] = &["sm", "md", "lg", "xl"];

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// One Dialtone stack token, or the same token per breakpoint.
///
/// A string applies at every breakpoint (`direction="row"`). Breakpoint fields
/// are the Vue object form (`{ default, sm, md, lg, xl }`). An unset `default`
/// leaves the base class off, which is what Vue does when the object has no
/// `default` key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StackAt {
    default: Option<&'static str>,
    sm: Option<&'static str>,
    md: Option<&'static str>,
    lg: Option<&'static str>,
    xl: Option<&'static str>,
}

impl StackAt {
    pub const fn all(value: &'static str) -> Self {
        Self {
            default: Some(value),
            sm: None,
            md: None,
            lg: None,
            xl: None,
        }
    }

    /// Breakpoint overrides with no base token.
    pub const fn breakpoints(
        sm: Option<&'static str>,
        md: Option<&'static str>,
        lg: Option<&'static str>,
        xl: Option<&'static str>,
    ) -> Self {
        Self {
            default: None,
            sm,
            md,
            lg,
            xl,
        }
    }

    pub const fn sm(mut self, value: &'static str) -> Self {
        self.sm = Some(value);
        self
    }

    pub const fn md(mut self, value: &'static str) -> Self {
        self.md = Some(value);
        self
    }

    pub const fn lg(mut self, value: &'static str) -> Self {
        self.lg = Some(value);
        self
    }

    pub const fn xl(mut self, value: &'static str) -> Self {
        self.xl = Some(value);
        self
    }

    fn at(self, breakpoint: &str) -> Option<&'static str> {
        match breakpoint {
            "sm" => self.sm,
            "md" => self.md,
            "lg" => self.lg,
            "xl" => self.xl,
            _ => None,
        }
    }
}

impl From<&'static str> for StackAt {
    fn from(value: &'static str) -> Self {
        Self::all(value)
    }
}

fn direction_token(value: &str) -> Option<&'static str> {
    match value {
        "default" | "column" => Some("column"),
        "row" => Some("row"),
        "row-reverse" => Some("row-reverse"),
        "column-reverse" => Some("column-reverse"),
        _ => None,
    }
}

fn direction_class(value: &StackAt) -> String {
    let Some(default) = value.default else {
        return String::new();
    };
    // Vue treats the direction string `column` as the CSS default and emits no class.
    if default == "column" {
        return String::new();
    }
    match direction_token(default) {
        Some(token) => format!("d-stack--{token}"),
        None => {
            leptos::logging::warn!("DtStack direction `{default}` is not a stack direction");
            String::new()
        }
    }
}

fn listed_class(
    kind: &str,
    value: Option<&str>,
    allowed: &[&str],
    normalize: fn(&str) -> &str,
) -> String {
    let Some(value) = value else {
        return String::new();
    };
    if allowed.contains(&value) {
        format!("d-stack--{kind}-{}", normalize(value))
    } else {
        leptos::logging::warn!("DtStack {kind} `{value}` is not a stack {kind} token");
        String::new()
    }
}

fn identity(value: &str) -> &str {
    value
}

fn justify_token(value: &str) -> &str {
    match value {
        "space-around" => "around",
        "space-between" => "between",
        "space-evenly" => "evenly",
        _ => value,
    }
}

fn responsive_direction(value: &StackAt) -> Vec<String> {
    BREAKPOINTS
        .iter()
        .filter_map(|breakpoint| {
            let token = value.at(breakpoint)?;
            if DIRECTIONS.contains(&token) {
                Some(format!("d-stack--{breakpoint}-{token}"))
            } else {
                leptos::logging::warn!("DtStack direction `{token}` is not a stack direction");
                None
            }
        })
        .collect()
}

fn responsive_listed(
    kind: &str,
    value: &StackAt,
    allowed: &[&str],
    normalize: fn(&str) -> &str,
) -> Vec<String> {
    BREAKPOINTS
        .iter()
        .filter_map(|breakpoint| {
            let token = value.at(breakpoint)?;
            if allowed.contains(&token) {
                Some(format!("d-stack--{breakpoint}-{kind}-{}", normalize(token)))
            } else {
                leptos::logging::warn!("DtStack {kind} `{token}` is not a stack {kind} token");
                None
            }
        })
        .collect()
}

/// `wrap` inserts `d-stack--wrap` immediately after `d-stack`.
fn stack_root_class(classes: String, wrap: bool, extra: &str) -> String {
    let with_wrap = if wrap {
        classes.replacen("d-stack", "d-stack d-stack--wrap", 1)
    } else {
        classes
    };
    join_class([with_wrap, extra.to_string()])
}

/// Class string for `DtStack`, in the Vue computed order.
pub(crate) fn stack_classes(
    direction: StackAt,
    gap: StackAt,
    align: Option<StackAt>,
    justify: StackAt,
) -> String {
    let mut parts = vec![
        "d-stack".to_string(),
        direction_class(&direction),
        listed_class("gap", gap.default, GAPS, identity),
        listed_class(
            "align",
            align.and_then(|value| value.default),
            ALIGNS,
            identity,
        ),
        listed_class("justify", justify.default, JUSTIFIES, justify_token),
    ];
    parts.extend(responsive_direction(&direction));
    parts.extend(responsive_listed("gap", &gap, GAPS, identity));
    if let Some(align) = align {
        parts.extend(responsive_listed("align", &align, ALIGNS, identity));
    }
    parts.extend(responsive_listed(
        "justify",
        &justify,
        JUSTIFIES,
        justify_token,
    ));
    join_class(parts)
}

/// Dialtone Vue `DtStack` (`dialtone/v10.7.0`).
///
/// `direction` defaults to `column`, which emits no direction class. The
/// string `default` emits `d-stack--column`. `gap` defaults to `0`
/// (`d-stack--gap-0`). `justify` defaults to `start`. `align` is unset unless
/// passed. `wrap` adds `d-stack--wrap`. `space-around`, `space-between`, and
/// `space-evenly` emit the shorthand suffixes. `around`, `between`, and
/// `evenly` still do. `d-stack--column` is not in the vendored CSS. `.d-stack`
/// already defaults to column and zero gap. `as` is the root element (`div`
/// by default).
#[component]
pub fn DtStack(
    /// Root element (`as`).
    #[prop(optional, default = "div")]
    r#as: &'static str,
    /// Main axis (`direction`). A string applies at every breakpoint.
    #[prop(optional, into, default = StackAt::all("column"))]
    direction: StackAt,
    /// Space between items (`gap`). A string applies at every breakpoint.
    #[prop(optional, into, default = StackAt::all("0"))]
    gap: StackAt,
    /// Cross axis (`align`). Unset uses the CSS default.
    #[prop(optional, into)]
    align: Option<StackAt>,
    /// Main-axis distribution (`justify`). A string applies at every breakpoint.
    #[prop(optional, into, default = StackAt::all("start"))]
    justify: StackAt,
    /// Wrap items (`wrap`).
    #[prop(optional)]
    wrap: bool,
    /// Extra classes appended after the stack classes.
    #[prop(optional, into)]
    class: String,
    /// Element id. Empty omits the attribute.
    #[prop(optional, into)]
    id: String,
    /// ARIA role. Empty omits the attribute.
    #[prop(optional, into)]
    role: String,
    /// `aria-label`. Empty omits the attribute.
    #[prop(optional, into)]
    aria_label: String,
    /// `aria-orientation`. Empty omits the attribute.
    #[prop(optional, into)]
    aria_orientation: String,
    /// `data-qa`. Default `dt-stack`. Segmented control sets its own.
    #[prop(optional, default = "dt-stack")]
    data_qa: &'static str,
    #[prop(optional)] on_keydown: Option<Callback<web_sys::KeyboardEvent>>,
    children: Children,
) -> impl IntoView {
    let class_name = stack_root_class(stack_classes(direction, gap, align, justify), wrap, &class);
    let id_attr = (!id.is_empty()).then_some(id);
    let role_attr = (!role.is_empty()).then_some(role);
    let aria_label_attr = (!aria_label.is_empty()).then_some(aria_label);
    let aria_orientation_attr = (!aria_orientation.is_empty()).then_some(aria_orientation);

    macro_rules! stack_el {
        ($tag:ident) => {
            view! {
                <$tag
                    id=id_attr.clone()
                    role=role_attr.clone()
                    class=class_name.clone()
                    aria-label=aria_label_attr.clone()
                    aria-orientation=aria_orientation_attr.clone()
                    data-qa=data_qa
                    on:keydown=move |event| {
                        if let Some(callback) = on_keydown {
                            callback.run(event);
                        }
                    }
                >
                    {children()}
                </$tag>
            }
            .into_any()
        };
    }

    match r#as {
        "section" => stack_el!(section),
        "span" => stack_el!(span),
        "nav" => stack_el!(nav),
        "ul" => stack_el!(ul),
        "ol" => stack_el!(ol),
        "li" => stack_el!(li),
        "article" => stack_el!(article),
        "header" => stack_el!(header),
        "footer" => stack_el!(footer),
        "main" => stack_el!(main),
        "aside" => stack_el!(aside),
        "p" => stack_el!(p),
        "form" => stack_el!(form),
        "div" => stack_el!(div),
        other => {
            leptos::logging::warn!(
                "DtStack `as` value `{other}` is not supported; rendering a div"
            );
            stack_el!(div)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_classes_omit_the_column_direction() {
        assert_eq!(
            stack_classes(
                StackAt::all("column"),
                StackAt::all("0"),
                None,
                StackAt::all("start")
            ),
            "d-stack d-stack--gap-0 d-stack--justify-start"
        );
        assert_eq!(
            stack_classes(
                StackAt::all("column"),
                StackAt::all("150"),
                None,
                StackAt::all("start")
            ),
            "d-stack d-stack--gap-150 d-stack--justify-start"
        );
    }

    #[test]
    fn direction_token_default_emits_column() {
        assert_eq!(
            stack_classes(
                StackAt::all("default"),
                StackAt::all("0"),
                None,
                StackAt::all("start")
            ),
            "d-stack d-stack--column d-stack--gap-0 d-stack--justify-start"
        );
    }

    #[test]
    fn row_replaces_the_default_direction() {
        assert_eq!(
            stack_classes(
                StackAt::all("row"),
                StackAt::all("0"),
                None,
                StackAt::all("start")
            ),
            "d-stack d-stack--row d-stack--gap-0 d-stack--justify-start"
        );
    }

    #[test]
    fn responsive_tokens_follow_breakpoint_order() {
        assert_eq!(
            stack_classes(
                StackAt::breakpoints(
                    Some("column"),
                    Some("row-reverse"),
                    Some("column-reverse"),
                    Some("row")
                ),
                StackAt::breakpoints(Some("200"), Some("400"), Some("500"), Some("600")),
                Some(StackAt::breakpoints(
                    Some("center"),
                    Some("end"),
                    Some("stretch"),
                    Some("baseline")
                )),
                StackAt::breakpoints(
                    Some("center"),
                    Some("end"),
                    Some("space-between"),
                    Some("space-around")
                ),
            ),
            "d-stack d-stack--sm-column d-stack--md-row-reverse d-stack--lg-column-reverse d-stack--xl-row d-stack--sm-gap-200 d-stack--md-gap-400 d-stack--lg-gap-500 d-stack--xl-gap-600 d-stack--sm-align-center d-stack--md-align-end d-stack--lg-align-stretch d-stack--xl-align-baseline d-stack--sm-justify-center d-stack--md-justify-end d-stack--lg-justify-between d-stack--xl-justify-around"
        );
    }

    #[test]
    fn wrap_follows_the_root_and_v10_gaps_replace_625() {
        assert_eq!(
            stack_root_class(
                stack_classes(
                    StackAt::all("row"),
                    StackAt::all("25"),
                    None,
                    StackAt::all("start")
                ),
                true,
                "extra"
            ),
            "d-stack d-stack--wrap d-stack--row d-stack--gap-25 d-stack--justify-start extra"
        );
        assert_eq!(
            stack_classes(
                StackAt::all("column"),
                StackAt::all("800"),
                None,
                StackAt::all("start")
            ),
            "d-stack d-stack--gap-800 d-stack--justify-start"
        );
        assert_eq!(
            stack_classes(
                StackAt::all("column"),
                StackAt::all("1"),
                None,
                StackAt::all("start")
            ),
            "d-stack d-stack--gap-1 d-stack--justify-start"
        );
        assert_eq!(
            stack_classes(
                StackAt::all("column"),
                StackAt::all("625"),
                None,
                StackAt::all("start")
            ),
            "d-stack d-stack--justify-start"
        );
    }

    #[test]
    fn justify_aliases_and_invalid_tokens() {
        assert_eq!(
            stack_classes(
                StackAt::all("nope"),
                StackAt::all("900"),
                Some(StackAt::all("invalid")),
                StackAt::all("between")
            ),
            "d-stack d-stack--justify-between"
        );
        assert_eq!(
            stack_classes(
                StackAt::all("row-reverse"),
                StackAt::all("500"),
                Some(StackAt::all("end")),
                StackAt::all("space-between")
            ),
            "d-stack d-stack--row-reverse d-stack--gap-500 d-stack--align-end d-stack--justify-between"
        );
    }
}
