use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use leptos::prelude::*;
use serde::Deserialize;

use super::skeleton::{DtSkeleton, DtSkeletonShapeKind, DtSkeletonShapeOption};

/// JoyPixels asset root used when a caller has not set one.
/// Dialtone `defaultEmojiAssetUrl` for emoji version `9.0`.
pub const DEFAULT_EMOJI_ASSET_URL: &str =
    "https://cdn.jsdelivr.net/joypixels/assets/9.0/png/unicode/32/";

const ICON_SIZES: &[&str] = &["100", "200", "300", "400", "500", "600", "700", "800"];

/// One emoji from `@dialpad/dialtone-emojis` at `dialtone/v9.190.0`.
///
/// `unicode_character` is the asset key (`1f604`, `261d-1f3fb`). Variation
/// selectors and zero-width joiners are omitted, matching `unicodeToString`.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct DtEmojiData {
    pub name: String,
    #[serde(default)]
    pub category: String,
    pub shortname: String,
    #[serde(default)]
    pub shortname_alternates: Vec<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub unicode_output: String,
    #[serde(default)]
    pub unicode_character: String,
    #[serde(default)]
    pub image: String,
    #[serde(default)]
    pub date_added: String,
    #[serde(default)]
    pub custom: bool,
}

struct AssetUrls {
    small_url: String,
    small_ext: String,
    large_url: String,
    large_ext: String,
}

struct EmojiCatalog {
    by_shortname: HashMap<String, DtEmojiData>,
    by_unicode: HashMap<String, DtEmojiData>,
    groups: HashMap<String, Vec<DtEmojiData>>,
}

fn assets() -> &'static Mutex<AssetUrls> {
    static ASSETS: OnceLock<Mutex<AssetUrls>> = OnceLock::new();
    ASSETS.get_or_init(|| {
        Mutex::new(AssetUrls {
            small_url: DEFAULT_EMOJI_ASSET_URL.to_string(),
            small_ext: ".png".to_string(),
            large_url: DEFAULT_EMOJI_ASSET_URL.to_string(),
            large_ext: ".png".to_string(),
        })
    })
}

fn custom_overlay() -> &'static Mutex<Vec<DtEmojiData>> {
    static CUSTOM: OnceLock<Mutex<Vec<DtEmojiData>>> = OnceLock::new();
    CUSTOM.get_or_init(|| Mutex::new(Vec::new()))
}

fn with_slash(url: &str) -> String {
    if url.ends_with('/') {
        url.to_string()
    } else {
        format!("{url}/")
    }
}

/// Dialtone `setEmojiAssetUrlSmall`. Sizes `100` and `200` use this URL.
pub fn set_emoji_asset_url_small(url: &str, extension: &str) {
    if let Ok(mut assets) = assets().lock() {
        assets.small_url = with_slash(url);
        assets.small_ext = extension.to_string();
    }
}

/// Dialtone `setEmojiAssetUrlLarge`. Sizes other than `100` and `200` use this URL.
pub fn set_emoji_asset_url_large(url: &str, extension: &str) {
    if let Ok(mut assets) = assets().lock() {
        assets.large_url = with_slash(url);
        assets.large_ext = extension.to_string();
    }
}

fn catalog() -> &'static EmojiCatalog {
    static CATALOG: OnceLock<EmojiCatalog> = OnceLock::new();
    CATALOG.get_or_init(load_catalog)
}

fn load_group(name: &str, json: &str) -> Vec<DtEmojiData> {
    match serde_json::from_str(json) {
        Ok(group) => group,
        Err(error) => {
            leptos::logging::warn!("DtEmoji group `{name}` did not parse: {error}");
            Vec::new()
        }
    }
}

