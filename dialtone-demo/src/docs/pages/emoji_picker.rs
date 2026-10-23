use super::*;
use dialtone::DtEmojiData;

const CUSTOM_EMOJI_IMAGE: &str =
    "https://cdn.jsdelivr.net/joypixels/assets/9.0/png/unicode/32/1f642.png";

fn custom_emoji(name: &str) -> DtEmojiData {
    let slug = name.to_ascii_lowercase().replace(' ', "_");
    DtEmojiData {
        name: name.to_string(),
        category: "Custom".to_string(),
        shortname: format!(":{slug}:"),
        shortname_alternates: Vec::new(),
        keywords: vec![name.to_string()],
        unicode_output: String::new(),
        unicode_character: String::new(),
        image: CUSTOM_EMOJI_IMAGE.to_string(),
        date_added: "custom".to_string(),
        custom: true,
    }
}

fn custom_list(raw: &str) -> Vec<DtEmojiData> {
    raw.split(',')
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(custom_emoji)
        .collect()
}

fn skin_token(tone: &str) -> &'static str {
    match tone {
        "Light" => "Light",
        "MediumLight" => "MediumLight",
        "Medium" => "Medium",
        "MediumDark" => "MediumDark",
        "Dark" => "Dark",
        _ => "Default",
    }
}

#[component]
pub(crate) fn EmojiPickerPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("emoji_picker.rs")));
    let show_search = RwSignal::new(true);
    let lazy_images = RwSignal::new(true);
    let skin = RwSignal::new("Default");
    let skin_tone = Signal::derive(move || skin.get().to_string());
    let custom_text = RwSignal::new(String::new());
    let custom_emojis = Signal::derive(move || custom_list(&custom_text.get()));
    let search_text = RwSignal::new(String::new());
    let show_add = RwSignal::new(false);
    let selected = RwSignal::new(String::new());
    let on_add = Callback::new(move |_: ()| selected.set("added emoji".to_string()));
    let preview = ViewFn::from(move || {
        view! {
            <div class="flex flex-col items-center gap-3">
                {move || {
                    let show_search = show_search.get();
                    let lazy_images = lazy_images.get();
                    let show_add = show_add.get();
                    view! {
                        <DtEmojiPicker
                            show_search=show_search
                            lazy_images=lazy_images
                            skin_tone=skin_tone
                            on_skin_tone=move |tone: String| skin.set(skin_token(&tone))
                            custom_emojis=custom_emojis
                            search_query=search_text.into()
                            show_add_emoji_button=show_add
                            on_add_emoji=on_add
                            on_selected_emoji=move |selection: DtEmojiSelection| {
                                selected.set(selection.emoji.name);
                            }
                        />
                    }
                }}
                <p class="docs-note">{move || {
                    let name = selected.get();
                    if name.is_empty() {
                        "Select an emoji.".to_string()
                    } else {
                        format!("Selected {name}.")
                    }
                }}</p>
            </div>
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingCheck label="Search" value=show_search />
            <SettingCheck label="Lazy images" value=lazy_images />
            <SettingText
                label="Custom emojis"
                hint="Comma-separated names. Empty hides the custom tab."
                value=custom_text
            />
            <SettingText label="Search query" hint="Empty leaves search uncontrolled." value=search_text />
            <SettingCheck label="Show add emoji" value=show_add />
            <SettingSelect
                label="Skin tone"
                value=skin
                options=vec![
                    ("Default", "Default"),
                    ("Light", "Light"),
                    ("MediumLight", "Medium light"),
                    ("Medium", "Medium"),
                    ("MediumDark", "Medium dark"),
                    ("Dark", "Dark"),
                ]
            />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        if !show_search.get() {
            props.push(("show_search", "false".to_string()));
        }
        if !lazy_images.get() {
            props.push(("lazy_images", "false".to_string()));
        }
        if !custom_text.get().trim().is_empty() {
            push_expr(&mut props, "custom_emojis", "custom_emojis");
        }
        if !search_text.get().is_empty() {
            push_expr(&mut props, "search_query", "search_query");
        }
        if show_add.get() {
            push_bool(&mut props, "show_add_emoji_button", true);
            push_expr(
                &mut props,
                "on_add_emoji",
                "Callback::new(|_| selected.set(\"added emoji\".to_string()))",
            );
        }
        push_expr(&mut props, "skin_tone", "skin_tone");
        push_expr(
            &mut props,
            "on_skin_tone",
            "move |tone: String| skin.set(skin_token(&tone))",
        );
        push_expr(
            &mut props,
            "on_selected_emoji",
            "move |selection: DtEmojiSelection| selected.set(selection.emoji.name)",
        );
        view_of(&element("DtEmojiPicker", &props, None))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"skin_tone is Signal::derive(move || skin.get().to_string()), created once for the page. skin_token maps the picker value onto Default, Light, MediumLight, Medium, MediumDark, or Dark. Search waits 300ms. Lazy images sets loading=\"lazy\" on the grid and on search results. The footer preview and the skin tone stay eager."</p>
            <Play stack=true preview=preview settings=settings code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Search hidden">
                    <DtEmojiPicker show_search=false />
                </Variant>
            </div>
        </section>
    }
}
