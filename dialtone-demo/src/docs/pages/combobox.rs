use super::*;

const LIST_CLASS: &str = "d-p-0 d-mbs-100 d-hmx-500 d-of-y-auto";

const ITEMS: [&str; 12] = [
    "combobox item 1",
    "combobox item 2",
    "combobox item 3",
    "combobox item 4",
    "combobox item 5",
    "combobox item 6",
    "combobox item 7",
    "combobox item 8",
    "combobox item 9",
    "combobox item 10",
    "combobox item 11",
    "combobox item 12",
];

const COMBO_SIZES: &[(&str, &str)] = &[
    ("md", "md"),
    ("xs", "xs"),
    ("sm", "sm"),
    ("lg", "lg"),
    ("xl", "xl"),
    ("100", "100"),
    ("200", "200"),
    ("300", "300"),
    ("400", "400"),
    ("500", "500"),
];

fn select_match(value: RwSignal<String>, index: usize) {
    if let Some(name) = ITEMS.get(index) {
        value.set((*name).to_string());
    }
}

#[component]
fn ComboOptions(names: Vec<String>, value: RwSignal<String>) -> impl IntoView {
    names
        .into_iter()
        .enumerate()
        .map(|(index, name)| {
            let selected = name.clone();
            let full_name = (index + 1).to_string();
            let seed = index.to_string();
            view! {
                <li
                    class="d-list-item"
                    tabindex="-1"
                    role="option"
                    aria-selected="false"
                    on:click=move |_| value.set(selected.clone())
                >
                    <div
                        class="d-item-layout--custom d-list-item__wrapper"
                        style="grid-template-columns: auto 1fr"
                        data-qa="dt-list-item-wrapper"
                    >
                        <section
                            class="d-list-item__left d-item-layout__left"
                            data-qa="dt-item-layout-left-wrapper"
                        >
                            <DtAvatar full_name=full_name seed=seed />
                        </section>
                        <section
                            class="d-list-item__content d-item-layout__content"
                            data-qa="dt-item-layout-content-wrapper"
                        >
                            <div
                                class="d-list-item__title d-item-layout__title"
                                data-qa="dt-item-layout-title-wrapper"
                            >
                                {name}
                            </div>
                        </section>
                    </div>
                </li>
            }
        })
        .collect_view()
}

fn field_slot(
    label: String,
    description: String,
    placeholder: String,
    size: &'static str,
    show_label: bool,
    value: RwSignal<String>,
) -> ViewFn {
    ViewFn::from(move || {
        let label = label.clone();
        let description = description.clone();
        let placeholder = placeholder.clone();
        view! {
            <DtInput
                label=label
                description=description
                placeholder=placeholder
                size=size
                show_label=show_label
                value=value
                on_input=move |next| value.set(next)
            />
        }
    })
}

macro_rules! render_combo {
    (
        $value:expr,
        $label:expr,
        $description:expr,
        $placeholder:expr,
        $size:expr,
        $show_label:expr,
        $list_id:expr,
        $message:expr,
        $class_name:expr,
        $show_list:expr,
        $rendered_outside:expr,
        $loading:expr,
        $empty_list:expr,
        $click_on_select:expr,
        $($extra:tt)*
    ) => {{
        let value = $value;
        let label = $label;
        let description = $description;
        let list_id = $list_id;
        let message = $message;
        let class_name = $class_name;
        let on_escape = Callback::new(move |_| value.set(String::new()));
        let on_select = Callback::new(move |index| select_match(value, index));
        let input = field_slot(
            label.clone(),
            description.clone(),
            $placeholder,
            $size,
            $show_label,
            value,
        );
        let list = list_slot(value);
        view! {
            <DtCombobox
                label=label
                show_label=$show_label
                description=description
                size=$size
                list_id=list_id
                show_list=$show_list
                list_rendered_outside=$rendered_outside
                loading=$loading
                empty_list=$empty_list
                empty_state_message=message
                empty_state_class=class_name
                click_on_select=$click_on_select
                on_escape=on_escape
                on_select=on_select
                input=input
                list=list
                $($extra)*
            />
        }
    }};
}

fn list_slot(value: RwSignal<String>) -> ViewFn {
    ViewFn::from(move || {
        let names = ITEMS
            .iter()
            .map(|name| (*name).to_string())
            .collect::<Vec<_>>();
        view! {
            <ol class=LIST_CLASS>
                <ComboOptions names=names value=value />
            </ol>
        }
    })
}

