use leptos::prelude::*;

use crate::unique_id;

/// Vue `strokeWidth`. The radius is `12 - strokeWidth / 2` inside a 24 view box.
const STROKE_WIDTH: f64 = 3.25;
/// Values above 0 and below this draw the same arc as this value. `aria-valuenow` stays real.
const MIN_VISUAL_PROGRESS: f64 = 5.0;

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn css_num(value: f64) -> String {
    if value == 0.0 {
        return "0".to_string();
    }
    let negative = value < 0.0;
    let text = format!("{:.12}", value.abs());
    let text = text.trim_end_matches('0').trim_end_matches('.');
    if negative {
        format!("-{text}")
    } else {
        text.to_string()
    }
}

fn circle_radius() -> f64 {
    12.0 - (STROKE_WIDTH / 2.0)
}

fn circle_circumference() -> f64 {
    2.0 * std::f64::consts::PI * circle_radius()
}

fn circle_path() -> String {
    let radius = circle_radius();
    let top = 12.0 - radius;
    let bottom = 12.0 + radius;
    format!(
        "M 12 {} A {} {} 0 0 1 12 {} A {} {} 0 0 1 12 {} Z",
        css_num(top),
        css_num(radius),
        css_num(radius),
        css_num(bottom),
        css_num(radius),
        css_num(radius),
        css_num(top)
    )
}

struct CirclePaint {
    style: String,
    show_track: bool,
    show_fill: bool,
}

/// Dash variables from `ProgressCircle.vue` `cssVars`.
fn circle_paint(progress: f64) -> CirclePaint {
    let circumference = circle_circumference();
    let visual = if progress > 0.0 && progress < MIN_VISUAL_PROGRESS {
        MIN_VISUAL_PROGRESS
    } else {
        progress
    };
    let fill_length = circumference * visual / 100.0;
    let cap_arc = STROKE_WIDTH / 2.0;
    let both = progress > 0.0 && progress < 100.0;
    let adj = if both { STROKE_WIDTH } else { 0.0 };
    let stroke_dasharray = if !both && progress >= 100.0 {
        "none".to_string()
    } else {
        css_num(circumference)
    };
    let fill_dashoffset = circumference - (fill_length - adj).max(0.0);
    let fill_rotate = if both {
        cap_arc / circumference * 360.0
    } else {
        0.0
    };
    let track_dasharray = if !both && progress <= 0.0 {
        "none".to_string()
    } else {
        format!(
            "{} {}",
            css_num((circumference - fill_length - adj).max(0.0)),
            css_num(circumference)
        )
    };
    let track_dashoffset = -(fill_length + if both { cap_arc } else { 0.0 });
    let style = format!(
        "--stroke-dasharray:{stroke_dasharray}; --fill-dashoffset:{}; --fill-rotate:{}deg; --track-dasharray:{track_dasharray}; --track-dashoffset:{}",
        css_num(fill_dashoffset),
        css_num(fill_rotate),
        css_num(track_dashoffset)
    );
    CirclePaint {
        style,
        show_track: progress <= 95.0,
        show_fill: progress > 0.0,
    }
}

fn size_class(size: &str) -> &'static str {
    match size {
        "100" => "d-progress-circle--size-100",
        "200" => "d-progress-circle--size-200",
        "300" => "d-progress-circle--size-300",
        "400" => "d-progress-circle--size-400",
        "500" => "d-progress-circle--size-500",
        "600" => "d-progress-circle--size-600",
        "700" => "d-progress-circle--size-700",
        "800" => "d-progress-circle--size-800",
        _ => {
            leptos::logging::warn!(
                "[DtProgressCircle] `size` must be one of 100, 200, 300, 400, 500, 600, 700, 800. Received {size}."
            );
            ""
        }
    }
}

fn kind_class(kind: &str) -> &'static str {
    match kind {
        "default" => "",
        "brand" => "d-progress-circle--brand",
        "critical" => "d-progress-circle--critical",
        "positive" => "d-progress-circle--positive",
        "warning" => "d-progress-circle--warning",
        "info" => "d-progress-circle--info",
        "ai" => "d-progress-circle--ai",
        _ => {
            leptos::logging::warn!(
                "[DtProgressCircle] `kind` must be one of default, brand, critical, positive, warning, info, ai. Received {kind}."
            );
            ""
        }
    }
}

fn progress_root_class(size: &str, kind: &str, class: &str) -> String {
    join_class([
        "d-progress-circle",
        size_class(size),
        kind_class(kind),
        class,
    ])
}

fn fill_stroke(kind: &str, gradient_id: &str) -> Option<String> {
    (kind == "ai").then(|| format!("stroke: url(#{gradient_id})"))
}

