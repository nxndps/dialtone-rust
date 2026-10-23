use leptos::prelude::*;

use super::emoji::{emoji_text_parts, icon_size_class, DtEmoji, EmojiTextPart};

const ELEMENTS: &[&str] = &[
    "div", "span", "p", "section", "article", "header", "footer", "li", "label",
];

/// Dialtone Vue `DtEmojiTextWrapper` (`dialtone/v9.190.0`).
///
/// `text` is the default slot's text. `:smile:` and unicode emoji become
/// [`DtEmoji`](crate::DtEmoji). Invalid shortcodes stay text. Vue also walks
/// element children and drops HTML comments. This port scans the string only.
/// `element_type` is the wrapper tag (`elementType`).
#[component]
pub fn DtEmojiTextWrapper(
    /// Wrapper tag (`elementType`). Default `div`.
    #[prop(optional, default = "div")]
    element_type: &'static str,
    /// Emoji icon size (`size`). Default `500`.
    #[prop(optional, default = "500")]
    size: &'static str,
    /// Text to scan for shortcodes and unicode emoji.
    #[prop(optional, into)]
    text: String,
) -> impl IntoView {
    if !ELEMENTS.contains(&element_type) {
        leptos::logging::warn!(
            "DtEmojiTextWrapper element_type `{element_type}` is not supported; using div"
        );
    }
    let _ = icon_size_class(size);
    let parts = emoji_text_parts(&text);
    let body = parts
        .into_iter()
        .map(|part| match part {
            EmojiTextPart::Text(value) => view! {
                <span class="d-emoji-text-wrapper__text">{value}</span>
            }
            .into_any(),
            EmojiTextPart::Emoji(code) => view! {
                <DtEmoji code=code size=size />
            }
            .into_any(),
        })
        .collect_view();
    let tag = if ELEMENTS.contains(&element_type) {
        element_type
    } else {
        "div"
    };

    macro_rules! wrap {
        ($name:ident) => {
            view! {
                <$name class="d-emoji-text-wrapper" data-qa="emoji-text-wrapper">
                    {body}
                </$name>
            }
            .into_any()
        };
    }

    match tag {
        "span" => wrap!(span),
        "p" => wrap!(p),
        "section" => wrap!(section),
        "article" => wrap!(article),
        "header" => wrap!(header),
        "footer" => wrap!(footer),
        "li" => wrap!(li),
        "label" => wrap!(label),
        _ => wrap!(div),
    }
}