#[component]
pub(crate) fn ComboboxPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("combobox.rs")));
    let value = RwSignal::new("example".to_string());
    let label = RwSignal::new("Label Text".to_string());
    let description = RwSignal::new(String::new());
    let placeholder = RwSignal::new("Type to show the items".to_string());
    let size = RwSignal::new("md");
    let show_label = RwSignal::new(true);
    let show_setting = RwSignal::new(true);
    let loading = RwSignal::new(false);
    let force_empty = RwSignal::new(false);
    let empty_message = RwSignal::new("No matches found.".to_string());
    let list_id = RwSignal::new(String::new());
    let list_rendered_outside = RwSignal::new(false);
    let on_beginning = RwSignal::new(false);
    let on_end = RwSignal::new(false);
    let beginning_hits = RwSignal::new(0u32);
    let end_hits = RwSignal::new(0u32);
    let empty_class = RwSignal::new(String::new());
    let click_on_select = RwSignal::new(false);
    let custom_empty = RwSignal::new(false);
    let show_list = Signal::derive(move || show_setting.get() && !value.get().is_empty());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let label_now = label.get();
                let description_now = description.get();
                let placeholder_now = placeholder.get();
                let size_now = size.get();
                let show_label_now = show_label.get();
                let message_now = empty_message.get();
                let class_now = empty_class.get();
                let custom_now = custom_empty.get();
                let list_id = list_id.get();
                let rendered_outside = list_rendered_outside.get();
                let begin = on_beginning.get();
                let end = on_end.get();
                let empty_text = message_now.clone();
                let empty_item = ViewFn::from(move || {
                    let message_now = empty_text.clone();
                    view! { <div class="d-py-100 d-fc-tertiary">{message_now}</div> }
                });
                let combo = match (custom_now, begin, end) {
                    (false, false, false) => render_combo!(
                        value,
                        label_now.clone(),
                        description_now.clone(),
                        placeholder_now.clone(),
                        size_now,
                        show_label_now,
                        list_id.clone(),
                        message_now.clone(),
                        class_now.clone(),
                        show_list,
                        rendered_outside,
                        loading,
                        force_empty,
                        click_on_select,
                    )
                    .into_any(),
                    (false, true, false) => render_combo!(
                        value,
                        label_now.clone(),
                        description_now.clone(),
                        placeholder_now.clone(),
                        size_now,
                        show_label_now,
                        list_id.clone(),
                        message_now.clone(),
                        class_now.clone(),
                        show_list,
                        rendered_outside,
                        loading,
                        force_empty,
                        click_on_select,
                        on_beginning_of_list=move || beginning_hits.update(|count| *count += 1)
                    )
                    .into_any(),
                    (false, false, true) => render_combo!(
                        value,
                        label_now.clone(),
                        description_now.clone(),
                        placeholder_now.clone(),
                        size_now,
                        show_label_now,
                        list_id.clone(),
                        message_now.clone(),
                        class_now.clone(),
                        show_list,
                        rendered_outside,
                        loading,
                        force_empty,
                        click_on_select,
                        on_end_of_list=move || end_hits.update(|count| *count += 1)
                    )
                    .into_any(),
                    (false, true, true) => render_combo!(
                        value,
                        label_now.clone(),
                        description_now.clone(),
                        placeholder_now.clone(),
                        size_now,
                        show_label_now,
                        list_id.clone(),
                        message_now.clone(),
                        class_now.clone(),
                        show_list,
                        rendered_outside,
                        loading,
                        force_empty,
                        click_on_select,
                        on_beginning_of_list=move || beginning_hits.update(|count| *count += 1)
                        on_end_of_list=move || end_hits.update(|count| *count += 1)
                    )
                    .into_any(),
                    (true, false, false) => render_combo!(
                        value,
                        label_now.clone(),
                        description_now.clone(),
                        placeholder_now.clone(),
                        size_now,
                        show_label_now,
                        list_id.clone(),
                        message_now.clone(),
                        class_now.clone(),
                        show_list,
                        rendered_outside,
                        loading,
                        force_empty,
                        click_on_select,
                        empty_list_item=empty_item
                    )
                    .into_any(),
                    (true, true, false) => render_combo!(
                        value,
                        label_now.clone(),
                        description_now.clone(),
                        placeholder_now.clone(),
                        size_now,
                        show_label_now,
                        list_id.clone(),
                        message_now.clone(),
                        class_now.clone(),
                        show_list,
                        rendered_outside,
                        loading,
                        force_empty,
                        click_on_select,
                        on_beginning_of_list=move || beginning_hits.update(|count| *count += 1)
                        empty_list_item=empty_item
                    )
                    .into_any(),
                    (true, false, true) => render_combo!(
                        value,
                        label_now.clone(),
                        description_now.clone(),
                        placeholder_now.clone(),
                        size_now,
                        show_label_now,
                        list_id.clone(),
                        message_now.clone(),
                        class_now.clone(),
                        show_list,
                        rendered_outside,
                        loading,
                        force_empty,
                        click_on_select,
                        on_end_of_list=move || end_hits.update(|count| *count += 1)
                        empty_list_item=empty_item
                    )
                    .into_any(),
                    (true, true, true) => render_combo!(
                        value,
                        label_now.clone(),
                        description_now.clone(),
                        placeholder_now.clone(),
                        size_now,
                        show_label_now,
                        list_id.clone(),
                        message_now.clone(),
                        class_now.clone(),
                        show_list,
                        rendered_outside,
                        loading,
                        force_empty,
                        click_on_select,
                        on_beginning_of_list=move || beginning_hits.update(|count| *count += 1)
                        on_end_of_list=move || end_hits.update(|count| *count += 1)
                        empty_list_item=empty_item
                    )
                    .into_any(),
                };
                if begin || end {
                    let beginning_count = beginning_hits.get();
                    let end_count = end_hits.get();
                    view! {
                        <div class="docs-stack">
                            {combo}
                            <span class="docs-readout">
                                {format!("Beginning {beginning_count}, end {end_count}")}
                            </span>
                        </div>
                    }
                    .into_any()
                } else {
                    combo
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Label" value=label />
            <SettingText label="Description" value=description />
            <SettingText label="Placeholder" value=placeholder />
            <SettingSelect label="Size" value=size options=COMBO_SIZES.to_vec() />
            <SettingCheck label="Show label" value=show_label />
            <SettingCheck label="Show list" value=show_setting />
            <SettingCheck label="Loading" value=loading />
            <SettingCheck label="Empty list" value=force_empty />
            <SettingCheck label="Click on select" value=click_on_select />
            <SettingText label="Empty message" value=empty_message />
            <SettingText label="List id" value=list_id />
            <SettingCheck label="List rendered outside" value=list_rendered_outside />
            <SettingCheck label="On beginning of list" value=on_beginning />
            <SettingCheck label="On end of list" value=on_end />
        }
    });
    let slots = ViewFn::from(move || {
        view! { <SettingCheck label="Custom empty item" value=custom_empty /> }
    });
    let code = example_panel(move || {
        let mut lines = Vec::new();
        lines.push(format!(
            "let value = RwSignal::new({});",
            rust_string(&value.get())
        ));
        if show_setting.get() {
            lines.push(
                "let show_list = Signal::derive(move || !value.get().is_empty());".to_string(),
            );
        } else {
            lines.push("let show_list = Signal::stored(false);".to_string());
        }
        lines.push("let input = ViewFn::from(move || view! {".to_string());
        lines.push("    <DtInput".to_string());
        lines.push(format!("        label={}", rust_string(&label.get())));
        if !description.get().is_empty() {
            lines.push(format!(
                "        description={}",
                rust_string(&description.get())
            ));
        }
        if !placeholder.get().is_empty() {
            lines.push(format!(
                "        placeholder={}",
                rust_string(&placeholder.get())
            ));
        }
        if size.get() != "md" {
            lines.push(format!("        size=\"{}\"", size.get()));
        }
        if !show_label.get() {
            lines.push("        show_label=false".to_string());
        }
        lines.push("        value=value".to_string());
        lines.push("        on_input=move |next| value.set(next)".to_string());
        lines.push("    />".to_string());
        lines.push("});".to_string());
        lines.push("let list = ViewFn::from(move || view! {".to_string());
        lines.push(format!("    <ol class=\"{}\">", LIST_CLASS));
        lines.push(
            "        <li class=\"d-list-item\" tabindex=\"-1\" role=\"option\">\"combobox item 1\"</li>"
                .to_string(),
        );
        lines.push("    </ol>".to_string());
        lines.push("});".to_string());
        let mut props = Vec::new();
        push_str(&mut props, "label", &label.get());
        if !show_label.get() {
            push_flag(&mut props, "show_label", false);
        }
        push_str(&mut props, "description", &description.get());
        if size.get() != "md" {
            push_str(&mut props, "size", size.get());
        }
        push_expr(&mut props, "show_list", "show_list");
        push_bool(&mut props, "loading", loading.get());
        push_bool(&mut props, "empty_list", force_empty.get());
        push_str(&mut props, "list_id", &list_id.get());
        push_bool(
            &mut props,
            "list_rendered_outside",
            list_rendered_outside.get(),
        );
        if on_beginning.get() {
            push_expr(
                &mut props,
                "on_beginning_of_list",
                "move || { /* beginning of list */ }",
            );
        }
        if on_end.get() {
            push_expr(
                &mut props,
                "on_end_of_list",
                "move || { /* end of list */ }",
            );
        }
        push_str(&mut props, "empty_state_message", &empty_message.get());
        push_str(&mut props, "empty_state_class", &empty_class.get());
        push_bool(&mut props, "click_on_select", click_on_select.get());
        push_expr(&mut props, "on_escape", "move |_| value.set(String::new())");
        push_expr(
            &mut props,
            "on_select",
            "move |index| { /* value = items[index] */ }",
        );
        push_expr(&mut props, "input", "input");
        push_expr(&mut props, "list", "list");
        if custom_empty.get() {
            push_expr(&mut props, "empty_list_item", "empty_item");
        }
        lines.push(view_of(&element("DtCombobox", &props, None)));
        lines.join("\n")
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"Type to show the list. Escape clears the field. The input and the list are slots. Copy label, show label, size, and description onto DtInput."</p>
            <Play
                stack=true
                preview=preview
                settings=settings
                slots=slots
                classes=class_panel(vec![ClassField {
                    label: "Empty state class",
                    hint: "Extra class on the empty option.",
                    value: empty_class,
                    off: None,
                }])
                code=code
            />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Base style">
                    <BaseCombo />
                </Variant>
                <Variant title="With empty list item">
                    <EmptyCombo />
                </Variant>
                <Variant title="Loading">
                    <LoadingCombo />
                </Variant>
            </div>
        </section>
    }
}

