use std::sync::atomic::{AtomicU64, Ordering};

use leptos::prelude::*;

use super::unique_id;

const PRESENCE_MINUS: &str = "M8.2334 5.25C8.64754 5.25009 8.9834 5.58584 8.9834 6C8.9834 6.41416 8.64754 6.74991 8.2334 6.75H3.7666C3.35249 6.74988 3.0166 6.41414 3.0166 6C3.0166 5.58586 3.35249 5.25012 3.7666 5.25H8.2334Z";
const PRESENCE_CHECK: &str = "M8.63559 2.44125C8.87637 2.1042 9.34444 2.02566 9.68149 2.26644C10.0185 2.5072 10.096 2.9753 9.85531 3.31234L5.3934 9.55941C5.25261 9.7564 5.02519 9.87289 4.78305 9.87289C4.54091 9.87288 4.31348 9.75641 4.1727 9.55941L2.14438 6.71957C1.90382 6.38254 1.98223 5.91439 2.31918 5.67367C2.65612 5.43319 3.12431 5.51077 3.36508 5.8475L4.78305 7.83285L8.63559 2.44125Z";

/// Java `String.hashCode`, matching Dialtone `javaHashCode` / `getRandomFamilyVariant`.
fn java_hash_code(value: &str) -> i32 {
    let mut hash: i32 = 0;
    for unit in value.encode_utf16() {
        hash = hash
            .wrapping_shl(5)
            .wrapping_sub(hash)
            .wrapping_add(i32::from(unit));
    }
    hash
}

fn random_index(len: usize) -> usize {
    static STATE: AtomicU64 = AtomicU64::new(0x1234_5678_9abc_def0);
    let mut state = STATE.load(Ordering::Relaxed);
    state = state
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1);
    STATE.store(state, Ordering::Relaxed);
    (state as usize) % len
}

/// `size` kept when it is a staging token or a t-shirt alias. Anything else is `300`.
fn canonical_size(size: &str) -> &'static str {
    match size {
        "100" => "100",
        "150" => "150",
        "200" => "200",
        "250" => "250",
        "300" => "300",
        "400" => "400",
        "500" => "500",
        "600" => "600",
        "700" => "700",
        "800" => "800",
        "900" => "900",
        "xs" => "xs",
        "sm" => "sm",
        "md" => "md",
        "lg" => "lg",
        "xl" => "xl",
        _ => "300",
    }
}

fn size_class(size: &str) -> &'static str {
    match canonical_size(size) {
        "100" => "d-avatar--size-100",
        "150" => "d-avatar--size-150",
        "200" => "d-avatar--size-200",
        "250" => "d-avatar--size-250",
        "400" => "d-avatar--size-400",
        "500" => "d-avatar--size-500",
        "600" => "d-avatar--size-600",
        "700" => "d-avatar--size-700",
        "800" => "d-avatar--size-800",
        "900" => "d-avatar--size-900",
        "xs" => "d-avatar--xs",
        "sm" => "d-avatar--sm",
        "lg" => "d-avatar--lg",
        "xl" => "d-avatar--xl",
        "md" => "d-avatar--md",
        _ => "d-avatar--size-300",
    }
}

fn presence_size_class(size: &str) -> &'static str {
    match canonical_size(size) {
        "100" | "xs" => "d-avatar__presence--100",
        "150" => "d-avatar__presence--150",
        "200" | "sm" => "d-avatar__presence--200",
        "250" => "d-avatar__presence--250",
        "400" => "d-avatar__presence--400",
        "500" | "lg" => "d-avatar__presence--500",
        "md" => "d-avatar__presence--md",
        _ => "d-avatar__presence--300",
    }
}

/// Presence is drawn for sizes 100–500 and the t-shirt aliases xs–lg.
fn presence_supported(size: &str) -> bool {
    matches!(
        canonical_size(size),
        "100" | "150" | "200" | "250" | "300" | "400" | "500" | "xs" | "sm" | "md" | "lg"
    )
}

