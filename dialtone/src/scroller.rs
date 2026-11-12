use leptos::context::Provider;
use leptos::prelude::*;

/// Visible row context. `active` is true for every mounted row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScrollerSlot {
    pub index: usize,
    pub active: bool,
}

/// One virtual row. `key` identifies the row. `view` renders only while the row is in the window.
/// `size` is the variable-mode height (or width). Empty uses `min_item_size`.
#[derive(Clone)]
pub struct DtScrollerItem {
    pub key: String,
    pub view: ViewFn,
    pub size: Option<f64>,
}

impl DtScrollerItem {
    pub fn new(key: impl Into<String>, view: impl Into<ViewFn>) -> Self {
        Self {
            key: key.into(),
            view: view.into(),
            size: None,
        }
    }

    pub fn with_size(mut self, size: f64) -> Self {
        self.size = Some(size);
        self
    }
}

/// CSS length for `scrollerWidth` and `scrollerHeight`. A number becomes `{n}px`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CssLength(String);

impl CssLength {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for CssLength {
    fn default() -> Self {
        Self("100%".to_string())
    }
}

impl From<&'static str> for CssLength {
    fn from(value: &'static str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for CssLength {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<i32> for CssLength {
    fn from(value: i32) -> Self {
        Self(format!("{value}px"))
    }
}

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Fixed or variable window. `end` is exclusive. `over_limit` means the span exceeded 1000 items.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ScrollerWindow {
    pub start: usize,
    pub end: usize,
    pub total_size: f64,
    pub over_limit: bool,
}

impl ScrollerWindow {
    fn empty() -> Self {
        Self {
            start: 0,
            end: 0,
            total_size: 0.0,
            over_limit: false,
        }
    }
}

fn trunc_toward_zero(value: f64) -> i64 {
    value as i64
}

fn px_number(value: f64) -> String {
    if value.fract() == 0.0 && value.is_finite() {
        format!("{}", value as i64)
    } else {
        format!("{value}")
    }
}

/// `translateY({pos}px) translateX(0px)`, or the horizontal swap.
pub(crate) fn item_transform(vertical: bool, position: f64) -> String {
    let px = px_number(position);
    if vertical {
        format!("transform: translateY({px}px) translateX(0px)")
    } else {
        format!("transform: translateX({px}px) translateY(0px)")
    }
}

pub(crate) fn scroller_root_class(direction: &str, ready: bool) -> String {
    join_class([
        "vue-recycle-scroller".to_string(),
        format!("direction-{direction}"),
        if ready {
            "ready".to_string()
        } else {
            String::new()
        },
    ])
}

pub(crate) fn scroller_wrapper_class(list_class: &str) -> String {
    join_class(["vue-recycle-scroller__item-wrapper", list_class])
}

pub(crate) fn scroller_item_class(item_class: &str, hover: bool) -> String {
    join_class([
        "vue-recycle-scroller__item-view",
        item_class,
        if hover { "hover" } else { "" },
    ])
}

/// `true` when the scroll moved less than one item, so the window stays put.
pub(crate) fn position_diff_skips(scroll: f64, last: f64, threshold: f64) -> bool {
    (scroll - last).abs() < threshold
}

/// User-position events for one scroll, in Vue order, plus the position left active.
///
/// The previous position starts as `blockStart` and is not emitted until a scroll.
/// A zero-size layout emits `middle`, then `blockStart` and `top`, then
/// `blockEnd` and `bottom`. The stored position is `blockStart` or `blockEnd`.
/// `top` and `bottom` are the deprecated names and are still emitted.
pub(crate) fn user_position_events(
    previous: &str,
    scroll: f64,
    client: f64,
    scroll_size: f64,
) -> (Vec<&'static str>, String) {
    let mut events = Vec::new();
    let mut current = previous.to_string();
    if current != "middle" {
        current = "middle".to_string();
        events.push("middle");
    }
    if scroll == 0.0 {
        current = "blockStart".to_string();
        events.push("blockStart");
        events.push("top");
    }
    if scroll + client == scroll_size {
        current = "blockEnd".to_string();
        events.push("blockEnd");
        events.push("bottom");
    }
    (events, current)
}

fn fixed_window(
    count: usize,
    item_size: f64,
    scroll: f64,
    viewport: f64,
    buffer: f64,
    before: f64,
    after: f64,
) -> ScrollerWindow {
    if count == 0 || item_size <= 0.0 {
        return ScrollerWindow::empty();
    }
    let start_px = scroll - buffer - before;
    let end_px = scroll + viewport + buffer + after;
    let mut start = trunc_toward_zero(start_px / item_size);
    if start < 0 {
        start = 0;
    }
    let mut end = (end_px / item_size).ceil() as i64;
    if end < 0 {
        end = 0;
    }
    let count_i = count as i64;
    if end > count_i {
        end = count_i;
    }
    if end < start {
        end = start;
    }
    let start = start as usize;
    let end = end as usize;
    let total_size = count as f64 * item_size;
    if end - start > 1000 {
        return ScrollerWindow {
            start: 0,
            end: 0,
            total_size,
            over_limit: true,
        };
    }
    ScrollerWindow {
        start,
        end,
        total_size,
        over_limit: false,
    }
}

fn variable_start(accum: &[f64], scroll_start: f64) -> usize {
    let count = accum.len();
    if count == 0 {
        return 0;
    }
    let mut lower = 0i64;
    let mut upper = count as i64 - 1;
    let mut index = (count as i64) / 2;
    loop {
        let previous = index;
        let end = accum.get(index as usize).copied().unwrap_or(0.0);
        if end < scroll_start {
            lower = index;
        } else if (index as usize) + 1 < count && accum[(index as usize) + 1] > scroll_start {
            upper = index;
        }
        index = (lower + upper) / 2;
        if index == previous {
            break;
        }
    }
    if index < 0 {
        0
    } else {
        index as usize
    }
}

fn variable_window(
    sizes: &[f64],
    scroll: f64,
    viewport: f64,
    buffer: f64,
    before: f64,
    after: f64,
) -> ScrollerWindow {
    let count = sizes.len();
    if count == 0 {
        return ScrollerWindow::empty();
    }
    let mut accum = Vec::with_capacity(count);
    let mut total = 0.0;
    for size in sizes {
        total += *size;
        accum.push(total);
    }
    let start_px = scroll - buffer - before;
    let end_px = scroll + viewport + buffer + after;
    let start = variable_start(&accum, start_px);
    let mut end = start;
    while end < count && accum[end] < end_px {
        end += 1;
    }
    end += 1;
    if end > count {
        end = count;
    }
    if end < start {
        end = start;
    }
    if end - start > 1000 {
        return ScrollerWindow {
            start: 0,
            end: 0,
            total_size: total,
            over_limit: true,
        };
    }
    ScrollerWindow {
        start,
        end,
        total_size: total,
        over_limit: false,
    }
}

pub(crate) fn scroller_window(
    dynamic: bool,
    item_size: Option<f64>,
    min_item_size: Option<f64>,
    item_sizes: &[Option<f64>],
    scroll: f64,
    viewport: f64,
    buffer: f64,
    before: f64,
    after: f64,
) -> ScrollerWindow {
    if dynamic {
        let min = min_item_size.unwrap_or(0.0);
        let sizes: Vec<f64> = item_sizes
            .iter()
            .map(|size| size.filter(|value| *value > 0.0).unwrap_or(min))
            .collect();
        variable_window(&sizes, scroll, viewport, buffer, before, after)
    } else {
        fixed_window(
            item_sizes.len(),
            item_size.unwrap_or(0.0),
            scroll,
            viewport,
            buffer,
            before,
            after,
        )
    }
}

fn scroller_direction(direction: &str) -> &'static str {
    match direction {
        "horizontal" => "horizontal",
        "vertical" => "vertical",
        other => {
            leptos::logging::warn!("DtScroller direction `{other}` is not vertical or horizontal");
            "vertical"
        }
    }
}

