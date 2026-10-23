use super::*;

const PHOTO: &str = "/assets/images/person.png";
const MARK: &str = "/dialtone/images/favicons/favicon-dialtone__512.png";
const PREVIEW_CLASS: &str = "d-wmn-100 d-hmn-100 d-wmx-500 d-hmx-500";

const OPEN_MODES: &[(&str, &str)] = &[("unset", "Unset"), ("open", "Open"), ("closed", "Closed")];

#[component]
pub(crate) fn ImageViewerPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("image_viewer.rs")));
    let image_src = RwSignal::new(PHOTO.to_string());
    let image_alt = RwSignal::new("Image Alt Text".to_string());
    let aria_label = RwSignal::new("Click to open image".to_string());
    let image_button_class = RwSignal::new(PREVIEW_CLASS.to_string());
    let append_to = RwSignal::new(String::new());
    let root_class = RwSignal::new(String::new());
    let mode = RwSignal::new("unset");
    let shown = RwSignal::new(false);
    Effect::new(move |_| match mode.get() {
        "open" => shown.set(true),
        "closed" => shown.set(false),
        _ => {}
    });
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let mode = mode.get();
                let image_src = image_src.get();
                let image_alt = image_alt.get();
                let aria_label = aria_label.get();
                let image_button_class = image_button_class.get();
                let append_to = append_to.get();
                let class_name = root_class.get();
                if mode == "unset" {
                    view! {
                        <DtImageViewer
                            image_src=image_src
                            image_alt=image_alt
                            aria_label=aria_label
                            image_button_class=image_button_class
                            append_to=append_to
                            class=class_name
                        />
                    }
                    .into_any()
                } else {
                    view! {
                        <DtImageViewer
                            image_src=image_src
                            image_alt=image_alt
                            aria_label=aria_label
                            image_button_class=image_button_class
                            append_to=append_to
                            class=class_name
                            open=shown
                            on_open=move |next| shown.set(next)
                        />
                    }
                    .into_any()
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Image src" value=image_src />
            <SettingText label="Image alt" value=image_alt />
            <SettingText label="Aria label" value=aria_label />
            <SettingText label="Image class" value=image_button_class />
            <SettingText label="Append to" value=append_to hint="Empty uses body, or the nearest dialog." />
            <SettingSelect label="Open" value=mode options=OPEN_MODES.to_vec() />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "image_src", &image_src.get());
        push_str(&mut props, "image_alt", &image_alt.get());
        push_str(&mut props, "aria_label", &aria_label.get());
        push_str(&mut props, "image_button_class", &image_button_class.get());
        push_str(&mut props, "append_to", &append_to.get());
        push_str(&mut props, "class", &root_class.get());
        if mode.get() != "unset" {
            push_expr(&mut props, "open", "shown");
            push_expr(&mut props, "on_open", "move |next: bool| shown.set(next)");
        }
        view_of(&element("DtImageViewer", &props, None))
    });
    let classes = class_panel(vec![ClassField {
        label: "Class",
        hint: ROOT_CLASS,
        value: root_class,
        off: None,
    }]);
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">
                "Unset opens on click. Open and closed are the controlled open prop. Closing a controlled viewer calls on_open with false. The close label is Click to close. That button's fade is not animated. append_to is a CSS selector. body uses the nearest dialog, then the document body."
            </p>
            <Play preview=preview settings=settings classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Photo">
                    <DtImageViewer
                        image_src=PHOTO
                        image_alt="Ada Lovelace"
                        image_button_class=PREVIEW_CLASS
                        aria_label="Click to open image"
                    />
                </Variant>
                <Variant title="Mark">
                    <DtImageViewer
                        image_src=MARK
                        image_alt="Dialtone mark"
                        image_button_class=PREVIEW_CLASS
                        aria_label="Click to open image"
                    />
                </Variant>
            </div>
        </section>
    }
}
