use super::*;

const WHITESPACE: &[(&str, &str)] = &[
    ("", "Default"),
    ("full", "Full"),
    ("true", "True"),
    ("false", "False"),
];

fn split_list(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(str::to_string)
        .collect()
}

fn rust_string_vec(raw: &str) -> String {
    let items = raw
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| format!("{}.to_string()", rust_string(part)))
        .collect::<Vec<_>>()
        .join(", ");
    format!("vec![{items}]")
}

#[component]
pub(crate) fn RichTextEditorPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("rich_text_editor.rs")));
    let value = RwSignal::new("Hello from Dialtone".to_string());
    let placeholder = RwSignal::new("Type here...".to_string());
    let aria_label = RwSignal::new("Message".to_string());
    let input_class = RwSignal::new(String::new());
    let output_format = RwSignal::new("html");
    let format_signal = Signal::derive(move || output_format.get().to_string());
    let editable = RwSignal::new(true);
    let allow_typing = RwSignal::new(true);
    let paste_rich_text = RwSignal::new(true);
    let allow_line_breaks = RwSignal::new(false);
    let link = RwSignal::new(true);
    let auto_focus = RwSignal::new(String::new());
    let custom_link = RwSignal::new(false);
    let link_phones = RwSignal::new(String::new());
    let show_link_bubble = RwSignal::new(true);
    let allow_blockquote = RwSignal::new(true);
    let allow_bold = RwSignal::new(true);
    let allow_bullet_list = RwSignal::new(true);
    let allow_italic = RwSignal::new(true);
    let allow_strike = RwSignal::new(true);
    let allow_underline = RwSignal::new(true);
    let allow_code = RwSignal::new(true);
    let allow_codeblock = RwSignal::new(true);
    let allow_inline_images = RwSignal::new(false);
    let allow_font_color = RwSignal::new(false);
    let allow_background_color = RwSignal::new(false);
    let allow_font_size = RwSignal::new(false);
    let allow_font_family = RwSignal::new(false);
    let allow_line_height = RwSignal::new(false);
    let allow_variable = RwSignal::new(false);
    let variable_items = RwSignal::new(String::new());
    let preserve_whitespace = RwSignal::new("");
    let use_div_tags = RwSignal::new(false);
    let allow_tables = RwSignal::new(false);
    let allow_text_align = RwSignal::new(true);
    let allow_image_resize = RwSignal::new(false);
    let preview = ViewFn::from(move || {
        view! {
            <div class="w-full">
                {move || {
                    let placeholder_text = placeholder.get();
                    let label = aria_label.get();
                    let extra = input_class.get();
                    let editing = editable.get();
                    let typing = allow_typing.get();
                    let paste = paste_rich_text.get();
                    let breaks = allow_line_breaks.get();
                    let links = link.get();
                    let focus = auto_focus.get();
                    let custom = custom_link.get();
                    let phones = split_list(&link_phones.get());
                    let bubble = show_link_bubble.get();
                    let blockquote = allow_blockquote.get();
                    let bold = allow_bold.get();
                    let bullets = allow_bullet_list.get();
                    let italic = allow_italic.get();
                    let strike = allow_strike.get();
                    let underline = allow_underline.get();
                    let code = allow_code.get();
                    let codeblock = allow_codeblock.get();
                    let images = allow_inline_images.get();
                    let font_color = allow_font_color.get();
                    let background = allow_background_color.get();
                    let font_size = allow_font_size.get();
                    let font_family = allow_font_family.get();
                    let line_height = allow_line_height.get();
                    let variable = allow_variable.get();
                    let variables = split_list(&variable_items.get());
                    let whitespace = preserve_whitespace.get();
                    let div_tags = use_div_tags.get();
                    let tables = allow_tables.get();
                    let text_align = allow_text_align.get();
                    let image_resize = allow_image_resize.get();
                    view! {
                        <DtRichTextEditor
                            value=value
                            on_change=move |next| value.set(next)
                            input_aria_label=label
                            input_class=extra
                            auto_focus=focus
                            output_format=format_signal
                            placeholder=placeholder_text
                            editable=editing
                            allow_typing=typing
                            paste_rich_text=paste
                            allow_line_breaks=breaks
                            link=links
                            custom_link=custom
                            link_phone_numbers=phones
                            show_link_bubble_menu=bubble
                            allow_blockquote=blockquote
                            allow_bold=bold
                            allow_bullet_list=bullets
                            allow_italic=italic
                            allow_strike=strike
                            allow_underline=underline
                            allow_code=code
                            allow_codeblock=codeblock
                            allow_inline_images=images
                            allow_font_color=font_color
                            allow_background_color=background
                            allow_font_size=font_size
                            allow_font_family=font_family
                            allow_line_height=line_height
                            allow_variable=variable
                            variable_items=variables
                            preserve_whitespace=whitespace
                            use_div_tags=div_tags
                            allow_tables=tables
                            allow_text_align=text_align
                            allow_image_resize=image_resize
                        />
                    }
                }}
                <p class="m-0 text-xs">{move || value.get()}</p>
            </div>
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Value" hint="The editor updates this value." value=value />
            <SettingText label="Accessible name" value=aria_label />
            <SettingText label="Placeholder" value=placeholder />
            <SettingText
                label="Auto focus"
                hint="Empty skips focus. true, false, start, end, all, or a character offset."
                value=auto_focus
            />
            <SettingSelect
                label="Output format"
                value=output_format
                options=vec![
                    ("html", "HTML"),
                    ("text", "Text"),
                    ("markdown", "Markdown"),
                    ("json", "JSON"),
                ]
            />
            <SettingSelect label="Preserve whitespace" value=preserve_whitespace options=WHITESPACE.to_vec() />
            <SettingText label="Link phone numbers" hint="Comma-separated. Empty leaves them unset." value=link_phones />
            <SettingText label="Variable items" hint="Comma-separated. Empty leaves them unset." value=variable_items />
            <SettingCheck label="Editable" value=editable />
            <SettingCheck label="Allow typing" value=allow_typing />
            <SettingCheck label="Paste rich text" value=paste_rich_text />
            <SettingCheck label="Line breaks" value=allow_line_breaks />
            <SettingCheck label="Links" value=link />
            <SettingCheck label="Custom link" value=custom_link />
            <SettingCheck label="Show link bubble menu" value=show_link_bubble />
            <SettingCheck label="Allow blockquote" value=allow_blockquote />
            <SettingCheck label="Allow bold" value=allow_bold />
            <SettingCheck label="Allow bullet list" value=allow_bullet_list />
            <SettingCheck label="Allow italic" value=allow_italic />
            <SettingCheck label="Allow strike" value=allow_strike />
            <SettingCheck label="Allow underline" value=allow_underline />
            <SettingCheck label="Allow code" value=allow_code />
            <SettingCheck label="Allow codeblock" value=allow_codeblock />
            <SettingCheck label="Allow inline images" value=allow_inline_images />
            <SettingCheck label="Allow font color" value=allow_font_color />
            <SettingCheck label="Allow background color" value=allow_background_color />
            <SettingCheck label="Allow font size" value=allow_font_size />
            <SettingCheck label="Allow font family" value=allow_font_family />
            <SettingCheck label="Allow line height" value=allow_line_height />
            <SettingCheck label="Allow variable" value=allow_variable />
            <SettingCheck label="Use div tags" value=use_div_tags />
            <SettingCheck label="Allow tables" value=allow_tables />
            <SettingCheck label="Allow text align" value=allow_text_align />
            <SettingCheck label="Allow image resize" value=allow_image_resize />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "input_aria_label", &aria_label.get());
        push_str(&mut props, "placeholder", &placeholder.get());
        push_str(&mut props, "output_format", output_format.get());
        push_str(&mut props, "input_class", &input_class.get());
        push_str(&mut props, "auto_focus", &auto_focus.get());
        push_str(&mut props, "value", &value.get());
        push_expr(&mut props, "on_change", "move |next| value.set(next)");
        push_str(&mut props, "preserve_whitespace", preserve_whitespace.get());
        let phones = link_phones.get();
        if !phones.trim().is_empty() {
            push_expr(&mut props, "link_phone_numbers", rust_string_vec(&phones));
        }
        let variables = variable_items.get();
        if !variables.trim().is_empty() {
            push_expr(&mut props, "variable_items", rust_string_vec(&variables));
        }
        push_flag(&mut props, "editable", editable.get());
        if !allow_typing.get() {
            push_flag(&mut props, "allow_typing", false);
        }
        if !paste_rich_text.get() {
            push_flag(&mut props, "paste_rich_text", false);
        }
        push_bool(&mut props, "allow_line_breaks", allow_line_breaks.get());
        push_bool(&mut props, "link", link.get());
        push_bool(&mut props, "custom_link", custom_link.get());
        if !show_link_bubble.get() {
            push_flag(&mut props, "show_link_bubble_menu", false);
        }
        if !allow_blockquote.get() {
            push_flag(&mut props, "allow_blockquote", false);
        }
        if !allow_bold.get() {
            push_flag(&mut props, "allow_bold", false);
        }
        if !allow_bullet_list.get() {
            push_flag(&mut props, "allow_bullet_list", false);
        }
        if !allow_italic.get() {
            push_flag(&mut props, "allow_italic", false);
        }
        if !allow_strike.get() {
            push_flag(&mut props, "allow_strike", false);
        }
        if !allow_underline.get() {
            push_flag(&mut props, "allow_underline", false);
        }
        if !allow_code.get() {
            push_flag(&mut props, "allow_code", false);
        }
        if !allow_codeblock.get() {
            push_flag(&mut props, "allow_codeblock", false);
        }
        push_bool(&mut props, "allow_inline_images", allow_inline_images.get());
        push_bool(&mut props, "allow_font_color", allow_font_color.get());
        push_bool(
            &mut props,
            "allow_background_color",
            allow_background_color.get(),
        );
        push_bool(&mut props, "allow_font_size", allow_font_size.get());
        push_bool(&mut props, "allow_font_family", allow_font_family.get());
        push_bool(&mut props, "allow_line_height", allow_line_height.get());
        push_bool(&mut props, "allow_variable", allow_variable.get());
        push_bool(&mut props, "use_div_tags", use_div_tags.get());
        push_bool(&mut props, "allow_tables", allow_tables.get());
        if !allow_text_align.get() {
            push_flag(&mut props, "allow_text_align", false);
        }
        push_bool(&mut props, "allow_image_resize", allow_image_resize.get());
        view_of(&element("DtRichTextEditor", &props, None))
    });
    let classes = class_panel(vec![ClassField {
        label: "Input class",
        hint: "Extra class on the editable element.",
        value: input_class,
        off: None,
    }]);
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <Play stack=true preview=preview settings=settings classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Placeholder">
                    <PlaceholderEditor />
                </Variant>
                <Variant title="Link">
                    <LinkEditor />
                </Variant>
                <Variant title="Read only">
                    <ReadOnlyEditor />
                </Variant>
            </div>
        </section>
    }
}

#[component]
fn PlaceholderEditor() -> impl IntoView {
    let value = RwSignal::new(String::new());
    view! {
        <DtRichTextEditor
            value=value
            on_change=move |next| value.set(next)
            input_aria_label="Message"
            placeholder="Type here..."
        />
    }
}

#[component]
fn LinkEditor() -> impl IntoView {
    let value = RwSignal::new(
        "<p>Visit <a href=\"https://www.dialpad.com\" class=\"d-link d-wb-break-all\">dialpad.com</a></p>"
            .to_string(),
    );
    view! {
        <DtRichTextEditor
            value=value
            on_change=move |next| value.set(next)
            input_aria_label="Message with links"
            link=true
        />
    }
}

#[component]
fn ReadOnlyEditor() -> impl IntoView {
    let value = RwSignal::new("This note is read only.".to_string());
    view! {
        <DtRichTextEditor
            value=value
            input_aria_label="Read only message"
            editable=false
            output_format="text"
        />
    }
}
