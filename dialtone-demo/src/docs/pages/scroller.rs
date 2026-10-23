use std::sync::Mutex;

use super::*;

const DIRECTIONS: &[(&str, &str)] = &[
    ("", "Default"),
    ("vertical", "Vertical"),
    ("horizontal", "Horizontal"),
];

const TAGS: &[(&str, &str)] = &[
    ("", "Default"),
    ("div", "div"),
    ("ul", "ul"),
    ("ol", "ol"),
    ("li", "li"),
    ("span", "span"),
];

fn intern(value: &str) -> &'static str {
    static CACHE: Mutex<Vec<(String, &'static str)>> = Mutex::new(Vec::new());
    let mut cache = CACHE.lock().expect("scroller intern");
    if let Some((_, leaked)) = cache.iter().find(|(key, _)| key == value) {
        return leaked;
    }
    let leaked: &'static str = Box::leak(value.to_string().into_boxed_str());
    cache.push((value.to_string(), leaked));
    leaked
}

fn row_count(value: &str) -> usize {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return 40;
    }
    trimmed.parse::<usize>().unwrap_or(40).min(500)
}

fn parse_f64(value: &str) -> Option<f64> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        trimmed.parse().ok()
    }
}

fn f64_lit(value: f64) -> String {
    if value.fract() == 0.0 && value.is_finite() {
        format!("{}.0", value as i64)
    } else {
        value.to_string()
    }
}

fn css_len(value: &str, fallback: &str) -> String {
    let trimmed = value.trim();
    let raw = if trimmed.is_empty() {
        fallback
    } else {
        trimmed
    };
    if raw.parse::<i32>().is_ok() {
        format!("{raw}px")
    } else {
        raw.to_string()
    }
}

fn tag_or_div(value: &str) -> &'static str {
    match value {
        "ul" => "ul",
        "ol" => "ol",
        "li" => "li",
        "span" => "span",
        "div" => "div",
        _ => "div",
    }
}

fn scroller_rows(count: usize, size: f64) -> Vec<DtScrollerItem> {
    let px = if size.is_finite() { size } else { 32.0 };
    (1..=count)
        .map(|index| {
            DtScrollerItem::new(
                index.to_string(),
                ViewFn::from(move || {
                    view! {
                        <div
                            class="px-3 text-sm"
                            style=format!("height: {px}px; display: flex; align-items: center;")
                        >
                            {format!("Incident {index}")}
                        </div>
                    }
                }),
            )
        })
        .collect()
}