fn load_catalog() -> EmojiCatalog {
    let sources: &[(&str, &str)] = &[
        (
            "ActivityDefault",
            include_str!("../data/emojis/activity-default.json"),
        ),
        (
            "ActivityLight",
            include_str!("../data/emojis/activity-light.json"),
        ),
        (
            "ActivityMediumLight",
            include_str!("../data/emojis/activity-medium-light.json"),
        ),
        (
            "ActivityMedium",
            include_str!("../data/emojis/activity-medium.json"),
        ),
        (
            "ActivityMediumDark",
            include_str!("../data/emojis/activity-medium-dark.json"),
        ),
        (
            "ActivityDark",
            include_str!("../data/emojis/activity-dark.json"),
        ),
        ("Flags", include_str!("../data/emojis/flags.json")),
        ("Food", include_str!("../data/emojis/food.json")),
        ("Nature", include_str!("../data/emojis/nature.json")),
        (
            "ObjectsDefault",
            include_str!("../data/emojis/objects-default.json"),
        ),
        (
            "ObjectsLight",
            include_str!("../data/emojis/objects-light.json"),
        ),
        (
            "ObjectsMediumLight",
            include_str!("../data/emojis/objects-medium-light.json"),
        ),
        (
            "ObjectsMedium",
            include_str!("../data/emojis/objects-medium.json"),
        ),
        (
            "ObjectsMediumDark",
            include_str!("../data/emojis/objects-medium-dark.json"),
        ),
        (
            "ObjectsDark",
            include_str!("../data/emojis/objects-dark.json"),
        ),
        (
            "PeopleDefault",
            include_str!("../data/emojis/people-default.json"),
        ),
        (
            "PeopleLight",
            include_str!("../data/emojis/people-light.json"),
        ),
        (
            "PeopleMediumLight",
            include_str!("../data/emojis/people-medium-light.json"),
        ),
        (
            "PeopleMedium",
            include_str!("../data/emojis/people-medium.json"),
        ),
        (
            "PeopleMediumDark",
            include_str!("../data/emojis/people-medium-dark.json"),
        ),
        (
            "PeopleDark",
            include_str!("../data/emojis/people-dark.json"),
        ),
        ("Symbols", include_str!("../data/emojis/symbols.json")),
        ("Travel", include_str!("../data/emojis/travel.json")),
    ];
    let mut groups = HashMap::new();
    let mut by_shortname = HashMap::new();
    let mut by_unicode = HashMap::new();
    for (name, json) in sources {
        let group = load_group(name, json);
        for emoji in &group {
            index_emoji(&mut by_shortname, &mut by_unicode, emoji.clone());
        }
        groups.insert((*name).to_string(), group);
    }
    EmojiCatalog {
        by_shortname,
        by_unicode,
        groups,
    }
}

fn index_emoji(
    by_shortname: &mut HashMap<String, DtEmojiData>,
    by_unicode: &mut HashMap<String, DtEmojiData>,
    emoji: DtEmojiData,
) {
    if !emoji.unicode_character.is_empty() {
        by_unicode
            .entry(emoji.unicode_character.clone())
            .or_insert_with(|| emoji.clone());
    }
    if !emoji.shortname.is_empty() {
        by_shortname
            .entry(emoji.shortname.clone())
            .or_insert_with(|| emoji.clone());
    }
    for alternate in &emoji.shortname_alternates {
        by_shortname
            .entry(alternate.clone())
            .or_insert_with(|| emoji.clone());
    }
}

/// Dialtone `setCustomEmojiJson`.
///
/// An empty string clears custom emoji. A value missing the required fields
/// warns and is skipped. Vue throws on an empty string.
pub fn set_custom_emoji_json(json: &str) {
    let Ok(mut overlay) = custom_overlay().lock() else {
        return;
    };
    if json.trim().is_empty() {
        overlay.clear();
        return;
    }
    let parsed: serde_json::Map<String, serde_json::Value> = match serde_json::from_str(json) {
        Ok(value) => value,
        Err(error) => {
            leptos::logging::warn!("DtEmoji custom JSON did not parse: {error}");
            return;
        }
    };
    let required = [
        "date_added",
        "image",
        "unicode_output",
        "shortname",
        "shortname_alternates",
        "custom",
        "name",
    ];
    for (key, value) in parsed {
        let Some(object) = value.as_object() else {
            leptos::logging::warn!("DtEmoji custom entry `{key}` is not an object");
            continue;
        };
        if catalog().by_unicode.contains_key(&key) || catalog().by_shortname.contains_key(&key) {
            let mut emoji = catalog()
                .by_unicode
                .get(&key)
                .or_else(|| catalog().by_shortname.get(&key))
                .cloned()
                .unwrap_or_else(|| DtEmojiData {
                    name: String::new(),
                    category: String::new(),
                    shortname: key.clone(),
                    shortname_alternates: Vec::new(),
                    keywords: Vec::new(),
                    unicode_output: String::new(),
                    unicode_character: key.clone(),
                    image: String::new(),
                    date_added: String::new(),
                    custom: false,
                });
            merge_custom(&mut emoji, object);
            overlay.retain(|item| item.shortname != emoji.shortname);
            overlay.push(emoji);
            continue;
        }
        if required.iter().any(|field| object.get(*field).is_none()) {
            leptos::logging::warn!("DtEmoji custom entry `{key}` is missing required properties");
            continue;
        }
        match serde_json::from_value::<DtEmojiData>(value.clone()) {
            Ok(emoji) => {
                overlay.retain(|item| item.shortname != emoji.shortname);
                overlay.push(emoji);
            }
            Err(error) => {
                leptos::logging::warn!("DtEmoji custom entry `{key}` did not parse: {error}");
            }
        }
    }
}

