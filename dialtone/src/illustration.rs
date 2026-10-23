use leptos::prelude::*;

use super::unique_id;

/// Kebab-case names from `@dialpad/dialtone-icons` illustrations at `dialtone/v10.7.0`.
pub const ILLUSTRATION_NAMES: &[&str] = &[
    "access-denied",
    "atomic-pathway",
    "black-hole",
    "blank-space",
    "browser-table-graph",
    "dialpad-logo-black",
    "dialpad-logo-inverted",
    "dialpad-logo-white",
    "dialpad-logo",
    "dialpad-mono-black",
    "dialpad-mono-inverted",
    "dialpad-mono-white",
    "dialpad-mono",
    "file-upload",
    "login-error",
    "magnifier",
    "mic-detected",
    "mind",
    "mobile-full-notifications",
    "new-meeting",
    "new-message",
    "nothing-to-see-here",
    "orbital-harmony",
    "playlist",
    "publish-graphic",
    "something-went-wrong",
    "vector-vortex",
    "wireless-screenshare",
];

struct SvgParts {
    class_name: String,
    width: String,
    height: String,
    view_box: String,
    data_name: String,
    inner: String,
}

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn illustration_source(name: &str) -> Option<&'static str> {
    Some(match name {
        "access-denied" => include_str!("../assets/illustrations/access-denied.svg"),
        "atomic-pathway" => include_str!("../assets/illustrations/atomic-pathway.svg"),
        "black-hole" => include_str!("../assets/illustrations/black-hole.svg"),
        "blank-space" => include_str!("../assets/illustrations/blank-space.svg"),
        "browser-table-graph" => include_str!("../assets/illustrations/browser-table-graph.svg"),
        "dialpad-logo-black" => include_str!("../assets/illustrations/dialpad-logo-black.svg"),
        "dialpad-logo-inverted" => {
            include_str!("../assets/illustrations/dialpad-logo-inverted.svg")
        }
        "dialpad-logo-white" => include_str!("../assets/illustrations/dialpad-logo-white.svg"),
        "dialpad-logo" => include_str!("../assets/illustrations/dialpad-logo.svg"),
        "dialpad-mono-black" => include_str!("../assets/illustrations/dialpad-mono-black.svg"),
        "dialpad-mono-inverted" => {
            include_str!("../assets/illustrations/dialpad-mono-inverted.svg")
        }
        "dialpad-mono-white" => include_str!("../assets/illustrations/dialpad-mono-white.svg"),
        "dialpad-mono" => include_str!("../assets/illustrations/dialpad-mono.svg"),
        "file-upload" => include_str!("../assets/illustrations/file-upload.svg"),
        "login-error" => include_str!("../assets/illustrations/login-error.svg"),
        "magnifier" => include_str!("../assets/illustrations/magnifier.svg"),
        "mic-detected" => include_str!("../assets/illustrations/mic-detected.svg"),
        "mind" => include_str!("../assets/illustrations/mind.svg"),
        "mobile-full-notifications" => {
            include_str!("../assets/illustrations/mobile-full-notifications.svg")
        }
        "new-meeting" => include_str!("../assets/illustrations/new-meeting.svg"),
        "new-message" => include_str!("../assets/illustrations/new-message.svg"),
        "nothing-to-see-here" => include_str!("../assets/illustrations/nothing-to-see-here.svg"),
        "orbital-harmony" => include_str!("../assets/illustrations/orbital-harmony.svg"),
        "playlist" => include_str!("../assets/illustrations/playlist.svg"),
        "publish-graphic" => include_str!("../assets/illustrations/publish-graphic.svg"),
        "something-went-wrong" => include_str!("../assets/illustrations/something-went-wrong.svg"),
        "vector-vortex" => include_str!("../assets/illustrations/vector-vortex.svg"),
        "wireless-screenshare" => include_str!("../assets/illustrations/wireless-screenshare.svg"),
        _ => return None,
    })
}

fn attr_value<'a>(open: &'a str, name: &str) -> Option<&'a str> {
    for quote in ['"', '\''] {
        let pattern = format!("{name}={quote}");
        let Some(at) = open.find(&pattern) else {
            continue;
        };
        if !attr_boundary(open, at) {
            continue;
        }
        let rest = &open[at + pattern.len()..];
        let end = rest.find(quote)?;
        return Some(&rest[..end]);
    }
    None
}

fn attr_boundary(input: &str, at: usize) -> bool {
    if at == 0 {
        return true;
    }
    let prev = input[..at].chars().next_back().unwrap_or(' ');
    prev.is_whitespace() || prev == '<' || prev == ':'
}