fn scroller_tag(tag: &'static str, label: &str) -> &'static str {
    if matches!(tag, "div" | "ul" | "ol" | "li" | "span") {
        tag
    } else {
        leptos::logging::warn!("DtScroller {label} `{tag}` is not supported; rendering a div");
        "div"
    }
}

fn axis_start(element: &web_sys::Element, vertical: bool) -> f64 {
    if vertical {
        element.scroll_top() as f64
    } else {
        element.scroll_left() as f64
    }
}

fn axis_client(element: &web_sys::Element, vertical: bool) -> f64 {
    if vertical {
        element.client_height() as f64
    } else {
        element.client_width() as f64
    }
}

fn axis_extent(element: &web_sys::Element, vertical: bool) -> f64 {
    if vertical {
        element.scroll_height() as f64
    } else {
        element.scroll_width() as f64
    }
}

/// Dialtone Vue `DtScroller` (`dialtone/v10.7.0`).
///
/// Fixed-size mode uses `item_size`. Dynamic mode uses each item's `size`, or
/// `min_item_size` when that is empty. Rendered rows are not measured. Vue's
/// dynamic scroller writes sizes from a `ResizeObserver`. The window is capped
/// at 1000 items. `scroll_to_index` sets the scroll offset to
/// `index * item_size` in fixed mode (the previous item's end in dynamic mode)
/// and does not itself emit `on_user_position`. `on_user_position` emits
/// `middle`, then `blockStart` and `top`, or `blockEnd` and `bottom`. Vue's
/// `handleScroll` reads `scrollTop` in both directions. This port uses the
/// active axis. `enable_hover` is `enableHover`.
#[component]
pub fn DtScroller(
    /// Rows (`items`). Pass `Signal::stored(vec)` or a read signal.
    #[prop(into)]
    items: Signal<Vec<DtScrollerItem>>,
    /// Fixed row size in px (`itemSize`). Required when `dynamic` is false.
    #[prop(optional)]
    item_size: Option<f64>,
    /// Fallback row size in dynamic mode (`minItemSize`).
    #[prop(optional)]
    min_item_size: Option<f64>,
    /// Variable sizes (`dynamic`).
    #[prop(optional)]
    dynamic: bool,
    /// Pixels rendered beyond the viewport (`buffer`).
    #[prop(optional, default = 200.0)]
    buffer: f64,
    /// `vertical` (default) or `horizontal` (`direction`).
    #[prop(optional, default = "vertical")]
    direction: &'static str,
    /// Vue `keyField`. Each [`DtScrollerItem`] already carries `key`.
    #[prop(optional, default = "id")]
    key_field: &'static str,
    /// List element (`listTag`).
    #[prop(optional, default = "div")]
    list_tag: &'static str,
    /// Row element (`itemTag`).
    #[prop(optional, default = "div")]
    item_tag: &'static str,
    /// Extra class on the item wrapper (`listClass`).
    #[prop(optional, into)]
    list_class: String,
    /// Extra class on each row (`itemClass`).
    #[prop(optional, into)]
    item_class: String,
    /// Hover class and mouse events (`enableHover`).
    #[prop(optional, default = true)]
    enable_hover: bool,
    /// Viewport width (`scrollerWidth`).
    #[prop(optional, into)]
    scroller_width: CssLength,
    /// Viewport height (`scrollerHeight`).
    #[prop(optional, into)]
    scroller_height: CssLength,
    /// Scroll to this index when the signal changes (`scrollToItem`).
    #[prop(optional)]
    scroll_to_index: Option<Signal<Option<usize>>>,
    /// `user-position`: `middle`, `top`, or `bottom`.
    #[prop(optional, into)]
    on_user_position: Option<Callback<String>>,
    /// First row entered the window (`scroll-start`).
    #[prop(optional, into)]
    on_scroll_start: Option<Callback<()>>,
    /// Last row entered the window (`scroll-end`).
    #[prop(optional, into)]
    on_scroll_end: Option<Callback<()>>,
    /// Content before the rows (`before`).
    #[prop(optional)]
    before: Option<ViewFn>,
    /// Content after the rows (`after`).
    #[prop(optional)]
    after: Option<ViewFn>,
    /// Shown inside the wrapper while `items` is empty (`empty`).
    #[prop(optional)]
    empty: Option<ViewFn>,
) -> impl IntoView {
    let _ = key_field;
    if dynamic && min_item_size.is_none() {
        leptos::logging::warn!("scroller error: 'minItemSize' is required on 'dynamic' mode.");
    }
    if !dynamic && item_size.filter(|size| *size > 0.0).is_none() {
        leptos::logging::warn!("scroller error: 'itemSize' is required.");
    }
    let direction = scroller_direction(direction);
    let list_tag = scroller_tag(list_tag, "list_tag");
    let item_tag = scroller_tag(item_tag, "item_tag");
    let vertical = direction == "vertical";
    let scroller_ref = NodeRef::<leptos::html::Div>::new();
    let before_ref = NodeRef::<leptos::html::Div>::new();
    let after_ref = NodeRef::<leptos::html::Div>::new();
    let scroll_px = RwSignal::new(0.0);
    let client_px = RwSignal::new(0.0);
    let ready = RwSignal::new(false);
    let hover = RwSignal::new(None::<String>);
    let position = RwSignal::new("blockStart".to_string());
    let last_update = StoredValue::new(0.0);
    let ignore_position = StoredValue::new(false);
    let limit_warned = StoredValue::new(false);
    let style = format!(
        "width: {}; height: {}",
        scroller_width.as_str(),
        scroller_height.as_str()
    );
    let list_class = StoredValue::new(list_class);
    let item_class = StoredValue::new(item_class);
    let empty = StoredValue::new(empty);

    let measure = move || {
        let sizes: Vec<Option<f64>> = items.get().iter().map(|item| item.size).collect();
        let before = before_ref
            .get()
            .map(|element| axis_extent(&element, vertical))
            .unwrap_or(0.0);
        let after = after_ref
            .get()
            .map(|element| axis_extent(&element, vertical))
            .unwrap_or(0.0);
        scroller_window(
            dynamic,
            item_size,
            min_item_size,
            &sizes,
            scroll_px.get(),
            client_px.get(),
            buffer,
            before,
            after,
        )
    };

    Effect::new(move |_| {
        let Some(element) = scroller_ref.get() else {
            return;
        };
        let start = axis_start(&element, vertical);
        let client = axis_client(&element, vertical);
        last_update.set_value(start);
        scroll_px.set(start);
        client_px.set(client);
        ready.set(true);
    });

    let previous_range = StoredValue::new((0usize, 0usize));
    Effect::new(move |_| {
        let count = items.get().len();
        let window = measure();
        if window.over_limit && !limit_warned.get_value() {
            limit_warned.set_value(true);
            leptos::logging::warn!(
                "DtScroller rendered item limit reached. Give the scroller a fixed size and overflow so it renders the visible rows."
            );
        }
        let next = (window.start, window.end);
        let previous = previous_range.get_value();
        let contains = |range: (usize, usize), index: usize| range.0 <= index && index < range.1;
        if count > 0 && contains(next, 0) && !contains(previous, 0) {
            if let Some(callback) = on_scroll_start {
                callback.run(());
            }
        }
        if count > 0 && contains(next, count - 1) && !contains(previous, count - 1) {
            if let Some(callback) = on_scroll_end {
                callback.run(());
            }
        }
        previous_range.set_value(next);
    });

    if let Some(scroll_to_index) = scroll_to_index {
        Effect::new(move |_| {
            let Some(index) = scroll_to_index.get() else {
                return;
            };
            let Some(element) = scroller_ref.get() else {
                return;
            };
            let list = items.get();
            let top = if dynamic {
                let min = min_item_size.unwrap_or(0.0);
                list.iter()
                    .take(index)
                    .map(|item| item.size.filter(|size| *size > 0.0).unwrap_or(min))
                    .sum()
            } else {
                index as f64 * item_size.unwrap_or(0.0)
            };
            ignore_position.set_value(true);
            if vertical {
                element.set_scroll_top(top as i32);
            } else {
                element.set_scroll_left(top as i32);
            }
            ignore_position.set_value(false);
            last_update.set_value(top);
            scroll_px.set(top);
        });
    }

    let on_scroll = move |_| {
        let Some(element) = scroller_ref.get_untracked() else {
            return;
        };
        let start = axis_start(&element, vertical);
        let client = axis_client(&element, vertical);
        let extent = axis_extent(&element, vertical);
        if !ignore_position.get_value() {
            let (events, next) =
                user_position_events(&position.get_untracked(), start, client, extent);
            position.set(next);
            if let Some(callback) = on_user_position {
                for event in events {
                    callback.run(event.to_string());
                }
            }
        }
        let threshold = if dynamic {
            min_item_size.unwrap_or(item_size.unwrap_or(0.0))
        } else {
            item_size.unwrap_or(0.0)
        };
        if ready.get_untracked() && position_diff_skips(start, last_update.get_value(), threshold) {
            return;
        }
        last_update.set_value(start);
        scroll_px.set(start);
        client_px.set(client);
    };

    let rows = move || {
        let list = items.get();
        let window = measure();
        let min = min_item_size.unwrap_or(0.0);
        let fixed = item_size.unwrap_or(0.0);
        let mut prefix = 0.0;
        let mut views = Vec::new();
        for (index, item) in list.iter().enumerate() {
            let size =
                item.size
                    .filter(|size| *size > 0.0)
                    .unwrap_or(if dynamic { min } else { fixed });
            let position = if dynamic {
                prefix
            } else {
                index as f64 * fixed
            };
            if dynamic {
                prefix += size;
            }
            if index < window.start || index >= window.end {
                continue;
            }
            let key = item.key.clone();
            let view = item.view.clone();
            let extra = item_class.get_value();
            let hover_key = key.clone();
            let class_name = move || {
                scroller_item_class(
                    &extra,
                    enable_hover && hover.get().as_deref() == Some(hover_key.as_str()),
                )
            };
            let transform = ready.get().then(|| item_transform(vertical, position));
            let row = match item_tag {
                "li" => view! {
                    <li
                        class=class_name
                        style=transform
                        data-qa="dt-scroller-item"
                        on:mouseenter=move |_| {
                            if enable_hover {
                                hover.set(Some(key.clone()));
                            }
                        }
                        on:mouseleave=move |_| {
                            if enable_hover {
                                hover.set(None);
                            }
                        }
                    >
                        <Provider value=ScrollerSlot { index, active: true }>
                            {view.run()}
                        </Provider>
                    </li>
                }
                .into_any(),
                "span" => view! {
                    <span
                        class=class_name
                        style=transform
                        data-qa="dt-scroller-item"
                        on:mouseenter=move |_| {
                            if enable_hover {
                                hover.set(Some(key.clone()));
                            }
                        }
                        on:mouseleave=move |_| {
                            if enable_hover {
                                hover.set(None);
                            }
                        }
                    >
                        <Provider value=ScrollerSlot { index, active: true }>
                            {view.run()}
                        </Provider>
                    </span>
                }
                .into_any(),
                _ => view! {
                    <div
                        class=class_name
                        style=transform
                        data-qa="dt-scroller-item"
                        on:mouseenter=move |_| {
                            if enable_hover {
                                hover.set(Some(key.clone()));
                            }
                        }
                        on:mouseleave=move |_| {
                            if enable_hover {
                                hover.set(None);
                            }
                        }
                    >
                        <Provider value=ScrollerSlot { index, active: true }>
                            {view.run()}
                        </Provider>
                    </div>
                }
                .into_any(),
            };
            views.push(row);
        }
        views.collect_view()
    };

    let wrapper_style = move || {
        let window = measure();
        let length = format!("{}px", px_number(window.total_size));
        if vertical {
            format!("min-height: {length}")
        } else {
            format!("min-width: {length}")
        }
    };
    let wrapper_class = move || scroller_wrapper_class(&list_class.get_value());
    let empty_view = move || {
        if items.get().is_empty() {
            empty.get_value().map(|slot| slot.run())
        } else {
            None
        }
    };

    macro_rules! item_list {
        ($tag:ident) => {
            view! {
                <$tag
                    class=wrapper_class
                    style=wrapper_style
                    data-qa="dt-scroller-item-wrapper"
                >
                    {rows}
                    {empty_view}
                </$tag>
            }
            .into_any()
        };
    }
    let list = match list_tag {
        "ul" => item_list!(ul),
        "ol" => item_list!(ol),
        "li" => item_list!(li),
        "span" => item_list!(span),
        _ => item_list!(div),
    };

    view! {
        <div
            node_ref=scroller_ref
            class=move || scroller_root_class(direction, ready.get())
            style=style
            tabindex="0"
            data-qa="dt-scroller"
            on:scroll=on_scroll
        >
            {before.map(|slot| view! {
                <div
                    node_ref=before_ref
                    class="vue-recycle-scroller__slot"
                    data-qa="dt-scroller-before"
                >
                    {slot.run()}
                </div>
            })}
            {list}
            {after.map(|slot| view! {
                <div
                    node_ref=after_ref
                    class="vue-recycle-scroller__slot"
                    data-qa="dt-scroller-after"
                >
                    {slot.run()}
                </div>
            })}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sizes(count: usize) -> Vec<Option<f64>> {
        vec![None; count]
    }

    #[test]
    fn fixed_window_matches_the_vue_scroll_math() {
        let window = scroller_window(
            false,
            Some(30.0),
            None,
            &sizes(20),
            0.0,
            60.0,
            200.0,
            0.0,
            0.0,
        );
        assert_eq!(window.start, 0);
        assert_eq!(window.end, 9);
        assert_eq!(window.total_size, 600.0);
        assert!(!window.over_limit);

        let scrolled = scroller_window(
            false,
            Some(30.0),
            None,
            &sizes(20),
            240.0,
            60.0,
            200.0,
            0.0,
            0.0,
        );
        assert_eq!(scrolled.start, 1);
        assert!(scrolled.end < 20);

        let end = scroller_window(
            false,
            Some(30.0),
            None,
            &sizes(20),
            540.0,
            60.0,
            200.0,
            0.0,
            0.0,
        );
        assert!(end.start > 0);
        assert_eq!(end.end, 20);
    }

    #[test]
    fn negative_start_truncates_toward_zero_then_clamps() {
        let window = scroller_window(
            false,
            Some(30.0),
            None,
            &sizes(20),
            0.0,
            60.0,
            200.0,
            30.0,
            10.0,
        );
        assert_eq!(window.start, 0);
        assert_eq!(trunc_toward_zero(-200.0 / 30.0), -6);
    }

    #[test]
    fn position_diff_below_one_item_skips_the_window_update() {
        assert!(position_diff_skips(25.0, 0.0, 30.0));
        assert!(!position_diff_skips(30.0, 0.0, 30.0));
    }

    #[test]
    fn user_position_on_a_zero_layout_emits_block_start_and_block_end() {
        let (events, next) = user_position_events("blockStart", 0.0, 0.0, 0.0);
        assert_eq!(events, ["middle", "blockStart", "top", "blockEnd", "bottom"]);
        assert_eq!(next, "blockEnd");
        let (events, next) = user_position_events("middle", 10.0, 60.0, 600.0);
        assert!(events.is_empty());
        assert_eq!(next, "middle");
        let (events, next) = user_position_events("middle", 0.0, 60.0, 600.0);
        assert_eq!(events, ["blockStart", "top"]);
        assert_eq!(next, "blockStart");
        let (events, next) = user_position_events("middle", 540.0, 60.0, 600.0);
        assert_eq!(events, ["blockEnd", "bottom"]);
        assert_eq!(next, "blockEnd");
    }

    #[test]
    fn variable_window_uses_the_size_accumulator() {
        let sizes = vec![Some(10.0), Some(20.0), Some(30.0)];
        let window = scroller_window(true, None, Some(8.0), &sizes, 0.0, 25.0, 0.0, 0.0, 0.0);
        assert_eq!(window.start, 0);
        assert_eq!(window.end, 2);
        assert_eq!(window.total_size, 60.0);
    }

    #[test]
    fn a_window_over_1000_items_is_not_rendered() {
        let window = scroller_window(
            false,
            Some(1.0),
            None,
            &sizes(1001),
            0.0,
            5000.0,
            0.0,
            0.0,
            0.0,
        );
        assert!(window.over_limit);
        assert_eq!(window.start, 0);
        assert_eq!(window.end, 0);
        assert_eq!(window.total_size, 1001.0);
    }

    #[test]
    fn classes_and_transforms_follow_the_vue_template() {
        assert_eq!(
            scroller_root_class("vertical", false),
            "vue-recycle-scroller direction-vertical"
        );
        assert_eq!(
            scroller_root_class("horizontal", true),
            "vue-recycle-scroller direction-horizontal ready"
        );
        assert_eq!(
            scroller_wrapper_class("list"),
            "vue-recycle-scroller__item-wrapper list"
        );
        assert_eq!(
            scroller_item_class("row", true),
            "vue-recycle-scroller__item-view row hover"
        );
        assert_eq!(
            item_transform(true, 30.0),
            "transform: translateY(30px) translateX(0px)"
        );
        assert_eq!(
            item_transform(false, 60.0),
            "transform: translateX(60px) translateY(0px)"
        );
        assert_eq!(CssLength::from(320).as_str(), "320px");
        assert_eq!(CssLength::from("50%").as_str(), "50%");
        assert_eq!(CssLength::default().as_str(), "100%");
    }
}
