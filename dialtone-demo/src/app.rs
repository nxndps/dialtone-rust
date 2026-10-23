use std::sync::Once;

use crate::catalog::Catalog;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use dialtone::{DtIconLaptop2, DtIconMoon, DtIconSun, DtSegmentedControl, DtSegmentedControlItem};

const THEME_KEY: &str = "dt-theme";

const GROUPS: &[(&str, &[(&str, &str)])] = &[
    (
        "Actions",
        &[
            ("breadcrumbs", "Breadcrumbs"),
            ("button", "Button"),
            ("button-group", "Button group"),
            ("split-button", "Split button"),
            ("dropdown", "Dropdown"),
            ("segmented-control", "Segmented control"),
        ],
    ),
    (
        "Data display",
        &[
            ("avatar", "Avatar"),
            ("presence", "Presence"),
            ("badge", "Badge"),
            ("card", "Card"),
            ("chip", "Chip"),
            ("codeblock", "Code block"),
            ("description-list", "Description list"),
            ("item-layout", "Item layout"),
            ("keyboard-shortcut", "Keyboard shortcut"),
            ("lazy-show", "Lazy show"),
            ("link", "Link"),
            ("list-item", "List item"),
            ("list-item-group", "List item group"),
            ("skeleton", "Skeleton"),
            ("text", "Text"),
            ("text-list", "Text list"),
            ("prose", "Prose"),
            ("emoji", "Emoji"),
            ("emoji-picker", "Emoji picker"),
            ("emoji-text-wrapper", "Emoji text wrapper"),
        ],
    ),
    (
        "Feedback",
        &[
            ("banner", "Banner"),
            ("empty-state", "Empty state"),
            ("loader", "Loader"),
            ("progress-circle", "Progress circle"),
            ("modal", "Modal"),
            ("mode-island", "Mode"),
            ("motion-text", "Motion text"),
            ("notice", "Notice"),
            ("pagination", "Pagination"),
            ("hovercard", "Hovercard"),
            ("popover", "Popover"),
            ("toast", "Toast"),
            ("tooltip", "Tooltip"),
            ("validation-messages", "Validation messages"),
        ],
    ),
    (
        "Data input",
        &[
            ("checkbox", "Checkbox"),
            ("checkbox-group", "Checkbox group"),
            ("combobox", "Combobox"),
            ("combobox-multi-select", "Combobox multi-select"),
            ("combobox-with-popover", "Combobox with popover"),
            ("datepicker", "Datepicker"),
            ("input", "Input"),
            ("input-group", "Input group"),
            ("radio", "Radio"),
            ("radio-group", "Radio group"),
            ("select-menu", "Select menu"),
            ("toggle", "Toggle"),
            ("filter-pill", "Filter pill"),
        ],
    ),
    (
        "Layout",
        &[
            ("box", "Box"),
            ("collapsible", "Collapsible"),
            ("resizable", "Resizable"),
            ("rich-text-editor", "Rich text editor"),
            ("root-layout", "Root layout"),
            ("scroller", "Scroller"),
            ("stack", "Stack"),
            ("tab", "Tab"),
            ("tab-group", "Tab group"),
        ],
    ),
    (
        "Icons",
        &[
            ("icon", "Icon"),
            ("illustration", "Illustration"),
            ("image-viewer", "Image viewer"),
        ],
    ),
];

fn stored_mode() -> String {
    let stored = web_sys::window()
        .and_then(|window| window.local_storage().ok().flatten())
        .and_then(|storage| storage.get_item(THEME_KEY).ok().flatten());
    match stored.as_deref() {
        Some(mode @ ("system" | "light" | "dark")) => mode.to_string(),
        _ => "light".to_string(),
    }
}

fn system_prefers_dark() -> bool {
    web_sys::window()
        .and_then(|window| {
            window
                .match_media("(prefers-color-scheme: dark)")
                .ok()
                .flatten()
        })
        .is_some_and(|query| query.matches())
}

fn mode_is_dark(mode: &str) -> bool {
    match mode {
        "dark" => true,
        "light" => false,
        _ => system_prefers_dark(),
    }
}

