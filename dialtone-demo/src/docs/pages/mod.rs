use super::class_suggest::{
    apply_token, byte_to_utf16, current_page_id, suggestions, token_span, utf16_to_byte,
};
use super::ShowCodeText;
pub(super) use dialtone::{
    DtAvatar, DtBadge, DtBanner, DtBox, DtBreadcrumb, DtBreadcrumbItem, DtBreadcrumbs, DtButton,
    DtButtonGroup, DtCard, DtCheckbox, DtCheckboxGroup, DtChip, DtCodeblock, DtCollapsible,
    DtCombobox, DtComboboxMultiSelect, DtComboboxWithPopover, DtDate, DtDatepicker,
    DtDescriptionList, DtDescriptionListItem, DtDropdown, DtDropdownSeparator, DtEmoji,
    DtEmojiPicker, DtEmojiSelection, DtEmojiTextWrapper, DtEmptyState, DtFilter, DtFilterPill,
    DtHovercard, DtIconAlertCircle, DtIconAlertTriangle, DtIconArrowLeftRight, DtIconBell,
    DtIconCheck, DtIconCheckCircle, DtIconChevronDown, DtIconChevronLeft, DtIconChevronRight,
    DtIconChevronsLeft, DtIconChevronsRight, DtIconClose, DtIconConnectionApi, DtIconExternalLink,
    DtIconInfo, DtIconLaptop2, DtIconLock, DtIconMoon, DtIconMoreHorizontal, DtIconMoreVertical,
    DtIconPhone, DtIconPlus, DtIconRefresh, DtIconSave, DtIconSearch, DtIconShare, DtIconSparkle,
    DtIconStar, DtIconSun, DtIconTrash, DtIconUser, DtIconUsers, DtIllustration, DtImageViewer,
    DtInput, DtInputGroup, DtItemLayout, DtKeyboardShortcut, DtLazyShow, DtLink, DtListItem,
    DtListItemGroup, DtLoader, DtModal, DtModeIsland, DtMotionText, DtMotionTextHandle, DtNotice,
    DtPagination, DtPopover, DtPresence, DtProgressCircle, DtProse, DtRadio, DtRadioGroup,
    DtResizable, DtResizableHandle, DtResizablePanel, DtRichTextEditor, DtRootLayout, DtScroller,
    DtScrollerItem, DtSegmentedControl, DtSegmentedControlItem, DtSelectMenu, DtSelectOption,
    DtSkeleton, DtSkeletonListItemOption, DtSkeletonParagraphOption, DtSkeletonShapeKind,
    DtSkeletonShapeOption, DtSkeletonTextOption, DtSplitButton, DtStack, DtTab, DtTabGroup,
    DtTabGroupChange, DtTabPanel, DtText, DtTextList, DtTextListItem, DtToast, DtToggle, DtTooltip,
    DtValidationMessage, DtValidationMessages, ILLUSTRATION_NAMES,
};
use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

pub(super) const TSHIRT: &[(&str, &str)] = &[
    ("xs", "xs"),
    ("sm", "sm"),
    ("md", "md"),
    ("lg", "lg"),
    ("xl", "xl"),
];
pub(super) const ICON_SIZE: &[(&str, &str)] = &[
    ("100", "100"),
    ("200", "200"),
    ("300", "300"),
    ("400", "400"),
    ("500", "500"),
    ("600", "600"),
    ("700", "700"),
    ("800", "800"),
];
pub(super) const PLACEMENTS: &[(&str, &str)] = &[
    ("top", "Top"),
    ("bottom", "Bottom"),
    ("left", "Left"),
    ("right", "Right"),
];
pub(super) const APPEND: &[(&str, &str)] = &[("body", "Body"), ("parent", "Parent")];
pub(super) const PRODUCTS: [&str; 4] = ["Voice", "Messaging", "Meetings", "Analytics"];
pub(super) const ICONS: &[(&str, &str)] = &[
    ("", "None"),
    ("user", "User"),
    ("plus", "Plus"),
    ("check", "Check"),
    ("info", "Info"),
    ("alert", "Alert"),
    ("bell", "Bell"),
    ("close", "Close"),
    ("chevron", "Chevron"),
];
pub(super) const ROOT_CLASS: &str = "Adds a class to the root element of this component.";

#[derive(Clone, Copy)]
pub(super) struct ClassField {
    pub(super) label: &'static str,
    pub(super) hint: &'static str,
    pub(super) value: RwSignal<String>,
    pub(super) off: Option<Signal<bool>>,
}

