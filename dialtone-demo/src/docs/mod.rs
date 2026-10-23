mod class_suggest;
mod copy;
mod guide;
mod highlight;
pub(crate) mod pages;
mod show_code;
pub(crate) use copy::install_copy_listener;
use leptos::prelude::*;
pub(crate) use show_code::ShowCodeText;
use std::sync::Once;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

/// Hash route for the demo. Empty and `#/catalog` keep the single-page catalog.
/// `#/components` is the grid. `#/components/{id}` is one component.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Screen {
    Catalog,
    Index,
    Page(&'static str),
}

pub(crate) struct Doc {
    pub id: &'static str,
    pub name: &'static str,
    /// Page on dialtone.dialpad.com, without `.html`.
    pub slug: Option<&'static str>,
}

/// One-line description from the staging Dialtone page. Code block has no page there.
pub(crate) fn summary(id: &str) -> &'static str {
    guide::summary(id).unwrap_or(match id {
        "codeblock" => "Preformatted source text.",
        "scroller" => "Virtual list that mounts the rows near the viewport.",
        "segmented-control" => "A single-choice group of buttons.",
        "select-menu" => "A native select with a label and validation messages.",
        "split-button" => "A main action and a second button for more actions.",
        "stack" => "A column or row of items with gap, alignment, and justification.",
        "tab" => "One v10 tab button. Outside a group, selected is its own state.",
        "tab-group" => "Tabs and panels that share one selected panel id.",
        "text" => "Typography compositions for body, headline, and label text.",
        "text-list" => "An ordered or unordered list of text items.",
        "validation-messages" => "Error, warning, and success messages. Error wins.",
        "emoji" => "An emoji image from a shortcode or unicode character.",
        "emoji-picker" => "Searchable emoji grid with skin tone and a selected emoji.",
        "emoji-text-wrapper" => {
            "Replaces :shortcodes: and unicode emoji in a string with emoji images."
        }
        "empty-state" => "Headline and body for a screen with nothing to show.",
        "filter-pill" => "A button that filters a list from a menu or checkboxes.",
        "resizable" => {
            "A layout component that allows users to resize panels by dragging handles between them."
        }
        "rich-text-editor" => "Text editing surface with formatting and links.",
        _ => "",
    })
}