/// Dialtone Vue `DtProgressCircle`.
///
/// `aria_label` is `ariaLabel` and is required. `progress` is 0–100 and
/// defaults to 0. Values from 1 to 4 draw the same arc as 5. `size` is
/// `100`–`800` (default `500`). `kind` is `default`, `brand`, `critical`,
/// `positive`, `warning`, `info`, or `ai`. `class` is `$attrs.class` on the root.
#[component]
pub fn DtProgressCircle(
    /// Accessible name (`ariaLabel`).
    #[prop(into)]
    aria_label: String,
    /// Current value, 0–100 (`progress`).
    #[prop(into, default = Signal::stored(0.0))]
    progress: Signal<f64>,
    /// Icon size token (`size`). Default `500`.
    #[prop(optional, default = "500")]
    size: &'static str,
    /// Color (`kind`). Default `default`.
    #[prop(optional, default = "default")]
    kind: &'static str,
    /// Extra class on the root (`$attrs.class`).
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    if aria_label.is_empty() {
        leptos::logging::warn!("[DtProgressCircle] `ariaLabel` is required.");
    }
    Effect::new(move |_| {
        let value = progress.get();
        if !value.is_finite() || !(0.0..=100.0).contains(&value) {
            leptos::logging::warn!(
                "[DtProgressCircle] `progress` must be between 0 and 100. Received {value}."
            );
        }
    });
    let root = progress_root_class(size, kind, &class);
    let track_path = circle_path();
    let fill_path = track_path.clone();
    let gradient_id = unique_id("progress-circle-ai-gradient");
    let fill_style = fill_stroke(kind, &gradient_id);
    let ai = kind == "ai";
    view! {
        <div
            class=root
            role="progressbar"
            aria-label=aria_label
            tabindex="-1"
            aria-valuenow=move || css_num(progress.get())
            aria-valuemin="0"
            aria-valuemax="100"
            data-qa="dt-progress-circle"
        >
            <svg
                class="d-progress-circle__bar"
                style=move || circle_paint(progress.get()).style
                viewBox="0 0 24 24"
            >
                {ai.then(|| {
                    let gradient_id = gradient_id.clone();
                    view! {
                        <defs>
                            <linearGradient id=gradient_id x1="0%" y1="0%" x2="100%" y2="100%">
                                <stop offset="0%" stop-color="var(--dt-color-brand-gold)" />
                                <stop offset="33%" stop-color="var(--dt-color-brand-red)" />
                                <stop offset="66%" stop-color="var(--dt-color-brand-magenta)" />
                                <stop offset="100%" stop-color="var(--dt-color-brand-purple)" />
                            </linearGradient>
                        </defs>
                    }
                })}
                // `Show` keeps the path mounted while `progress` changes so the stroke transition runs.
                <Show when=move || circle_paint(progress.get()).show_track>
                    <path
                        d=track_path.clone()
                        class="d-progress-circle__shape d-progress-circle__shape--track"
                        fill="none"
                        stroke-width="3.25"
                        stroke-linecap="round"
                    />
                </Show>
                <Show when=move || circle_paint(progress.get()).show_fill>
                    <path
                        d=fill_path.clone()
                        class="d-progress-circle__shape d-progress-circle__shape--fill"
                        fill="none"
                        stroke-width="3.25"
                        stroke-linecap="round"
                        style=fill_style.clone()
                    />
                </Show>
            </svg>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_is_two_arcs_around_the_24_box() {
        assert_eq!(
            circle_path(),
            "M 12 1.625 A 10.375 10.375 0 0 1 12 22.375 A 10.375 10.375 0 0 1 12 1.625 Z"
        );
    }

    #[test]
    fn small_progress_matches_the_visual_floor() {
        assert_eq!(circle_paint(2.0).style, circle_paint(5.0).style);
        assert!(circle_paint(2.0).show_fill);
        assert!(circle_paint(2.0).show_track);
    }

    #[test]
    fn ends_hide_the_unused_arc() {
        let empty = circle_paint(0.0);
        assert!(!empty.show_fill);
        assert!(empty.show_track);
        assert!(empty.style.contains("--track-dasharray:none"));
        assert!(!empty.style.contains("--stroke-dasharray:none"));

        let full = circle_paint(100.0);
        assert!(full.show_fill);
        assert!(!full.show_track);
        assert!(full.style.contains("--stroke-dasharray:none"));

        let almost = circle_paint(96.0);
        assert!(almost.show_fill);
        assert!(!almost.show_track);
    }

    #[test]
    fn partial_progress_rotates_the_fill_for_the_round_caps() {
        let paint = circle_paint(50.0);
        assert!(paint.show_fill);
        assert!(paint.show_track);
        assert!(!paint.style.contains("--fill-rotate:0deg"));
    }

    #[test]
    fn classes_follow_the_vue_array() {
        assert_eq!(
            progress_root_class("500", "default", ""),
            "d-progress-circle d-progress-circle--size-500"
        );
        assert_eq!(
            progress_root_class("200", "brand", "extra"),
            "d-progress-circle d-progress-circle--size-200 d-progress-circle--brand extra"
        );
        assert_eq!(progress_root_class("nope", "nope", ""), "d-progress-circle");
    }

    #[test]
    fn ai_kind_paints_the_gradient_stroke() {
        assert_eq!(
            fill_stroke("ai", "progress-circle-ai-gradient-1").as_deref(),
            Some("stroke: url(#progress-circle-ai-gradient-1)")
        );
        assert_eq!(fill_stroke("brand", "progress-circle-ai-gradient-1"), None);
    }
}
