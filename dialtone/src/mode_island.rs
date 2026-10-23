use leptos::context::Provider;
use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

/// Dialtone Vue `DtModeIsland`.
///
/// The root is the `as` element (default `div`) with class `d-mode-island`.
/// `mode` is `inverted` (default), `light`, or `dark`. An unknown mode warns
/// and still inverts, and it does not set `data-mode-island-inverted`.
/// `inverted` is the opposite of a parent island, then the nearest ancestor
/// `data-dt-mode`, then the document element. `light` and `dark` stay fixed.
/// The island copies `data-dt-contrast` and `data-dt-material` from the
/// document element and watches those attributes. `class` and `id` are
/// `$attrs` on the root. `data_dt_brand` is `data-dt-brand`. Vue throws when
/// that attribute is set. This port warns and does not write it. An unknown
/// `as` warns and renders `div`. The `v-dt-mode` directive is not ported.
/// `d-mode-island` is in the vendored CSS.
#[component]
pub fn DtModeIsland(
    /// Element (`as`). `div`, `section`, `nav`, `article`, `aside`, `header`,
    /// `footer`, or `main`.
    #[prop(optional, default = "div")]
    r#as: &'static str,
    /// `inverted`, `light`, or `dark` (`mode`).
    #[prop(optional, default = "inverted")]
    mode: &'static str,
    /// `$attrs.class` on the root.
    #[prop(optional, into)]
    class: String,
    /// `$attrs.id` on the root. Empty omits it.
    #[prop(optional, into)]
    id: String,
    /// `data-dt-brand`. Non-empty warns and is not written.
    #[prop(optional, into)]
    data_dt_brand: String,
    /// Default slot.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    if let Some(message) = invalid_mode_message(mode) {
        leptos::logging::warn!("{message}");
    }
    if let Some(message) = brand_message(&data_dt_brand) {
        leptos::logging::warn!("{message}");
    }
    let tag = island_tag(r#as);
    let parent = use_context::<ModeIslandContext>();
    let contrast = RwSignal::new(read_root_attr("data-dt-contrast", "default"));
    let material = RwSignal::new(read_root_attr("data-dt-material", "sandstone"));
    let root_mode = RwSignal::new(read_root_attr("data-dt-mode", "light"));
    let ancestor = RwSignal::new(None::<String>);
    let resolved = Memo::new(move |_| {
        let parent_mode = parent.map(|ctx| ctx.mode.get());
        let ancestor_mode = ancestor.get();
        resolved_island_mode(
            mode,
            parent_mode.as_deref(),
            ancestor_mode.as_deref(),
            &root_mode.get(),
        )
    });
    let state = IslandWatch {
        contrast,
        material,
        root_mode,
        ancestor,
        watch_mode: mode != "light" && mode != "dark",
    };
    let class_name = island_class(&class);
    let id = (!id.is_empty()).then_some(id);
    let show_inverted = inverted_attribute(mode);
    let context = ModeIslandContext { mode: resolved };

    view! {
        <Provider value=context>
            {island_view(
                tag,
                class_name,
                id,
                show_inverted,
                resolved,
                contrast,
                material,
                state,
                children,
            )}
        </Provider>
    }
}

#[derive(Copy, Clone)]
struct ModeIslandContext {
    mode: Memo<&'static str>,
}

#[derive(Copy, Clone)]
struct IslandWatch {
    contrast: RwSignal<String>,
    material: RwSignal<String>,
    root_mode: RwSignal<String>,
    ancestor: RwSignal<Option<String>>,
    watch_mode: bool,
}

const ISLAND_TAGS: &[&str] = &[
    "div", "section", "nav", "article", "aside", "header", "footer", "main",
];

const BRAND_MESSAGE: &str = "[DtModeIsland] The data-dt-brand attribute is not allowed on mode islands. Brand should only be set at the root level.";

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn opposite_mode(current: &str) -> &'static str {
    if current == "light" {
        "dark"
    } else {
        "light"
    }
}

fn resolved_island_mode(
    mode: &str,
    parent_island: Option<&str>,
    ancestor_mode: Option<&str>,
    root_mode: &str,
) -> &'static str {
    match mode {
        "light" => "light",
        "dark" => "dark",
        _ => {
            let source = parent_island.or(ancestor_mode).unwrap_or(root_mode);
            opposite_mode(source)
        }
    }
}

fn inverted_attribute(mode: &str) -> bool {
    mode == "inverted"
}