/// Icon token for `size` when `icon_size` is empty. Dialtone `AVATAR_ICON_SIZES`.
pub fn avatar_icon_size(size: &str, icon_size: &str) -> &'static str {
    if !icon_size.is_empty() {
        return match icon_size {
            "100" => "100",
            "200" => "200",
            "300" => "300",
            "400" => "400",
            "500" => "500",
            "600" => "600",
            "700" => "700",
            "800" => "800",
            _ => "300",
        };
    }
    match canonical_size(size) {
        "100" | "150" | "xs" => "100",
        "200" | "250" | "sm" => "200",
        "400" => "500",
        "500" | "lg" | "600" | "xl" => "600",
        "700" => "700",
        "800" | "900" => "800",
        _ => "300",
    }
}

fn show_group(group: Option<i32>) -> bool {
    group.is_some_and(|count| count > 1)
}

fn small_group_size(size: &str) -> bool {
    matches!(
        canonical_size(size),
        "xs" | "sm" | "100" | "150" | "200" | "250"
    )
}

/// Sizes 100–250 cap at `9+`. Larger sizes cap at `99+`.
fn formatted_group(group: i32, size: &str) -> String {
    if small_group_size(size) {
        if group > 9 {
            "9+".to_string()
        } else {
            group.to_string()
        }
    } else if group > 99 {
        "99+".to_string()
    } else {
        group.to_string()
    }
}

/// Dialtone `extractInitialsFromName`.
fn extract_initials(full_name: &str) -> String {
    if full_name.trim().is_empty() {
        return String::new();
    }
    let sanitized: String = full_name
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect();
    let names: Vec<&str> = sanitized.split_whitespace().collect();
    if names.is_empty() {
        return String::new();
    }
    if names.len() == 1 {
        return names[0].chars().take(2).collect();
    }
    let initial = |name: &str| {
        name.chars()
            .next()
            .map(|c| c.to_uppercase().collect::<String>())
            .unwrap_or_default()
    };
    format!("{}{}", initial(names[0]), initial(names[names.len() - 1]))
}

fn format_initials(full_name: &str, size: &str) -> String {
    let initials = extract_initials(full_name);
    match canonical_size(size) {
        "xs" | "100" => String::new(),
        "sm" | "150" | "200" => initials
            .chars()
            .next()
            .map(|c| c.to_string())
            .unwrap_or_default(),
        _ => initials,
    }
}

/// Dialtone `colorToFamilyVariant`. `540` is family 5, variant 4.
fn color_to_family_variant(color: &str) -> Option<(u8, u8)> {
    let num: i32 = color.parse().ok()?;
    if num < 0 || num % 10 != 0 {
        return None;
    }
    let family = num / 100;
    let variant = (num % 100) / 10;
    if (1..=12).contains(&family) && (0..10).contains(&variant) {
        Some((family as u8, variant as u8))
    } else {
        None
    }
}

fn family_variant_from_seed(seed: &str) -> (u8, u8) {
    let hash = if seed.is_empty() {
        random_index(120) as i64
    } else {
        i64::from(java_hash_code(seed))
    };
    let abs_hash = hash.unsigned_abs();
    let family = (abs_hash % 12) + 1;
    let variant = (abs_hash / 12) % 10;
    (family as u8, variant as u8)
}

fn parse_scale(value: &str, min: u8, max: u8) -> Option<u8> {
    let parsed: u8 = value.parse().ok()?;
    (min..=max).contains(&parsed).then_some(parsed)
}