fn theme_apply(mode: &str) {
    let mode = if matches!(mode, "system" | "light" | "dark") {
        mode
    } else {
        "light"
    };
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        let _ = storage.set_item(THEME_KEY, mode);
    }
    let dark = mode_is_dark(mode);
    let apply = |element: &web_sys::Element| {
        let list = element.class_list();
        if dark {
            let _ = list.add_1("dt-theme-dark");
            let _ = list.add_1("dialtone-theme-dark");
            let _ = list.remove_1("dialtone-theme-light");
        } else {
            let _ = list.remove_1("dt-theme-dark");
            let _ = list.remove_1("dialtone-theme-dark");
            let _ = list.add_1("dialtone-theme-light");
        }
        let _ = element.set_attribute("data-dt-mode", if dark { "dark" } else { "light" });
    };
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        if let Some(html) = document.document_element() {
            apply(&html);
        }
        if let Some(body) = document.body() {
            apply(body.as_ref());
        }
    }
}

fn watch_system_mode() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let Some(window) = web_sys::window() else {
            return;
        };
        let Ok(Some(query)) = window.match_media("(prefers-color-scheme: dark)") else {
            return;
        };
        let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |_event| {
            if stored_mode() == "system" {
                theme_apply("system");
            }
        });
        let _ = query.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref());
        closure.forget();
    });
}

#[component]
pub(crate) fn ThemeSwitch() -> impl IntoView {
    let mode = RwSignal::new(stored_mode());
    theme_apply(&mode.get_untracked());
    watch_system_mode();
    view! {
        <div class="theme-mode">
            <DtSegmentedControl
                value=mode
                aria_label="Appearance mode"
                on_change=move |next: String| {
                    mode.set(next.clone());
                    theme_apply(&next);
                }
            >
                <DtSegmentedControlItem
                    value="system"
                    label="System"
                    start_icon=ViewFn::from(|| view! { <DtIconLaptop2 size="200" /> })
                />
                <DtSegmentedControlItem
                    value="light"
                    label="Light"
                    start_icon=ViewFn::from(|| view! { <DtIconSun size="200" /> })
                />
                <DtSegmentedControlItem
                    value="dark"
                    label="Dark"
                    start_icon=ViewFn::from(|| view! { <DtIconMoon size="200" /> })
                />
            </DtSegmentedControl>
        </div>
    }
}

#[component]
fn CatalogShell() -> impl IntoView {
    view! {
        <div class="demo-shell">
            <aside class="demo-side">
                <p class="demo-mark">
                    "Dialtone"
                    <span>"Leptos components"</span>
                </p>
                <a class="demo-docs-link" href="#/components">"Component docs"</a>
                <div class="demo-side-scroll">
                {GROUPS.iter().copied().map(|(title, links)| {
                    view! {
                        <p class="demo-kicker">{title}</p>
                        <nav class="demo-nav">
                            {links.iter().copied().map(|(id, label)| {
                                view! { <a href=format!("#{id}")>{label}</a> }
                            }).collect_view()}
                        </nav>
                    }
                }).collect_view()}
                </div>
                <div class="demo-side-foot">
                    <ThemeSwitch />
                </div>
            </aside>
            <main class="demo-main">
                <h1>"Components"</h1>
                <p class="demo-lead">
                    "Live Leptos ports of Dialtone. Each section loads as it comes into view. Each example is the component, then the call that renders it. "
                    <a href="#/components">"Component docs"</a>
                    " give each one its own page and a settings panel."
                </p>
                <Catalog />
            </main>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let screen = RwSignal::new(crate::docs::screen_from_location());
    crate::docs::install_hash_listener(screen);
    crate::docs::install_copy_listener();
    Effect::new(move |_| {
        crate::docs::sync_chrome(screen.get());
    });
    view! {
        {move || match screen.get() {
            crate::docs::Screen::Catalog => view! { <CatalogShell /> }.into_any(),
            screen => view! { <crate::docs::DocsShell screen=screen /> }.into_any(),
        }}
    }
}