const DOCS: &[Doc] = &[
    Doc {
        id: "breadcrumbs",
        name: "Breadcrumbs",
        slug: Some("breadcrumbs"),
    },
    Doc {
        id: "button",
        name: "Button",
        slug: Some("button"),
    },
    Doc {
        id: "button-group",
        name: "Button group",
        slug: Some("button-group"),
    },
    Doc {
        id: "split-button",
        name: "Split button",
        slug: Some("split-button"),
    },
    Doc {
        id: "dropdown",
        name: "Dropdown",
        slug: Some("dropdown"),
    },
    Doc {
        id: "segmented-control",
        name: "Segmented control",
        slug: Some("segmented-control"),
    },
    Doc {
        id: "avatar",
        name: "Avatar",
        slug: Some("avatar"),
    },
    Doc {
        id: "presence",
        name: "Presence",
        slug: Some("presence"),
    },
    Doc {
        id: "badge",
        name: "Badge",
        slug: Some("badge"),
    },
    Doc {
        id: "card",
        name: "Card",
        slug: Some("card"),
    },
    Doc {
        id: "chip",
        name: "Chip",
        slug: Some("chip"),
    },
    Doc {
        id: "codeblock",
        name: "Code block",
        slug: Some("codeblock"),
    },
    Doc {
        id: "description-list",
        name: "Description list",
        slug: Some("description-list"),
    },
    Doc {
        id: "item-layout",
        name: "Item layout",
        slug: Some("item-layout"),
    },
    Doc {
        id: "keyboard-shortcut",
        name: "Keyboard shortcut",
        slug: Some("keyboard-shortcut"),
    },
    Doc {
        id: "lazy-show",
        name: "Lazy show",
        slug: Some("lazy-show"),
    },
    Doc {
        id: "link",
        name: "Link",
        slug: Some("link"),
    },
    Doc {
        id: "list-item",
        name: "List item",
        slug: Some("list-item"),
    },
    Doc {
        id: "list-item-group",
        name: "List item group",
        slug: Some("list-item-group"),
    },
    Doc {
        id: "skeleton",
        name: "Skeleton",
        slug: Some("skeleton"),
    },
    Doc {
        id: "text",
        name: "Text",
        slug: Some("text"),
    },
    Doc {
        id: "text-list",
        name: "Text list",
        slug: Some("text-list"),
    },
    Doc {
        id: "prose",
        name: "Prose",
        slug: Some("prose"),
    },
    Doc {
        id: "emoji",
        name: "Emoji",
        slug: Some("emoji"),
    },
    Doc {
        id: "emoji-picker",
        name: "Emoji picker",
        slug: Some("emoji-picker"),
    },
    Doc {
        id: "emoji-text-wrapper",
        name: "Emoji text wrapper",
        slug: Some("emoji-text-wrapper"),
    },
    Doc {
        id: "banner",
        name: "Banner",
        slug: Some("banner"),
    },
    Doc {
        id: "empty-state",
        name: "Empty state",
        slug: Some("empty-state"),
    },
    Doc {
        id: "loader",
        name: "Loader",
        slug: Some("loader"),
    },
    Doc {
        id: "progress-circle",
        name: "Progress circle",
        slug: Some("progress-circle"),
    },
    Doc {
        id: "modal",
        name: "Modal",
        slug: Some("modal"),
    },
    Doc {
        id: "mode-island",
        name: "Mode",
        slug: Some("mode-island"),
    },
    Doc {
        id: "motion-text",
        name: "Motion text",
        slug: Some("motion-text"),
    },
    Doc {
        id: "notice",
        name: "Notice",
        slug: Some("notice"),
    },
    Doc {
        id: "pagination",
        name: "Pagination",
        slug: Some("pagination"),
    },
    Doc {
        id: "hovercard",
        name: "Hovercard",
        slug: Some("hovercard"),
    },
    Doc {
        id: "popover",
        name: "Popover",
        slug: Some("popover"),
    },
    Doc {
        id: "toast",
        name: "Toast",
        slug: Some("toast"),
    },
    Doc {
        id: "tooltip",
        name: "Tooltip",
        slug: Some("tooltip"),
    },
    Doc {
        id: "validation-messages",
        name: "Validation messages",
        slug: Some("validation-messages"),
    },
    Doc {
        id: "checkbox",
        name: "Checkbox",
        slug: Some("checkbox"),
    },
    Doc {
        id: "checkbox-group",
        name: "Checkbox group",
        slug: Some("checkbox-group"),
    },
    Doc {
        id: "combobox",
        name: "Combobox",
        slug: Some("combobox"),
    },
    Doc {
        id: "combobox-multi-select",
        name: "Combobox multi-select",
        slug: Some("combobox"),
    },
    Doc {
        id: "combobox-with-popover",
        name: "Combobox with popover",
        slug: Some("combobox"),
    },
    Doc {
        id: "datepicker",
        name: "Datepicker",
        slug: Some("datepicker"),
    },
    Doc {
        id: "input",
        name: "Input",
        slug: Some("input"),
    },
    Doc {
        id: "input-group",
        name: "Input group",
        slug: Some("input-group"),
    },
    Doc {
        id: "radio",
        name: "Radio",
        slug: Some("radio"),
    },
    Doc {
        id: "radio-group",
        name: "Radio group",
        slug: Some("radio-group"),
    },
    Doc {
        id: "select-menu",
        name: "Select menu",
        slug: Some("select-menu"),
    },
    Doc {
        id: "toggle",
        name: "Toggle",
        slug: Some("toggle"),
    },
    Doc {
        id: "filter-pill",
        name: "Filter pill",
        slug: Some("filter-pill"),
    },
    Doc {
        id: "box",
        name: "Box",
        slug: Some("box"),
    },
    Doc {
        id: "collapsible",
        name: "Collapsible",
        slug: Some("collapsible"),
    },
    Doc {
        id: "resizable",
        name: "Resizable",
        slug: Some("resizable"),
    },
    Doc {
        id: "rich-text-editor",
        name: "Rich text editor",
        slug: Some("rich-text-editor"),
    },
    Doc {
        id: "root-layout",
        name: "Root layout",
        slug: Some("root-layout"),
    },
    Doc {
        id: "scroller",
        name: "Scroller",
        slug: Some("scroller"),
    },
    Doc {
        id: "stack",
        name: "Stack",
        slug: Some("stack"),
    },
    Doc {
        id: "tab",
        name: "Tab",
        slug: Some("tab"),
    },
    Doc {
        id: "tab-group",
        name: "Tab group",
        slug: Some("tabs"),
    },
    Doc {
        id: "icon",
        name: "Icon",
        slug: Some("icon"),
    },
    Doc {
        id: "illustration",
        name: "Illustration",
        slug: Some("illustration"),
    },
    Doc {
        id: "image-viewer",
        name: "Image viewer",
        slug: Some("image-viewer"),
    },
];

