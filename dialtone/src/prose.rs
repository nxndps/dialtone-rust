use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

/// `NodeFilter.SHOW_ELEMENT`.
const SHOW_ELEMENT: u32 = 1;

const URL_ATTRIBUTES: &[&str] = &["href", "src", "cite"];

const GLOBAL_ATTRIBUTES: &[&str] = &["id", "lang", "dir", "translate", "title", "role"];

const DISALLOWED_ELEMENTS: &[&str] = &[
    "form", "input", "select", "textarea", "button", "fieldset", "output", "datalist", "progress",
    "meter", "optgroup", "option",
];

/// Dialtone Vue `DtProse`.
///
/// The root is `div.d-prose` (`data-qa="dt-prose"`). `size` is `100`, `200`,
/// or `300` (default). `300` adds no size class. `density` is `100`, `200`
/// (default), or `300`. `200` adds no density class. An unknown token warns
/// and omits that modifier. `class` is `$attrs.class` on the root. `id` is
/// `$attrs.id`. Descendants lose attributes that are not in the Vue allow
/// list. Unsafe `href`, `src`, and `cite` values (`javascript`, `vbscript`,
/// `data`, `blob`, `file`) are removed. A debug build warns for form controls
/// and custom elements. `<input type="checkbox">` inside `<li>` does not warn.
/// The root is not sanitized. Vue uses `console.error` outside production.
/// This port uses `warn` in debug builds. `d-prose*` is in the vendored CSS.
#[component]
pub fn DtProse(
    /// Typography scale (`size`). `100`, `200`, or `300`.
    #[prop(optional, default = "300")]
    size: &'static str,
    /// Line height (`density`). `100`, `200`, or `300`.
    #[prop(optional, default = "200")]
    density: &'static str,
    /// `$attrs.class` on the root.
    #[prop(optional, into)]
    class: String,
    /// `$attrs.id` on the root. Empty omits it.
    #[prop(optional, into)]
    id: String,
    /// Default slot.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let class_name = prose_classes(size, density, &class);
    let id = (!id.is_empty()).then_some(id);
    let node_ref = NodeRef::<html::Div>::new();
    node_ref.on_load(|element| {
        let element: web_sys::Element = element.unchecked_into();
        scrub_prose(&element);
        let Some(observer) = watch_prose(element) else {
            return;
        };
        on_cleanup(move || observer.disconnect());
    });
    view! {
        <div node_ref=node_ref class=class_name id=id data-qa="dt-prose">
            {children.map(|children| children())}
        </div>
    }
}

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn size_modifier(size: &str) -> Option<&'static str> {
    match size {
        "100" => Some("d-prose--size-100"),
        "200" => Some("d-prose--size-200"),
        "300" => Some(""),
        _ => None,
    }
}

fn density_modifier(density: &str) -> Option<&'static str> {
    match density {
        "100" => Some("d-prose--density-100"),
        "200" => Some(""),
        "300" => Some("d-prose--density-300"),
        _ => None,
    }
}

fn prose_classes(size: &str, density: &str, class: &str) -> String {
    let size_class = match size_modifier(size) {
        Some(class) => class,
        None => {
            leptos::logging::warn!(
                "[DtProse] Invalid prop value: \"{size}\". Valid values: 100, 200, 300"
            );
            ""
        }
    };
    let density_class = match density_modifier(density) {
        Some(class) => class,
        None => {
            leptos::logging::warn!(
                "[DtProse] Invalid prop value: \"{density}\". Valid values: 100, 200, 300"
            );
            ""
        }
    };
    join_class(["d-prose", size_class, density_class, class])
}

fn allowed_for_tag(tag: &str) -> &'static [&'static str] {
    match tag {
        "a" => &["href", "target", "rel", "hreflang"],
        "img" => &["src", "alt", "width", "height", "loading", "decoding"],
        "th" => &["scope", "colspan", "rowspan", "headers"],
        "td" => &["colspan", "rowspan", "headers"],
        "ol" => &["start", "reversed", "type"],
        "blockquote" | "q" | "ins" | "del" => &["cite"],
        "time" => &["datetime"],
        "abbr" => &["title"],
        "input" => &["type", "checked", "disabled"],
        "details" => &["open"],
        "table" => &["summary"],
        "col" | "colgroup" => &["span"],
        _ => &[],
    }
}

