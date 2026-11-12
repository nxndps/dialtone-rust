use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, KeyboardEvent};

use super::button::DtButton;
use super::icon::{DtIconChevronLeft, DtIconChevronRight, DtIconChevronsLeft, DtIconChevronsRight};
use super::list_nav::{join_class, role_items};
use super::tooltip::DtTooltip;
use super::unique_id;

/// en-US Dialtone strings (`packages/dialtone-vue/localization/en-US.ftl`).
const SELECT_DAY: &str = "Select day";
const CHANGE_TO: &str = "Change to";
const PREVIOUS_MONTH: &str = "Previous month";
const NEXT_MONTH: &str = "Next month";
const PREVIOUS_YEAR: &str = "Previous year";
const NEXT_YEAR: &str = "Next year";

/// Civil date. `month` is 1–12.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DtDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl DtDate {
    pub fn new(year: i32, month: u8, day: u8) -> Self {
        let month = month.clamp(1, 12);
        let day = day.clamp(1, days_in_month(year, month));
        Self { year, month, day }
    }

    /// Local today. Uses `Date` in the browser and 1970-01-01 otherwise.
    pub fn today() -> Self {
        let Some(window) = web_sys::window() else {
            return Self::new(1970, 1, 1);
        };
        let _ = window;
        let now = js_sys::Date::new_0();
        Self::new(
            js_sys::Date::get_full_year(&now) as i32,
            js_sys::Date::get_month(&now) as u8 + 1,
            js_sys::Date::get_date(&now) as u8,
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CalendarDay {
    pub date: DtDate,
    pub current_month: bool,
    pub disabled: bool,
    pub selected: bool,
    pub first_of_month: bool,
}

fn days_from_civil(year: i32, month: u32, day: u32) -> i32 {
    let mut y = year;
    if month <= 2 {
        y -= 1;
    }
    let era = (if y >= 0 { y } else { y - 399 }) / 400;
    let yoe = (y - era * 400) as u32;
    let mp = if month > 2 { month - 3 } else { month + 9 };
    let doy = (153 * mp + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe as i32 - 719468
}

fn civil_from_days(mut z: i32) -> (i32, u32, u32) {
    z += 719468;
    let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
    let doe = (z - era * 146097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i32 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (y + if m <= 2 { 1 } else { 0 }, m, d)
}

pub fn days_in_month(year: i32, month: u8) -> u8 {
    let month = month.clamp(1, 12) as u32;
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    let start = days_from_civil(year, month, 1);
    let next = days_from_civil(next_year, next_month, 1);
    (next - start) as u8
}

/// Sunday is 0, matching `Date.getDay` and Dialtone `weekStartsOn`.
pub fn weekday_index(date: DtDate) -> u8 {
    let days = days_from_civil(date.year, date.month as u32, date.day as u32);
    (days + 4).rem_euclid(7) as u8
}

fn date_ord(date: DtDate) -> i32 {
    days_from_civil(date.year, date.month as u32, date.day as u32)
}

pub fn shift_month(year: i32, month: u8, delta: i32) -> (i32, u8) {
    let index = month as i32 - 1 + delta;
    let year = year + index.div_euclid(12);
    let month = index.rem_euclid(12) as u8 + 1;
    (year, month)
}

/// `getWeekDayNames` rotation. `names` is Sunday through Saturday.
pub fn rotate_weekday_names(names: &[String], week_start: u8) -> Vec<String> {
    if names.len() != 7 {
        return names.to_vec();
    }
    let start = (week_start as usize) % 7;
    names[start..]
        .iter()
        .chain(names[..start].iter())
        .cloned()
        .collect()
}

/// `getCalendarDays`. `month` is 1–12. `selected_day` is the day-of-month highlight.
pub fn calendar_weeks(
    year: i32,
    month: u8,
    selected_day: Option<u8>,
    min_date: Option<DtDate>,
    max_date: Option<DtDate>,
    week_start: u8,
) -> Vec<Vec<CalendarDay>> {
    if !(1..=12).contains(&month) {
        return Vec::new();
    }
    let week_start = week_start % 7;
    let first_weekday = weekday_index(DtDate {
        year,
        month,
        day: 1,
    });
    let offset = (first_weekday as i32 - week_start as i32 + 7).rem_euclid(7);
    let mut cursor = days_from_civil(year, month as u32, 1) - offset;
    let last = days_from_civil(year, month as u32, days_in_month(year, month) as u32);
    let mut weeks = Vec::new();
    loop {
        let mut week = Vec::with_capacity(7);
        let mut contains_last = false;
        for _ in 0..7 {
            if cursor == last {
                contains_last = true;
            }
            let (y, m, d) = civil_from_days(cursor);
            let date = DtDate {
                year: y,
                month: m as u8,
                day: d as u8,
            };
            let outside = date.year != year || date.month != month;
            let before_min = min_date.is_some_and(|min| date_ord(date) < date_ord(min));
            let after_max = max_date.is_some_and(|max| date_ord(date) > date_ord(max));
            let disabled = outside || before_min || after_max;
            let selected = selected_day.is_some_and(|day| date.day == day && !disabled);
            week.push(CalendarDay {
                date,
                current_month: !outside,
                disabled,
                selected,
                first_of_month: !outside && date.day == 1,
            });
            cursor += 1;
        }
        weeks.push(week);
        if contains_last || weeks.len() > 6 {
            break;
        }
    }
    weeks
}

pub fn next_focus_day(current: DtDate) -> u8 {
    let (year, month) = shift_month(current.year, current.month, 1);
    let start = DtDate {
        year,
        month,
        day: 1,
    };
    let diff = (weekday_index(current) as i32 - weekday_index(start) as i32 + 7).rem_euclid(7);
    1 + diff as u8
}

pub fn prev_focus_day(current: DtDate) -> u8 {
    let (year, month) = shift_month(current.year, current.month, -1);
    let wanted = weekday_index(current);
    let mut day = days_in_month(year, month);
    while weekday_index(DtDate { year, month, day }) != wanted {
        day -= 1;
    }
    day
}

pub fn prev_month_disabled(year: i32, month: u8, min_date: Option<DtDate>) -> bool {
    let Some(min_date) = min_date else {
        return false;
    };
    let (year, month) = shift_month(year, month, -1);
    let end = DtDate {
        year,
        month,
        day: days_in_month(year, month),
    };
    date_ord(end) < date_ord(min_date)
}

pub fn next_month_disabled(year: i32, month: u8, max_date: Option<DtDate>) -> bool {
    let Some(max_date) = max_date else {
        return false;
    };
    let (year, month) = shift_month(year, month, 1);
    date_ord(DtDate {
        year,
        month,
        day: 1,
    }) > date_ord(max_date)
}

pub fn prev_year_disabled(year: i32, month: u8, min_date: Option<DtDate>) -> bool {
    let Some(min_date) = min_date else {
        return false;
    };
    let year = year - 1;
    let end = DtDate {
        year,
        month,
        day: days_in_month(year, month),
    };
    date_ord(end) < date_ord(min_date)
}

pub fn next_year_disabled(year: i32, month: u8, max_date: Option<DtDate>) -> bool {
    let Some(max_date) = max_date else {
        return false;
    };
    date_ord(DtDate {
        year: year + 1,
        month,
        day: 1,
    }) > date_ord(max_date)
}

fn set_opt(options: &js_sys::Object, key: &str, value: &str) {
    let _ = js_sys::Reflect::set(
        options,
        &wasm_bindgen::JsValue::from_str(key),
        &wasm_bindgen::JsValue::from_str(value),
    );
}

fn intl_format(date: DtDate, locale: &str, fill: impl FnOnce(&js_sys::Object)) -> String {
    let fallback = format!("{:04}-{:02}-{:02}", date.year, date.month, date.day);
    if web_sys::window().is_none() {
        return fallback;
    }
    let global = js_sys::global();
    let Ok(intl) = js_sys::Reflect::get(&global, &"Intl".into()) else {
        return fallback;
    };
    let Ok(ctor) = js_sys::Reflect::get(&intl, &"DateTimeFormat".into()) else {
        return fallback;
    };
    let Ok(ctor) = ctor.dyn_into::<js_sys::Function>() else {
        return fallback;
    };
    let options = js_sys::Object::new();
    fill(&options);
    let locale_value = if locale.is_empty() || locale == "default" {
        wasm_bindgen::JsValue::UNDEFINED
    } else {
        wasm_bindgen::JsValue::from_str(locale)
    };
    let args = js_sys::Array::of2(&locale_value, &options);
    let Ok(formatter) = js_sys::Reflect::construct(&ctor, &args) else {
        return fallback;
    };
    let Ok(format_fn) = js_sys::Reflect::get(&formatter, &"format".into()) else {
        return fallback;
    };
    let Ok(format_fn) = format_fn.dyn_into::<js_sys::Function>() else {
        return fallback;
    };
    let js_date = js_sys::Date::new_with_year_month_day(
        date.year.max(0) as u32,
        i32::from(date.month) - 1,
        i32::from(date.day),
    );
    format_fn
        .call1(&formatter, &js_date)
        .ok()
        .and_then(|value| value.as_string())
        .unwrap_or(fallback)
}

fn preferred_locale(locale: &str) -> &str {
    if locale.is_empty() {
        "default"
    } else {
        locale
    }
}

/// `Intl` `dateStyle`. The day button uses `"long"`.
pub fn format_date_style(date: DtDate, locale: &str, style: &str) -> String {
    let locale = preferred_locale(locale);
    intl_format(date, locale, |options| {
        set_opt(options, "dateStyle", style);
    })
}

/// Dialtone `formatLong`.
pub fn format_long(date: DtDate, locale: &str) -> String {
    let locale = preferred_locale(locale);
    intl_format(date, locale, |options| {
        set_opt(options, "weekday", "long");
        set_opt(options, "year", "numeric");
        set_opt(options, "month", "long");
        set_opt(options, "day", "numeric");
    })
}

/// Dialtone `formatMedium`.
pub fn format_medium(date: DtDate, locale: &str) -> String {
    let locale = preferred_locale(locale);
    intl_format(date, locale, |options| {
        set_opt(options, "year", "numeric");
        set_opt(options, "month", "long");
        set_opt(options, "day", "numeric");
    })
}

/// Dialtone `formatShort`. `show_weekday` defaults to true in Vue.
pub fn format_short(date: DtDate, locale: &str, show_weekday: bool) -> String {
    let locale = preferred_locale(locale);
    intl_format(date, locale, |options| {
        if show_weekday {
            set_opt(options, "weekday", "short");
        }
        set_opt(options, "year", "numeric");
        set_opt(options, "month", "short");
        set_opt(options, "day", "numeric");
    })
}

/// Dialtone `formatNoYear`.
pub fn format_no_year(date: DtDate, locale: &str, abbreviated: bool) -> String {
    let locale = preferred_locale(locale);
    intl_format(date, locale, |options| {
        set_opt(options, "month", if abbreviated { "short" } else { "long" });
        set_opt(options, "day", "numeric");
    })
}

/// Dialtone `formatNumerical`.
pub fn format_numerical(date: DtDate, locale: &str) -> String {
    let locale = preferred_locale(locale);
    intl_format(date, locale, |options| {
        set_opt(options, "year", "2-digit");
        set_opt(options, "month", "2-digit");
        set_opt(options, "day", "2-digit");
    })
}

pub fn format_month_year(year: i32, month: u8, locale: &str) -> String {
    intl_format(
        DtDate {
            year,
            month,
            day: 1,
        },
        preferred_locale(locale),
        |options| {
            set_opt(options, "month", "long");
            set_opt(options, "year", "numeric");
        },
    )
}

pub fn format_month(year: i32, month: u8, locale: &str) -> String {
    intl_format(
        DtDate {
            year,
            month,
            day: 1,
        },
        preferred_locale(locale),
        |options| {
            set_opt(options, "month", "long");
        },
    )
}

/// `getWeekDayNames`. Short weekday labels, Sunday-first, then rotated.
pub fn weekday_names(locale: &str, week_start: u8) -> Vec<String> {
    let locale = preferred_locale(locale);
    let mut names = Vec::with_capacity(7);
    for day in 1..=7 {
        let label = if web_sys::window().is_none() {
            ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"][(day - 1) as usize].to_string()
        } else {
            let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_str(&format!(
                "2017-01-0{day}T00:00:00+00:00"
            )));
            let formatted = intl_format_date(&date, locale, |options| {
                set_opt(options, "weekday", "short");
                set_opt(options, "timeZone", "UTC");
            });
            formatted.chars().take(2).collect()
        };
        names.push(label);
    }
    rotate_weekday_names(&names, week_start)
}

fn intl_format_date(
    date: &js_sys::Date,
    locale: &str,
    fill: impl FnOnce(&js_sys::Object),
) -> String {
    let global = js_sys::global();
    let Ok(intl) = js_sys::Reflect::get(&global, &"Intl".into()) else {
        return String::new();
    };
    let Ok(ctor) = js_sys::Reflect::get(&intl, &"DateTimeFormat".into()) else {
        return String::new();
    };
    let Ok(ctor) = ctor.dyn_into::<js_sys::Function>() else {
        return String::new();
    };
    let options = js_sys::Object::new();
    fill(&options);
    let locale_value = if locale.is_empty() || locale == "default" {
        wasm_bindgen::JsValue::UNDEFINED
    } else {
        wasm_bindgen::JsValue::from_str(locale)
    };
    let args = js_sys::Array::of2(&locale_value, &options);
    let Ok(formatter) = js_sys::Reflect::construct(&ctor, &args) else {
        return String::new();
    };
    let Ok(format_fn) = js_sys::Reflect::get(&formatter, &"format".into()) else {
        return String::new();
    };
    let Ok(format_fn) = format_fn.dyn_into::<js_sys::Function>() else {
        return String::new();
    };
    format_fn
        .call1(&formatter, date)
        .ok()
        .and_then(|value| value.as_string())
        .unwrap_or_default()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FocusTarget {
    First,
    Last,
    Day(u8),
    Nav(u8),
}

fn enabled_day_buttons(table: &HtmlElement) -> Vec<HtmlElement> {
    role_items(table, "[role=option]")
        .into_iter()
        .filter(|element| !element.class_name().contains("d-datepicker__day--disabled"))
        .filter_map(|element| element.dyn_into().ok())
        .collect()
}

fn focus_html(element: &HtmlElement) {
    let _ = element.focus();
}

fn button_by_id(id: &str) -> Option<HtmlElement> {
    let document = web_sys::window()?.document()?;
    document
        .get_element_by_id(id)?
        .dyn_into::<HtmlElement>()
        .ok()
}

/// Dialtone Vue `DtDatepicker` (`dialtone/v10.7.0`).
///
/// The calendar itself, not a popover input. `month` on [`DtDate`] is 1–12.
/// Pass `selected_date` and write it from `on_select`, or leave it unset and the
/// component keeps the selection. `on_close` is the Escape `close-datepicker` event.
/// Aria phrases are the en-US Dialtone strings. Month and weekday text uses `Intl`.
/// A day button's accessible name uses `dateStyle: "long"`.
/// Button ids are `{id}-prevYearButton`, `{id}-prevMonthButton`, `{id}-nextMonthButton`,
/// `{id}-nextYearButton`, and `{id}-day-{index}`. The heading is `{id}-calendar-heading`.
/// Vue hardcodes those ids. The prefix keeps two calendars from sharing one id.
#[component]
pub fn DtDatepicker(
    /// Controlled selection (`selectedDate`). Unset keeps an internal date.
    #[prop(optional)]
    selected_date: Option<Signal<DtDate>>,
    #[prop(optional, into)] on_select: Option<Callback<DtDate>>,
    /// Escape (`close-datepicker`).
    #[prop(optional, into)]
    on_close: Option<Callback<()>>,
    #[prop(optional)] min_date: Option<DtDate>,
    #[prop(optional)] max_date: Option<DtDate>,
    /// `0` Sunday through `6` Saturday (`weekStartsOn`). Default Sunday.
    #[prop(optional, default = 0)]
    week_starts_on: u8,
    /// `Intl` locale. Empty uses `default`.
    #[prop(optional, into)]
    locale: String,
) -> impl IntoView {
    let week_start = week_starts_on % 7;
    let max_date = match (min_date, max_date) {
        (Some(min_date), Some(max_date)) if date_ord(max_date) < date_ord(min_date) => {
            leptos::logging::warn!(
                "[DtDatepicker]: maxDate must be after or equal to minDate."
            );
            None
        }
        _ => max_date,
    };
    let root_id = unique_id("dt-datepicker");
    let heading_id = format!("{root_id}-calendar-heading");
    let nav_ids = [
        format!("{root_id}-prevYearButton"),
        format!("{root_id}-prevMonthButton"),
        format!("{root_id}-nextMonthButton"),
        format!("{root_id}-nextYearButton"),
    ];
    let names = StoredValue::new(weekday_names(&locale, week_start));
    let locale = StoredValue::new(locale);
    let heading_id = StoredValue::new(heading_id);
    let nav_ids = StoredValue::new(nav_ids);
    let internal = RwSignal::new(
        selected_date
            .map(|date| date.get_untracked())
            .unwrap_or_else(DtDate::today),
    );
    let read_selected = move || -> DtDate {
        selected_date
            .map(|date| date.get())
            .unwrap_or_else(|| internal.get())
    };
    let initial = selected_date
        .map(|date| date.get_untracked())
        .unwrap_or_else(|| internal.get_untracked());
    let view_year = RwSignal::new(initial.year);
    let view_month = RwSignal::new(initial.month);
    Effect::new(move |_| {
        let selected = read_selected();
        view_year.set(selected.year);
        view_month.set(selected.month);
    });

    let table_ref = NodeRef::<html::Table>::new();
    let pending = RwSignal::new(None::<FocusTarget>);
    let on_select = StoredValue::new(on_select);
    let on_close = StoredValue::new(on_close);

    let select_date = move |date: DtDate| {
        if selected_date.is_none() {
            internal.set(date);
        }
        if let Some(on_select) = on_select.get_value() {
            on_select.run(date);
        }
    };

    let focus_pending = move |target: FocusTarget| {
        if let FocusTarget::Nav(index) = target {
            let ids = nav_ids.get_value();
            if let Some(element) = ids.get(index as usize).and_then(|id| button_by_id(id)) {
                focus_html(&element);
            }
            return;
        }
        let Some(table) = table_ref.get() else {
            return;
        };
        let table: HtmlElement = table.unchecked_into();
        let buttons = enabled_day_buttons(&table);
        let element = match target {
            FocusTarget::First => buttons.first(),
            FocusTarget::Last => buttons.last(),
            FocusTarget::Day(day) => buttons
                .iter()
                .find(|button| button.text_content().unwrap_or_default().trim() == day.to_string()),
            FocusTarget::Nav(_) => None,
        };
        if let Some(element) = element {
            focus_html(element);
        }
    };

    Effect::new(move |_| {
        let _ = (view_year.get(), view_month.get());
        let Some(target) = pending.get() else {
            return;
        };
        request_animation_frame(move || {
            pending.set(None);
            focus_pending(target);
        });
    });

    Effect::new(move |_| {
        request_animation_frame(move || {
            let ids = nav_ids.get_value();
            if let Some(element) = ids.first().and_then(|id| button_by_id(id)) {
                focus_html(&element);
            }
        });
    });

    let shift = move |delta: i32, nav_index: u8| {
        let (year, month) =
            shift_month(view_year.get_untracked(), view_month.get_untracked(), delta);
        view_year.set(year);
        view_month.set(month);
        pending.set(Some(FocusTarget::Nav(nav_index)));
    };

    view! {
        <div class="d-stack d-datepicker d-stack--gap-100 d-stack--justify-start" data-qa="dt-stack">
            <div
                class="d-datepicker__hd"
                data-qa="dt-datepicker-header"
                on:keydown=move |event: KeyboardEvent| {
                    let key = event.key();
                    let ids = nav_ids.get_value();
                    let current = event
                        .target()
                        .and_then(|target| target.dyn_into::<Element>().ok())
                        .and_then(|element| ids.iter().position(|id| element.id() == *id));
                    if key == "Escape" {
                        if let Some(on_close) = on_close.get_value() {
                            on_close.run(());
                        }
                        return;
                    }
                    if key == "ArrowDown" || key == "Tab" {
                        event.prevent_default();
                        pending.set(Some(FocusTarget::First));
                        return;
                    }
                    let Some(current) = current else {
                        return;
                    };
                    let next = if key == "ArrowLeft" {
                        if current == 0 { 3 } else { current - 1 }
                    } else if key == "ArrowRight" {
                        if current == 3 { 0 } else { current + 1 }
                    } else {
                        return;
                    };
                    event.prevent_default();
                    if let Some(element) = ids.get(next).and_then(|id| button_by_id(id)) {
                        focus_html(&element);
                    }
                }
            >
                {move || {
                    let year = view_year.get();
                    let month = view_month.get();
                    let locale = locale.get_value();
                    let ids = nav_ids.get_value();
                    let (prev_month_year, prev_month) = shift_month(year, month, -1);
                    let (next_month_year, next_month) = shift_month(year, month, 1);
                    let month_name = format_month(year, month, &locale);
                    let prev_month_name = format_month(prev_month_year, prev_month, &locale);
                    let next_month_name = format_month(next_month_year, next_month, &locale);
                    let prev_year_label = format!("{CHANGE_TO} {PREVIOUS_YEAR} {}", year - 1);
                    let next_year_label = format!("{CHANGE_TO} {NEXT_YEAR} {}", year + 1);
                    let prev_month_label = format!("{CHANGE_TO} {PREVIOUS_MONTH} {prev_month_name}");
                    let next_month_label = format!("{CHANGE_TO} {NEXT_MONTH} {next_month_name}");
                    let prev_year_id = ids[0].clone();
                    let prev_month_id = ids[1].clone();
                    let next_month_id = ids[2].clone();
                    let next_year_id = ids[3].clone();
                    view! {
                        <div class="d-stack d-datepicker__month-year d-stack--row d-stack--gap-50 d-stack--justify-start" data-qa="dt-stack">
                            <nav class="d-stack d-datepicker__nav d-stack--row d-stack--gap-25 d-stack--justify-start" data-qa="dt-stack">
                                <DtTooltip message=PREVIOUS_YEAR placement="top" fallback_placements="top-start,auto">
                                    <DtButton
                                        id=prev_year_id
                                        class="d-datepicker__nav-btn"
                                        size="100"
                                        importance="clear"
                                        kind="muted"
                                        disabled=prev_year_disabled(year, month, min_date)
                                        aria_label=prev_year_label
                                        icon=ViewFn::from(|| view! { <DtIconChevronsLeft size="200" /> })
                                        on_click=Callback::new(move |_| shift(-12, 0))
                                    />
                                </DtTooltip>
                                <DtTooltip message=PREVIOUS_MONTH placement="top" fallback_placements="top-start,auto">
                                    <DtButton
                                        id=prev_month_id
                                        class="d-datepicker__nav-btn"
                                        size="100"
                                        importance="clear"
                                        kind="muted"
                                        disabled=prev_month_disabled(year, month, min_date)
                                        aria_label=prev_month_label
                                        icon=ViewFn::from(|| view! { <DtIconChevronLeft size="200" /> })
                                        on_click=Callback::new(move |_| shift(-1, 1))
                                    />
                                </DtTooltip>
                            </nav>
                            <div id=heading_id.get_value() class="d-datepicker__month-year-title" data-qa="dt-datepicker-heading">
                                {month_name} " " {year.to_string()}
                            </div>
                            <nav class="d-stack d-datepicker__nav d-stack--row d-stack--gap-25 d-stack--justify-start" data-qa="dt-stack">
                                <DtTooltip message=NEXT_MONTH placement="top" fallback_placements="top-end,auto">
                                    <DtButton
                                        id=next_month_id
                                        class="d-datepicker__nav-btn"
                                        size="100"
                                        importance="clear"
                                        kind="muted"
                                        disabled=next_month_disabled(year, month, max_date)
                                        aria_label=next_month_label
                                        icon=ViewFn::from(|| view! { <DtIconChevronRight size="200" /> })
                                        on_click=Callback::new(move |_| shift(1, 2))
                                    />
                                </DtTooltip>
                                <DtTooltip message=NEXT_YEAR placement="top" fallback_placements="top-end,auto">
                                    <DtButton
                                        id=next_year_id
                                        class="d-datepicker__nav-btn"
                                        size="100"
                                        importance="clear"
                                        kind="muted"
                                        disabled=next_year_disabled(year, month, max_date)
                                        aria_label=next_year_label
                                        icon=ViewFn::from(|| view! { <DtIconChevronsRight size="200" /> })
                                        on_click=Callback::new(move |_| shift(12, 3))
                                    />
                                </DtTooltip>
                            </nav>
                        </div>
                    }
                }}
            </div>
            <div
                class="d-datepicker__bd"
                data-qa="dt-datepicker-body"
                on:keydown=move |event: KeyboardEvent| {
                    let key = event.key();
                    if key == "Escape" {
                        if let Some(on_close) = on_close.get_value() {
                            on_close.run(());
                        }
                        return;
                    }
                    if key == "Tab" {
                        event.prevent_default();
                        pending.set(Some(FocusTarget::Nav(0)));
                        return;
                    }
                    let Some(table) = table_ref.get() else {
                        return;
                    };
                    let table: HtmlElement = table.unchecked_into();
                    let buttons = enabled_day_buttons(&table);
                    if buttons.is_empty() {
                        return;
                    }
                    let target_id = event
                        .target()
                        .and_then(|target| target.dyn_into::<Element>().ok())
                        .map(|element| element.id())
                        .unwrap_or_default();
                    let current = buttons.iter().position(|button| button.id() == target_id);
                    let Some(current) = current else {
                        return;
                    };
                    let year = view_year.get_untracked();
                    let month = view_month.get_untracked();
                    let text = buttons[current]
                        .text_content()
                        .unwrap_or_default()
                        .trim()
                        .parse::<u8>()
                        .unwrap_or(1);
                    let current_date = DtDate { year, month, day: text };
                    match key.as_str() {
                        "ArrowLeft" => {
                            event.prevent_default();
                            if current == 0 {
                                shift(-1, 1);
                                pending.set(Some(FocusTarget::Last));
                            } else if let Some(button) = buttons.get(current - 1) {
                                focus_html(button);
                            }
                        }
                        "ArrowRight" => {
                            event.prevent_default();
                            if current + 1 >= buttons.len() {
                                shift(1, 2);
                                pending.set(Some(FocusTarget::First));
                            } else if let Some(button) = buttons.get(current + 1) {
                                focus_html(button);
                            }
                        }
                        "ArrowUp" => {
                            event.prevent_default();
                            if current < 7 {
                                let day = prev_focus_day(current_date);
                                shift(-1, 1);
                                pending.set(Some(FocusTarget::Day(day)));
                            } else if let Some(button) = buttons.get(current - 7) {
                                focus_html(button);
                            }
                        }
                        "ArrowDown" => {
                            event.prevent_default();
                            if current + 7 >= buttons.len() {
                                let day = next_focus_day(current_date);
                                shift(1, 2);
                                pending.set(Some(FocusTarget::Day(day)));
                            } else if let Some(button) = buttons.get(current + 7) {
                                focus_html(button);
                            }
                        }
                        _ => {}
                    }
                }
            >
                {move || {
                    let root_id = root_id.clone();
                    let selected = read_selected();
                    let year = view_year.get();
                    let month = view_month.get();
                    let highlight = (selected.year == year && selected.month == month).then_some(selected.day);
                    let weeks = calendar_weeks(year, month, highlight, min_date, max_date, week_start);
                    let names = names.get_value();
                    let heading = heading_id.get_value();
                    let locale_name = locale.get_value();
                    view! {
                        <table
                            node_ref=table_ref
                            class="d-datepicker__calendar"
                            aria-labelledby=heading
                            data-qa="dt-datepicker-calendar"
                        >
                            <thead>
                                <tr>
                                    {names.into_iter().map(|name| {
                                        let title = name.clone();
                                        let label = name.clone();
                                        view! {
                                            <th scope="col" class="d-datepicker__cell d-datepicker__cell--header">
                                                <span class="d-datepicker__weekday" data-qa="dt-datepicker-weekday" title=title aria-label=label>
                                                    {name}
                                                </span>
                                            </th>
                                        }
                                    }).collect_view()}
                                </tr>
                            </thead>
                            <tbody>
                                {weeks.into_iter().enumerate().map(|(week_index, week)| view! {
                                    <tr data-qa="dt-datepicker-week">
                                        {week.into_iter().enumerate().map(|(day_index, day)| {
                                            let day_id = format!("{root_id}-day-{}", week_index * 7 + day_index);
                                            let class_name = join_class([
                                                "d-datepicker__day",
                                                day.disabled.then_some("d-datepicker__day--disabled").unwrap_or(""),
                                                day.selected.then_some("d-datepicker__day--selected").unwrap_or(""),
                                            ]);
                                            let label = format!(
                                                "{SELECT_DAY} {}",
                                                format_date_style(day.date, &locale_name, "long")
                                            );
                                            let day_number = day.date.day.to_string();
                                            view! {
                                                <td class="d-datepicker__cell" role="listbox">
                                                    <DtButton
                                                        id=day_id
                                                        class=class_name
                                                        size="200"
                                                        kind="muted"
                                                        importance="clear"
                                                        disabled=day.disabled
                                                        aria_selected=day.selected
                                                        aria_label=label
                                                        role="option"
                                                        on_click=Callback::new(move |_| {
                                                            if day.disabled {
                                                                return;
                                                            }
                                                            select_date(day.date);
                                                            pending.set(Some(FocusTarget::Day(day.date.day)));
                                                        })
                                                    >
                                                        {day_number}
                                                    </DtButton>
                                                </td>
                                            }
                                        }).collect_view()}
                                    </tr>
                                }).collect_view()}
                            </tbody>
                        </table>
                    }
                }}
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_and_known_weekdays() {
        assert_eq!(days_from_civil(1970, 1, 1), 0);
        assert_eq!(weekday_index(DtDate::new(1970, 1, 1)), 4);
        assert_eq!(weekday_index(DtDate::new(2017, 1, 1)), 0);
        assert_eq!(weekday_index(DtDate::new(2024, 3, 1)), 5);
        assert_eq!(days_in_month(2024, 2), 29);
        assert_eq!(days_in_month(2023, 2), 28);
    }

    #[test]
    fn march_2024_sunday_grid_includes_leading_and_trailing_days() {
        let weeks = calendar_weeks(2024, 3, Some(15), None, None, 0);
        assert_eq!(weeks.len(), 6);
        assert_eq!(weeks[0][0].date, DtDate::new(2024, 2, 25));
        assert!(!weeks[0][0].current_month);
        assert!(weeks[0][0].disabled);
        assert_eq!(weeks[0][5].date, DtDate::new(2024, 3, 1));
        assert!(weeks[0][5].first_of_month);
        assert!(weeks
            .iter()
            .flatten()
            .any(|day| day.date == DtDate::new(2024, 3, 15) && day.selected));
        assert!(weeks
            .iter()
            .flatten()
            .any(|day| day.date == DtDate::new(2024, 4, 6) && day.disabled));
    }

    #[test]
    fn monday_start_and_min_date_disable_earlier_days() {
        let weeks = calendar_weeks(2024, 3, None, Some(DtDate::new(2024, 3, 10)), None, 1);
        assert_eq!(weeks[0][0].date, DtDate::new(2024, 2, 26));
        let march_9 = weeks
            .iter()
            .flatten()
            .find(|day| day.date == DtDate::new(2024, 3, 9))
            .unwrap();
        let march_10 = weeks
            .iter()
            .flatten()
            .find(|day| day.date == DtDate::new(2024, 3, 10))
            .unwrap();
        assert!(march_9.disabled);
        assert!(!march_10.disabled);
    }

    #[test]
    fn weekday_rotation_matches_vue_slice() {
        let names: Vec<String> = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"]
            .into_iter()
            .map(str::to_string)
            .collect();
        assert_eq!(rotate_weekday_names(&names, 0), names);
        assert_eq!(
            rotate_weekday_names(&names, 1),
            ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"]
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn focus_days_follow_the_same_weekday() {
        let wednesday = DtDate::new(2024, 3, 13);
        assert_eq!(next_focus_day(wednesday), 3);
        assert_eq!(prev_focus_day(wednesday), 28);
    }

    #[test]
    fn month_navigation_wraps_and_honors_min_max() {
        assert_eq!(shift_month(2024, 1, -1), (2023, 12));
        assert_eq!(shift_month(2024, 12, 1), (2025, 1));
        let min = Some(DtDate::new(2024, 3, 1));
        assert!(prev_month_disabled(2024, 3, min));
        assert!(prev_year_disabled(2024, 3, min));
        assert!(!next_month_disabled(2024, 3, None));
        let max = Some(DtDate::new(2024, 3, 31));
        assert!(next_month_disabled(2024, 3, max));
        assert!(next_year_disabled(2024, 3, max));
    }
}