#[component]
fn BaseCombo() -> impl IntoView {
    let value = RwSignal::new("example".to_string());
    let show_list = Signal::derive(move || !value.get().is_empty());
    let input = field_slot(
        "Label Text".to_string(),
        String::new(),
        "Type to show the items".to_string(),
        "md",
        true,
        value,
    );
    let list = list_slot(value);
    view! {
        <DtCombobox
            label="Label Text".to_string()
            show_list=show_list
            on_escape=move |_| value.set(String::new())
            on_select=move |index| select_match(value, index)
            input=input
            list=list
        />
    }
}

#[component]
fn EmptyCombo() -> impl IntoView {
    let value = RwSignal::new("example".to_string());
    let show_list = Signal::derive(move || !value.get().is_empty());
    let input = field_slot(
        "Label Text".to_string(),
        String::new(),
        "Type to show the items".to_string(),
        "md",
        true,
        value,
    );
    let empty_item = ViewFn::from(|| {
        view! { <div class="d-py-100 d-fc-tertiary">"No matches found."</div> }
    });
    view! {
        <DtCombobox
            label="Label Text".to_string()
            show_list=show_list
            empty_list=true
            on_escape=move |_| value.set(String::new())
            input=input
            empty_list_item=empty_item
        />
    }
}

#[component]
fn LoadingCombo() -> impl IntoView {
    let value = RwSignal::new("example".to_string());
    let show_list = Signal::derive(move || !value.get().is_empty());
    let input = field_slot(
        "Label Text".to_string(),
        String::new(),
        "Type to show the items".to_string(),
        "md",
        true,
        value,
    );
    view! {
        <DtCombobox
            label="Label Text".to_string()
            show_list=show_list
            loading=true
            on_escape=move |_| value.set(String::new())
            input=input
        />
    }
}