fn attribute_allowed(tag: &str, name: &str) -> bool {
    GLOBAL_ATTRIBUTES.contains(&name)
        || name.starts_with("aria-")
        || allowed_for_tag(tag).contains(&name)
}

/// Vue `_isSafeProseUrl`. Empty and fragment URLs are safe.
fn prose_url_is_safe(value: &str) -> bool {
    let lower = value.trim_start().to_ascii_lowercase();
    !["javascript:", "vbscript:", "data:", "blob:", "file:"]
        .iter()
        .any(|scheme| lower.starts_with(scheme))
}

fn strip_prose_attribute(tag: &str, name: &str, value: &str) -> bool {
    if URL_ATTRIBUTES.contains(&name) && !prose_url_is_safe(value) {
        return true;
    }
    !attribute_allowed(tag, name)
}

fn prose_warning(tag: &str, input_type: Option<&str>, parent_tag: Option<&str>) -> Option<String> {
    if tag.contains('-') {
        return Some(format!(
            "DtProse: Custom element <{tag}> is not allowed inside prose content. Only plain HTML elements are permitted."
        ));
    }
    if !DISALLOWED_ELEMENTS.contains(&tag) {
        return None;
    }
    if tag == "input" && input_type == Some("checkbox") && parent_tag == Some("li") {
        return None;
    }
    Some(format!(
        "DtProse: <{tag}> is not allowed inside prose content. Form elements and interactive controls should not be used in prose blocks."
    ))
}

fn scrub_prose(root: &web_sys::Element) {
    let Some(document) = as_node(root).owner_document() else {
        return;
    };
    let Ok(walker) = document.create_tree_walker_with_what_to_show(as_node(root), SHOW_ELEMENT)
    else {
        return;
    };
    while let Ok(Some(node)) = walker.next_node() {
        let Some(element) = node.dyn_ref::<web_sys::Element>() else {
            continue;
        };
        let tag = element.tag_name().to_ascii_lowercase();
        let input_type = element.get_attribute("type");
        let parent_tag =
            parent_element(element).map(|parent| parent.tag_name().to_ascii_lowercase());
        strip_attributes(element, &tag);
        #[cfg(debug_assertions)]
        if let Some(message) = prose_warning(&tag, input_type.as_deref(), parent_tag.as_deref()) {
            leptos::logging::warn!("{message}");
        }
    }
}

fn strip_attributes(element: &web_sys::Element, tag: &str) {
    let names = element.get_attribute_names();
    let mut remove = Vec::new();
    for index in 0..names.length() {
        let Some(name) = names.get(index).as_string() else {
            continue;
        };
        let value = element.get_attribute(&name).unwrap_or_default();
        if strip_prose_attribute(tag, &name, &value) {
            remove.push(name);
        }
    }
    for name in remove {
        let _ = element.remove_attribute(&name);
    }
}

fn as_node(element: &web_sys::Element) -> &web_sys::Node {
    element.unchecked_ref()
}

fn parent_element(element: &web_sys::Element) -> Option<web_sys::Element> {
    as_node(element).parent_element()
}

fn prose_observer_options() -> web_sys::MutationObserverInit {
    let options = web_sys::MutationObserverInit::new();
    options.set_child_list(true);
    options.set_subtree(true);
    options.set_attributes(true);
    options
}