fn merge_custom(emoji: &mut DtEmojiData, object: &serde_json::Map<String, serde_json::Value>) {
    for (name, value) in object {
        match name.as_str() {
            "name" => {
                if let Some(text) = value.as_str() {
                    emoji.name = text.to_string();
                }
            }
            "shortname" => {
                if let Some(text) = value.as_str() {
                    emoji.shortname = text.to_string();
                }
            }
            "unicode_output" => {
                if let Some(text) = value.as_str() {
                    emoji.unicode_output = text.to_string();
                }
            }
            "unicode_character" => {
                if let Some(text) = value.as_str() {
                    emoji.unicode_character = text.to_string();
                }
            }
            "image" => {
                if let Some(text) = value.as_str() {
                    emoji.image = text.to_string();
                }
            }
            "date_added" => {
                if let Some(text) = value.as_str() {
                    emoji.date_added = text.to_string();
                }
            }
            "custom" => emoji.custom = value.as_bool().unwrap_or(false),
            "category" => {
                if let Some(text) = value.as_str() {
                    emoji.category = text.to_string();
                }
            }
            "shortname_alternates" => {
                if let Some(list) = value.as_array() {
                    for item in list {
                        if let Some(text) = item.as_str() {
                            emoji.shortname_alternates.push(text.to_string());
                        }
                    }
                }
            }
            "keywords" => {
                if let Some(list) = value.as_array() {
                    for item in list {
                        if let Some(text) = item.as_str() {
                            emoji.keywords.push(text.to_string());
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

/// Hex key for an emoji character sequence. Skips `fe0f` and `200d`.
pub fn unicode_to_string(emoji: &str) -> String {
    let mut key = String::new();
    for code_point in emoji.chars() {
        let codepoint = format!("{:04x}", code_point as u32);
        if codepoint == "200d" || codepoint == "fe0f" {
            continue;
        }
        if !key.is_empty() {
            key.push('-');
        }
        key.push_str(&codepoint);
    }
    key
}

/// Dialtone `stringToUnicode`. `1f604` becomes the grinning-face character.
pub fn string_to_unicode(value: &str) -> String {
    value
        .split('-')
        .filter_map(|part| u32::from_str_radix(part, 16).ok())
        .filter_map(char::from_u32)
        .collect()
}

fn overlay_match(code: &str, unicode_key: &str) -> Option<DtEmojiData> {
    let overlay = custom_overlay().lock().ok()?;
    overlay
        .iter()
        .find(|emoji| {
            emoji.shortname == code
                || emoji.shortname_alternates.iter().any(|item| item == code)
                || emoji.unicode_character == unicode_key
                || emoji.unicode_character == code
        })
        .cloned()
}

/// Dialtone `codeToEmojiData`. Accepts `:smile:` or a unicode character.
pub fn code_to_emoji_data(code: &str) -> Option<DtEmojiData> {
    let code = code.trim();
    if code.is_empty() {
        return None;
    }
    if code.starts_with(':') && code.ends_with(':') && code.len() > 2 {
        if let Some(emoji) = overlay_match(code, "") {
            return Some(emoji);
        }
        return catalog().by_shortname.get(code).cloned();
    }
    let key = unicode_to_string(code);
    if let Some(emoji) = overlay_match(code, &key) {
        return Some(emoji);
    }
    catalog().by_unicode.get(&key).cloned()
}

/// Shortcodes `:word:` that exist in the catalog. Order follows the text.
pub(crate) fn find_short_codes(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut start = 0;
    while let Some(relative) = text[start..].find(':') {
        let colon = start + relative;
        let previous_ok =
            colon == 0 || text[..colon].ends_with(' ') || text[..colon].ends_with(':');
        if previous_ok {
            if let Some(end_relative) = text[colon + 1..].find(':') {
                let end = colon + 1 + end_relative;
                let body = &text[colon + 1..end];
                let word = !body.is_empty()
                    && body
                        .chars()
                        .all(|character| character.is_ascii_alphanumeric() || character == '_');
                if word {
                    let code = text[colon..=end].to_string();
                    if code_to_emoji_data(&code).is_some() && !found.contains(&code) {
                        found.push(code);
                    }
                    start = end + 1;
                    continue;
                }
            }
        }
        start = colon + 1;
    }
    found
}

/// Unicode emoji in `text` that exist in the catalog. Longest match wins.
pub(crate) fn find_emojis(text: &str) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    let mut found = Vec::new();
    let mut index = 0;
    while index < chars.len() {
        let mut best: Option<(usize, String)> = None;
        let mut buffer = String::new();
        let end = (index + 12).min(chars.len());
        for next in index..end {
            buffer.push(chars[next]);
            let key = unicode_to_string(&buffer);
            if catalog().by_unicode.contains_key(&key) || overlay_has_unicode(&key) {
                best = Some((next + 1, buffer.clone()));
            }
        }
        if let Some((next, emoji)) = best {
            if !found.contains(&emoji) {
                found.push(emoji);
            }
            index = next;
        } else {
            index += 1;
        }
    }
    found
}

fn overlay_has_unicode(key: &str) -> bool {
    custom_overlay()
        .lock()
        .ok()
        .is_some_and(|overlay| overlay.iter().any(|emoji| emoji.unicode_character == key))
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum EmojiTextPart {
    Text(String),
    Emoji(String),
}

/// Split `text` into plain runs and emoji codes, in Dialtone's replace order.
pub(crate) fn emoji_text_parts(text: &str) -> Vec<EmojiTextPart> {
    let mut tokens = find_short_codes(text);
    tokens.extend(find_emojis(text));
    if tokens.is_empty() {
        if text.trim().is_empty() {
            return Vec::new();
        }
        return vec![EmojiTextPart::Text(text.to_string())];
    }
    let mut parts = Vec::new();
    let mut rest = text;
    while !rest.is_empty() {
        let mut best: Option<(usize, usize)> = None;
        for (token_index, token) in tokens.iter().enumerate() {
            if let Some(position) = rest.find(token.as_str()) {
                let replace = best
                    .map(|(found_at, _)| position < found_at)
                    .unwrap_or(true);
                if replace {
                    best = Some((position, token_index));
                }
            }
        }
        let Some((position, token_index)) = best else {
            if !rest.trim().is_empty() {
                parts.push(EmojiTextPart::Text(rest.to_string()));
            }
            break;
        };
        let token = tokens[token_index].as_str();
        let before = &rest[..position];
        if !before.trim().is_empty() {
            parts.push(EmojiTextPart::Text(before.to_string()));
        }
        parts.push(EmojiTextPart::Emoji(token.to_string()));
        rest = &rest[position + token.len()..];
    }
    parts
}

pub(crate) fn icon_size_class(size: &str) -> &'static str {
    match size {
        "100" => "d-icon--size-100",
        "200" => "d-icon--size-200",
        "300" => "d-icon--size-300",
        "400" => "d-icon--size-400",
        "500" => "d-icon--size-500",
        "600" => "d-icon--size-600",
        "700" => "d-icon--size-700",
        "800" => "d-icon--size-800",
        _ => {
            leptos::logging::warn!("DtEmoji size `{size}` is not an icon size");
            ""
        }
    }
}

pub(crate) fn emoji_src_with(
    emoji: &DtEmojiData,
    size: &str,
    small: (&str, &str),
    large: (&str, &str),
) -> String {
    if emoji.custom {
        return emoji.image.clone();
    }
    let (url, extension) = if size == "100" || size == "200" {
        small
    } else {
        large
    };
    format!("{url}{}{extension}", emoji.unicode_character)
}

fn emoji_src(emoji: &DtEmojiData, size: &str) -> String {
    let Ok(assets) = assets().lock() else {
        return String::new();
    };
    emoji_src_with(
        emoji,
        size,
        (assets.small_url.as_str(), assets.small_ext.as_str()),
        (assets.large_url.as_str(), assets.large_ext.as_str()),
    )
}

pub(crate) fn emoji_group(name: &str) -> Vec<DtEmojiData> {
    catalog().groups.get(name).cloned().unwrap_or_default()
}

/// Categories the picker searches, with the skin-tone lists for people, activity, and objects.
pub(crate) fn searchable_emojis(skin: &str) -> Vec<DtEmojiData> {
    let mut emojis = Vec::new();
    for name in [
        format!("People{skin}"),
        "Nature".to_string(),
        "Food".to_string(),
        format!("Activity{skin}"),
        "Travel".to_string(),
        format!("Objects{skin}"),
        "Symbols".to_string(),
        "Flags".to_string(),
    ] {
        emojis.extend(emoji_group(&name));
    }
    emojis
}

pub(crate) fn filter_emojis<'a>(emojis: &'a [DtEmojiData], query: &str) -> Vec<&'a DtEmojiData> {
    let query = query.to_lowercase();
    emojis
        .iter()
        .filter(|emoji| {
            emoji.name.to_lowercase().contains(&query)
                || emoji
                    .keywords
                    .iter()
                    .any(|keyword| keyword.to_lowercase().contains(&query))
        })
        .collect()
}

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Dialtone Vue `DtEmoji` (`dialtone/v10.7.0`).
///
/// `code` is a shortcode (`:smile:`) or a unicode character. Sizes `100` and
/// `200` use the small asset URL. Other sizes use the large URL. Both default
/// to JoyPixels 9.0 PNG. Set them with [`set_emoji_asset_url_small`] and
/// [`set_emoji_asset_url_large`]. Custom emoji come from [`set_custom_emoji_json`].
/// The root is `data-qa="dt-emoji"`. The image is `data-qa="dt-emoji-image"`.
/// `show_skeleton` draws [`DtSkeleton`](crate::DtSkeleton) at `offset` `0`
/// with the icon size class until the image loads or errors.
#[component]
pub fn DtEmoji(
    /// Shortcode or unicode (`code`).
    #[prop(into)]
    code: String,
    /// Icon size token (`size`). Default `500`.
    #[prop(optional, default = "500")]
    size: &'static str,
    /// Extra class on the image (`imgClass`).
    #[prop(optional, into)]
    img_class: String,
    /// Screen-reader name (`ariaLabel`). Empty uses the emoji name.
    #[prop(optional, into)]
    aria_label: String,
    /// Skeleton while the image loads (`showSkeleton`).
    #[prop(optional, default = true)]
    show_skeleton: bool,
) -> impl IntoView {
    if !ICON_SIZES.contains(&size) {
        leptos::logging::warn!("DtEmoji size `{size}` is not an icon size");
    }
    let emoji = code_to_emoji_data(&code);
    let size_class = icon_size_class(size);
    let root_class = join_class(["d-emoji", "d-icon", size_class]);
    let image_class = join_class(["d-icon", size_class, img_class.as_str()]);
    let skeleton_class = join_class(["d-icon", size_class]);
    let loading = RwSignal::new(true);
    let (src, label, alt) = if let Some(emoji) = emoji {
        let label = if aria_label.is_empty() {
            emoji.name.clone()
        } else {
            aria_label
        };
        let alt = if emoji.unicode_output.is_empty() {
            emoji.name.clone()
        } else {
            string_to_unicode(&emoji.unicode_output)
        };
        (emoji_src(&emoji, size), label, Some(alt))
    } else {
        ("invalid".to_string(), "Invalid Emoji".to_string(), None)
    };
    let title = label.clone();
    let skeleton_label = label.clone();
    view! {
        <span class=root_class data-qa="dt-emoji">
            <Show when=move || show_skeleton && loading.get()>
                <DtSkeleton
                    aria_label=skeleton_label.clone()
                    class=skeleton_class.clone()
                    offset=0.0
                    shape_option=DtSkeletonShapeOption {
                        shape: DtSkeletonShapeKind::Circle,
                        size: "100%",
                    }
                />
            </Show>
            <img
                data-qa="dt-emoji-image"
                class=image_class.clone()
                alt=alt.clone()
                aria-label=label.clone()
                title=title.clone()
                src=src.clone()
                style=move || loading.get().then_some("display:none")
                on:load=move |_| loading.set(false)
                on:error=move |_| loading.set(false)
            />
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smile_shortcode_and_skin_tone_resolve() {
        let smile = code_to_emoji_data(":smile:").expect("smile");
        assert_eq!(smile.unicode_character, "1f604");
        assert_eq!(smile.name, "grinning face with smiling eyes");
        assert_eq!(
            emoji_src_with(
                &smile,
                "500",
                ("https://cdn.example/small/", ".png"),
                ("https://cdn.example/large/", ".svg"),
            ),
            "https://cdn.example/large/1f604.svg"
        );
        assert_eq!(
            emoji_src_with(
                &smile,
                "200",
                ("https://cdn.example/small/", ".png"),
                ("https://cdn.example/large/", ".svg"),
            ),
            "https://cdn.example/small/1f604.png"
        );
        let pointed = code_to_emoji_data(":point_up_tone1:").expect("point");
        assert_eq!(pointed.unicode_character, "261d-1f3fb");
        assert_eq!(
            code_to_emoji_data(&string_to_unicode("261d-1f3fb"))
                .expect("unicode")
                .unicode_character,
            "261d-1f3fb"
        );
        assert!(code_to_emoji_data(":invalidcode:").is_none());
        assert_eq!(icon_size_class("800"), "d-icon--size-800");
        assert_eq!(icon_size_class("nope"), "");
    }

    #[test]
    fn custom_emoji_image_wins_and_empty_json_clears_it() {
        set_custom_emoji_json(
            r#"{"baby_yoda":{"custom":true,"date_added":"2020","image":"https://example/baby_yoda","unicode_output":"baby","shortname":":baby_yoda:","shortname_alternates":[],"name":"baby yoda"}}"#,
        );
        let custom = code_to_emoji_data(":baby_yoda:").expect("custom");
        assert_eq!(
            emoji_src_with(
                &custom,
                "500",
                ("https://cdn.example/small/", ".png"),
                ("https://cdn.example/large/", ".svg"),
            ),
            "https://example/baby_yoda"
        );
        set_custom_emoji_json("");
        assert!(code_to_emoji_data(":baby_yoda:").is_none());
    }

    #[test]
    fn unicode_key_skips_variation_selector_and_joiner() {
        assert_eq!(unicode_to_string("😄"), "1f604");
        assert_eq!(string_to_unicode("1f604"), "😄");
        let pointed = string_to_unicode("261d-1f3fb");
        assert_eq!(unicode_to_string(&pointed), "261d-1f3fb");
    }

    #[test]
    fn text_parts_keep_invalid_shortcodes_as_text() {
        let parts = emoji_text_parts("Content with :smile: emoji.");
        assert_eq!(
            parts,
            vec![
                EmojiTextPart::Text("Content with ".to_string()),
                EmojiTextPart::Emoji(":smile:".to_string()),
                EmojiTextPart::Text(" emoji.".to_string()),
            ]
        );
        assert_eq!(
            emoji_text_parts("Content with :invalid: emoji."),
            vec![EmojiTextPart::Text(
                "Content with :invalid: emoji.".to_string()
            )]
        );
        let pointed = string_to_unicode("1f604");
        let mixed = format!("Content with valid {pointed} emoji.");
        assert!(emoji_text_parts(&mixed)
            .iter()
            .any(|part| matches!(part, EmojiTextPart::Emoji(_))));
    }
}