pub(super) fn icon_label(name: &str) -> &'static str {
    match name {
        "user" => "User",
        "plus" => "Plus",
        "check" => "Check",
        "info" => "Info",
        "alert" => "Alert",
        "bell" => "Bell",
        "close" => "Close",
        "chevron" => "Chevron",
        _ => "",
    }
}

pub(super) fn icon_expr(name: &str) -> Option<&'static str> {
    Some(match name {
        "user" => "ViewFn::from(|| view! { <DtIconUser /> })",
        "plus" => "ViewFn::from(|| view! { <DtIconPlus /> })",
        "check" => "ViewFn::from(|| view! { <DtIconCheck /> })",
        "info" => "ViewFn::from(|| view! { <DtIconInfo /> })",
        "alert" => "ViewFn::from(|| view! { <DtIconAlertCircle /> })",
        "bell" => "ViewFn::from(|| view! { <DtIconBell /> })",
        "close" => "ViewFn::from(|| view! { <DtIconClose /> })",
        "chevron" => "ViewFn::from(|| view! { <DtIconChevronDown /> })",
        _ => return None,
    })
}

pub(super) fn icon_slot(name: &str) -> Option<ViewFn> {
    Some(match name {
        "user" => ViewFn::from(|| view! { <DtIconUser /> }),
        "plus" => ViewFn::from(|| view! { <DtIconPlus /> }),
        "check" => ViewFn::from(|| view! { <DtIconCheck /> }),
        "info" => ViewFn::from(|| view! { <DtIconInfo /> }),
        "alert" => ViewFn::from(|| view! { <DtIconAlertCircle /> }),
        "bell" => ViewFn::from(|| view! { <DtIconBell /> }),
        "close" => ViewFn::from(|| view! { <DtIconClose /> }),
        "chevron" => ViewFn::from(|| view! { <DtIconChevronDown /> }),
        _ => return None,
    })
}