/// Icon avatars and `iconOnly` skip the color. An explicit family or variant
/// wins, then a legacy `color` code, then `seed`.
fn resolved_color(
    icon: bool,
    icon_only: bool,
    family: &str,
    variant: &str,
    color: &str,
    seed: &str,
) -> Option<(u8, u8)> {
    if icon || icon_only {
        return None;
    }
    let from_color = color_to_family_variant(color);
    let from_seed = family_variant_from_seed(seed);
    let family = parse_scale(family, 1, 12)
        .or(from_color.map(|pair| pair.0))
        .unwrap_or(from_seed.0);
    let variant = parse_scale(variant, 0, 9)
        .or(from_color.map(|pair| pair.1))
        .unwrap_or(from_seed.1);
    Some((family, variant))
}

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn avatar_classes(
    size: &str,
    group: Option<i32>,
    avatar_class: &str,
    interactive: bool,
    presence: &str,
    icon_only: bool,
    deactivated: bool,
) -> String {
    let size = canonical_size(size);
    let grouped = show_group(group);
    let mut parts = vec!["d-avatar".to_string(), size_class(size).to_string()];
    if !avatar_class.is_empty() {
        parts.push(avatar_class.to_string());
    }
    if grouped {
        parts.push("d-avatar--group".to_string());
        if let Some(count) = group {
            let label = formatted_group(count, size);
            if label.len() == 2 {
                parts.push("d-avatar--group-digits-2".to_string());
            } else if label.len() >= 3 {
                parts.push("d-avatar--group-digits-3".to_string());
            }
        }
    }
    if interactive {
        parts.push("d-avatar--clickable".to_string());
    }
    if !presence.is_empty() && !grouped && presence_supported(size) {
        parts.push("d-avatar--presence".to_string());
    }
    if icon_only {
        parts.push("d-avatar--icon-only".to_string());
    }
    if deactivated {
        parts.push("d-avatar--deactivated".to_string());
    }
    join_class(parts)
}

fn presence_glyph(presence: &str) -> Option<AnyView> {
    let (qa, path) = match presence {
        "active" => ("dt-presence-active-icon", PRESENCE_CHECK),
        "busy" => ("dt-presence-busy-icon", PRESENCE_MINUS),
        "dnd" => ("dt-presence-dnd-icon", PRESENCE_MINUS),
        _ => return None,
    };
    Some(
        view! {
            <svg
                class="d-presence__icon"
                data-qa=qa
                viewBox="0 0 12 12"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
                aria-hidden="true"
            >
                <path d=path fill="currentColor" />
            </svg>
        }
        .into_any(),
    )
}

