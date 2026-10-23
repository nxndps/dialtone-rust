use std::time::Duration;

use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, KeyboardEvent, MouseEvent};

use super::button::DtButton;
use super::emoji::{emoji_group, filter_emojis, searchable_emojis, DtEmojiData};
use super::icon::{DtIconClose, DtIconSearch};
use super::input::DtInput;
use super::tab::DtTab;
use super::tooltip::DtTooltip;
use super::unique_id;

/// Emoji buttons in one picker row. Dialtone `EMOJIS_PER_ROW`.
pub const EMOJIS_PER_ROW: usize = 9;
/// Picker image root. Dialtone `CDN_URL`.
pub const EMOJI_PICKER_CDN: &str = "https://static.dialpadcdn.com/joypixels/png/unicode/32/";
/// Dialtone search debounce in the emoji selector.
const SEARCH_DEBOUNCE_MS: u64 = 300;

const SKIN_TONES: &[&str] = &[
    "Default",
    "Light",
    "MediumLight",
    "Medium",
    "MediumDark",
    "Dark",
];

const TAB_KEYS: &[&str] = &[
    "Recently used",
    "People",
    "Nature",
    "Food",
    "Activity",
    "Travel",
    "Objects",
    "Symbols",
    "Flags",
    "Custom",
];

const TAB_LABELS: &[&str] = &[
    "Most recently used",
    "Smileys and people",
    "Nature",
    "Food",
    "Activity",
    "Travel",
    "Objects",
    "Symbols",
    "Flags",
    "Custom",
];

const SKIN_CHOICES: &[(&str, &str, &str)] = &[
    (":wave_tone1:", "1f44b-1f3fb", "Light"),
    (":wave_tone2:", "1f44b-1f3fc", "MediumLight"),
    (":wave_tone3:", "1f44b-1f3fd", "Medium"),
    (":wave_tone4:", "1f44b-1f3fe", "MediumDark"),
    (":wave_tone5:", "1f44b-1f3ff", "Dark"),
    (":wave:", "1f44b", "Default"),
];

/// Emoji chosen from [`DtEmojiPicker`], including whether Shift was held.
#[derive(Clone, Debug, PartialEq)]
pub struct DtEmojiSelection {
    pub emoji: DtEmojiData,
    /// Vue adds `shift_key` on the selected object.
    pub shift_key: bool,
}

pub(crate) fn category_storage_key(tab: &str, skin: &str) -> String {
    match tab {
        "People" | "Activity" | "Objects" => format!("{tab}{skin}"),
        other => other.to_string(),
    }
}

pub(crate) fn visible_tabs(
    has_recent: bool,
    has_custom: bool,
) -> Vec<(&'static str, &'static str)> {
    TAB_KEYS
        .iter()
        .zip(TAB_LABELS.iter())
        .filter(|(key, _)| {
            if **key == "Recently used" {
                return has_recent;
            }
            if **key == "Custom" {
                return has_custom;
            }
            true
        })
        .map(|(key, label)| (*key, *label))
        .collect()
}

fn in_grid(counts: &[usize], tab: isize, index: isize) -> Option<(usize, usize)> {
    if tab < 0 || index < 0 {
        return None;
    }
    let tab = tab as usize;
    let index = index as usize;
    counts
        .get(tab)
        .filter(|len| index < **len)
        .map(|_| (tab, index))
}