fn watch_prose(root: web_sys::Element) -> Option<web_sys::MutationObserver> {
    let watched = root.clone();
    let closure = Closure::wrap(Box::new(
        move |_records: js_sys::Array, observer: web_sys::MutationObserver| {
            observer.disconnect();
            scrub_prose(&watched);
            let _ = observer.observe_with_options(&watched, &prose_observer_options());
        },
    )
        as Box<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>);
    let Ok(observer) = web_sys::MutationObserver::new(closure.as_ref().unchecked_ref()) else {
        return None;
    };
    if observer
        .observe_with_options(&root, &prose_observer_options())
        .is_err()
    {
        observer.disconnect();
        return None;
    }
    closure.forget();
    Some(observer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes_omit_the_default_size_and_density() {
        assert_eq!(prose_classes("300", "200", ""), "d-prose");
        assert_eq!(prose_classes("100", "200", ""), "d-prose d-prose--size-100");
        assert_eq!(
            prose_classes("200", "300", "d-w100p"),
            "d-prose d-prose--size-200 d-prose--density-300 d-w100p"
        );
        assert_eq!(
            prose_classes("100", "100", ""),
            "d-prose d-prose--size-100 d-prose--density-100"
        );
        assert_eq!(size_modifier("300"), Some(""));
        assert_eq!(density_modifier("200"), Some(""));
        assert_eq!(size_modifier("400"), None);
        assert_eq!(density_modifier("md"), None);
    }

    #[test]
    fn strips_class_style_and_data_and_keeps_structure() {
        assert!(strip_prose_attribute("p", "class", "foo"));
        assert!(strip_prose_attribute("p", "style", "color: red"));
        assert!(strip_prose_attribute("p", "data-testid", "test"));
        assert!(strip_prose_attribute("p", "onclick", "evil()"));
        assert!(!strip_prose_attribute("p", "id", "intro"));
        assert!(!strip_prose_attribute("div", "role", "note"));
        assert!(!strip_prose_attribute("a", "aria-label", "home"));
        assert!(!strip_prose_attribute("a", "aria-describedby", "hint"));
        assert!(!strip_prose_attribute(
            "a",
            "href",
            "https://example.com/path"
        ));
        assert!(!strip_prose_attribute("a", "href", "#"));
        assert!(!strip_prose_attribute("img", "src", "image.png"));
        assert!(!strip_prose_attribute("img", "alt", "desc"));
        assert!(!strip_prose_attribute("th", "scope", "col"));
        assert!(strip_prose_attribute("th", "class", "header"));
        assert!(!strip_prose_attribute("input", "type", "checkbox"));
        assert!(!strip_prose_attribute("input", "checked", ""));
        assert!(!strip_prose_attribute("input", "disabled", ""));
    }

    #[test]
    fn strips_unsafe_url_schemes() {
        for value in [
            "javascript:alert(1)",
            "  javascript:alert(1)",
            "JavaScript:alert(1)",
            "vbscript:msg",
            "data:text/html,<script>x</script>",
            "blob:https://example.com/id",
            "file:///tmp/x",
        ] {
            assert!(strip_prose_attribute("a", "href", value), "{value}");
        }
        assert!(!strip_prose_attribute(
            "a",
            "href",
            "https://example.com/path"
        ));
        assert!(!strip_prose_attribute(
            "blockquote",
            "cite",
            "https://example.com"
        ));
        assert!(strip_prose_attribute(
            "blockquote",
            "cite",
            "javascript:alert(1)"
        ));
    }

    #[test]
    fn warns_for_form_controls_and_custom_elements_not_task_checkboxes() {
        assert!(prose_warning("form", None, Some("div"))
            .unwrap()
            .contains("<form> is not allowed"));
        assert!(prose_warning("input", Some("text"), Some("div"))
            .unwrap()
            .contains("<input> is not allowed"));
        assert!(prose_warning("select", None, Some("div")).is_some());
        assert!(prose_warning("textarea", None, Some("div")).is_some());
        assert!(prose_warning("button", None, Some("div")).is_some());
        assert!(prose_warning("input", Some("checkbox"), Some("li")).is_none());
        assert!(prose_warning("input", Some("checkbox"), Some("div")).is_some());
        assert!(prose_warning("my-component", None, Some("div"))
            .unwrap()
            .contains("<my-component> is not allowed"));
        assert!(prose_warning("p", None, Some("div")).is_none());
    }
}
