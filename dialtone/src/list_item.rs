use leptos::prelude::*;
use web_sys::KeyboardEvent;

use super::icon::DtIconCheck;
use super::unique_id;

const ROLES: &[&str] = &["listitem", "menuitem", "option"];
const NAVIGATION: &[&str] = &["arrow-keys", "tab", "none"];
const TYPES: &[&str] = &["default", "custom"];
const ELEMENTS: &[&str] = &[
    "li", "div", "span", "p", "button", "a", "section", "article", "label",
];

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn list_role(value: &str) -> &'static str {
    match value {
        "menuitem" => "menuitem",
        "option" => "option",
        _ => "listitem",
    }
}

fn element_tag(value: &str) -> &'static str {
    match value {
        "div" => "div",
        "span" => "span",
        "p" => "p",
        "button" => "button",
        "a" => "a",
        "section" => "section",
        "article" => "article",
        "label" => "label",
        _ => "li",
    }
}

/// Root classes in Vue order: base, focusable, highlighted, static, then `class`.
fn list_item_class(navigation: &str, highlighted: bool, class: &str) -> String {
    join_class([
        "d-list-item",
        (navigation == "tab")
            .then_some("d-list-item--focusable")
            .unwrap_or(""),
        highlighted
            .then_some("d-list-item--highlighted")
            .unwrap_or(""),
        (navigation == "none")
            .then_some("d-list-item--static")
            .unwrap_or(""),
        class,
    ])
}

fn item_wrapper_class(wrapper_class: &str) -> String {
    join_class([
        "d-item-layout--custom",
        "d-list-item__wrapper",
        wrapper_class,
    ])
}

/// Vue `dynamicGridTemplateColumns`: empty columns stay in the string.
fn grid_template_columns(start: bool, end: bool, selected: bool) -> String {
    let start = if start { "auto" } else { "" };
    let end = if end { "auto" } else { "" };
    let selected = if selected { "auto" } else { "" };
    format!("grid-template-columns: {start} 1fr {end} {selected};")
}

/// `highlightId` wins when it is non-empty. Otherwise the pointer highlights a hoverable row.
fn item_highlighted(hoverable: bool, highlight_id: Option<&str>, id: &str, mouse: bool) -> bool {
    if !hoverable {
        return false;
    }
    match highlight_id {
        Some(current) if !current.is_empty() => current == id,
        _ => mouse,
    }
}

fn section_class(slot_class: &str, layout_class: &str) -> String {
    join_class([slot_class, layout_class])
}