fn invalid_mode_message(mode: &str) -> Option<String> {
    match mode {
        "inverted" | "light" | "dark" => None,
        _ => Some(format!(
            "[DtModeIsland] Invalid prop value: \"{mode}\". Valid values: inverted, light, dark"
        )),
    }
}

fn brand_message(value: &str) -> Option<&'static str> {
    if value.is_empty() {
        None
    } else {
        Some(BRAND_MESSAGE)
    }
}

fn island_tag(value: &str) -> &'static str {
    if let Some(tag) = ISLAND_TAGS.iter().copied().find(|tag| *tag == value) {
        return tag;
    }
    leptos::logging::warn!(
        "[DtModeIsland] Invalid prop value: \"{value}\". Valid values: div, section, nav, article, aside, header, footer, main"
    );
    "div"
}

fn island_class(class: &str) -> String {
    join_class(["d-mode-island", class])
}

fn read_root_attr(name: &str, fallback: &str) -> String {
    let Some(root) = document_element() else {
        return fallback.to_string();
    };
    match root.get_attribute(name) {
        Some(value) if !value.is_empty() => value,
        _ => fallback.to_string(),
    }
}

fn document_element() -> Option<web_sys::Element> {
    web_sys::window()?.document()?.document_element()
}

fn as_node(element: &web_sys::Element) -> &web_sys::Node {
    element.unchecked_ref()
}

fn parent_element(element: &web_sys::Element) -> Option<web_sys::Element> {
    as_node(element).parent_element()
}

fn is_document_element(element: &web_sys::Element) -> bool {
    let node = as_node(element);
    node.owner_document()
        .and_then(|document| document.document_element())
        .is_some_and(|root| node.is_same_node(Some(as_node(&root))))
}

/// Nearest ancestor `data-dt-mode`, skipping the document element. Vue
/// `findParentMode` then falls back to the root mode.
fn find_parent_mode(element: &web_sys::Element) -> String {
    let mut parent = parent_element(element);
    while let Some(node) = parent {
        if is_document_element(&node) {
            break;
        }
        if node.has_attribute("data-dt-mode") {
            return node.get_attribute("data-dt-mode").unwrap_or_default();
        }
        parent = parent_element(&node);
    }
    read_root_attr("data-dt-mode", "light")
}

fn attribute_options(name: &str) -> web_sys::MutationObserverInit {
    let options = web_sys::MutationObserverInit::new();
    options.set_attributes(true);
    let filter = js_sys::Array::of1(&wasm_bindgen::JsValue::from_str(name));
    options.set_attribute_filter(filter.as_ref());
    options
}

fn watch_root_attr(
    name: &'static str,
    slot: RwSignal<String>,
    fallback: &'static str,
) -> Option<web_sys::MutationObserver> {
    let root = document_element()?;
    let closure = Closure::wrap(Box::new(
        move |_records: js_sys::Array, _observer: web_sys::MutationObserver| {
            slot.set(read_root_attr(name, fallback));
        },
    )
        as Box<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>);
    let Ok(observer) = web_sys::MutationObserver::new(closure.as_ref().unchecked_ref()) else {
        return None;
    };
    if observer
        .observe_with_options(&root, &attribute_options(name))
        .is_err()
    {
        observer.disconnect();
        return None;
    }
    closure.forget();
    Some(observer)
}

fn watch_ancestor_mode(
    element: web_sys::Element,
    root_mode: RwSignal<String>,
    ancestor: RwSignal<Option<String>>,
) -> Option<web_sys::MutationObserver> {
    let watched = element.clone();
    let closure = Closure::wrap(Box::new(
        move |_records: js_sys::Array, _observer: web_sys::MutationObserver| {
            root_mode.set(read_root_attr("data-dt-mode", "light"));
            ancestor.set(Some(find_parent_mode(&watched)));
        },
    )
        as Box<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>);
    let Ok(observer) = web_sys::MutationObserver::new(closure.as_ref().unchecked_ref()) else {
        return None;
    };
    let options = attribute_options("data-dt-mode");
    let Some(root) = document_element() else {
        return None;
    };
    if observer.observe_with_options(&root, &options).is_err() {
        observer.disconnect();
        return None;
    }
    let mut parent = parent_element(&element);
    while let Some(node) = parent {
        if is_document_element(&node) {
            break;
        }
        if node.has_attribute("data-dt-mode") {
            let _ = observer.observe_with_options(&node, &options);
        }
        parent = parent_element(&node);
    }
    closure.forget();
    Some(observer)
}