/// Next focused emoji for the category grid. `counts` is the length of each tab.
pub(crate) fn next_emoji(
    key: &str,
    tab: usize,
    index: usize,
    counts: &[usize],
) -> Option<(usize, usize)> {
    if counts.is_empty() || counts.get(tab).is_none() {
        return None;
    }
    let per = EMOJIS_PER_ROW as isize;
    match key {
        "ArrowRight" => in_grid(counts, tab as isize, index as isize + 1)
            .or_else(|| in_grid(counts, tab as isize + 1, 0))
            .or_else(|| in_grid(counts, 0, 0)),
        "ArrowLeft" => {
            if let Some(next) = in_grid(counts, tab as isize, index as isize - 1) {
                return Some(next);
            }
            if tab > 0 {
                let last = counts[tab - 1].saturating_sub(1) as isize;
                return in_grid(counts, tab as isize - 1, last);
            }
            let last_tab = counts.len() - 1;
            let last = counts[last_tab].saturating_sub(1) as isize;
            in_grid(counts, last_tab as isize, last)
        }
        "ArrowDown" => {
            if let Some(next) = in_grid(counts, tab as isize, index as isize + per) {
                return Some(next);
            }
            let position = (index % EMOJIS_PER_ROW) as isize;
            let row_marker = index as isize + (per - position);
            if in_grid(counts, tab as isize, row_marker).is_some() {
                let last = counts[tab].saturating_sub(1) as isize;
                return in_grid(counts, tab as isize, last);
            }
            in_grid(counts, tab as isize + 1, position)
                .or_else(|| in_grid(counts, 0, position))
                .or_else(|| in_grid(counts, 0, counts[0].saturating_sub(1) as isize))
        }
        "ArrowUp" => {
            let position = index % EMOJIS_PER_ROW;
            if tab == 0 {
                let last_tab = counts.len() - 1;
                let last_len = counts[last_tab];
                let missing = EMOJIS_PER_ROW - (last_len % EMOJIS_PER_ROW);
                let jump = last_len as isize + missing as isize
                    - (EMOJIS_PER_ROW as isize - position as isize);
                return in_grid(counts, last_tab as isize, jump).or_else(|| {
                    in_grid(
                        counts,
                        last_tab as isize,
                        last_len.saturating_sub(1) as isize,
                    )
                });
            }
            if let Some(next) = in_grid(counts, tab as isize, index as isize - per) {
                return Some(next);
            }
            let previous = tab - 1;
            let previous_len = counts[previous];
            let last_pos = previous_len - (previous_len % EMOJIS_PER_ROW) + position;
            in_grid(counts, previous as isize, last_pos as isize).or_else(|| {
                in_grid(
                    counts,
                    previous as isize,
                    previous_len.saturating_sub(1) as isize,
                )
            })
        }
        _ => None,
    }
}

/// Next index in the filtered grid. The filtered list is a single tab.
pub(crate) fn next_filtered(key: &str, index: usize, len: usize) -> Option<usize> {
    if len == 0 {
        return None;
    }
    let per = EMOJIS_PER_ROW;
    match key {
        "ArrowRight" => Some(if index + 1 < len { index + 1 } else { 0 }),
        "ArrowLeft" => Some(index.checked_sub(1).unwrap_or(len - 1)),
        "ArrowDown" => {
            if index + per < len {
                Some(index + per)
            } else {
                let position = index % per;
                if index + (per - position) < len {
                    Some(len - 1)
                } else {
                    Some(position.min(len - 1))
                }
            }
        }
        "ArrowUp" => {
            if let Some(next) = index.checked_sub(per) {
                Some(next)
            } else {
                let position = index % per;
                let last = len - (len % per) + position;
                if last < len {
                    Some(last)
                } else {
                    Some(len - 1)
                }
            }
        }
        _ => None,
    }
}

pub(crate) fn picker_emoji_src(emoji: &DtEmojiData) -> String {
    if !emoji.date_added.is_empty() {
        emoji.image.clone()
    } else {
        format!("{EMOJI_PICKER_CDN}{}.png", emoji.unicode_character)
    }
}

/// `loading` for a category or search image. `None` leaves the browser eager.
pub(crate) fn grid_image_loading(lazy_images: bool) -> Option<&'static str> {
    lazy_images.then_some("lazy")
}

fn focus_element(id: &str) {
    let Some(element) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.get_element_by_id(id))
    else {
        return;
    };
    if let Ok(element) = element.dyn_into::<HtmlElement>() {
        let _ = element.focus();
    }
}

