use super::*;

#[component]
pub(crate) fn EmojiPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("emoji.rs")));
    let code = RwSignal::new(":smile:".to_string());
    let size = RwSignal::new("500");
    let show_skeleton = RwSignal::new(true);
    let aria_label = RwSignal::new(String::new());
    let img_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let code = code.get();
                let size = size.get();
                let show_skeleton = show_skeleton.get();
                let aria_label = aria_label.get();
                let img_class = img_class.get();
                view! {
                    <DtEmoji
                        code=code
                        size=size
                        show_skeleton=show_skeleton
                        aria_label=aria_label
                        img_class=img_class
                    />
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Code" hint="Shortcode or unicode." value=code />
            <SettingSelect label="Size" value=size options=ICON_SIZE.to_vec() />
            <SettingCheck label="Skeleton" value=show_skeleton />
            <SettingText label="Aria label" hint="Empty uses the emoji name." value=aria_label />
        }
    });
    let code_sample = example_panel(move || {
        let mut props = Vec::new();
        props.push(("code", rust_string(&code.get())));
        push_str(&mut props, "size", size.get());
        if !show_skeleton.get() {
            props.push(("show_skeleton", "false".to_string()));
        }
        push_str(&mut props, "aria_label", &aria_label.get());
        push_str(&mut props, "img_class", &img_class.get());
        view_of(&element("DtEmoji", &props, None))
    });
    let classes = class_panel(vec![ClassField {
        label: "Image class",
        hint: "Extra class on the image.",
        value: img_class,
        off: None,
    }]);
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">":smile:, :grinning:, and :smiley: resolve. Any other shortcode stays an invalid emoji. The skeleton shows until the image loads."</p>
            <Play preview=preview settings=settings classes=classes code=code_sample />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Shortcodes" row=true>
                    <DtEmoji code=":grinning:" />
                    <DtEmoji code=":smile:" />
                    <DtEmoji code=":smiley:" />
                </Variant>
                <Variant title="Sizes" row=true>
                    <DtEmoji code=":smile:" size="200" />
                    <DtEmoji code=":smile:" size="500" />
                    <DtEmoji code=":smile:" size="800" />
                </Variant>
            </div>
        </section>
    }
}
