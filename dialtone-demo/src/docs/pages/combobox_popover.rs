use super::*;

macro_rules! combo_preview {
    (
        $label:expr,
        $description:expr,
        $placeholder:expr,
        $size:expr,
        $value:expr,
        $on_input:expr,
        $append_to:expr,
        $content_width:expr,
        $max_height:expr,
        $max_width:expr,
        $suggestions:expr,
        $arrow_keys:expr,
        $loading:expr,
        $empty:expr,
        $dialog_class:expr,
        $list_class:expr,
        $empty_state_class:expr,
        $header:expr,
        $footer:expr;
        $($extra:tt)*
    ) => {
        open_two!(DtComboboxWithPopover {
            label=$label
            description=$description
            placeholder=$placeholder
            size=$size
            value=$value
            on_input=$on_input
            append_to=$append_to
            content_width=$content_width
            max_height=$max_height
            max_width=$max_width
            has_suggestion_list=$suggestions
            open_with_arrow_keys=$arrow_keys
            loading=$loading
            empty_list=$empty
            empty_message="No matches"
            dialog_class=$dialog_class
            list_class=$list_class
            empty_state_class=$empty_state_class
            $($extra)*
        } {
            {PRODUCTS.iter().copied().map(|name| {
                view! { <li class="d-list-item" role="option">{name}</li> }
            }).collect_view()}
        }, $header, header, $footer, footer)
    };
}