pub(super) fn push_icon(props: &mut Vec<(&'static str, String)>, name: &'static str, icon: &str) {
    if let Some(expr) = icon_expr(icon) {
        push_expr(props, name, expr);
    }
}

pub(super) fn push_span_slot(
    props: &mut Vec<(&'static str, String)>,
    name: &'static str,
    value: &str,
) {
    if value.is_empty() {
        return;
    }
    push_expr(
        props,
        name,
        format!(
            "ViewFn::from(|| view! {{ <span>{}</span> }})",
            rust_string(value)
        ),
    );
}

pub(super) fn join_extra(base: &str, extra: &str) -> String {
    let extra = extra.trim();
    if extra.is_empty() {
        base.to_string()
    } else if base.is_empty() {
        extra.to_string()
    } else {
        format!("{base} {extra}")
    }
}

pub(super) fn action_slot(label: String) -> Option<ViewFn> {
    if label.is_empty() {
        None
    } else {
        Some(ViewFn::from(move || {
            let label = label.clone();
            view! { <DtButton importance="outlined" size="sm">{label}</DtButton> }
        }))
    }
}

pub(super) fn text_slot(value: String) -> Option<ViewFn> {
    if value.is_empty() {
        None
    } else {
        Some(ViewFn::from(move || view! { <span>{value.clone()}</span> }))
    }
}

pub(super) fn layout_bar(text: String) -> ViewFn {
    ViewFn::from(move || {
        let text = text.clone();
        view! { <div class="docs-layout-bar">{text}</div> }
    })
}

pub(super) fn layout_bar_slot(text: String) -> Option<ViewFn> {
    if text.is_empty() {
        None
    } else {
        Some(layout_bar(text))
    }
}

pub(super) fn class_panel(fields: Vec<ClassField>) -> ViewFn {
    ViewFn::from(move || {
        let fields = fields.clone();
        view! {
            {fields.into_iter().map(|field| {
                let ClassField { label, hint, value, off } = field;
                if let Some(off) = off {
                    view! {
                        <SettingClass label=label hint=hint value=value off=off />
                    }.into_any()
                } else {
                    view! { <SettingClass label=label hint=hint value=value /> }.into_any()
                }
            }).collect_view()}
        }
    })
}

macro_rules! void_two {
    ($comp:ident { $($prop:tt)* }, $first:expr, $first_name:ident, $second:expr, $second_name:ident) => {{
        match ($first, $second) {
            (Some(first_slot), Some(second_slot)) => {
                view! { <$comp $first_name=first_slot $second_name=second_slot $($prop)* /> }.into_any()
            }
            (Some(first_slot), None) => {
                view! { <$comp $first_name=first_slot $($prop)* /> }.into_any()
            }
            (None, Some(second_slot)) => {
                view! { <$comp $second_name=second_slot $($prop)* /> }.into_any()
            }
            (None, None) => view! { <$comp $($prop)* /> }.into_any(),
        }
    }};
}

macro_rules! open_one {
    ($comp:ident { $($prop:tt)* } { $($child:tt)* }, $slot:expr, $slot_name:ident) => {{
        match $slot {
            Some(slot_value) => {
                view! { <$comp $slot_name=slot_value $($prop)*> $($child)* </$comp> }.into_any()
            }
            None => view! { <$comp $($prop)*> $($child)* </$comp> }.into_any(),
        }
    }};
}

macro_rules! open_two {
    ($comp:ident { $($prop:tt)* } { $($child:tt)* }, $first:expr, $first_name:ident, $second:expr, $second_name:ident) => {{
        match ($first, $second) {
            (Some(first_slot), Some(second_slot)) => {
                view! {
                    <$comp $first_name=first_slot $second_name=second_slot $($prop)*>
                        $($child)*
                    </$comp>
                }.into_any()
            }
            (Some(first_slot), None) => {
                view! { <$comp $first_name=first_slot $($prop)*> $($child)* </$comp> }.into_any()
            }
            (None, Some(second_slot)) => {
                view! { <$comp $second_name=second_slot $($prop)*> $($child)* </$comp> }.into_any()
            }
            (None, None) => view! { <$comp $($prop)*> $($child)* </$comp> }.into_any(),
        }
    }};
}

#[component]
pub(super) fn SettingSelect(
    label: &'static str,
    value: RwSignal<&'static str>,
    options: Vec<(&'static str, &'static str)>,
) -> impl IntoView {
    let choices = options.clone();
    view! {
        <label class="docs-field">
            <span>{label}</span>
            <select
                prop:value=move || value.get()
                on:change=move |event| {
                    let next = event_target_value(&event);
                    if let Some(index) = choices.iter().position(|pair| pair.0 == next.as_str()) {
                        value.set(choices[index].0);
                    }
                }
            >
                {options.into_iter().map(|(token, name)| {
                    view! { <option value=token>{name}</option> }
                }).collect_view()}
            </select>
        </label>
    }
}

#[component]
pub(super) fn SettingText(
    label: &'static str,
    value: RwSignal<String>,
    #[prop(optional)] hint: &'static str,
    #[prop(optional)] off: Option<Signal<bool>>,
) -> impl IntoView {
    let disabled = move || off.is_some_and(|signal| signal.get());
    view! {
        <label class="docs-field">
            <span>{label}</span>
            {(!hint.is_empty()).then(|| view! { <span class="docs-field-hint">{hint}</span> })}
            <input
                type="text"
                autocomplete="off"
                prop:value=move || value.get()
                prop:disabled=disabled
                on:input=move |event| {
                    if !disabled() {
                        value.set(event_target_value(&event));
                    }
                }
            />
        </label>
    }
}

fn field_id(label: &str) -> String {
    let mut id = String::from("docs-class-");
    for ch in label.chars() {
        if ch.is_ascii_alphanumeric() {
            id.push(ch.to_ascii_lowercase());
        } else {
            id.push('-');
        }
    }
    id
}

fn caret_byte(event: &web_sys::Event) -> usize {
    let Some(input) = event
        .target()
        .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
    else {
        return 0;
    };
    let utf16 = input.selection_start().ok().flatten().unwrap_or(0) as usize;
    utf16_to_byte(&input.value(), utf16)
}

fn restore_caret(node: NodeRef<html::Input>, utf16: u32) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let script = Closure::once_into_js(move || {
        if let Some(input) = node.get_untracked() {
            let _ = input.set_selection_range(utf16, utf16);
        }
    });
    let Some(function) = script.dyn_ref::<js_sys::Function>() else {
        return;
    };
    let _ = window.request_animation_frame(function);
}

fn accept_class(
    node: NodeRef<html::Input>,
    value: RwSignal<String>,
    caret: RwSignal<usize>,
    active: RwSignal<usize>,
    class_name: &str,
) {
    let current = value.get_untracked();
    let at = caret.get_untracked();
    let (next, next_caret) = apply_token(&current, at, class_name);
    value.set(next.clone());
    caret.set(next_caret);
    active.set(0);
    let utf16 = byte_to_utf16(&next, next_caret) as u32;
    if let Some(input) = node.get() {
        input.set_value(&next);
        let _ = input.set_selection_range(utf16, utf16);
        let _ = input.focus();
    }
    restore_caret(node, utf16);
}

#[component]
fn SettingClass(
    label: &'static str,
    value: RwSignal<String>,
    #[prop(optional)] hint: &'static str,
    #[prop(optional)] off: Option<Signal<bool>>,
) -> impl IntoView {
    let disabled = move || off.is_some_and(|signal| signal.get());
    let open = RwSignal::new(false);
    let active = RwSignal::new(0usize);
    let caret = RwSignal::new(0usize);
    let node = NodeRef::<html::Input>::new();
    let list_id = field_id(label);
    let list_id_for_input = list_id.clone();
    let shown = move || {
        if disabled() {
            return Vec::new();
        }
        suggestions(&current_page_id(), label, &value.get(), caret.get(), 10)
    };
    view! {
        <label class="docs-field">
            <span>{label}</span>
            {(!hint.is_empty()).then(|| view! { <span class="docs-field-hint">{hint}</span> })}
            <span class="docs-class">
                <input
                    type="text"
                    role="combobox"
                    autocomplete="off"
                    spellcheck="false"
                    autocapitalize="off"
                    aria-autocomplete="list"
                    aria-controls=list_id_for_input.clone()
                    aria-expanded=move || {
                        if open.get() && !shown().is_empty() { "true" } else { "false" }
                    }
                    aria-activedescendant=move || {
                        if !open.get() {
                            return String::new();
                        }
                        format!("{}-{}", list_id_for_input, active.get())
                    }
                    node_ref=node
                    prop:value=move || value.get()
                    prop:disabled=disabled
                    on:focus=move |event| {
                        if disabled() {
                            return;
                        }
                        caret.set(caret_byte(&event));
                        open.set(true);
                    }
                    on:blur=move |_| open.set(false)
                    on:click=move |event| {
                        if disabled() {
                            return;
                        }
                        caret.set(caret_byte(&event));
                        active.set(0);
                        open.set(true);
                    }
                    on:input=move |event| {
                        if disabled() {
                            return;
                        }
                        value.set(event_target_value(&event));
                        caret.set(caret_byte(&event));
                        active.set(0);
                        open.set(true);
                    }
                    on:keyup=move |event| {
                        if disabled() {
                            return;
                        }
                        match event.key().as_str() {
                            "ArrowLeft" | "ArrowRight" | "Home" | "End" => {
                                caret.set(caret_byte(&event));
                                active.set(0);
                            }
                            _ => {}
                        }
                    }
                    on:keydown=move |event| {
                        if disabled() {
                            return;
                        }
                        let items = suggestions(
                            &current_page_id(),
                            label,
                            &value.get_untracked(),
                            caret.get_untracked(),
                            10,
                        );
                        match event.key().as_str() {
                            "ArrowDown" => {
                                if items.is_empty() {
                                    return;
                                }
                                event.prevent_default();
                                open.set(true);
                                let len = items.len();
                                active.update(|index| {
                                    *index = if *index + 1 >= len { 0 } else { *index + 1 };
                                });
                            }
                            "ArrowUp" => {
                                if items.is_empty() {
                                    return;
                                }
                                event.prevent_default();
                                open.set(true);
                                let len = items.len();
                                active.update(|index| {
                                    *index = if *index == 0 { len - 1 } else { *index - 1 };
                                });
                            }
                            "Enter" => {
                                if !open.get_untracked() || items.is_empty() {
                                    return;
                                }
                                event.prevent_default();
                                let index = active.get_untracked().min(items.len() - 1);
                                accept_class(node, value, caret, active, items[index]);
                            }
                            "Escape" => {
                                if open.get_untracked() {
                                    event.prevent_default();
                                    open.set(false);
                                }
                            }
                            _ => {}
                        }
                    }
                />
                {move || {
                    let is_open = open.get();
                    let items = shown();
                    let (start, end) = token_span(&value.get(), caret.get());
                    let query_empty = start == end;
                    if !is_open {
                        return ().into_any();
                    }
                    if items.is_empty() {
                        let text = if query_empty {
                            "Type to search Dialtone classes"
                        } else {
                            "No matching classes"
                        };
                        return view! { <p class="docs-class-empty">{text}</p> }.into_any();
                    }
                    let current = active.get().min(items.len() - 1);
                    let list_id = list_id.clone();
                    view! {
                        <ul id=list_id.clone() class="docs-class-list" role="listbox" aria-label="Classes">
                            {items.into_iter().enumerate().map(|(index, name)| {
                                let selected = index == current;
                                let option_class = if selected {
                                    "docs-class-option docs-class-option--active"
                                } else {
                                    "docs-class-option"
                                };
                                let option_id = format!("{list_id}-{index}");
                                view! {
                                    <li
                                        id=option_id
                                        role="option"
                                        class=option_class
                                        aria-selected=if selected { "true" } else { "false" }
                                        on:mousedown=move |event| {
                                            event.prevent_default();
                                            accept_class(node, value, caret, active, name);
                                        }
                                        on:mouseenter=move |_| active.set(index)
                                    >
                                        {name}
                                    </li>
                                }
                            }).collect_view()}
                        </ul>
                    }.into_any()
                }}
            </span>
        </label>
    }
}

#[component]
pub(super) fn SettingArea(
    label: &'static str,
    value: RwSignal<String>,
    #[prop(optional)] hint: &'static str,
) -> impl IntoView {
    view! {
        <label class="docs-field">
            <span>{label}</span>
            {(!hint.is_empty()).then(|| view! { <span class="docs-field-hint">{hint}</span> })}
            <textarea
                rows="5"
                prop:value=move || value.get()
                on:input=move |event| value.set(event_target_value(&event))
            />
        </label>
    }
}

#[component]
pub(super) fn SettingCheck(label: &'static str, value: RwSignal<bool>) -> impl IntoView {
    view! {
        <div class="docs-field docs-field--row">
            <span>{label}</span>
            <DtToggle
                checked=value
                size="sm"
                aria_label=label
                on_change=move |next| value.set(next)
            />
        </div>
    }
}

pub(super) fn rust_string(value: &str) -> String {
    let mut out = String::from("\"");
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

pub(super) fn push_str(props: &mut Vec<(&'static str, String)>, name: &'static str, value: &str) {
    if !value.is_empty() {
        props.push((name, rust_string(value)));
    }
}

pub(super) fn push_bool(props: &mut Vec<(&'static str, String)>, name: &'static str, value: bool) {
    if value {
        props.push((name, "true".to_string()));
    }
}

pub(super) fn push_flag(props: &mut Vec<(&'static str, String)>, name: &'static str, value: bool) {
    props.push((
        name,
        if value {
            "true".to_string()
        } else {
            "false".to_string()
        },
    ));
}

pub(super) fn push_expr(
    props: &mut Vec<(&'static str, String)>,
    name: &'static str,
    value: impl Into<String>,
) {
    let value = value.into();
    if !value.is_empty() {
        props.push((name, value));
    }
}

pub(super) fn element(name: &str, props: &[(&str, String)], children: Option<&str>) -> String {
    let mut out = String::from("<");
    out.push_str(name);
    if props.is_empty() {
        match children {
            Some(body) if !body.is_empty() => {
                out.push_str(">\n");
                for line in body.lines() {
                    out.push_str("    ");
                    out.push_str(line);
                    out.push('\n');
                }
                out.push_str("</");
                out.push_str(name);
                out.push('>');
            }
            _ => out.push_str(" />"),
        }
        return out;
    }
    out.push('\n');
    for (key, value) in props {
        out.push_str("    ");
        out.push_str(key);
        out.push('=');
        out.push_str(value);
        out.push('\n');
    }
    if let Some(body) = children.filter(|body| !body.is_empty()) {
        out.push_str(">\n");
        for line in body.lines() {
            out.push_str("    ");
            out.push_str(line);
            out.push('\n');
        }
        out.push_str("</");
        out.push_str(name);
        out.push('>');
    } else {
        out.push_str("/>");
    }
    out
}

pub(super) fn view_of(body: &str) -> String {
    let mut out = String::from("view! {\n");
    for line in body.lines() {
        out.push_str("    ");
        out.push_str(line);
        out.push('\n');
    }
    out.push_str("}\n");
    out
}

pub(super) fn example_panel(source: impl Fn() -> String + Copy + Send + Sync + 'static) -> ViewFn {
    ViewFn::from(move || {
        view! {
            {move || {
                let text = source();
                let html = super::highlight::code(&text);
                view! { <pre><code class="d-codeblock" inner_html=html></code></pre> }
            }}
        }
    })
}

#[component]
pub(super) fn Play(
    #[prop(optional)] stack: bool,
    preview: ViewFn,
    #[prop(optional)] settings: Option<ViewFn>,
    #[prop(optional)] slots: Option<ViewFn>,
    #[prop(optional)] classes: Option<ViewFn>,
    code: ViewFn,
) -> impl IntoView {
    let stage = if stack {
        "docs-stage docs-stage--stack"
    } else {
        "docs-stage"
    };
    let has_props = settings.is_some();
    let has_slots = slots.is_some();
    let has_classes = classes.is_some();
    let tabbed = has_slots || has_classes;
    let initial = if has_props {
        "props"
    } else if has_slots {
        "slots"
    } else {
        "class"
    };
    let tab = RwSignal::new(initial);
    view! {
        <div class="docs-play">
            <div class="docs-play-start">
            <div class=stage>{preview.run()}</div>
            <div class="docs-settings">
                {if tabbed {
                    view! {
                        <div class="docs-tabs" role="tablist">
                            {has_props.then(|| view! {
                                <button
                                    type="button"
                                    role="tab"
                                    class=move || if tab.get() == "props" { "docs-tab docs-tab--selected" } else { "docs-tab" }
                                    on:click=move |_| tab.set("props")
                                >"Props"</button>
                            })}
                            {has_slots.then(|| view! {
                                <button
                                    type="button"
                                    role="tab"
                                    class=move || if tab.get() == "slots" { "docs-tab docs-tab--selected" } else { "docs-tab" }
                                    on:click=move |_| tab.set("slots")
                                >"Slots"</button>
                            })}
                            {has_classes.then(|| view! {
                                <button
                                    type="button"
                                    role="tab"
                                    class=move || if tab.get() == "class" { "docs-tab docs-tab--selected" } else { "docs-tab" }
                                    on:click=move |_| tab.set("class")
                                >"Class"</button>
                            })}
                        </div>
                    }.into_any()
                } else {
                    view! { <p class="docs-settings-title">"Settings"</p> }.into_any()
                }}
                {settings.map(|settings| view! {
                    <div class=move || if tabbed && tab.get() != "props" { "docs-tab-panel docs-tab-panel--hidden" } else { "docs-tab-panel" }>
                        {settings.run()}
                    </div>
                })}
                {slots.map(|slots| view! {
                    <div class=move || if tab.get() != "slots" { "docs-tab-panel docs-tab-panel--hidden" } else { "docs-tab-panel" }>
                        {slots.run()}
                    </div>
                })}
                {classes.map(|classes| view! {
                    <div class=move || if tab.get() != "class" { "docs-tab-panel docs-tab-panel--hidden" } else { "docs-tab-panel" }>
                        {classes.run()}
                    </div>
                })}
            </div>
            </div>
            <div class="docs-example docs-code-shell" id="example">
                <p class="docs-settings-title">"Leptos"</p>
                <div class="docs-code-body">{code.run()}</div>
                <div class="docs-copy-slot" inner_html=super::copy::copy_button_markup()></div>
            </div>
        </div>
    }
}

/// Page source passed from each component page so a variant can show its Leptos.
#[derive(Copy, Clone)]
struct VariantSource(&'static str);

#[component]
pub(super) fn Variant(
    title: &'static str,
    #[prop(optional)] row: bool,
    children: ChildrenFn,
) -> impl IntoView {
    let class = if row {
        "docs-variant-stage docs-variant-stage--row"
    } else {
        "docs-variant-stage"
    };
    let snippet = use_context::<VariantSource>().and_then(|source| variant_code(source.0, title));
    if snippet.is_none() {
        leptos::logging::warn!("No Leptos source for variant \"{title}\"");
    }
    view! {
        <section class="docs-variant">
            <h3>{title}</h3>
            <div class=class>{children()}</div>
            {snippet.map(|snippet| view! { <ShowCodeText text=snippet /> })}
        </section>
    }
}

/// Body of `<Variant title="{title}" ...>...</Variant>` in `source`, dedented.
fn variant_code(source: &str, title: &str) -> Option<String> {
    let needle = format!("<Variant title=\"{title}");
    let mut from = 0;
    while let Some(at) = source[from..].find(&needle) {
        let tag_start = from + at;
        let quote_at = tag_start + needle.len();
        let closer = source.as_bytes().get(quote_at).copied()?;
        if closer != b'"' {
            from = quote_at;
            continue;
        }
        let open_end = source[tag_start..].find('>')? + tag_start;
        let body = &source[open_end + 1..];
        let close = variant_close(body)?;
        return Some(dedent(&body[..close]));
    }
    None
}

/// Byte index of the `</Variant>` that closes the variant whose body is `body`.
fn variant_close(body: &str) -> Option<usize> {
    let mut depth = 1;
    let mut index = 0;
    while index < body.len() {
        let rest = &body[index..];
        if rest.starts_with("<Variant>") || rest.starts_with("<Variant ") {
            depth += 1;
            index += "<Variant".len();
            continue;
        }
        if rest.starts_with("</Variant>") {
            depth -= 1;
            if depth == 0 {
                return Some(index);
            }
            index += "</Variant>".len();
            continue;
        }
        index += rest.chars().next().map(char::len_utf8).unwrap_or(1);
    }
    None
}

fn dedent(body: &str) -> String {
    let trimmed = body.trim_matches('\n');
    let indent = trimmed
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.chars()
                .take_while(|character| *character == ' ')
                .count()
        })
        .min()
        .unwrap_or(0);
    let mut out = String::new();
    for (index, line) in trimmed.lines().enumerate() {
        if index > 0 {
            out.push('\n');
        }
        if line.trim().is_empty() {
            continue;
        }
        let skip = indent.min(
            line.chars()
                .take_while(|character| *character == ' ')
                .count(),
        );
        out.push_str(&line[skip..]);
    }
    out
}

mod avatar;
mod badge;
mod banner;
#[path = "box.rs"]
mod r#box;
mod breadcrumbs;
mod button;
mod button_group;
mod card;
mod checkbox;
mod checkbox_group;
mod chip;
mod codeblock;
mod collapsible;
mod combobox;
mod combobox_popover;
mod datepicker;
mod description_list;
mod dropdown;
mod emoji;
mod emoji_picker;
mod emoji_text_wrapper;
mod empty_state;
mod filter_pill;
mod hovercard;
mod icon;
mod illustration;
mod image_viewer;
mod input;
mod input_group;
mod item_layout;
mod keyboard_shortcut;
mod lazy_show;
mod link;
mod list_item;
mod list_item_group;
mod loader;
mod modal;
mod mode_island;
mod motion_text;
mod multi_select;
mod notice;
mod pagination;
mod popover;
mod presence;
mod progress_circle;
mod prose;
mod radio;
mod radio_group;
mod resizable;
mod rich_text_editor;
mod root_layout;
mod scroller;
mod segmented_control;
mod select_menu;
mod skeleton;
mod split_button;
mod stack;
mod tab;
mod tab_group;
mod text;
mod text_list;
mod toast;
mod toggle;
mod tooltip;
mod validation_messages;

pub(crate) use avatar::AvatarPage;
pub(crate) use badge::BadgePage;
pub(crate) use banner::BannerPage;
pub(crate) use breadcrumbs::BreadcrumbsPage;
pub(crate) use button::ButtonPage;
pub(crate) use button_group::ButtonGroupPage;
pub(crate) use card::CardPage;
pub(crate) use checkbox::CheckboxPage;
pub(crate) use checkbox_group::CheckboxGroupPage;
pub(crate) use chip::ChipPage;
pub(crate) use codeblock::CodeblockPage;
pub(crate) use collapsible::CollapsiblePage;
pub(crate) use combobox::ComboboxPage;
pub(crate) use combobox_popover::ComboboxPopoverPage;
pub(crate) use datepicker::DatepickerPage;
pub(crate) use description_list::DescriptionListPage;
pub(crate) use dropdown::DropdownPage;
pub(crate) use emoji::EmojiPage;
pub(crate) use emoji_picker::EmojiPickerPage;
pub(crate) use emoji_text_wrapper::EmojiTextWrapperPage;
pub(crate) use empty_state::EmptyStatePage;
pub(crate) use filter_pill::FilterPillPage;
pub(crate) use hovercard::HovercardPage;
pub(crate) use icon::IconPage;
pub(crate) use illustration::IllustrationPage;
pub(crate) use image_viewer::ImageViewerPage;
pub(crate) use input::InputPage;
pub(crate) use input_group::InputGroupPage;
pub(crate) use item_layout::ItemLayoutPage;
pub(crate) use keyboard_shortcut::KeyboardShortcutPage;
pub(crate) use lazy_show::LazyShowPage;
pub(crate) use link::LinkPage;
pub(crate) use list_item::ListItemPage;
pub(crate) use list_item_group::ListItemGroupPage;
pub(crate) use loader::LoaderPage;
pub(crate) use modal::ModalPage;
pub(crate) use mode_island::ModeIslandPage;
pub(crate) use motion_text::MotionTextPage;
pub(crate) use multi_select::MultiSelectPage;
pub(crate) use notice::NoticePage;
pub(crate) use pagination::PaginationPage;
pub(crate) use popover::PopoverPage;
pub(crate) use presence::PresencePage;
pub(crate) use progress_circle::ProgressCirclePage;
pub(crate) use prose::ProsePage;
pub(crate) use r#box::BoxPage;
pub(crate) use radio::RadioPage;
pub(crate) use radio_group::RadioGroupPage;
pub(crate) use resizable::ResizablePage;
pub(crate) use rich_text_editor::RichTextEditorPage;
pub(crate) use root_layout::RootLayoutPage;
pub(crate) use scroller::ScrollerPage;
pub(crate) use segmented_control::SegmentedControlPage;
pub(crate) use select_menu::SelectMenuPage;
pub(crate) use skeleton::SkeletonPage;
pub(crate) use split_button::SplitButtonPage;
pub(crate) use stack::StackPage;
pub(crate) use tab::TabPage;
pub(crate) use tab_group::TabGroupPage;
pub(crate) use text::TextPage;
pub(crate) use text_list::TextListPage;
pub(crate) use toast::ToastPage;
pub(crate) use toggle::TogglePage;
pub(crate) use tooltip::TooltipPage;
pub(crate) use validation_messages::ValidationMessagesPage;

#[cfg(test)]
mod variant_source_tests {
    use super::variant_code;

    #[test]
    fn keeps_comparisons_and_stops_at_the_matching_close() {
        let source = r#"
            <Variant title="Compare" row=true>
                {move || items.iter().filter(|item| item.count > 1).count()}
                <DtButton>"Go"</DtButton>
            </Variant>
            <Variant title="Next">
                <DtButton>"Next"</DtButton>
            </Variant>
        "#;
        let code = variant_code(source, "Compare").unwrap();
        assert!(code.contains("item.count > 1"));
        assert!(code.contains("<DtButton>\"Go\"</DtButton>"));
        assert!(!code.contains("Next"));
        assert!(!code.contains("</Variant>"));
    }

    #[test]
    fn multibyte_text_stays_in_the_snippet() {
        let source = "<Variant title=\"Mark\">\n    <p>\"A · B\"</p>\n</Variant>\n";
        let code = variant_code(source, "Mark").unwrap();
        assert_eq!(code, "<p>\"A · B\"</p>");
    }

    #[test]
    fn title_prefix_does_not_match_a_longer_title() {
        let source = r#"
            <Variant title="Sizes" row=true>
                <DtButton size="sm">"sm"</DtButton>
            </Variant>
        "#;
        assert!(variant_code(source, "Size").is_none());
        let code = variant_code(source, "Sizes").unwrap();
        assert!(code.contains("size=\"sm\""));
    }

    #[test]
    fn every_variant_on_a_page_has_leptos_source() {
        let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/src/docs/pages");
        let mut pages = 0;
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            if name == "mod.rs" || path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
                continue;
            }
            let source = std::fs::read_to_string(&path).unwrap();
            if !source.contains("<Variant ") {
                continue;
            }
            pages += 1;
            let marker = format!("VariantSource(include_str!(\"{name}\"))");
            assert!(
                source.contains(&marker),
                "{name} renders variants without its own source"
            );
            let mut seen = std::collections::BTreeSet::new();
            for title in variant_titles(&source) {
                assert!(
                    seen.insert(title.clone()),
                    "duplicate variant {title} in {name}"
                );
                let code = variant_code(&source, &title)
                    .unwrap_or_else(|| panic!("{name} variant {title} has no source"));
                assert!(!code.trim().is_empty(), "{name} {title}");
                assert!(!code.contains("</Variant>"), "{name} {title}");
            }
            assert!(!seen.is_empty(), "{name}");
        }
        assert!(pages > 40, "expected variant pages, found {pages}");
    }

    fn variant_titles(source: &str) -> Vec<String> {
        let mut titles = Vec::new();
        let mut rest = source;
        let needle = "<Variant title=\"";
        while let Some(at) = rest.find(needle) {
            let after = &rest[at + needle.len()..];
            let end = after.find('"').unwrap();
            titles.push(after[..end].to_string());
            rest = &after[end + 1..];
        }
        titles
    }
}
