use leptos::prelude::*;

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// CSS suffix for `size`. `CODEBLOCK_SIZE_MAP`, then the token itself.
///
/// `100` is `xs`, `200` is `sm`, `300` is `md`, and `400` is `lg`. `xs`, `sm`,
/// `md`, and `lg` stay as written. Anything else warns and is still the suffix,
/// matching `CODEBLOCK_SIZE_MAP[size] || String(size)`.
pub fn codeblock_size_class(size: &str) -> String {
    match size {
        "100" => "xs".to_string(),
        "200" => "sm".to_string(),
        "300" => "md".to_string(),
        "400" => "lg".to_string(),
        "xs" | "sm" | "md" | "lg" => size.to_string(),
        other => {
            leptos::logging::warn!(
                "DtCodeblock size `{other}` is not 100, 200, 300, 400, xs, sm, md, or lg"
            );
            other.to_string()
        }
    }
}

pub(crate) fn codeblock_root_class(bordered: bool) -> &'static str {
    if bordered {
        "d-codeblock d-codeblock--bordered"
    } else {
        "d-codeblock"
    }
}

pub(crate) fn codeblock_code_class(size: &str) -> String {
    let suffix = codeblock_size_class(size);
    let modifier = format!("d-codeblock__code--{suffix}");
    join_class(["d-codeblock__code", modifier.as_str()])
}

/// Dialtone Vue `DtCodeblock` (`dialtone/v10.7.0`).
///
/// `text` is the source shown in the block. The `<pre>` and `<code>` stay on
/// one line so the block does not pick up extra whitespace. `size` defaults
/// to `200` (`d-codeblock__code--sm`). `bordered` adds `d-codeblock--bordered`.
#[component]
pub fn DtCodeblock(
    /// Source text (`text`).
    #[prop(into)]
    text: String,
    /// Styled container (`bordered`). Default false.
    #[prop(optional)]
    bordered: bool,
    /// Font size (`size`). Default `200`.
    #[prop(optional, default = "200")]
    size: &'static str,
) -> impl IntoView {
    let root = codeblock_root_class(bordered);
    let code = codeblock_code_class(size);
    view! {
        <pre class=root data-qa="dt-codeblock"><code class=code data-qa="dt-codeblock-code">{text}</code></pre>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_map_follows_vue() {
        assert_eq!(codeblock_size_class("100"), "xs");
        assert_eq!(codeblock_size_class("200"), "sm");
        assert_eq!(codeblock_size_class("300"), "md");
        assert_eq!(codeblock_size_class("400"), "lg");
        assert_eq!(codeblock_size_class("lg"), "lg");
        assert_eq!(codeblock_size_class("xs"), "xs");
    }

    #[test]
    fn classes_follow_the_template() {
        assert_eq!(codeblock_root_class(false), "d-codeblock");
        assert_eq!(
            codeblock_root_class(true),
            "d-codeblock d-codeblock--bordered"
        );
        assert_eq!(codeblock_code_class("200"), "d-codeblock__code d-codeblock__code--sm");
        assert_eq!(codeblock_code_class("400"), "d-codeblock__code d-codeblock__code--lg");
        assert_eq!(codeblock_code_class("lg"), "d-codeblock__code d-codeblock__code--lg");
    }
}