#[component]
pub(crate) fn ComboboxPopoverPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("combobox_popover.rs")));
    let value = RwSignal::new(String::new());
    let label = RwSignal::new("Product".to_string());
    let description = RwSignal::new(String::new());
    let placeholder = RwSignal::new("Search products".to_string());
    let size = RwSignal::new("");
    let append_to = RwSignal::new("body");
    let show_list = RwSignal::new(false);
    let max_height = RwSignal::new(String::new());
    let max_width = RwSignal::new(String::new());
    let content_width = RwSignal::new("");
    let arrow_keys = RwSignal::new(false);
    let suggestions = RwSignal::new(true);
    let loading = RwSignal::new(false);
    let input_text = RwSignal::new(String::new());
    let empty = RwSignal::new(false);
    let dialog_class = RwSignal::new(String::new());
    let list_class = RwSignal::new(String::new());
    let empty_state_class = RwSignal::new(String::new());
    let header = RwSignal::new(String::new());
    let footer = RwSignal::new(String::new());
    let on_input = Callback::new(move |next: String| value.set(next));
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let label = label.get();
                let description = description.get();
                let placeholder = placeholder.get();
                let size = size.get();
                let append_to = append_to.get();
                let content_width = content_width.get();
                let max_height = max_height.get();
                let max_width = max_width.get();
                let suggestions = suggestions.get();
                let arrow_keys = arrow_keys.get();
                let loading = loading.get();
                let empty = empty.get();
                let dialog_class = dialog_class.get();
                let list_class = list_class.get();
                let empty_state_class = empty_state_class.get();
                let header_view = text_slot(header.get());
                let footer_view = text_slot(footer.get());
                let input_view = if input_text.get().is_empty() {
                    None
                } else {
                    Some(ViewFn::from(move || {
                        view! {
                            <input
                                class="d-input"
                                prop:value=move || value.get()
                                on:input=move |event| value.set(event_target_value(&event))
                            />
                        }
                    }))
                };
                let show = show_list.get();
                match (show, input_view) {
                    (false, None) => combo_preview!(
                        label, description, placeholder, size, value, on_input, append_to,
                        content_width, max_height, max_width, suggestions, arrow_keys, loading,
                        empty, dialog_class, list_class, empty_state_class, header_view, footer_view;
                    ),
                    (true, None) => combo_preview!(
                        label, description, placeholder, size, value, on_input, append_to,
                        content_width, max_height, max_width, suggestions, arrow_keys, loading,
                        empty, dialog_class, list_class, empty_state_class, header_view, footer_view;
                        show_list=Signal::derive(|| true)
                    ),
                    (false, Some(input_view)) => combo_preview!(
                        label, description, placeholder, size, value, on_input, append_to,
                        content_width, max_height, max_width, suggestions, arrow_keys, loading,
                        empty, dialog_class, list_class, empty_state_class, header_view, footer_view;
                        input=input_view
                    ),
                    (true, Some(input_view)) => combo_preview!(
                        label, description, placeholder, size, value, on_input, append_to,
                        content_width, max_height, max_width, suggestions, arrow_keys, loading,
                        empty, dialog_class, list_class, empty_state_class, header_view, footer_view;
                        show_list=Signal::derive(|| true) input=input_view
                    ),
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Label" value=label />
            <SettingText label="Description" value=description />
            <SettingText label="Placeholder" value=placeholder />
            <SettingSelect
                label="Size"
                value=size
                options=vec![("", "Default"), ("sm", "sm"), ("md", "md"), ("lg", "lg")]
            />
            <SettingSelect
                label="Append to"
                value=append_to
                options=vec![
                    ("body", "Body"),
                    ("parent", "Parent"),
                    ("root", "Root"),
                    ("HTMLElement", "HTMLElement"),
                ]
            />
            <SettingCheck label="Empty list" value=empty />
            <SettingCheck label="Show list" value=show_list />
            <SettingText label="Max height" hint="CSS length. Empty leaves it unset." value=max_height />
            <SettingText label="Max width" hint="CSS length. Empty leaves it unset." value=max_width />
            <SettingSelect
                label="Content width"
                value=content_width
                options=vec![("", "Default"), ("anchor", "Anchor")]
            />
            <SettingCheck label="Open with arrow keys" value=arrow_keys />
            <SettingCheck label="Suggestion list" value=suggestions />
            <SettingCheck label="Loading" value=loading />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "label", &label.get());
        push_str(&mut props, "description", &description.get());
        push_str(&mut props, "placeholder", &placeholder.get());
        push_str(&mut props, "size", size.get());
        push_str(&mut props, "value", &value.get());
        push_expr(&mut props, "on_input", "move |next| value.set(next)");
        push_str(&mut props, "append_to", append_to.get());
        if show_list.get() {
            push_expr(&mut props, "show_list", "Signal::derive(|| true)");
        }
        push_str(&mut props, "max_height", &max_height.get());
        push_str(&mut props, "max_width", &max_width.get());
        push_str(&mut props, "content_width", content_width.get());
        if arrow_keys.get() {
            push_flag(&mut props, "open_with_arrow_keys", true);
        }
        if !suggestions.get() {
            push_flag(&mut props, "has_suggestion_list", false);
        }
        push_bool(&mut props, "loading", loading.get());
        push_bool(&mut props, "empty_list", empty.get());
        if empty.get() {
            push_str(&mut props, "empty_message", "No matches");
        }
        if !input_text.get().is_empty() {
            push_expr(
                &mut props,
                "input",
                "ViewFn::from(|| view! { <input class=\"d-input\" prop:value=move || value.get() on:input=move |event| value.set(event_target_value(&event)) /> })",
            );
        }
        push_str(&mut props, "dialog_class", &dialog_class.get());
        push_str(&mut props, "list_class", &list_class.get());
        push_str(&mut props, "empty_state_class", &empty_state_class.get());
        push_span_slot(&mut props, "header", &header.get());
        push_span_slot(&mut props, "footer", &footer.get());
        let options = PRODUCTS
            .iter()
            .map(|name| {
                element(
                    "li",
                    &[
                        ("class", rust_string("d-list-item")),
                        ("role", rust_string("option")),
                    ],
                    Some(&rust_string(name)),
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        view_of(&element("DtComboboxWithPopover", &props, Some(&options)))
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingText label="Input" hint="Input slot. Empty uses the default field." value=input_text />
            <SettingText label="Header" hint="Header slot. Empty hides it." value=header />
            <SettingText label="Footer" hint="Footer slot. Empty hides it." value=footer />
        }
    });
    let classes = class_panel(vec![
        ClassField {
            label: "Dialog class",
            hint: "Extra class on the dialog.",
            value: dialog_class,
            off: None,
        },
        ClassField {
            label: "List class",
            hint: "Extra class on the list.",
            value: list_class,
            off: None,
        },
        ClassField {
            label: "Empty state class",
            hint: "Extra class on the empty state.",
            value: empty_state_class,
            off: None,
        },
    ]);
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"The list opens in a popover on focus. The recipe alias is DtRecipeComboboxWithPopover."</p>
            <Play stack=true preview=preview settings=settings slots=slots classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Parent">
                    <p class="docs-note">"Append to parent keeps the popover next to the field instead of portaling it to the document body."</p>
                </Variant>
            </div>
        </section>
    }
}