fn split_svg(source: &str) -> Option<(&str, &str)> {
    let start = source.find("<svg")?;
    let rest = &source[start..];
    let end = rest.find('>')?;
    let open = &rest[..=end];
    let after = &rest[end + 1..];
    let close = after.rfind("</svg>")?;
    Some((open, &after[..close]))
}

fn find_marker(input: &str) -> Option<usize> {
    const NEEDLES: [&str; 5] = ["url(#", "href=\"#", "href='#", "id=\"", "id='"];
    let mut search_from = 0;
    while search_from < input.len() {
        let mut found: Option<(usize, &str)> = None;
        for needle in NEEDLES {
            if let Some(at) = input[search_from..].find(needle) {
                let at = search_from + at;
                if found.is_none_or(|(current, _)| at < current) {
                    found = Some((at, needle));
                }
            }
        }
        let Some((at, needle)) = found else {
            return None;
        };
        let needs_boundary = needle.starts_with("id") || needle.starts_with("href");
        if !needs_boundary || attr_boundary(input, at) {
            return Some(at);
        }
        search_from = at + 1;
    }
    None
}

/// Prefix gradient, mask, and clip ids. Vue's build only prefixes `fill`/`clip-path`
/// urls and gradient ids. Mask ids collide when two copies share a document, so
/// every `id` and `url(#…)` reference gets the same prefix.
fn prefix_svg_refs(input: &str, prefix: &str) -> String {
    let mut out = String::with_capacity(input.len() + prefix.len() * 4);
    let mut rest = input;
    while !rest.is_empty() {
        let Some(at) = find_marker(rest) else {
            out.push_str(rest);
            break;
        };
        out.push_str(&rest[..at]);
        rest = &rest[at..];
        let matched = ["url(#", "href=\"#", "href='#", "id=\"", "id='"]
            .into_iter()
            .find(|needle| rest.starts_with(needle));
        let Some(needle) = matched else {
            let mut chars = rest.chars();
            if let Some(ch) = chars.next() {
                out.push(ch);
                rest = chars.as_str();
            }
            continue;
        };
        out.push_str(needle);
        out.push_str(prefix);
        rest = &rest[needle.len()..];
    }
    out
}

fn illustration_class(base: &str, name: &str, extra: &str) -> String {
    let base = if base.is_empty() {
        format!("d-illustration d-illustration--{name}")
    } else {
        base.to_string()
    };
    join_class([base.as_str(), extra])
}

fn prepare_svg(source: &str, name: &str, prefix: &str, extra_class: &str) -> Option<SvgParts> {
    let (open, inner) = split_svg(source)?;
    let class_name = illustration_class(attr_value(open, "class").unwrap_or(""), name, extra_class);
    Some(SvgParts {
        class_name,
        width: attr_value(open, "width").unwrap_or("200").to_string(),
        height: attr_value(open, "height").unwrap_or("200").to_string(),
        view_box: attr_value(open, "viewBox")
            .unwrap_or("0 0 200 200")
            .to_string(),
        data_name: attr_value(open, "data-name").unwrap_or("").to_string(),
        inner: prefix_svg_refs(inner, prefix),
    })
}