/// Dialtone Vue `DtListItem` (`dialtone/v10.7.0`).
///
/// `navigation_type` (`navigationType`) is `arrow-keys`, `tab`, or `none`
/// (default). `tab` adds `d-list-item--focusable` and `tabindex="0"`. `none`
/// adds `d-list-item--static` and does not highlight. Anything else warns and
/// behaves as `arrow-keys` for hover. `type` is `default` or `custom`. `custom`
/// renders only the default slot. `element_type` (`elementType`) is the root
/// tag. An unknown tag warns and renders `li`. `role` is `listitem`, `menuitem`,
/// or `option`. `listitem` omits `aria-selected`. The other roles set it from
/// the highlight.
///
/// `highlight_id` (`highlightId`) is the id injected by
/// `keyboard_list_navigation`. That mixin is not a component. A non-empty value
/// replaces the mouse highlight. Pass a `Signal<String>`.
///
/// `class` is Vue `$attrs.class` on the root. `wrapper_class` (`wrapperClass`)
/// is extra classes on `.d-list-item__wrapper` and applies to `type="default"`.
/// `start` wins over the deprecated `left` slot. `end` wins over `right`.
/// `block_end` (`blockEnd`) wins over `bottom`. `selected` draws `DtIconCheck`
/// size `200` and replaces the `selected` slot (`selected_slot`).
///
/// Enter and Space call `on_click` and then `on_keydown`. The Vue event object
/// is not forwarded. The layout is inlined from `DtItemLayout` `unstyled`
/// because that port does not take the v10 section class props.
#[component]
pub fn DtListItem(
    /// Element id. Empty generates one.
    #[prop(optional, into)]
    id: String,
    /// `listitem` (default), `menuitem`, or `option`.
    #[prop(optional, default = "listitem")]
    role: &'static str,
    /// Root tag (`elementType`). Default `li`.
    #[prop(optional, default = "li")]
    element_type: &'static str,
    /// `default` or `custom`.
    #[prop(optional, default = "default")]
    r#type: &'static str,
    /// `arrow-keys`, `tab`, or `none` (`navigationType`). Default `none`.
    #[prop(optional, default = "none")]
    navigation_type: &'static str,
    /// Draws the check icon in the selected section.
    #[prop(optional)]
    selected: bool,
    /// Extra classes on the root (`$attrs.class`).
    #[prop(optional, into)]
    class: String,
    /// Extra classes on the layout (`wrapperClass`).
    #[prop(optional, into)]
    wrapper_class: String,
    /// Keyboard highlight id (`highlightId`).
    #[prop(optional)]
    highlight_id: Option<Signal<String>>,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
    #[prop(optional, into)] on_keydown: Option<Callback<()>>,
    #[prop(optional, into)] on_mousemove: Option<Callback<()>>,
    #[prop(optional, into)] on_mouseleave: Option<Callback<()>>,
    #[prop(optional)] start: Option<ViewFn>,
    /// Deprecated. Use `start`.
    #[prop(optional)]
    left: Option<ViewFn>,
    #[prop(optional)] subtitle: Option<ViewFn>,
    /// Content under the subtitle (`blockEnd`).
    #[prop(optional)]
    block_end: Option<ViewFn>,
    /// Deprecated. Use `block_end`.
    #[prop(optional)]
    bottom: Option<ViewFn>,
    #[prop(optional)] end: Option<ViewFn>,
    /// Deprecated. Use `end`.
    #[prop(optional)]
    right: Option<ViewFn>,
    /// `selected` slot. Ignored while `selected` is true.
    #[prop(optional)]
    selected_slot: Option<ViewFn>,
    children: Children,
) -> impl IntoView {
    if !ROLES.contains(&role) {
        leptos::logging::warn!("DtListItem role `{role}` is not listitem, menuitem, or option");
    }
    if !ELEMENTS.contains(&element_type) {
        leptos::logging::warn!(
            "DtListItem element_type `{element_type}` is not supported; using li"
        );
    }
    if !TYPES.contains(&r#type) {
        leptos::logging::warn!("DtListItem type `{type}` is not default or custom");
    }
    if !NAVIGATION.contains(&navigation_type) {
        leptos::logging::warn!(
            "DtListItem navigation_type `{navigation_type}` is not arrow-keys, tab, or none"
        );
    }
    let role = list_role(role);
    let tag = element_tag(element_type);
    let default_type = r#type == "default";
    let id = if id.is_empty() {
        unique_id("dt-list-item")
    } else {
        id
    };
    let hoverable = navigation_type != "none";
    let tabindex = if navigation_type == "tab" { "0" } else { "-1" };
    let mouse = RwSignal::new(false);
    let item_id = id.clone();
    let highlighted = Signal::derive(move || {
        let current = highlight_id.map(|signal| signal.get());
        item_highlighted(hoverable, current.as_deref(), &item_id, mouse.get())
    });
    let root_class = class;
    let navigation = navigation_type;
    let body = children();
    let start_slot = start.or(left);
    let end_slot = end.or(right);
    let bottom_slot = block_end.or(bottom);
    let has_start = start_slot.is_some();
    let has_end = end_slot.is_some();
    let has_selected = selected || selected_slot.is_some();
    let grid = grid_template_columns(has_start, has_end, has_selected);
    let wrapper = item_wrapper_class(&wrapper_class);
    let inner = if default_type {
        let subtitle_class = join_class([
            "d-list-item__subtitle",
            "d-item-layout--subtitle",
            "d-item-layout--subtitle--with-title",
        ]);
        view! {
            <div
                class=wrapper
                style=grid
                data-qa="dt-list-item-wrapper"
            >
                {start_slot.map(|slot| view! {
                    <section
                        class=section_class("d-list-item__left", "d-item-layout__left")
                        data-qa="dt-item-layout-left-wrapper"
                    >
                        {slot.run()}
                    </section>
                })}
                <section
                    class=section_class("d-list-item__content", "d-item-layout__content")
                    data-qa="dt-item-layout-content-wrapper"
                >
                    <div
                        class=section_class("d-list-item__title", "d-item-layout__title")
                        data-qa="dt-item-layout-title-wrapper"
                    >
                        {body}
                    </div>
                    {subtitle.map(|slot| view! {
                        <div class=subtitle_class.clone() data-qa="dt-item-layout-subtitle-wrapper">
                            {slot.run()}
                        </div>
                    })}
                    {bottom_slot.map(|slot| view! {
                        <div
                            class=section_class("d-list-item__bottom", "d-item-layout__bottom")
                            data-qa="dt-item-layout-bottom-wrapper"
                        >
                            {slot.run()}
                        </div>
                    })}
                </section>
                {end_slot.map(|slot| view! {
                    <section
                        class=section_class("d-list-item__right", "d-item-layout__right")
                        data-qa="dt-item-layout-right-wrapper"
                    >
                        {slot.run()}
                    </section>
                })}
                {has_selected.then(|| view! {
                    <section
                        class=section_class("d-list-item__selected", "d-item-layout__selected")
                        data-qa="dt-item-layout-selected-wrapper"
                    >
                        {if selected {
                            view! { <DtIconCheck size="200" /> }.into_any()
                        } else if let Some(slot) = selected_slot {
                            slot.run().into_any()
                        } else {
                            ().into_any()
                        }}
                    </section>
                })}
            </div>
        }
        .into_any()
    } else {
        body.into_any()
    };

    let class_for = move || list_item_class(navigation, highlighted.get(), &root_class);
    let aria_selected = move || {
        if role == "listitem" {
            None
        } else if highlighted.get() {
            Some("true")
        } else {
            Some("false")
        }
    };
    let activate = move || {
        if let Some(callback) = on_click {
            callback.run(());
        }
    };
    let on_key = move |event: KeyboardEvent| {
        let code = event.code();
        if code == "Enter" || code == "Space" {
            activate();
        }
        if let Some(callback) = on_keydown {
            callback.run(());
        }
    };
    let on_move = move |_| {
        mouse.set(true);
        if let Some(callback) = on_mousemove {
            callback.run(());
        }
    };
    let on_leave = move |_| {
        mouse.set(false);
        if let Some(callback) = on_mouseleave {
            callback.run(());
        }
    };

    macro_rules! shell {
        ($tag:ident) => {
            view! {
                <$tag
                    id=id.clone()
                    class=class_for
                    tabindex=tabindex
                    role=role
                    aria-selected=aria_selected
                    on:click=move |_| activate()
                    on:keydown=on_key
                    on:mousemove=on_move
                    on:mouseleave=on_leave
                >
                    {inner}
                </$tag>
            }
            .into_any()
        };
    }

    match tag {
        "div" => shell!(div),
        "span" => shell!(span),
        "p" => shell!(p),
        "button" => shell!(button),
        "a" => shell!(a),
        "section" => shell!(section),
        "article" => shell!(article),
        "label" => shell!(label),
        _ => shell!(li),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_item_class_follows_navigation() {
        assert_eq!(
            list_item_class("none", false, ""),
            "d-list-item d-list-item--static"
        );
        assert_eq!(
            list_item_class("tab", false, ""),
            "d-list-item d-list-item--focusable"
        );
        assert_eq!(
            list_item_class("arrow-keys", true, "d-pis-100"),
            "d-list-item d-list-item--highlighted d-pis-100"
        );
        assert_eq!(list_item_class("nope", false, ""), "d-list-item");
    }

    #[test]
    fn wrapper_and_grid_match_the_vue_layout() {
        assert_eq!(
            item_wrapper_class("custom-class"),
            "d-item-layout--custom d-list-item__wrapper custom-class"
        );
        assert_eq!(
            grid_template_columns(true, true, true),
            "grid-template-columns: auto 1fr auto auto;"
        );
        assert_eq!(
            grid_template_columns(false, false, false),
            "grid-template-columns:  1fr  ;"
        );
    }

    #[test]
    fn role_and_tag_fall_back() {
        assert_eq!(list_role("option"), "option");
        assert_eq!(list_role("button"), "listitem");
        assert_eq!(element_tag("div"), "div");
        assert_eq!(element_tag("table"), "li");
    }

    #[test]
    fn highlight_prefers_a_non_empty_injected_id() {
        assert!(item_highlighted(true, Some("a"), "a", false));
        assert!(!item_highlighted(true, Some("a"), "b", true));
        assert!(item_highlighted(true, Some(""), "a", true));
        assert!(item_highlighted(true, None, "a", true));
        assert!(!item_highlighted(false, None, "a", true));
    }
}