fn mount_island(element: web_sys::Element, state: IslandWatch) {
    state
        .contrast
        .set(read_root_attr("data-dt-contrast", "default"));
    state
        .material
        .set(read_root_attr("data-dt-material", "sandstone"));
    state.root_mode.set(read_root_attr("data-dt-mode", "light"));
    state.ancestor.set(Some(find_parent_mode(&element)));
    let mut observers = Vec::new();
    if let Some(observer) = watch_root_attr("data-dt-contrast", state.contrast, "default") {
        observers.push(observer);
    }
    if let Some(observer) = watch_root_attr("data-dt-material", state.material, "sandstone") {
        observers.push(observer);
    }
    if state.watch_mode {
        if let Some(observer) = watch_ancestor_mode(element, state.root_mode, state.ancestor) {
            observers.push(observer);
        }
    }
    on_cleanup(move || {
        for observer in observers {
            observer.disconnect();
        }
    });
}

fn island_view(
    tag: &'static str,
    class_name: String,
    id: Option<String>,
    show_inverted: bool,
    resolved: Memo<&'static str>,
    contrast: RwSignal<String>,
    material: RwSignal<String>,
    state: IslandWatch,
    children: Option<Children>,
) -> AnyView {
    macro_rules! island {
        ($el:ty, $tag:ident) => {{
            let node_ref = NodeRef::<$el>::new();
            let state = state;
            node_ref.on_load(move |el| {
                let element: web_sys::Element = el.unchecked_into();
                mount_island(element, state);
            });
            view! {
                <$tag
                    node_ref=node_ref
                    class=class_name.clone()
                    id=id.clone()
                    data-dt-mode=move || resolved.get()
                    data-dt-contrast=move || contrast.get()
                    data-dt-material=move || material.get()
                    data-mode-island-inverted=show_inverted
                >
                    {children.map(|children| children())}
                </$tag>
            }
            .into_any()
        }};
    }

    match tag {
        "section" => island!(html::Section, section),
        "nav" => island!(html::Nav, nav),
        "article" => island!(html::Article, article),
        "aside" => island!(html::Aside, aside),
        "header" => island!(html::Header, header),
        "footer" => island!(html::Footer, footer),
        "main" => island!(html::Main, main),
        _ => island!(html::Div, div),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opposite_of_light_is_dark_and_anything_else_is_light() {
        assert_eq!(opposite_mode("light"), "dark");
        assert_eq!(opposite_mode("dark"), "light");
        assert_eq!(opposite_mode("inverted"), "light");
        assert_eq!(opposite_mode(""), "light");
    }

    #[test]
    fn inverted_uses_parent_island_before_ancestor_and_root() {
        assert_eq!(
            resolved_island_mode("inverted", Some("dark"), Some("light"), "light"),
            "light"
        );
        assert_eq!(
            resolved_island_mode("inverted", Some("light"), None, "light"),
            "dark"
        );
        assert_eq!(
            resolved_island_mode("inverted", None, Some("dark"), "light"),
            "light"
        );
        assert_eq!(
            resolved_island_mode("inverted", None, None, "light"),
            "dark"
        );
        assert_eq!(
            resolved_island_mode("inverted", None, None, "dark"),
            "light"
        );
    }

    #[test]
    fn explicit_mode_ignores_parent_and_unknown_mode_still_inverts() {
        assert_eq!(
            resolved_island_mode("light", Some("dark"), Some("dark"), "dark"),
            "light"
        );
        assert_eq!(
            resolved_island_mode("dark", Some("light"), None, "light"),
            "dark"
        );
        assert_eq!(resolved_island_mode("blue", None, None, "light"), "dark");
        assert!(inverted_attribute("inverted"));
        assert!(!inverted_attribute("light"));
        assert!(!inverted_attribute("blue"));
        assert!(invalid_mode_message("inverted").is_none());
        assert!(invalid_mode_message("blue")
            .unwrap()
            .contains("Valid values: inverted, light, dark"));
    }

    #[test]
    fn brand_message_matches_the_vue_error() {
        assert!(brand_message("").is_none());
        assert_eq!(
            brand_message("dialpad"),
            Some(
                "[DtModeIsland] The data-dt-brand attribute is not allowed on mode islands. Brand should only be set at the root level."
            )
        );
    }

    #[test]
    fn island_class_and_tag() {
        assert_eq!(island_class(""), "d-mode-island");
        assert_eq!(island_class("d-p-200"), "d-mode-island d-p-200");
        assert_eq!(island_tag("section"), "section");
        assert_eq!(island_tag("div"), "div");
        assert_eq!(island_tag("span"), "div");
    }
}