fn category_emojis(
    tab: &str,
    skin: &str,
    recent: &[DtEmojiData],
    custom: &[DtEmojiData],
) -> Vec<DtEmojiData> {
    match tab {
        "Recently used" => recent.to_vec(),
        "Custom" => custom.to_vec(),
        _ => emoji_group(&category_storage_key(tab, skin)),
    }
}

/// Dialtone Vue `DtEmojiPicker` (`dialtone/v9.190.0`).
///
/// Category data is `@dialpad/dialtone-emojis` from that tag. Images use
/// `https://static.dialpadcdn.com/joypixels/png/unicode/32/`. Search waits
/// 300ms. `skin_tone` is `Default`, `Light`, `MediumLight`, `Medium`,
/// `MediumDark`, or `Dark`. Tab buttons show the category label. Vue draws
/// icon-only tabs; those icon components are not in this crate.
/// `lazy_images` defaults to true and sets `loading="lazy"` on the category
/// grid and on search results. The footer preview and the skin-tone images
/// stay eager. Vue has no such prop. The catalog JSON stays in the bundle,
/// and every emoji button still mounts.
/// `d-emoji-picker--body` is emitted and is not in the vendored CSS.
#[component]
pub fn DtEmojiPicker(
    /// Recently used emoji (`recentlyUsedEmojis`).
    #[prop(optional)]
    recently_used: Option<Signal<Vec<DtEmojiData>>>,
    /// Custom emoji for the custom tab (`customEmojis`).
    #[prop(optional)]
    custom_emojis: Option<Signal<Vec<DtEmojiData>>>,
    /// Controlled skin tone (`skinTone`). Unset starts at `Default`.
    #[prop(optional)]
    skin_tone: Option<Signal<String>>,
    /// Called with the next skin tone (`skin-tone`).
    #[prop(optional, into)]
    on_skin_tone: Option<Callback<String>>,
    /// Controlled search string (`searchQuery`).
    #[prop(optional)]
    search_query: Option<Signal<String>>,
    /// Called when the search field changes.
    #[prop(optional, into)]
    on_search_query: Option<Callback<String>>,
    /// Show the search field (`showSearch`).
    #[prop(optional, default = true)]
    show_search: bool,
    /// Defer category and search images until they near the list (`lazyImages`).
    /// The footer preview and skin-tone images stay eager. Vue has no such prop.
    #[prop(optional, default = true)]
    lazy_images: bool,
    /// Footer button when nothing is highlighted (`showAddEmojiButton`).
    #[prop(optional)]
    show_add_emoji_button: bool,
    /// Selected emoji (`selected-emoji`).
    #[prop(optional, into)]
    on_selected_emoji: Option<Callback<DtEmojiSelection>>,
    /// List scrolled to the end (`scroll-bottom-reached`).
    #[prop(optional, into)]
    on_scroll_bottom: Option<Callback<()>>,
    /// Escape (`close`).
    #[prop(optional, into)]
    on_close: Option<Callback<()>>,
    /// Add-emoji button (`add-emoji`).
    #[prop(optional, into)]
    on_add_emoji: Option<Callback<()>>,
) -> impl IntoView {
    let picker_id = StoredValue::new(unique_id("dt-emoji-picker"));
    let list_id = StoredValue::new(format!("{}-list", picker_id.get_value()));
    let search_id = StoredValue::new(format!("{}-search", picker_id.get_value()));
    let initial_skin = skin_tone
        .map(|signal| signal.get_untracked())
        .unwrap_or_else(|| "Default".to_string());
    if !SKIN_TONES.contains(&initial_skin.as_str()) {
        leptos::logging::warn!("DtEmojiPicker skin_tone `{initial_skin}` is not a skin tone");
    }
    let skin = RwSignal::new(initial_skin);
    Effect::new(move |_| {
        if let Some(signal) = skin_tone {
            skin.set(signal.get());
        }
    });
    let initial_query = search_query
        .map(|signal| signal.get_untracked())
        .unwrap_or_default();
    let query = RwSignal::new(initial_query.clone());
    let debounced = RwSignal::new(initial_query);
    Effect::new(move |_| {
        if let Some(signal) = search_query {
            let next = signal.get();
            query.set(next);
        }
    });
    let search_ticket = StoredValue::new(0u32);
    Effect::new(move |_| {
        let next = query.get();
        let ticket = search_ticket.get_value().wrapping_add(1);
        search_ticket.set_value(ticket);
        if next.is_empty() {
            debounced.set(String::new());
            return;
        }
        let pending = next.clone();
        if set_timeout_with_handle(
            move || {
                if search_ticket.get_value() == ticket {
                    debounced.set(pending);
                }
            },
            Duration::from_millis(SEARCH_DEBOUNCE_MS),
        )
        .is_err()
        {
            debounced.set(next);
        }
    });

    let highlighted = RwSignal::new(None::<DtEmojiData>);
    let hover_first = RwSignal::new(true);
    let skin_open = RwSignal::new(false);
    let fixed_index = RwSignal::new(0usize);
    let list_ref = NodeRef::<html::Div>::new();
    let search_ref = NodeRef::<html::Input>::new();
    let on_selected_emoji = StoredValue::new(on_selected_emoji);
    let on_scroll_bottom = StoredValue::new(on_scroll_bottom);
    let on_close = StoredValue::new(on_close);
    let on_add_emoji = StoredValue::new(on_add_emoji);
    let on_skin_tone = StoredValue::new(on_skin_tone);
    let on_search_query = StoredValue::new(on_search_query);

    Effect::new(move |_| {
        if show_search {
            if let Some(input) = search_ref.get() {
                let _ = input.focus();
            }
        }
    });

    let recent_now = move || recently_used.map(|signal| signal.get()).unwrap_or_default();
    let custom_now = move || custom_emojis.map(|signal| signal.get()).unwrap_or_default();

    let select_emoji = move |emoji: DtEmojiData, shift_key: bool| {
        if let Some(callback) = on_selected_emoji.get_value() {
            callback.run(DtEmojiSelection { emoji, shift_key });
        }
    };

    let on_list_scroll = move |_| {
        let Some(list) = list_ref.get() else {
            return;
        };
        let list: HtmlElement = list.unchecked_into();
        let scroll = list.scroll_top();
        if scroll + list.client_height() >= list.scroll_height() {
            if let Some(callback) = on_scroll_bottom.get_value() {
                callback.run(());
            }
        }
        if !debounced.get_untracked().is_empty() {
            return;
        }
        let Ok(nodes) = list.query_selector_all("[data-emoji-category]") else {
            return;
        };
        let mut current = 0usize;
        for index in 0..nodes.length() {
            let Some(node) = nodes.item(index) else {
                continue;
            };
            let Ok(element) = node.dyn_into::<HtmlElement>() else {
                continue;
            };
            if element.offset_top() - 15 <= scroll {
                current = index as usize;
            }
        }
        fixed_index.set(current);
    };

    let focus_emoji = move |tab: usize, index: usize| {
        let id = picker_id.get_value();
        focus_element(&format!("{id}-emoji-{tab}-{index}"));
    };
    let focus_filtered = move |index: usize| {
        let id = picker_id.get_value();
        focus_element(&format!("{id}-filtered-{index}"));
    };

    view! {
        <div
            class="d-emoji-picker"
            on:keydown=move |event: KeyboardEvent| {
                if event.key() == "Escape" {
                    if let Some(callback) = on_close.get_value() {
                        callback.run(());
                    }
                }
            }
        >
            <div class="d-emoji-picker--header">
                <div class="d-emoji-picker__tabset">
                    <div class="d-emoji-picker__tabset-list">
                    {move || {
                        let tabs = visible_tabs(!recent_now().is_empty(), !custom_now().is_empty());
                        let searching = !debounced.get().is_empty();
                        let selected = fixed_index.get();
                        tabs.into_iter().enumerate().map(|(index, (_, label))| {
                            let picker_id = picker_id.get_value();
                            let label_text = label.to_string();
                            view! {
                                <DtTab
                                    id=format!("{picker_id}-tab-{index}")
                                    panel_id=list_id.get_value()
                                    label=label_text.clone()
                                    selected=selected == index && !searching
                                    on_click=Callback::new(move |_| {
                                        query.set(String::new());
                                        debounced.set(String::new());
                                        fixed_index.set(index);
                                        let category = format!("{picker_id}-category-{index}");
                                        if let Some(list) = list_ref.get() {
                                            let list: HtmlElement = list.unchecked_into();
                                            if index == 0 {
                                                list.set_scroll_top(0);
                                            } else if let Some(element) = web_sys::window()
                                                .and_then(|window| window.document())
                                                .and_then(|document| document.get_element_by_id(&category))
                                            {
                                                if let Ok(element) = element.dyn_into::<HtmlElement>() {
                                                    list.set_scroll_top((element.offset_top() - 15).max(0));
                                                }
                                            }
                                        }
                                        focus_emoji(index, 0);
                                    })
                                >
                                    {label_text}
                                </DtTab>
                            }
                        }).collect_view()
                    }}
                    </div>
                </div>
            </div>
            <div class="d-emoji-picker--body">
                <Show when=move || show_search>
                    <div
                        class="d-emoji-picker__search d-emoji-picker__alignment"
                        on:keydown=move |event: KeyboardEvent| {
                            let key = event.key();
                            if key == "ArrowUp" {
                                focus_element(&format!("dt-tab-{}-tab-0", picker_id.get_value()));
                            } else if key == "ArrowDown" {
                                event.prevent_default();
                                if debounced.get_untracked().is_empty() {
                                    focus_emoji(0, 0);
                                } else {
                                    focus_filtered(0);
                                }
                            } else if key == "Enter" {
                                let skin_now = skin.get_untracked();
                                let query_now = debounced.get_untracked();
                                if query_now.is_empty() {
                                    return;
                                }
                                let pool = searchable_emojis(&skin_now);
                                if let Some(emoji) = filter_emojis(&pool, &query_now).into_iter().next() {
                                    select_emoji(emoji.clone(), event.shift_key());
                                }
                            }
                        }
                    >
                        <DtIconSearch size="200" />
                        <DtInput
                            id=search_id.get_value()
                            value=Signal::derive(move || query.get())
                            on_input=Callback::new(move |value: String| {
                                query.set(value.clone());
                                if let Some(callback) = on_search_query.get_value() {
                                    callback.run(value);
                                }
                            })
                            placeholder="Search..."
                            node_ref=search_ref
                        />
                        <Show when=move || !query.get().is_empty()>
                            <DtButton
                                importance="clear"
                                size="xs"
                                class="d-emoji-picker__search-x-button"
                                circle=true
                                kind="muted"
                                aria_label="Clear search"
                                on_click=Callback::new(move |_| {
                                    query.set(String::new());
                                    debounced.set(String::new());
                                    if let Some(input) = search_ref.get() {
                                        let _ = input.focus();
                                    }
                                })
                                icon=ViewFn::from(|| view! { <DtIconClose size="200" /> })
                            />
                        </Show>
                    </div>
                </Show>
                <div class="d-emoji-picker__selector">
                    <div
                        id=list_id.get_value()
                        node_ref=list_ref
                        class="d-emoji-picker__list"
                        on:scroll=on_list_scroll
                    >
                        {move || {
                            let query_now = debounced.get();
                            let searching = !query_now.is_empty();
                            let label = if searching {
                                let skin_now = skin.get();
                                let pool = searchable_emojis(&skin_now);
                                let matches = filter_emojis(&pool, &query_now);
                                if matches.is_empty() {
                                    "No results".to_string()
                                } else {
                                    "Search results".to_string()
                                }
                            } else {
                                let tabs = visible_tabs(!recent_now().is_empty(), !custom_now().is_empty());
                                tabs.get(fixed_index.get()).map(|(_, label)| (*label).to_string()).unwrap_or_default()
                            };
                            view! {
                                <p class=move || if searching {
                                    "d-emoji-picker__search-label d-emoji-picker__alignment".to_string()
                                } else {
                                    "d-emoji-picker__category d-emoji-picker__alignment".to_string()
                                }>{label}</p>
                            }
                        }}
                        {move || {
                            let skin_now = skin.get();
                            let recent = recent_now();
                            let custom = custom_now();
                            let tabs = visible_tabs(!recent.is_empty(), !custom.is_empty());
                            let searching = !debounced.get().is_empty();
                            let counts: Vec<usize> = tabs.iter().map(|(key, _)| {
                                category_emojis(key, &skin_now, &recent, &custom).len()
                            }).collect();
                            tabs.into_iter().enumerate().map(|(tab_index, (key, label))| {
                                let emojis = category_emojis(key, &skin_now, &recent, &custom);
                                let counts = counts.clone();
                                let category_id = format!("{}-category-{tab_index}", picker_id.get_value());
                                view! {
                                    <div
                                        id=category_id
                                        class="d-emoji-picker__alignment"
                                        data-emoji-category=tab_index.to_string()
                                        style:display=searching.then_some("none")
                                    >
                                        {tab_index.gt(&0).then(|| view! { <p>{label.to_string()}</p> })}
                                        <div class="d-emoji-picker__tab">
                                            {emojis.into_iter().enumerate().map(|(index, emoji)| {
                                                let counts = counts.clone();
                                                let name = emoji.name.clone();
                                                let src = picker_emoji_src(&emoji);
                                                let button_id = format!("{}-emoji-{tab_index}-{index}", picker_id.get_value());
                                                let emoji_click = emoji.clone();
                                                let emoji_focus = emoji.clone();
                                                let emoji_over = emoji.clone();
                                                let emoji_key = emoji;
                                                view! {
                                                    <button
                                                        id=button_id
                                                        type="button"
                                                        aria-label=name.clone()
                                                        on:click=move |event: MouseEvent| {
                                                            select_emoji(emoji_click.clone(), event.shift_key());
                                                        }
                                                        on:focusin=move |_| highlighted.set(Some(emoji_focus.clone()))
                                                        on:focusout=move |_| highlighted.set(None)
                                                        on:mouseover=move |_| {
                                                            skin_open.set(false);
                                                            highlighted.set(Some(emoji_over.clone()));
                                                        }
                                                        on:mouseleave=move |_| highlighted.set(None)
                                                        on:keydown=move |event: KeyboardEvent| {
                                                            let key = event.key();
                                                            if matches!(key.as_str(), "ArrowUp" | "ArrowDown" | "ArrowLeft" | "ArrowRight") {
                                                                event.prevent_default();
                                                                if let Some((next_tab, next_index)) = next_emoji(&key, tab_index, index, &counts) {
                                                                    focus_emoji(next_tab, next_index);
                                                                }
                                                            } else if key == "Enter" {
                                                                select_emoji(emoji_key.clone(), event.shift_key());
                                                            }
                                                        }
                                                        on:error=move |_event| {}
                                                    >
                                                        <img
                                                            class="d-icon d-icon--size-500"
                                                            alt=name.clone()
                                                            aria-label=name.clone()
                                                            title=name.clone()
                                                            src=src
                                                            loading=grid_image_loading(lazy_images)
                                                            on:error=move |event| {
                                                                if let Some(target) = event.target() {
                                                                    if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                                                                        if let Some(parent) = element.parent_element() {
                                                                            let _ = parent.set_attribute("style", "display:none");
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        />
                                                    </button>
                                                }
                                            }).collect_view()}
                                        </div>
                                    </div>
                                }
                            }).collect_view()
                        }}
                        {move || {
                            let query_now = debounced.get();
                            if query_now.is_empty() {
                                return ().into_any();
                            }
                            let pool = searchable_emojis(&skin.get());
                            let matches: Vec<DtEmojiData> = filter_emojis(&pool, &query_now).into_iter().cloned().collect();
                            let len = matches.len();
                            if hover_first.get_untracked() {
                                highlighted.set(matches.first().cloned());
                            }
                            view! {
                                <div class="d-emoji-picker__alignment">
                                    <div class="d-emoji-picker__tab" data-qa="filtered-emojis">
                                        {matches.into_iter().enumerate().map(|(index, emoji)| {
                                            let name = emoji.name.clone();
                                            let src = picker_emoji_src(&emoji);
                                            let button_id = format!("{}-filtered-{index}", picker_id.get_value());
                                            let emoji_click = emoji.clone();
                                            let emoji_over = emoji.clone();
                                            let emoji_key = emoji;
                                            view! {
                                                <button
                                                    id=button_id
                                                    type="button"
                                                    aria-label=name.clone()
                                                    class=("hover-emoji", index == 0 && hover_first.get())
                                                    on:click=move |event: MouseEvent| {
                                                        select_emoji(emoji_click.clone(), event.shift_key());
                                                    }
                                                    on:mouseover=move |_| {
                                                        hover_first.set(false);
                                                        skin_open.set(false);
                                                        highlighted.set(Some(emoji_over.clone()));
                                                    }
                                                    on:mouseleave=move |_| highlighted.set(None)
                                                    on:keydown=move |event: KeyboardEvent| {
                                                        let key = event.key();
                                                        if matches!(key.as_str(), "ArrowUp" | "ArrowDown" | "ArrowLeft" | "ArrowRight") {
                                                            event.prevent_default();
                                                            hover_first.set(false);
                                                            if let Some(next) = next_filtered(&key, index, len) {
                                                                focus_filtered(next);
                                                            }
                                                        } else if key == "Enter" {
                                                            select_emoji(emoji_key.clone(), event.shift_key());
                                                        }
                                                    }
                                                >
                                                    <img
                                                        class="d-icon d-icon--size-500"
                                                        alt=name.clone()
                                                        aria-label=name.clone()
                                                        src=src
                                                        loading=grid_image_loading(lazy_images)
                                                    />
                                                </button>
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>
                            }.into_any()
                        }}
                    </div>
                </div>
            </div>
            <div class="d-emoji-picker--footer">
                <Show when=move || show_add_emoji_button && highlighted.get().is_none()>
                    <DtButton
                        importance="outlined"
                        aria_label="Add emoji"
                        class="d-emoji-picker__add-emoji"
                        on_click=Callback::new(move |_| {
                            if let Some(callback) = on_add_emoji.get_value() {
                                callback.run(());
                            }
                        })
                    >
                        "Add emoji"
                    </DtButton>
                </Show>
                <div class="d-emoji-picker__data">
                    {move || highlighted.get().map(|emoji| {
                        let name = emoji.name.clone();
                        let src = picker_emoji_src(&emoji);
                        view! {
                            <img
                                class="d-icon d-icon--size-500"
                                alt=name.clone()
                                aria-label=name.clone()
                                title=name.clone()
                                src=src
                            />
                            <div>{name}</div>
                        }
                    })}
                </div>
                <div data-qa="skin-selector">
                    <div class="d-emoji-picker__skin-list" style:display=move || (!skin_open.get()).then_some("none")>
                        {SKIN_CHOICES.iter().enumerate().map(|(index, (name, code, tone))| {
                            let tone = (*tone).to_string();
                            let tone_selected = tone.clone();
                            let tone_click = tone.clone();
                            let tone_key = tone;
                            let src = format!("{EMOJI_PICKER_CDN}{code}.png");
                            let name = (*name).to_string();
                            let skin_id = picker_id.get_value();
                            let skin_click = skin_id.clone();
                            let skin_key = skin_id.clone();
                            view! {
                                <button
                                    type="button"
                                    id=format!("{skin_id}-skin-{index}")
                                    class=("selected", move || skin.get() == tone_selected)
                                    on:click=move |_| {
                                        skin.set(tone_click.clone());
                                        skin_open.set(false);
                                        if let Some(callback) = on_skin_tone.get_value() {
                                            callback.run(tone_click.clone());
                                        }
                                        focus_element(&format!("{skin_click}-skin-current"));
                                    }
                                    on:keydown=move |event: KeyboardEvent| {
                                        event.prevent_default();
                                        let key = event.key();
                                        if key == "ArrowLeft" {
                                            let next = if index == 0 { SKIN_CHOICES.len() - 1 } else { index - 1 };
                                            focus_element(&format!("{skin_key}-skin-{next}"));
                                        } else if key == "ArrowRight" {
                                            let next = if index + 1 == SKIN_CHOICES.len() { 0 } else { index + 1 };
                                            focus_element(&format!("{skin_key}-skin-{next}"));
                                        } else if key == "Enter" {
                                            skin.set(tone_key.clone());
                                            skin_open.set(false);
                                            if let Some(callback) = on_skin_tone.get_value() {
                                                callback.run(tone_key.clone());
                                            }
                                        }
                                    }
                                >
                                    <img class="d-icon d-icon--size-500" alt=name.clone() aria-label=name.clone() title=name.clone() src=src />
                                </button>
                            }
                        }).collect_view()}
                    </div>
                    <div class="d-emoji-picker__skin-selected" style:display=move || skin_open.get().then_some("none")>
                        {move || {
                            let tone = skin.get();
                            let choice = SKIN_CHOICES.iter().find(|(_, _, name)| *name == tone).unwrap_or(&SKIN_CHOICES[5]);
                            let src = format!("{EMOJI_PICKER_CDN}{}.png", choice.1);
                            let name = choice.0.to_string();
                            view! {
                                <DtTooltip message="Change default skin tone" placement="top-end">
                                    <button
                                        type="button"
                                        id=format!("{}-skin-current", picker_id.get_value())
                                        aria-label="Change default skin tone"
                                        tabindex="-1"
                                        on:click=move |_| {
                                            if highlighted.get_untracked().is_some() {
                                                skin_open.set(false);
                                            }
                                            skin_open.update(|open| *open = !*open);
                                        }
                                    >
                                        <img class="d-icon d-icon--size-500" alt=name.clone() aria-label=name.clone() title=name.clone() src=src />
                                    </button>
                                </DtTooltip>
                            }
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrow_keys_move_across_the_grid() {
        let counts = vec![10, 9];
        assert_eq!(next_emoji("ArrowRight", 0, 8, &counts), Some((0, 9)));
        assert_eq!(next_emoji("ArrowRight", 0, 9, &counts), Some((1, 0)));
        assert_eq!(next_emoji("ArrowLeft", 1, 0, &counts), Some((0, 9)));
        assert_eq!(next_emoji("ArrowDown", 0, 0, &counts), Some((0, 9)));
        assert_eq!(next_filtered("ArrowRight", 0, 3), Some(1));
        assert_eq!(next_filtered("ArrowLeft", 0, 3), Some(2));
        assert_eq!(next_filtered("ArrowDown", 0, 3), Some(0));
    }

    #[test]
    fn tabs_drop_empty_recent_and_custom_lists() {
        let tabs = visible_tabs(false, false);
        assert_eq!(tabs.first().map(|(key, _)| *key), Some("People"));
        assert!(tabs
            .iter()
            .all(|(key, _)| *key != "Custom" && *key != "Recently used"));
        assert_eq!(visible_tabs(true, true).len(), TAB_KEYS.len());
        assert_eq!(category_storage_key("People", "Light"), "PeopleLight");
        assert_eq!(category_storage_key("Nature", "Light"), "Nature");
    }

    #[test]
    fn grid_image_loading_is_lazy_or_omitted() {
        assert_eq!(grid_image_loading(true), Some("lazy"));
        assert_eq!(grid_image_loading(false), None);
    }

    #[test]
    fn search_matches_a_name() {
        let smile = searchable_emojis("Default");
        let found = filter_emojis(&smile, "grinning face with smiling eyes");
        assert!(found.iter().any(|emoji| emoji.shortname == ":smile:"));
    }
}