/// Dialtone Vue `DtAvatar`.
///
/// The shape is a rounded square (`border-radius` is 12.5% of the size).
/// `size` is `100`–`900` (`300` default). `xs`, `sm`, `md`, `lg`, and `xl`
/// remain as aliases. Color comes from `seed`, or from `family` (1–12) and
/// `variant` (0–9). `color` is the legacy `family * 100 + variant * 10` code.
/// `interactive` renders a button (`clickable` is the deprecated alias).
/// A group count above 1 draws the count badge and does not change the size.
/// Presence (`busy`, `away`, `offline`, `active`, `dnd`) is omitted above size 500.
#[component]
pub fn DtAvatar(
    #[prop(optional, into)] id: String,
    /// Stable color seed (`seed`). Ignored when `family`, `variant`, or `color` set that channel.
    #[prop(optional, into)]
    seed: String,
    /// Palette code (`color`), for example `"540"` (family 5, variant 4).
    #[prop(optional, into)]
    color: String,
    /// Hue family `1`–`12` (`family`). Empty defers to `color`, then `seed`.
    #[prop(optional, into)]
    family: String,
    /// Lightness step `0`–`9` (`variant`). Empty defers to `color`, then `seed`.
    #[prop(optional, into)]
    variant: String,
    /// `100`–`900`, or `xs` / `sm` / `md` / `lg` / `xl`. Default `300`.
    #[prop(optional, default = "300")]
    size: &'static str,
    /// Extra class on the root (`avatarClass`).
    #[prop(optional, into)]
    avatar_class: String,
    /// Extra class on `.d-avatar__canvas` (`canvasClass`).
    #[prop(optional, into)]
    canvas_class: String,
    /// Extra class on the icon wrapper (`iconClass`).
    #[prop(optional, into)]
    icon_class: String,
    /// Icon size token. Empty uses the size default (`iconSize`).
    #[prop(optional, into)]
    icon_size: String,
    /// Presence state. Empty hides the indicator.
    #[prop(optional, default = "")]
    presence: &'static str,
    /// Screen-reader text for the presence dot (`presenceProps.srText`).
    #[prop(optional, into)]
    presence_sr_text: String,
    /// `aria-live` on the presence dot. Default `off`.
    #[prop(optional, default = "off")]
    presence_aria_live: &'static str,
    /// Participant count. Values above 1 show the group badge (`group`).
    #[prop(optional)]
    group: Option<i32>,
    /// Overlay caption (`overlayText`).
    #[prop(optional, into)]
    overlay_text: String,
    /// Extra class on `.d-avatar__overlay` (`overlayClass`).
    #[prop(optional, into)]
    overlay_class: String,
    /// Image URL (`imageSrc`).
    #[prop(optional, into)]
    image_src: String,
    /// Image alt (`imageAlt`). Required by Vue when `imageSrc` is set; `""` is allowed.
    #[prop(optional)]
    image_alt: Option<String>,
    /// Name used for initials (`fullName`).
    #[prop(optional, into)]
    full_name: String,
    /// Renders a `<button>` (`interactive`).
    #[prop(optional)]
    interactive: bool,
    /// Deprecated alias of `interactive` (`clickable`).
    #[prop(optional)]
    clickable: bool,
    /// Accessible name for an icon avatar (`iconAriaLabel`).
    #[prop(optional, into)]
    icon_aria_label: String,
    /// Transparent background and no generated color (`iconOnly`).
    #[prop(optional)]
    icon_only: bool,
    /// Desaturated treatment (`deactivated`).
    #[prop(optional)]
    deactivated: bool,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
    /// Icon slot. Shown when there is no image.
    #[prop(optional)]
    icon: Option<ViewFn>,
    /// Overlay icon slot (`overlayIcon`). Wins over `overlay_text`.
    #[prop(optional)]
    overlay_icon: Option<ViewFn>,
) -> impl IntoView {
    let _icon_size = avatar_icon_size(size, &icon_size);
    let id = if id.is_empty() {
        unique_id("dt-avatar")
    } else {
        id
    };
    let size = canonical_size(size);
    let interactive = clickable || interactive;
    let has_icon = icon.is_some();
    let color_pair = resolved_color(has_icon, icon_only, &family, &variant, &color, &seed);
    let family_attr = color_pair.map(|pair| pair.0.to_string());
    let variant_attr = color_pair.map(|pair| pair.1.to_string());
    let root_class = avatar_classes(
        size,
        group,
        &avatar_class,
        interactive,
        presence,
        icon_only,
        deactivated,
    );

    let image_ok = RwSignal::new(None::<bool>);
    let has_image = !image_src.is_empty();
    let canvas_extra = canvas_class.clone();
    let initials = format_initials(&full_name, size);
    let icon_extra = icon_class.clone();
    let grouped = show_group(group);
    let group_count = group.filter(|_| grouped);
    let group_label = group_count.map(|count| formatted_group(count, size));
    let group_name = group_count.map(|count| format!("{count} participants"));

    let image_src_attr = image_src.clone();
    let show_image = move || has_image && image_ok.get() != Some(false);
    let accessible_name = if !interactive {
        String::new()
    } else if has_icon && !icon_aria_label.is_empty() {
        icon_aria_label.clone()
    } else if has_image {
        image_alt.clone().unwrap_or_default()
    } else {
        full_name.clone()
    };

    let image_src_attr = image_src_attr;
    let body = view! {
        <div class=move || {
            join_class([
                canvas_extra.as_str(),
                "d-avatar__canvas",
                if image_ok.get() == Some(true) {
                    "d-avatar--image-loaded"
                } else {
                    ""
                },
            ])
        }>
            <div class="d-avatar__canvas-inner">
                {has_image.then(|| {
                    let image_src_attr = image_src_attr.clone();
                    let image_alt = image_alt.clone();
                    view! {
                        <img
                            class=move || {
                                if image_ok.get() == Some(false) {
                                    "d-avatar__image d-d-none"
                                } else {
                                    "d-avatar__image"
                                }
                            }
                            data-qa="dt-avatar-image"
                            src=image_src_attr
                            alt=image_alt
                            on:load=move |_| image_ok.set(Some(true))
                            on:error=move |_| image_ok.set(Some(false))
                        />
                    }
                })}
                {icon.map(|slot| {
                    let label = icon_aria_label.clone();
                    let labelled = interactive && !label.is_empty();
                    view! {
                        <div
                            class=move || {
                                if show_image() {
                                    "d-d-none".to_string()
                                } else {
                                    join_class([icon_extra.as_str(), "d-avatar__icon"])
                                }
                            }
                            aria-label=labelled.then_some(label)
                            data-qa="dt-avatar-icon"
                        >
                            {slot.run()}
                        </div>
                    }
                })}
                {(!has_icon).then(|| {
                    view! {
                        <span class=move || {
                            if show_image() {
                                "d-d-none".to_string()
                            } else {
                                "d-avatar__initials".to_string()
                            }
                        }>{initials.clone()}</span>
                    }
                })}
            </div>
        </div>
        {(overlay_icon.is_some() || !overlay_text.is_empty()).then(|| {
            let overlay_class = join_class([
                "d-avatar__overlay",
                overlay_class.as_str(),
                if overlay_icon.is_some() {
                    "d-avatar__overlay-icon"
                } else {
                    ""
                },
            ]);
            view! {
                <div class=overlay_class>
                    {if let Some(slot) = overlay_icon {
                        slot.run().into_any()
                    } else {
                        view! { <p class="d-avatar__overlay-text">{overlay_text.clone()}</p> }.into_any()
                    }}
                </div>
            }
        })}
        {group_label.map(|label| {
            let group_name = group_name.clone().unwrap_or_default();
            view! {
                <span
                    class="d-avatar__count"
                    data-qa="dt-avatar-count"
                    role="img"
                    aria-label=group_name
                >
                    {label}
                </span>
            }
        })}
        {(!presence.is_empty() && !grouped && presence_supported(size)).then(|| {
            let presence_class = join_class([
                "d-avatar__presence",
                presence_size_class(size),
            ]);
            let inner_class = match presence {
                "active" => "d-presence__inner d-presence__inner--active",
                "away" => "d-presence__inner d-presence__inner--away",
                "busy" => "d-presence__inner d-presence__inner--busy",
                "offline" => "d-presence__inner d-presence__inner--offline",
                "dnd" => "d-presence__inner d-presence__inner--dnd",
                _ => "d-presence__inner",
            };
            let glyph = presence_glyph(presence);
            view! {
                <div class=format!("d-presence {presence_class}") data-qa="dt-presence" role="status" aria-live=presence_aria_live>
                    {(!presence_sr_text.is_empty()).then(|| {
                        view! {
                            <span data-qa="dt-presence-sr-text" class="d-vi-visible-sr">
                                {format!("{presence_sr_text} ")}
                            </span>
                        }
                    })}
                    <div class=inner_class data-qa="dt-presence-inner">
                        {glyph}
                    </div>
                </div>
            }
        })}
    };

    let labelled = !accessible_name.is_empty();
    if interactive {
        view! {
            <button
                id=id
                type="button"
                class=root_class
                data-qa="dt-avatar"
                data-avatar-family=family_attr
                data-avatar-variant=variant_attr
                aria-label=labelled.then_some(accessible_name)
                on:click=move |_| {
                    if let Some(on_click) = on_click {
                        on_click.run(());
                    }
                }
            >
                {body}
            </button>
        }
        .into_any()
    } else {
        view! {
            <div
                id=id
                class=root_class
                data-qa="dt-avatar"
                data-avatar-family=family_attr
                data-avatar-variant=variant_attr
            >
                {body}
            </div>
        }
        .into_any()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_matches_dialtone_java_hash_code() {
        assert_eq!(java_hash_code("seed"), 3_526_257);
        assert_eq!(java_hash_code("user-1"), -836_031_825);
        assert_eq!(java_hash_code("Ada Lovelace"), -878_886_773);
    }

    #[test]
    fn seed_picks_family_and_variant() {
        assert_eq!(family_variant_from_seed("seed"), (10, 4));
        assert_eq!(family_variant_from_seed("user-1"), (10, 8));
        assert_eq!(family_variant_from_seed("Ada Lovelace"), (6, 4));
        assert_eq!(family_variant_from_seed("ada"), (7, 4));
        assert_eq!(color_to_family_variant("540"), Some((5, 4)));
        assert_eq!(color_to_family_variant("500"), Some((5, 0)));
        assert_eq!(color_to_family_variant("1200"), Some((12, 0)));
        assert_eq!(color_to_family_variant("000"), None);
        assert_eq!(color_to_family_variant("1800"), None);
        assert_eq!(color_to_family_variant("545"), None);
        assert_eq!(
            resolved_color(false, false, "", "", "540", "seed"),
            Some((5, 4))
        );
        assert_eq!(
            resolved_color(false, false, "2", "", "540", "seed"),
            Some((2, 4))
        );
        assert_eq!(
            resolved_color(false, false, "", "9", "", "seed"),
            Some((10, 9))
        );
        assert_eq!(resolved_color(true, false, "2", "3", "540", "seed"), None);
        assert_eq!(resolved_color(false, true, "2", "3", "540", "seed"), None);
    }

    #[test]
    fn initials_follow_the_vue_helper() {
        assert_eq!(extract_initials("Ada"), "Ad");
        assert_eq!(extract_initials("Ada Lovelace"), "AL");
        assert_eq!(extract_initials("José María"), "JM");
        assert_eq!(extract_initials("John Q. Public"), "JP");
        assert_eq!(extract_initials("  "), "");
        assert_eq!(extract_initials("李 白"), "李白");
        assert_eq!(extract_initials("A"), "A");
        assert_eq!(format_initials("Ada Lovelace", "sm"), "A");
        assert_eq!(format_initials("Ada Lovelace", "200"), "A");
        assert_eq!(format_initials("Ada Lovelace", "150"), "A");
        assert_eq!(format_initials("Ada Lovelace", "xs"), "");
        assert_eq!(format_initials("Ada Lovelace", "100"), "");
        assert_eq!(format_initials("Ada", "md"), "Ad");
        assert_eq!(format_initials("Ada", "250"), "Ad");
    }

    #[test]
    fn group_keeps_size_and_caps_the_count() {
        assert_eq!(canonical_size("lg"), "lg");
        assert_eq!(canonical_size("nope"), "300");
        assert_eq!(formatted_group(9, "300"), "9");
        assert_eq!(formatted_group(12, "200"), "9+");
        assert_eq!(formatted_group(12, "sm"), "9+");
        assert_eq!(formatted_group(12, "300"), "12");
        assert_eq!(formatted_group(100, "500"), "99+");
        assert_eq!(formatted_group(100, "100"), "9+");
        assert!(show_group(Some(2)));
        assert!(!show_group(Some(1)));
        assert!(presence_supported("500"));
        assert!(presence_supported("lg"));
        assert!(!presence_supported("600"));
        assert!(!presence_supported("xl"));
    }

    #[test]
    fn classes_follow_the_staging_avatar() {
        assert_eq!(
            avatar_classes("300", None, "", false, "", false, false),
            "d-avatar d-avatar--size-300"
        );
        assert_eq!(
            avatar_classes("md", None, "", false, "active", false, false),
            "d-avatar d-avatar--md d-avatar--presence"
        );
        assert_eq!(
            avatar_classes("lg", Some(12), "extra", true, "active", false, false),
            "d-avatar d-avatar--lg extra d-avatar--group d-avatar--group-digits-2 d-avatar--clickable"
        );
        assert_eq!(
            avatar_classes("200", Some(12), "", false, "busy", false, false),
            "d-avatar d-avatar--size-200 d-avatar--group d-avatar--group-digits-2"
        );
        assert_eq!(
            avatar_classes("md", Some(120), "", false, "busy", true, true),
            "d-avatar d-avatar--md d-avatar--group d-avatar--group-digits-3 d-avatar--icon-only d-avatar--deactivated"
        );
        assert_eq!(
            avatar_classes("xl", None, "", false, "active", false, false),
            "d-avatar d-avatar--xl"
        );
        assert_eq!(avatar_icon_size("lg", ""), "600");
        assert_eq!(avatar_icon_size("400", ""), "500");
        assert_eq!(avatar_icon_size("lg", "100"), "100");
    }
}