const GROUPS: &[(&str, &[&str])] = &[
    (
        "Actions",
        &[
            "breadcrumbs",
            "button",
            "button-group",
            "split-button",
            "dropdown",
            "segmented-control",
        ],
    ),
    (
        "Data display",
        &[
            "avatar",
            "presence",
            "badge",
            "card",
            "chip",
            "codeblock",
            "description-list",
            "item-layout",
            "keyboard-shortcut",
            "lazy-show",
            "link",
            "list-item",
            "list-item-group",
            "skeleton",
            "text",
            "text-list",
            "prose",
            "emoji",
            "emoji-picker",
            "emoji-text-wrapper",
        ],
    ),
    (
        "Feedback",
        &[
            "banner",
            "empty-state",
            "loader",
            "progress-circle",
            "modal",
            "mode-island",
            "motion-text",
            "notice",
            "pagination",
            "hovercard",
            "popover",
            "toast",
            "tooltip",
            "validation-messages",
        ],
    ),
    (
        "Forms",
        &[
            "checkbox",
            "checkbox-group",
            "combobox",
            "combobox-multi-select",
            "combobox-with-popover",
            "datepicker",
            "input",
            "input-group",
            "radio",
            "radio-group",
            "select-menu",
            "toggle",
            "filter-pill",
        ],
    ),
    (
        "Layout",
        &[
            "box",
            "collapsible",
            "resizable",
            "rich-text-editor",
            "root-layout",
            "scroller",
            "stack",
            "tab",
            "tab-group",
        ],
    ),
    ("Icons", &["icon", "illustration", "image-viewer"]),
];

pub(crate) fn find(id: &str) -> Option<&'static Doc> {
    DOCS.iter().find(|doc| doc.id == id)
}

pub(crate) fn screen_from_location() -> Screen {
    let hash = web_sys::window()
        .and_then(|window| window.location().hash().ok())
        .unwrap_or_default();
    screen_from_hash(&hash)
}

fn screen_from_hash(hash: &str) -> Screen {
    let path = hash.trim().trim_start_matches('#').trim_start_matches('/');
    if path.is_empty() || path == "catalog" {
        return Screen::Catalog;
    }
    if path == "components" {
        return Screen::Index;
    }
    if let Some(id) = path.strip_prefix("components/") {
        let id = id.trim_matches('/');
        if let Some(doc) = find(id) {
            return Screen::Page(doc.id);
        }
        return Screen::Index;
    }
    // Catalog section anchors (`#button`) stay on the single-page catalog.
    Screen::Catalog
}

pub(crate) fn install_hash_listener(screen: RwSignal<Screen>) {
    static ONCE: Once = Once::new();
    ONCE.call_once(move || {
        let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |_event| {
            let next = screen_from_location();
            if screen.get_untracked() != next {
                screen.set(next);
            }
        });
        if let Some(window) = web_sys::window() {
            let _ = window
                .add_event_listener_with_callback("hashchange", closure.as_ref().unchecked_ref());
        }
        closure.forget();
    });
}

fn title_for(screen: Screen) -> String {
    match screen {
        Screen::Catalog => "Dialtone components".to_string(),
        Screen::Index => "Components — Dialtone".to_string(),
        Screen::Page(id) => find(id)
            .map(|doc| format!("{} — Dialtone", doc.name))
            .unwrap_or_else(|| "Components — Dialtone".to_string()),
    }
}

pub(crate) fn sync_chrome(screen: Screen) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    document.set_title(&title_for(screen));
    window.scroll_to_with_x_and_y(0.0, 0.0);
    for selector in [".docs-main", ".docs-page-scroll"] {
        if let Ok(Some(node)) = document.query_selector(selector) {
            node.set_scroll_top(0);
        }
    }
}