macro_rules! scroller_view {
    (
        $items:expr,
        $size:expr,
        $dynamic:expr,
        $buffer:expr,
        $direction:expr,
        $key_field:expr,
        $list_tag:expr,
        $item_tag:expr,
        $width_css:expr,
        $height_css:expr,
        $before:expr,
        $after:expr,
        $empty:expr,
        $($extra:tt)*
    ) => {
        match ($before, $after, $empty) {
            (Some(before), Some(after), Some(empty)) => {
                view! {
                    <DtScroller
                        items=$items
                        item_size=$size
                        dynamic=$dynamic
                        buffer=$buffer
                        direction=$direction
                        key_field=$key_field
                        list_tag=$list_tag
                        item_tag=$item_tag
                        scroller_width=$width_css
                        scroller_height=$height_css
                        before=before
                        after=after
                        empty=empty
                        $($extra)*
                    />
                }
                .into_any()
            }
            (Some(before), Some(after), None) => {
                view! {
                    <DtScroller
                        items=$items
                        item_size=$size
                        dynamic=$dynamic
                        buffer=$buffer
                        direction=$direction
                        key_field=$key_field
                        list_tag=$list_tag
                        item_tag=$item_tag
                        scroller_width=$width_css
                        scroller_height=$height_css
                        before=before
                        after=after
                        $($extra)*
                    />
                }
                .into_any()
            }
            (Some(before), None, Some(empty)) => {
                view! {
                    <DtScroller
                        items=$items
                        item_size=$size
                        dynamic=$dynamic
                        buffer=$buffer
                        direction=$direction
                        key_field=$key_field
                        list_tag=$list_tag
                        item_tag=$item_tag
                        scroller_width=$width_css
                        scroller_height=$height_css
                        before=before
                        empty=empty
                        $($extra)*
                    />
                }
                .into_any()
            }
            (None, Some(after), Some(empty)) => {
                view! {
                    <DtScroller
                        items=$items
                        item_size=$size
                        dynamic=$dynamic
                        buffer=$buffer
                        direction=$direction
                        key_field=$key_field
                        list_tag=$list_tag
                        item_tag=$item_tag
                        scroller_width=$width_css
                        scroller_height=$height_css
                        after=after
                        empty=empty
                        $($extra)*
                    />
                }
                .into_any()
            }
            (Some(before), None, None) => {
                view! {
                    <DtScroller
                        items=$items
                        item_size=$size
                        dynamic=$dynamic
                        buffer=$buffer
                        direction=$direction
                        key_field=$key_field
                        list_tag=$list_tag
                        item_tag=$item_tag
                        scroller_width=$width_css
                        scroller_height=$height_css
                        before=before
                        $($extra)*
                    />
                }
                .into_any()
            }
            (None, Some(after), None) => {
                view! {
                    <DtScroller
                        items=$items
                        item_size=$size
                        dynamic=$dynamic
                        buffer=$buffer
                        direction=$direction
                        key_field=$key_field
                        list_tag=$list_tag
                        item_tag=$item_tag
                        scroller_width=$width_css
                        scroller_height=$height_css
                        after=after
                        $($extra)*
                    />
                }
                .into_any()
            }
            (None, None, Some(empty)) => {
                view! {
                    <DtScroller
                        items=$items
                        item_size=$size
                        dynamic=$dynamic
                        buffer=$buffer
                        direction=$direction
                        key_field=$key_field
                        list_tag=$list_tag
                        item_tag=$item_tag
                        scroller_width=$width_css
                        scroller_height=$height_css
                        empty=empty
                        $($extra)*
                    />
                }
                .into_any()
            }
            (None, None, None) => {
                view! {
                    <DtScroller
                        items=$items
                        item_size=$size
                        dynamic=$dynamic
                        buffer=$buffer
                        direction=$direction
                        key_field=$key_field
                        list_tag=$list_tag
                        item_tag=$item_tag
                        scroller_width=$width_css
                        scroller_height=$height_css
                        $($extra)*
                    />
                }
                .into_any()
            }
        }
    };
}

