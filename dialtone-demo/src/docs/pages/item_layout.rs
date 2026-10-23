use super::*;

const TAGS: &[(&str, &str)] = &[
    ("div", "div"),
    ("section", "section"),
    ("span", "span"),
    ("nav", "nav"),
    ("ul", "ul"),
    ("ol", "ol"),
    ("li", "li"),
    ("article", "article"),
    ("header", "header"),
    ("footer", "footer"),
    ("main", "main"),
    ("aside", "aside"),
    ("p", "p"),
    ("form", "form"),
];

#[component]
pub(crate) fn ItemLayoutPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("item_layout.rs")));
    let title = RwSignal::new("Ada Lovelace".to_string());
    let subtitle = RwSignal::new("Primary".to_string());
    let show_subtitle = RwSignal::new(true);
    let show_start = RwSignal::new(true);
    let show_end = RwSignal::new(true);
    let block_end = RwSignal::new(String::new());
    let selected_text = RwSignal::new(String::new());
    let unstyled = RwSignal::new(false);
    let tag = RwSignal::new("div");
    let root_class = RwSignal::new(String::new());
    let start_class = RwSignal::new(String::new());
    let content_class = RwSignal::new(String::new());
    let title_class = RwSignal::new(String::new());
    let subtitle_class = RwSignal::new(String::new());
    let block_end_class = RwSignal::new(String::new());
    let end_class = RwSignal::new(String::new());
    let selected_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let layout_class = root_class.get();
                let unstyled = unstyled.get();
                let element = tag.get();
                let start_class = start_class.get();
                let content_class = content_class.get();
                let title_class = title_class.get();
                let subtitle_class = subtitle_class.get();
                let block_end_class = block_end_class.get();
                let end_class = end_class.get();
                let selected_class = selected_class.get();
                let title_text = title.get();
                let start = show_start.get().then(|| {
                    ViewFn::from(move || {
                        let name = title.get();
                        if name == "Ada Lovelace" {
                            view! {
                                <DtAvatar
                                    full_name=name
                                    size="sm"
                                    image_src="/assets/images/person.png"
                                    image_alt="Ada Lovelace".to_string()
                                />
                            }
                            .into_any()
                        } else {
                            view! { <DtAvatar full_name=name size="sm" /> }.into_any()
                        }
                    })
                });
                let subtitle_view = show_subtitle.get().then(|| {
                    ViewFn::from(move || view! { <span>{subtitle.get()}</span> })
                });
                let bottom_view = text_slot(block_end.get());
                let end = show_end.get().then(|| {
                    ViewFn::from(|| view! { <DtBadge text="On call" type_="success" /> })
                });
                let selected_view = text_slot(selected_text.get());
                macro_rules! item_layout {
                    ($title:expr $(, $name:ident = $value:expr)* $(,)?) => {
                        view! {
                            <DtItemLayout
                                class=layout_class.clone()
                                unstyled=unstyled
                                r#as=element
                                start_class=start_class.clone()
                                content_class=content_class.clone()
                                title_class=title_class.clone()
                                subtitle_class=subtitle_class.clone()
                                block_end_class=block_end_class.clone()
                                end_class=end_class.clone()
                                selected_class=selected_class.clone()
                                $($name=$value)*
                            >
                                {$title.clone()}
                            </DtItemLayout>
                        }
                        .into_any()
                    };
                }
                match (start, subtitle_view, bottom_view, end, selected_view) {
                    (None, None, None, None, None) => item_layout!(title_text),
                    (Some(left), None, None, None, None) => item_layout!(title_text, start = left),
                    (None, Some(subtitle_view), None, None, None) => item_layout!(title_text, subtitle = subtitle_view),
                    (Some(left), Some(subtitle_view), None, None, None) => item_layout!(title_text, start = left, subtitle = subtitle_view),
                    (None, None, Some(bottom_view), None, None) => item_layout!(title_text, block_end = bottom_view),
                    (Some(left), None, Some(bottom_view), None, None) => item_layout!(title_text, start = left, block_end = bottom_view),
                    (None, Some(subtitle_view), Some(bottom_view), None, None) => item_layout!(title_text, subtitle = subtitle_view, block_end = bottom_view),
                    (Some(left), Some(subtitle_view), Some(bottom_view), None, None) => item_layout!(title_text, start = left, subtitle = subtitle_view, block_end = bottom_view),
                    (None, None, None, Some(right), None) => item_layout!(title_text, end = right),
                    (Some(left), None, None, Some(right), None) => item_layout!(title_text, start = left, end = right),
                    (None, Some(subtitle_view), None, Some(right), None) => item_layout!(title_text, subtitle = subtitle_view, end = right),
                    (Some(left), Some(subtitle_view), None, Some(right), None) => item_layout!(title_text, start = left, subtitle = subtitle_view, end = right),
                    (None, None, Some(bottom_view), Some(right), None) => item_layout!(title_text, block_end = bottom_view, end = right),
                    (Some(left), None, Some(bottom_view), Some(right), None) => item_layout!(title_text, start = left, block_end = bottom_view, end = right),
                    (None, Some(subtitle_view), Some(bottom_view), Some(right), None) => item_layout!(title_text, subtitle = subtitle_view, block_end = bottom_view, end = right),
                    (Some(left), Some(subtitle_view), Some(bottom_view), Some(right), None) => item_layout!(title_text, start = left, subtitle = subtitle_view, block_end = bottom_view, end = right),
                    (None, None, None, None, Some(selected_view)) => item_layout!(title_text, selected = selected_view),
                    (Some(left), None, None, None, Some(selected_view)) => item_layout!(title_text, start = left, selected = selected_view),
                    (None, Some(subtitle_view), None, None, Some(selected_view)) => item_layout!(title_text, subtitle = subtitle_view, selected = selected_view),
                    (Some(left), Some(subtitle_view), None, None, Some(selected_view)) => item_layout!(title_text, start = left, subtitle = subtitle_view, selected = selected_view),
                    (None, None, Some(bottom_view), None, Some(selected_view)) => item_layout!(title_text, block_end = bottom_view, selected = selected_view),
                    (Some(left), None, Some(bottom_view), None, Some(selected_view)) => item_layout!(title_text, start = left, block_end = bottom_view, selected = selected_view),
                    (None, Some(subtitle_view), Some(bottom_view), None, Some(selected_view)) => item_layout!(title_text, subtitle = subtitle_view, block_end = bottom_view, selected = selected_view),
                    (Some(left), Some(subtitle_view), Some(bottom_view), None, Some(selected_view)) => item_layout!(title_text, start = left, subtitle = subtitle_view, block_end = bottom_view, selected = selected_view),
                    (None, None, None, Some(right), Some(selected_view)) => item_layout!(title_text, end = right, selected = selected_view),
                    (Some(left), None, None, Some(right), Some(selected_view)) => item_layout!(title_text, start = left, end = right, selected = selected_view),
                    (None, Some(subtitle_view), None, Some(right), Some(selected_view)) => item_layout!(title_text, subtitle = subtitle_view, end = right, selected = selected_view),
                    (Some(left), Some(subtitle_view), None, Some(right), Some(selected_view)) => item_layout!(title_text, start = left, subtitle = subtitle_view, end = right, selected = selected_view),
                    (None, None, Some(bottom_view), Some(right), Some(selected_view)) => item_layout!(title_text, block_end = bottom_view, end = right, selected = selected_view),
                    (Some(left), None, Some(bottom_view), Some(right), Some(selected_view)) => item_layout!(title_text, start = left, block_end = bottom_view, end = right, selected = selected_view),
                    (None, Some(subtitle_view), Some(bottom_view), Some(right), Some(selected_view)) => item_layout!(title_text, subtitle = subtitle_view, block_end = bottom_view, end = right, selected = selected_view),
                    (Some(left), Some(subtitle_view), Some(bottom_view), Some(right), Some(selected_view)) => item_layout!(title_text, start = left, subtitle = subtitle_view, block_end = bottom_view, end = right, selected = selected_view),
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingSelect label="As" value=tag options=TAGS.to_vec() />
            <SettingCheck label="Unstyled" value=unstyled />
        }
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingCheck label="Show start" value=show_start />
            <SettingText label="Title" value=title />
            <SettingText label="Subtitle" value=subtitle />
            <SettingCheck label="Show subtitle" value=show_subtitle />
            <SettingCheck label="Show end" value=show_end />
            <SettingText label="Block end" hint="Block end slot. Empty hides it." value=block_end />
            <SettingText label="Selected" hint="Selected slot. Empty hides it." value=selected_text />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        let name = title.get();
        if show_start.get() {
            let avatar = if name == "Ada Lovelace" {
                format!(
                    "ViewFn::from(|| view! {{ <DtAvatar full_name={} size=\"sm\" image_src=\"/assets/images/person.png\" image_alt=\"Ada Lovelace\".to_string() /> }})",
                    rust_string(&name)
                )
            } else {
                format!(
                    "ViewFn::from(|| view! {{ <DtAvatar full_name={} size=\"sm\" /> }})",
                    rust_string(&name)
                )
            };
            push_expr(&mut props, "start", avatar);
        }
        if show_subtitle.get() {
            push_expr(
                &mut props,
                "subtitle",
                format!(
                    "ViewFn::from(|| view! {{ <span>{}</span> }})",
                    rust_string(&subtitle.get())
                ),
            );
        }
        if show_end.get() {
            push_expr(
                &mut props,
                "end",
                "ViewFn::from(|| view! { <DtBadge text=\"On call\" type_=\"success\" /> })",
            );
        }
        if !block_end.get().is_empty() {
            push_expr(
                &mut props,
                "block_end",
                format!(
                    "ViewFn::from(|| view! {{ <span>{}</span> }})",
                    rust_string(&block_end.get())
                ),
            );
        }
        if !selected_text.get().is_empty() {
            push_expr(
                &mut props,
                "selected",
                format!(
                    "ViewFn::from(|| view! {{ <span>{}</span> }})",
                    rust_string(&selected_text.get())
                ),
            );
        }
        if tag.get() != "div" {
            push_str(&mut props, "r#as", tag.get());
        }
        push_bool(&mut props, "unstyled", unstyled.get());
        push_str(&mut props, "class", &root_class.get());
        push_str(&mut props, "start_class", &start_class.get());
        push_str(&mut props, "content_class", &content_class.get());
        push_str(&mut props, "title_class", &title_class.get());
        push_str(&mut props, "subtitle_class", &subtitle_class.get());
        push_str(&mut props, "block_end_class", &block_end_class.get());
        push_str(&mut props, "end_class", &end_class.get());
        push_str(&mut props, "selected_class", &selected_class.get());
        view_of(&element(
            "DtItemLayout",
            &props,
            Some(&rust_string(&title.get())),
        ))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <Play
                stack=true
                preview=preview
                settings=settings
                slots=slots
                classes=class_panel(vec![
                    ClassField {
                        label: "Class",
                        hint: ROOT_CLASS,
                        value: root_class,
                        off: None,
                    },
                    ClassField {
                        label: "Start class",
                        hint: "Extra class on the start section (`startClass`).",
                        value: start_class,
                        off: None,
                    },
                    ClassField {
                        label: "Content class",
                        hint: "Extra class on the content section (`contentClass`).",
                        value: content_class,
                        off: None,
                    },
                    ClassField {
                        label: "Title class",
                        hint: "Extra class on the title (`titleClass`).",
                        value: title_class,
                        off: None,
                    },
                    ClassField {
                        label: "Subtitle class",
                        hint: "Extra class on the subtitle (`subtitleClass`).",
                        value: subtitle_class,
                        off: None,
                    },
                    ClassField {
                        label: "Block end class",
                        hint: "Extra class on the block end (`blockEndClass`).",
                        value: block_end_class,
                        off: None,
                    },
                    ClassField {
                        label: "End class",
                        hint: "Extra class on the end section (`endClass`).",
                        value: end_class,
                        off: None,
                    },
                    ClassField {
                        label: "Selected class",
                        hint: "Extra class on the selected section (`selectedClass`).",
                        value: selected_class,
                        off: None,
                    },
                ])
                code=code
            />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="With Default Styling">
                    <div class="d-d-block d-w-500">
                        <DtItemLayout
                            start=ViewFn::from(|| view! { <DtIconLock size="300" /> })
                            subtitle=ViewFn::from(|| view! { <span>"Subtitle"</span> })
                            block_end=ViewFn::from(|| view! { <DtBadge text="Content" /> })
                            end=ViewFn::from(|| view! { <DtIconShare size="300" /> })
                            selected=ViewFn::from(|| view! { <DtIconCheck size="300" /> })
                        >
                            "Layout title"
                        </DtItemLayout>
                    </div>
                </Variant>
                <Variant title="Without Styling">
                    <div class="d-d-block d-w-500">
                        <DtItemLayout
                            unstyled=true
                            start=ViewFn::from(|| view! { <DtIconLock size="300" /> })
                            subtitle=ViewFn::from(|| view! { <span>"Subtitle"</span> })
                            block_end=ViewFn::from(|| view! { <DtBadge text="Content" /> })
                            end=ViewFn::from(|| view! { <DtIconShare size="300" /> })
                            selected=ViewFn::from(|| view! { <DtIconCheck size="300" /> })
                        >
                            "Layout title"
                        </DtItemLayout>
                    </div>
                </Variant>
                <Variant title="Two rows">
                    <DtItemLayout
                        start=ViewFn::from(|| view! { <DtAvatar full_name="Grace Hopper" size="sm" /> })
                        subtitle=ViewFn::from(|| view! { <span>"Secondary"</span> })
                        end=ViewFn::from(|| view! { <DtBadge text="Next" type_="info" /> })
                    >
                        "Grace Hopper"
                    </DtItemLayout>
                </Variant>
            </div>
        </section>
    }
}
