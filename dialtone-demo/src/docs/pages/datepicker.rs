use super::*;

fn format_date(date: DtDate) -> String {
    format!("{:04}-{:02}-{:02}", date.year, date.month, date.day)
}

fn parse_date(value: &str) -> Option<DtDate> {
    let mut parts = value.split('-');
    let year = parts.next()?.parse::<i32>().ok()?;
    let month = parts.next()?.parse::<u8>().ok()?;
    let day = parts.next()?.parse::<u8>().ok()?;
    if parts.next().is_some() || !(1..=12).contains(&month) || day == 0 {
        return None;
    }
    Some(DtDate::new(year, month, day))
}

fn week_start(value: &str) -> u8 {
    value.parse::<u8>().unwrap_or(0).min(6)
}

#[component]
pub(crate) fn DatepickerPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("datepicker.rs")));
    let today = DtDate::today();
    let selected = RwSignal::new(today);
    let selected_text = RwSignal::new(format_date(today));
    let min_text = RwSignal::new(String::new());
    let max_text = RwSignal::new(String::new());
    let week = RwSignal::new("0");
    let locale = RwSignal::new("");
    Effect::new(move |_| {
        if let Some(date) = parse_date(&selected_text.get()) {
            if selected.get_untracked() != date {
                selected.set(date);
            }
        }
    });
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let week = week_start(week.get());
                let locale = locale.get();
                let min_date = parse_date(&min_text.get());
                let max_date = parse_date(&max_text.get());
                let on_select = move |date: DtDate| {
                    selected.set(date);
                    selected_text.set(format_date(date));
                };
                let calendar = match (min_date, max_date) {
                    (Some(min_date), Some(max_date)) => view! {
                        <DtDatepicker
                            selected_date=Signal::derive(move || selected.get())
                            on_select=on_select
                            week_starts_on=week
                            locale=locale
                            min_date=min_date
                            max_date=max_date
                        />
                    }.into_any(),
                    (Some(min_date), None) => view! {
                        <DtDatepicker
                            selected_date=Signal::derive(move || selected.get())
                            on_select=on_select
                            week_starts_on=week
                            locale=locale
                            min_date=min_date
                        />
                    }.into_any(),
                    (None, Some(max_date)) => view! {
                        <DtDatepicker
                            selected_date=Signal::derive(move || selected.get())
                            on_select=on_select
                            week_starts_on=week
                            locale=locale
                            max_date=max_date
                        />
                    }.into_any(),
                    (None, None) => view! {
                        <DtDatepicker
                            selected_date=Signal::derive(move || selected.get())
                            on_select=on_select
                            week_starts_on=week
                            locale=locale
                        />
                    }.into_any(),
                };
                view! {
                    <div class="docs-stack">
                        {calendar}
                        <p class="docs-readout">
                            {move || {
                                let date = selected.get();
                                format!("Selected {:04}-{:02}-{:02}", date.year, date.month, date.day)
                            }}
                        </p>
                    </div>
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingSelect
                label="Week starts"
                value=week
                options=vec![
                    ("0", "Sunday"),
                    ("1", "Monday"),
                    ("2", "Tuesday"),
                    ("3", "Wednesday"),
                    ("4", "Thursday"),
                    ("5", "Friday"),
                    ("6", "Saturday"),
                ]
            />
            <SettingSelect
                label="Locale"
                value=locale
                options=vec![("", "Default"), ("en-US", "en-US"), ("fr", "fr"), ("de", "de"), ("ja", "ja")]
            />
            <SettingText label="Selected date" hint="YYYY-MM-DD." value=selected_text />
            <SettingText label="Min date" hint="YYYY-MM-DD. Empty leaves it unset." value=min_text />
            <SettingText label="Max date" hint="YYYY-MM-DD. Empty leaves it unset." value=max_text />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_expr(&mut props, "selected_date", "selected");
        push_expr(&mut props, "on_select", "move |date| selected.set(date)");
        push_expr(
            &mut props,
            "week_starts_on",
            week_start(week.get()).to_string(),
        );
        push_str(&mut props, "locale", locale.get());
        if let Some(date) = parse_date(&min_text.get()) {
            push_expr(
                &mut props,
                "min_date",
                format!("DtDate::new({}, {}, {})", date.year, date.month, date.day),
            );
        }
        if let Some(date) = parse_date(&max_text.get()) {
            push_expr(
                &mut props,
                "max_date",
                format!("DtDate::new({}, {}, {})", date.year, date.month, date.day),
            );
        }
        let date = selected.get();
        format!(
            "// selected {:04}-{:02}-{:02}\n{}",
            date.year,
            date.month,
            date.day,
            view_of(&element("DtDatepicker", &props, None))
        )
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"The calendar is inline. Choosing a day writes the selected date signal."</p>
            <Play stack=true preview=preview settings=settings code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Monday">
                    <MondayCalendar />
                </Variant>
            </div>
        </section>
    }
}

#[component]
fn MondayCalendar() -> impl IntoView {
    let selected = RwSignal::new(DtDate::today());
    view! {
        <DtDatepicker
            selected_date=Signal::derive(move || selected.get())
            on_select=move |date| selected.set(date)
            week_starts_on=1
        />
    }
}