fn scroll_section(id: &str) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Some(element) = document.get_element_by_id(id) else {
        return;
    };
    let container = document
        .query_selector(".docs-page-scroll")
        .ok()
        .flatten()
        .or_else(|| document.query_selector(".docs-main").ok().flatten());
    let Some(container) = container else {
        element.scroll_into_view_with_bool(true);
        return;
    };
    let header = document
        .get_element_by_id("page-sticky-header")
        .map(|node| node.get_bounding_client_rect().height())
        .unwrap_or(0.0);
    let top = container.scroll_top() as f64 + element.get_bounding_client_rect().top()
        - container.get_bounding_client_rect().top()
        - header
        - 16.0;
    container.set_scroll_top(top.max(0.0) as i32);
}

fn component_ids() -> Vec<&'static str> {
    GROUPS
        .iter()
        .flat_map(|(_, ids)| ids.iter().copied())
        .collect()
}

fn neighbors(id: &str) -> (Option<&'static Doc>, Option<&'static Doc>) {
    let ids = component_ids();
    let Some(index) = ids.iter().position(|item| *item == id) else {
        return (None, None);
    };
    let prev = index.checked_sub(1).and_then(|at| find(ids[at]));
    let next = ids.get(index + 1).copied().and_then(find);
    (prev, next)
}

fn has_variants(id: &str) -> bool {
    !matches!(
        id,
        "scroller"
            | "segmented-control"
            | "select-menu"
            | "stack"
            | "tab"
            | "text"
            | "text-list"
            | "validation-messages"
    )
}

/// Move `#playground` under the sticky header, or into the right-hand dock at
/// the Dialtone `xxxl` breakpoint (1764px). One observer for the page lifetime.
fn watch_playground_dock() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(document) = window.document() else {
            return;
        };
        let Some(body) = document.body() else {
            return;
        };
        if let Ok(Some(query)) = window.match_media("(min-width: 1764px)") {
            let on_change = Closure::<dyn FnMut(web_sys::Event)>::new(move |_event| {
                dock_playground();
            });
            let _ = query
                .add_event_listener_with_callback("change", on_change.as_ref().unchecked_ref());
            on_change.forget();
            std::mem::forget(query);
        }
        let closure = Closure::wrap(Box::new(
            move |_records: js_sys::Array, _observer: web_sys::MutationObserver| {
                dock_playground();
            },
        )
            as Box<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>);
        let Ok(observer) = web_sys::MutationObserver::new(closure.as_ref().unchecked_ref()) else {
            return;
        };
        let options = web_sys::MutationObserverInit::new();
        options.set_child_list(true);
        options.set_subtree(true);
        let _ = observer.observe_with_options(&body, &options);
        closure.forget();
        std::mem::forget(observer);
    });
    dock_playground();
}

/// `/` focuses the sidebar search, unless the key would type into another field.
fn watch_search_hotkey() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let Some(document) = web_sys::window().and_then(|window| window.document()) else {
            return;
        };
        let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
            let Ok(event) = event.dyn_into::<web_sys::KeyboardEvent>() else {
                return;
            };
            focus_docs_search(event);
        });
        let _ =
            document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
        closure.forget();
    });
}

fn focus_docs_search(event: web_sys::KeyboardEvent) {
    if event.key() != "/"
        || event.default_prevented()
        || event.is_composing()
        || event.alt_key()
        || event.ctrl_key()
        || event.meta_key()
    {
        return;
    }
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        return;
    };
    let target = event
        .target()
        .and_then(|node| node.dyn_into::<web_sys::Element>().ok())
        .or_else(|| document.active_element());
    if target.as_ref().is_some_and(accepts_text) {
        return;
    }
    let Ok(Some(input)) = document.query_selector(".docs-search") else {
        return;
    };
    let Some(input) = input.dyn_ref::<web_sys::HtmlElement>() else {
        return;
    };
    event.prevent_default();
    let _ = input.focus();
}

fn accepts_text(element: &web_sys::Element) -> bool {
    if element
        .closest("[contenteditable=''], [contenteditable='true'], [role='combobox'], [role='searchbox'], [role='spinbutton'], [role='textbox'], select, textarea")
        .ok()
        .flatten()
        .is_some()
    {
        return true;
    }
    let Ok(input) = element.clone().dyn_into::<web_sys::HtmlInputElement>() else {
        return false;
    };
    let kind = input.type_().to_ascii_lowercase();
    !matches!(
        kind.as_str(),
        "button"
            | "checkbox"
            | "color"
            | "file"
            | "hidden"
            | "image"
            | "radio"
            | "range"
            | "reset"
            | "submit"
    )
}

