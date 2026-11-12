use super::*;

fn chosen_items(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(str::to_string)
        .collect()
}

fn or_static<'a>(value: &'a str, default: &'a str) -> &'a str {
    if value.is_empty() {
        default
    } else {
        value
    }
}

macro_rules! multi_preview {
    (
        $label:expr,
        $description:expr,
        $placeholder:expr,
        $size:expr,
        $selected:expr,
        $max:expr,
        $messages:expr,
        $loading:expr,
        $loading_message:expr,
        $list_max_height:expr,
        $list_max_width:expr,
        $append_to:expr,
        $dialog_class:expr,
        $collapse:expr,
        $chip_max_width:expr,
        $input_class:expr,
        $input_wrapper_class:expr,
        $disabled:expr,
        $reserved:expr,
        $suggestions:expr,
        $on_select:expr,
        $on_remove:expr,
        $header:expr,
        $footer:expr;
        $($extra:tt)*
    ) => {
        open_two!(DtComboboxMultiSelect {
            label=$label
            description=$description
            placeholder=$placeholder
            size=$size
            selected_items=$selected
            max_selected=$max
            max_selected_message=$messages
            loading=$loading
            loading_message=$loading_message
            list_max_height=$list_max_height
            list_max_width=$list_max_width
            append_to=$append_to
            dialog_class=$dialog_class
            collapse_on_focus_out=$collapse
            chip_max_width=$chip_max_width
            input_class=$input_class
            input_wrapper_class=$input_wrapper_class
            disabled=$disabled
            reserved_right_space=$reserved
            has_suggestion_list=$suggestions
            on_select=$on_select
            on_remove=$on_remove
            $($extra)*
        } {
            {PRODUCTS.iter().copied().map(|name| {
                view! { <li class="d-list-item" role="option">{name}</li> }
            }).collect_view()}
        }, $header, header, $footer, footer)
    };
}