/// Dialtone Vue `DtIllustration` (`dialtone/v10.7.0`).
///
/// `name` is the kebab-case illustration. An unknown name warns and renders
/// nothing. The root is the SVG, with `data-qa="dt-illustration"` unless
/// `data_qa` is set (`$attrs['data-qa']`). `class` is `$attrs.class` on that
/// SVG. The inner artwork is the vendored SVG. Ids are prefixed so two copies
/// do not share a gradient or mask. The inner illustration `ariaLabel` is not
/// a prop of `DtIllustration`, so the SVG stays `aria-hidden`.
#[component]
pub fn DtIllustration(
    /// Illustration name in kebab-case.
    #[prop(into)]
    name: String,
    /// `data-qa`. Vue defaults to `dt-illustration`.
    #[prop(optional, default = "dt-illustration")]
    data_qa: &'static str,
    /// Extra class on the SVG (`$attrs.class`).
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let Some(source) = illustration_source(&name) else {
        leptos::logging::warn!(
            "DtIllustration name \"{name}\" is not an illustration. Expected one of ILLUSTRATION_NAMES."
        );
        return ().into_any();
    };
    let Some(parts) = prepare_svg(source, &name, &unique_id("dt-illustration"), &class) else {
        leptos::logging::warn!("DtIllustration \"{name}\" has no SVG root.");
        return ().into_any();
    };
    let data_name = (!parts.data_name.is_empty()).then_some(parts.data_name);
    view! {
        <svg
            class=parts.class_name
            width=parts.width
            height=parts.height
            viewBox=parts.view_box
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            aria-hidden="true"
            focusable="false"
            role="img"
            data-qa=data_qa
            attr:data-name=data_name
            inner_html=parts.inner
        />
    }
    .into_any()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn referenced_ids_resolve(inner: &str) -> bool {
        let mut ids = Vec::new();
        let mut refs = Vec::new();
        collect_quoted(&mut ids, inner, "id=\"", '"');
        collect_quoted(&mut ids, inner, "id='", '\'');
        collect_until(&mut refs, inner, "url(#", ')');
        collect_quoted(&mut refs, inner, "href=\"#", '"');
        collect_quoted(&mut refs, inner, "href='#", '\'');
        refs.into_iter()
            .all(|id| ids.iter().any(|known| known == &id))
    }

    fn collect_quoted(out: &mut Vec<String>, input: &str, prefix: &str, quote: char) {
        let mut rest = input;
        while let Some(at) = rest.find(prefix) {
            rest = &rest[at + prefix.len()..];
            let Some(end) = rest.find(quote) else {
                break;
            };
            out.push(rest[..end].to_string());
            rest = &rest[end + 1..];
        }
    }

    fn collect_until(out: &mut Vec<String>, input: &str, prefix: &str, end: char) {
        let mut rest = input;
        while let Some(at) = rest.find(prefix) {
            rest = &rest[at + prefix.len()..];
            let Some(stop) = rest.find(end) else {
                break;
            };
            out.push(rest[..stop].to_string());
            rest = &rest[stop + 1..];
        }
    }

    #[test]
    fn every_name_has_a_vendored_svg() {
        for name in ILLUSTRATION_NAMES {
            let source = illustration_source(name).unwrap_or_else(|| panic!("{name} missing"));
            let parts = prepare_svg(source, name, "p", "").unwrap_or_else(|| panic!("{name} svg"));
            assert!(
                parts
                    .class_name
                    .contains(&format!("d-illustration--{name}")),
                "{name} class {}",
                parts.class_name
            );
            assert!(
                referenced_ids_resolve(&parts.inner),
                "{name} id reference missing"
            );
        }
    }

    #[test]
    fn unknown_name_has_no_source() {
        assert!(illustration_source("not-an-illustration").is_none());
    }

    #[test]
    fn prefixes_gradient_and_mask_ids() {
        let source = r##"<svg class="d-illustration d-illustration--mind" width="200" height="200" viewBox="0 0 200 200" data-name="Mind"><linearGradient id="paint0"></linearGradient><path fill="url(#paint0)" mask="url(#mask0)"/><mask id="mask0"></mask><use href="#paint0" xlink:href="#mask0"/></svg>"##;
        let parts = prepare_svg(source, "mind", "x", "extra").unwrap();
        assert_eq!(
            parts.class_name,
            "d-illustration d-illustration--mind extra"
        );
        assert_eq!(parts.data_name, "Mind");
        assert_eq!(parts.view_box, "0 0 200 200");
        assert!(parts.inner.contains("id=\"xpaint0\""));
        assert!(parts.inner.contains("id=\"xmask0\""));
        assert!(parts.inner.contains("url(#xpaint0)"));
        assert!(parts.inner.contains("url(#xmask0)"));
        assert!(parts.inner.contains("href=\"#xpaint0\""));
        assert!(parts.inner.contains("xlink:href=\"#xmask0\""));
        assert!(!parts.inner.contains("url(#paint0)"));
        assert!(referenced_ids_resolve(&parts.inner));
    }

    #[test]
    fn missing_class_uses_the_name_modifier() {
        let source = r#"<svg width="10" height="12" viewBox="0 0 10 12"></svg>"#;
        let parts = prepare_svg(source, "mind", "x", "").unwrap();
        assert_eq!(parts.class_name, "d-illustration d-illustration--mind");
        assert_eq!(parts.height, "12");
    }

    #[test]
    fn single_quoted_attrs_are_prefixed() {
        let source = r#"<svg class='d-illustration d-illustration--mind' width='10' height='10' viewBox='0 0 10 10'><path id='a' fill='url(#a)'/></svg>"#;
        let parts = prepare_svg(source, "mind", "z", "").unwrap();
        assert!(parts.inner.contains("id='za'"));
        assert!(parts.inner.contains("url(#za)"));
        assert!(referenced_ids_resolve(&parts.inner));
    }
}