fn dock_playground() {
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        return;
    };
    let Ok(Some(play)) = document.query_selector("section#playground") else {
        return;
    };
    let wide = web_sys::window()
        .and_then(|window| window.match_media("(min-width: 1764px)").ok())
        .flatten()
        .is_some_and(|query| query.matches());
    let target_id = if wide {
        "combinator-wide-target"
    } else {
        "combinator-inline-target"
    };
    let Some(target) = document.get_element_by_id(target_id) else {
        return;
    };
    if play
        .parent_element()
        .is_some_and(|parent| parent.id() == target_id)
    {
        return;
    }
    let _ = target.append_child(&play);
}

#[component]
pub fn DocsShell(screen: Screen) -> impl IntoView {
    let query = RwSignal::new(String::new());
    watch_search_hotkey();
    view! {
        <div class="docs-shell">
            <header class="docs-top">
                <a class="docs-brand" href=format!("#/components")>
                    "Dialtone"
                    <span>"Components"</span>
                </a>
                <span class="docs-top-fill"></span>
                <a class="docs-catalog-link" href=format!("#/catalog")>"Catalog"</a>
            </header>
            <div class="docs-body">
                <aside class="docs-side">
                    <div class="docs-search-pin">
                        <span class="docs-search-icon" aria-hidden="true">
                            <dialtone::DtIconSearch size="200" />
                        </span>
                        <input
                            class="docs-search"
                            type="search"
                            placeholder="Search"
                            aria-label="Search components"
                            aria-keyshortcuts="/"
                            prop:value=move || query.get()
                            on:input=move |event| query.set(event_target_value(&event))
                        />
                        <kbd
                            class="docs-search-key"
                            class:docs-search-key--hidden=move || !query.get().is_empty()
                            aria-hidden="true"
                        >
                            "/"
                        </kbd>
                    </div>
                    <div class="docs-side-scroll">
                    <a class="docs-back" href=format!("#/catalog")>"All on one page"</a>
                    {move || {
                        let q = query.get().trim().to_lowercase();
                        let sections = GROUPS.iter().copied().filter_map(|(title, ids)| {
                            let links: Vec<&Doc> = ids
                                .iter()
                                .copied()
                                .filter_map(find)
                                .filter(|doc| {
                                    q.is_empty()
                                        || doc.name.to_lowercase().contains(&q)
                                        || doc.id.contains(q.as_str())
                                        || summary(doc.id).to_lowercase().contains(&q)
                                })
                                .collect();
                            if links.is_empty() {
                                return None;
                            }
                            Some(view! {
                                <div>
                                    <p class="demo-kicker">{title}</p>
                                    <nav class="demo-nav docs-nav">
                                        {links.into_iter().map(|doc| {
                                            let href = format!("#/components/{}", doc.id);
                                            let active = matches!(screen, Screen::Page(current) if current == doc.id);
                                            view! {
                                                <a
                                                    href=href
                                                    class=if active { "is-active" } else { "" }
                                                >
                                                    {doc.name}
                                                </a>
                                            }
                                        }).collect_view()}
                                    </nav>
                                </div>
                            })
                        }).collect::<Vec<_>>();
                        if sections.is_empty() {
                            view! { <p class="docs-empty">"No components match."</p> }.into_any()
                        } else {
                            sections.into_iter().collect_view().into_any()
                        }
                    }}
                    </div>
                    <div class="docs-side-foot">
                        <crate::app::ThemeSwitch />
                    </div>
                </aside>
                <main class="docs-main">
                    {match screen {
                        Screen::Page(id) => view! { <DocArticle id=id /> }.into_any(),
                        _ => view! { <DocsIndex /> }.into_any(),
                    }}
                </main>
            </div>
        </div>
    }
}

#[component]
fn DocsIndex() -> impl IntoView {
    view! {
        <p class="docs-kicker">"Dialtone"</p>
        <h1>"Components"</h1>
        <p class="docs-blurb">
            "Each Leptos port has its own page, with a live preview and settings. Previews load as they come into view. The "
            <a href=format!("#/catalog")>"catalog"</a>
            " keeps every example on one page."
        </p>
        {GROUPS.iter().copied().map(|(title, ids)| {
            view! {
                <h2 class="docs-group">{title}</h2>
                <div class="docs-grid">
                    {ids.iter().copied().map(|id| view! { <DocCard id=id /> }).collect_view()}
                </div>
            }
        }).collect_view()}
    }
}