#[component]
pub(crate) fn MultiSelectPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("multi_select.rs")));
    let selected = RwSignal::new(vec!["Voice".to_string()]);
    let selected_text = RwSignal::new("Voice".to_string());
    let label = RwSignal::new("Products".to_string());
    let description = RwSignal::new(String::new());
    let size = RwSignal::new("300");
    let placeholder = RwSignal::new("Select one or start typing");
    let max_selected = RwSignal::new("0");
    let max_message = RwSignal::new(String::new());
    let loading = RwSignal::new(false);
    let loading_message = RwSignal::new("");
    let show_list = RwSignal::new(false);
    let list_max_height = RwSignal::new("");
    let list_max_width = RwSignal::new(String::new());
    let append_to = RwSignal::new("");
    let collapse = RwSignal::new(false);
    let chip_max_width = RwSignal::new(String::new());
    let suggestions = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let reserved_text = RwSignal::new(String::new());
    let dialog_class = RwSignal::new(String::new());
    let input_class = RwSignal::new(String::new());
    let input_wrapper_class = RwSignal::new(String::new());
    let header = RwSignal::new(String::new());
    let footer = RwSignal::new(String::new());
    Effect::new(move |_| {
        let items = chosen_items(&selected_text.get());
        if selected.get_untracked() != items {
            selected.set(items);
        }
    });
    let on_select = Callback::new(move |index: usize| {
        if index >= PRODUCTS.len() {
            return;
        }
        let name: &str = PRODUCTS[index];
        selected.update(|items| {
            if !items.iter().any(|item| item == name) {
                items.push(name.to_string());
            }
        });
        selected_text.set(selected.get_untracked().join(", "));
    });
    let on_remove = Callback::new(move |item: String| {
        selected.update(|items| items.retain(|current| current != &item));
        selected_text.set(selected.get_untracked().join(", "));
    });
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let max = match max_selected.get() {
                    "1" => 1,
                    "2" => 2,
                    "3" => 3,
                    _ => 0,
                };
                let label = label.get();
                let description = description.get();
                let placeholder = placeholder.get();
                let size = size.get();
                let message = max_message.get();
                let messages = if message.is_empty() {
                    Vec::new()
                } else {
                    vec![DtValidationMessage::error(message)]
                };
                let loading = loading.get();
                let loading_message = or_static(loading_message.get(), "loading...");
                let list_max_height = or_static(list_max_height.get(), "300px");
                let list_max_width = list_max_width.get();
                let append_to = or_static(append_to.get(), "body");
                let dialog_class = dialog_class.get();
                let collapse = collapse.get();
                let chip_max_width = chip_max_width.get();
                let input_class = input_class.get();
                let input_wrapper_class = input_wrapper_class.get();
                let disabled = disabled.get();
                let reserved = reserved_text.get().parse().unwrap_or(64.0);
                let suggestions = suggestions.get();
                let header_view = text_slot(header.get());
                let footer_view = text_slot(footer.get());
                let show = show_list.get();
                if show {
                    multi_preview!(
                        label,
                        description,
                        placeholder,
                        size,
                        selected,
                        max,
                        messages,
                        loading,
                        loading_message,
                        list_max_height,
                        list_max_width,
                        append_to,
                        dialog_class,
                        collapse,
                        chip_max_width,
                        input_class,
                        input_wrapper_class,
                        disabled,
                        reserved,
                        suggestions,
                        on_select,
                        on_remove,
                        header_view,
                        footer_view;
                        show_list=Signal::derive(|| true)
                    )
                } else {
                    multi_preview!(
                        label,
                        description,
                        placeholder,
                        size,
                        selected,
                        max,
                        messages,
                        loading,
                        loading_message,
                        list_max_height,
                        list_max_width,
                        append_to,
                        dialog_class,
                        collapse,
                        chip_max_width,
                        input_class,
                        input_wrapper_class,
                        disabled,
                        reserved,
                        suggestions,
                        on_select,
                        on_remove,
                        header_view,
                        footer_view;
                    )
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Label" value=label />
            <SettingText label="Description" value=description />
            <SettingSelect
                label="Placeholder"
                value=placeholder
                options=vec![
                    ("Select one or start typing", "Select one or start typing"),
                    ("Choose products", "Choose products"),
                    ("Add a product", "Add a product"),
                ]
            />
            <SettingSelect
                label="Size"
                value=size
                options=vec![
                    ("100", "100"),
                    ("200", "200"),
                    ("300", "300"),
                    ("xs", "xs"),
                    ("sm", "sm"),
                    ("md", "md"),
                ]
            />
            <SettingSelect
                label="Max selected"
                value=max_selected
                options=vec![("0", "Unlimited"), ("1", "1"), ("2", "2"), ("3", "3")]
            />
            <SettingText label="Selected items" hint="Comma-separated chips." value=selected_text />
            <SettingText
                label="Max selected message"
                hint="Shown when the max is reached. Empty hides it."
                value=max_message
            />
            <SettingCheck label="Loading" value=loading />
            <SettingSelect
                label="Loading message"
                value=loading_message
                options=vec![
                    ("", "Default"),
                    ("loading...", "loading..."),
                    ("Loading results", "Loading results"),
                    ("Please wait", "Please wait"),
                ]
            />
            <SettingCheck label="Show list" value=show_list />
            <SettingCheck label="Suggestion list" value=suggestions />
            <SettingSelect
                label="Append to"
                value=append_to
                options=vec![
                    ("", "Default"),
                    ("body", "Body"),
                    ("parent", "Parent"),
                    ("root", "Root"),
                    ("HTMLElement", "HTMLElement"),
                ]
            />
            <SettingSelect
                label="List max height"
                value=list_max_height
                options=vec![
                    ("", "Default"),
                    ("160px", "160px"),
                    ("240px", "240px"),
                    ("300px", "300px"),
                    ("480px", "480px"),
                ]
            />
            <SettingText label="List max width" hint="CSS length. Empty leaves it unset." value=list_max_width />
            <SettingText label="Chip max width" hint="CSS length. Empty leaves it unset." value=chip_max_width />
            <SettingText label="Reserved right space" hint="Empty uses 64." value=reserved_text />
            <SettingCheck label="Collapse on focus out" value=collapse />
            <SettingCheck label="Disabled" value=disabled />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "label", &label.get());
        push_str(&mut props, "description", &description.get());
        push_str(&mut props, "placeholder", placeholder.get());
        push_str(&mut props, "size", size.get());
        push_expr(&mut props, "selected_items", "selected");
        let max = max_selected.get().parse::<i32>().unwrap_or(0);
        if max > 0 {
            push_expr(&mut props, "max_selected", max.to_string());
        }
        if !max_message.get().is_empty() {
            push_expr(
                &mut props,
                "max_selected_message",
                format!(
                    "vec![DtValidationMessage::error({})]",
                    rust_string(&max_message.get())
                ),
            );
        }
        push_bool(&mut props, "loading", loading.get());
        push_str(&mut props, "loading_message", loading_message.get());
        if show_list.get() {
            push_expr(&mut props, "show_list", "Signal::derive(|| true)");
        }
        push_str(&mut props, "list_max_height", list_max_height.get());
        push_str(&mut props, "list_max_width", &list_max_width.get());
        if !suggestions.get() {
            push_flag(&mut props, "has_suggestion_list", false);
        }
        push_str(&mut props, "append_to", append_to.get());
        push_bool(&mut props, "collapse_on_focus_out", collapse.get());
        push_str(&mut props, "chip_max_width", &chip_max_width.get());
        if let Ok(space) = reserved_text.get().parse::<f64>() {
            push_expr(&mut props, "reserved_right_space", space.to_string());
        }
        push_bool(&mut props, "disabled", disabled.get());
        push_expr(
            &mut props,
            "on_select",
            "move |index| { /* push PRODUCTS[index] */ }",
        );
        push_expr(
            &mut props,
            "on_remove",
            "move |item| selected.retain(|current| current != &item)",
        );
        push_str(&mut props, "dialog_class", &dialog_class.get());
        push_str(&mut props, "input_class", &input_class.get());
        push_str(
            &mut props,
            "input_wrapper_class",
            &input_wrapper_class.get(),
        );
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
        let chosen = selected.get().join(", ");
        format!(
            "// selected: {}\n{}",
            if chosen.is_empty() {
                "none".into()
            } else {
                chosen
            },
            view_of(&element("DtComboboxMultiSelect", &props, Some(&options)))
        )
    });
    let slots = ViewFn::from(move || {
        view! {
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
            label: "Input class",
            hint: "Extra class on the input.",
            value: input_class,
            off: None,
        },
        ClassField {
            label: "Input wrapper class",
            hint: "Extra class on the input wrapper.",
            value: input_wrapper_class,
            off: None,
        },
    ]);
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"Choose an option to add a chip. Close removes it. The recipe alias is DtRecipeComboboxMultiSelect."</p>
            <Play stack=true preview=preview settings=settings slots=slots classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Sizes" row=true>
                    <DtChip size="xs" show_close=false>"xs chip"</DtChip>
                    <DtChip size="sm" show_close=false>"sm chip"</DtChip>
                </Variant>
            </div>
        </section>
    }
}