#[component]
pub(crate) fn ScrollerPage() -> impl IntoView {
    let count = RwSignal::new("40".to_string());
    let item_size = RwSignal::new("32".to_string());
    let min_item_size = RwSignal::new(String::new());
    let buffer = RwSignal::new(String::new());
    let direction = RwSignal::new("");
    let dynamic = RwSignal::new(false);
    let key_field = RwSignal::new(String::new());
    let list_tag = RwSignal::new("");
    let item_tag = RwSignal::new("");
    let scroller_width = RwSignal::new(String::new());
    let scroller_height = RwSignal::new("160".to_string());
    let before = RwSignal::new(String::new());
    let after = RwSignal::new(String::new());
    let empty = RwSignal::new(String::new());
    let items = RwSignal::new(scroller_rows(40, 32.0));
    Effect::new(move |_| {
        let count = row_count(&count.get());
        let size = parse_f64(&item_size.get()).unwrap_or(32.0);
        items.set(scroller_rows(count, size));
    });
    let preview = ViewFn::from(move || {
        view! {
            <div class="w-full overflow-hidden rounded-lg border border-black/10">
                {move || {
                    let size = parse_f64(&item_size.get()).unwrap_or(32.0);
                    let min_size = parse_f64(&min_item_size.get());
                    let buffer = parse_f64(&buffer.get()).unwrap_or(200.0);
                    let direction = if direction.get() == "horizontal" {
                        "horizontal"
                    } else {
                        "vertical"
                    };
                    let dynamic = dynamic.get();
                    let key_raw = key_field.get();
                    let key_field = if key_raw.trim().is_empty() {
                        "id"
                    } else {
                        intern(key_raw.trim())
                    };
                    let list_tag = tag_or_div(list_tag.get());
                    let item_tag = tag_or_div(item_tag.get());
                    let width_css = css_len(&scroller_width.get(), "100%");
                    let height_css = css_len(&scroller_height.get(), "160");
                    let before_slot = text_slot(before.get());
                    let after_slot = text_slot(after.get());
                    let empty_slot = text_slot(empty.get());
                    if let Some(min) = min_size {
                        scroller_view!(
                            items,
                            size,
                            dynamic,
                            buffer,
                            direction,
                            key_field,
                            list_tag,
                            item_tag,
                            width_css.clone(),
                            height_css.clone(),
                            before_slot.clone(),
                            after_slot.clone(),
                            empty_slot.clone(),
                            min_item_size=min
                        )
                    } else {
                        scroller_view!(
                            items,
                            size,
                            dynamic,
                            buffer,
                            direction,
                            key_field,
                            list_tag,
                            item_tag,
                            width_css,
                            height_css,
                            before_slot,
                            after_slot,
                            empty_slot,
                        )
                    }
                }}
            </div>
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Items" hint="Number of rows. Empty uses 40." value=count />
            <SettingText label="Item size" hint="Fixed row size in pixels." value=item_size />
            <SettingText
                label="Min item size"
                hint="Dynamic mode fallback, in pixels."
                value=min_item_size
            />
            <SettingText label="Buffer" hint="Pixels outside the viewport. Empty uses 200." value=buffer />
            <SettingSelect label="Direction" value=direction options=DIRECTIONS.to_vec() />
            <SettingCheck label="Dynamic" value=dynamic />
            <SettingText label="Key field" hint="Empty uses id." value=key_field />
            <SettingSelect label="List tag" value=list_tag options=TAGS.to_vec() />
            <SettingSelect label="Item tag" value=item_tag options=TAGS.to_vec() />
            <SettingText label="Scroller width" hint="Number is pixels. Empty uses 100%." value=scroller_width />
            <SettingText label="Scroller height" hint="Number is pixels." value=scroller_height />
        }
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingText label="Before" hint="Content before the rows." value=before />
            <SettingText label="Empty" hint="Shown when there are no rows." value=empty />
            <SettingText label="After" hint="Content after the rows." value=after />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        let count = row_count(&count.get());
        if count == 40 {
            push_expr(&mut props, "items", "items");
        } else {
            push_expr(
                &mut props,
                "items",
                format!(
                    "(1..={count}).map(|index| DtScrollerItem::new(index.to_string(), ViewFn::from(move || view! {{ <div>{{format!(\"Incident {{index}}\")}}</div> }}))).collect::<Vec<_>>()"
                ),
            );
        }
        if let Some(size) = parse_f64(&item_size.get()) {
            push_expr(&mut props, "item_size", f64_lit(size));
        }
        if let Some(min) = parse_f64(&min_item_size.get()) {
            push_expr(&mut props, "min_item_size", f64_lit(min));
        }
        push_bool(&mut props, "dynamic", dynamic.get());
        if let Some(buffer) = parse_f64(&buffer.get()) {
            push_expr(&mut props, "buffer", f64_lit(buffer));
        }
        if !direction.get().is_empty() {
            push_str(&mut props, "direction", direction.get());
        }
        push_str(&mut props, "key_field", key_field.get().trim());
        if !list_tag.get().is_empty() {
            push_str(&mut props, "list_tag", list_tag.get());
        }
        if !item_tag.get().is_empty() {
            push_str(&mut props, "item_tag", item_tag.get());
        }
        let width = scroller_width.get();
        if !width.trim().is_empty() {
            if width.trim().parse::<i32>().is_ok() {
                push_expr(&mut props, "scroller_width", width.trim().to_string());
            } else {
                push_str(&mut props, "scroller_width", width.trim());
            }
        }
        let height = scroller_height.get();
        if !height.trim().is_empty() {
            if height.trim().parse::<i32>().is_ok() {
                push_expr(&mut props, "scroller_height", height.trim().to_string());
            } else {
                push_str(&mut props, "scroller_height", height.trim());
            }
        }
        push_span_slot(&mut props, "before", &before.get());
        push_span_slot(&mut props, "empty", &empty.get());
        push_span_slot(&mut props, "after", &after.get());
        view_of(&element("DtScroller", &props, None))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"Forty rows at 32px. Rows outside the window are not mounted."</p>
            <Play stack=true preview=preview settings=settings slots=slots code=code />
        </section>
    }
}