#[component]
fn DocCard(id: &'static str) -> impl IntoView {
    let doc = find(id).expect("doc id");
    let href = format!("#/components/{}", doc.id);
    let preview = match id {
        "datepicker" | "root-layout" | "banner" | "emoji-picker" | "empty-state" => {
            "docs-card-preview docs-card-preview--top"
        }
        _ => "docs-card-preview",
    };
    view! {
        <a class="docs-card" href=href>
            <div class=preview aria-hidden="true">
                <crate::chunks::LazyThumb id=id />
            </div>
            <h2>{doc.name}</h2>
            <p>{summary(id)}</p>
        </a>
    }
}

#[component]
fn TocList(id: &'static str) -> impl IntoView {
    let items = guide::toc(id);
    let variants = has_variants(id);
    view! {
        {items.into_iter().map(|item| {
            let target = item.id.clone();
            let level = item.level.to_string();
            view! {
                <button type="button" data-level=level on:click=move |_| scroll_section(&target)>
                    {item.text}
                </button>
            }
        }).collect_view()}
        {variants.then(|| {
            view! {
                <button type="button" data-level="2" on:click=move |_| scroll_section("variants")>
                    "Variants"
                </button>
            }
        })}
    }
}

fn pager_link(doc: &'static Doc, next: bool) -> impl IntoView {
    let href = format!("#/components/{}", doc.id);
    let class = if next {
        "docs-pager docs-pager--next"
    } else {
        "docs-pager"
    };
    let kicker = if next { "Next" } else { "Previous" };
    view! {
        <a class=class href=href>
            <span class="docs-pager-kicker">{kicker}</span>
            <span>{doc.name}</span>
        </a>
    }
}

#[component]
fn DocArticle(id: &'static str) -> impl IntoView {
    let doc = find(id).expect("doc id");
    let slug = doc.slug;
    let blurb = summary(id);
    let (prev, next) = neighbors(id);
    let show_toc = !guide::toc(id).is_empty() || has_variants(id);
    watch_playground_dock();
    view! {
        <div class="docs-page">
            <div class="docs-page-column docs-page-scroll">
                <header id="page-sticky-header" class="docs-page-head">
                    <p class="docs-crumb">
                        <a href=format!("#/components")>"Components"</a>
                        <span aria-hidden="true">"/"</span>
                        {doc.name}
                    </p>
                    <div class="docs-page-title-row">
                        <h1>{doc.name}</h1>
                        <div class="docs-page-actions">
                            {guide::storybook(id).map(|href| {
                                view! {
                                    <a class="docs-action" href=href target="_blank" rel="noreferrer">"Storybook"</a>
                                }
                            })}
                            {slug.map(|slug| {
                                let href = format!("https://dialtone.dialpad.com/components/{slug}.html");
                                view! {
                                    <a class="docs-action" href=href target="_blank" rel="noreferrer">"Dialtone docs"</a>
                                }
                            })}
                        </div>
                    </div>
                    {(!blurb.is_empty()).then(|| view! { <p class="docs-page-desc">{blurb}</p> })}
                    {show_toc.then(|| {
                        view! {
                            <details class="docs-toc-menu">
                                <summary>"On this page"</summary>
                                <div class="docs-toc-menu-list">
                                    <TocList id=id />
                                </div>
                            </details>
                        }
                    })}
                </header>
                <div id="combinator-inline-target"></div>
                <article class="docs-article">
                    {guide::body_html(id).map(|html| {
                        view! { <div id="documentation" class="docs-md" inner_html=html></div> }
                    })}
                    <crate::chunks::LazyDocs id=id />
                </article>
                <nav class="docs-page-pager" aria-label="Components">
                    {prev.map(|doc| pager_link(doc, false))}
                    {next.map(|doc| pager_link(doc, true))}
                </nav>
            </div>
            <div id="combinator-wide-target" class="docs-combinator-dock"></div>
            {show_toc.then(|| {
                view! {
                    <aside class="docs-toc">
                        <p>"On this page"</p>
                        <TocList id=id />
                    </aside>
                }
            })}
        </div>
    }
}
