use super::*;

#[component]
pub(crate) fn EmojiTextWrapperPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("emoji_text_wrapper.rs")));
    let text = RwSignal::new("Nice work :smile: today".to_string());
    let element_type = RwSignal::new("div");
    let size = RwSignal::new("500");
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let text = text.get();
                let element_type = element_type.get();
                let size = size.get();
                view! { <DtEmojiTextWrapper text=text element_type=element_type size=size /> }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Text" value=text />
            <SettingSelect
                label="Element"
                value=element_type
                options=vec![
                    ("div", "div"),
                    ("span", "span"),
                    ("p", "p"),
                    ("section", "section"),
                    ("article", "article"),
                    ("header", "header"),
                    ("footer", "footer"),
                    ("li", "li"),
                    ("label", "label"),
                ]
            />
            <SettingSelect label="Size" value=size options=ICON_SIZE.to_vec() />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "element_type", element_type.get());
        push_str(&mut props, "size", size.get());
        props.push(("text", rust_string(&text.get())));
        view_of(&element("DtEmojiTextWrapper", &props, None))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"Words that are not a shortcode or a unicode emoji stay text. An unknown shortcode stays text too."</p>
            <Play stack=true preview=preview settings=settings code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Paragraph">
                    <DtEmojiTextWrapper element_type="p" text="Nice work :smile: today" />
                </Variant>
                <Variant title="Unicode">
                    <DtEmojiTextWrapper text="Hello 😀" size="500" />
                </Variant>
            </div>
        </section>
    }
}
