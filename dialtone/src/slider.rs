//! Dialtone Vue `DtSlider` from tag `dialtone/v10.11.0`
//! (`4b56a42e5c5eeead57181a961060dd2ea8eaa053`).
//!
//! The Vue sources match staging `84923549`. The component is not in tag
//! `dialtone/v10.7.0`. See [`DtSlider`].

use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};

use super::popper::{compute_position, Alignment, Options as PopperOptions, Placement, Rect, Side};
use super::text::DtText;
use super::unique_id;

const MAX_GENERATED_POINTS: usize = 1000;
const SNAP_THRESHOLD_PX: f64 = 10.0;
const SNAP_RELEASE_MULTIPLIER: f64 = 2.0;
const COLLISION_PADDING: f64 = 4.0;
const EDGE_CLAMP_TOLERANCE_PX: f64 = 32.0;
const READOUT_EDGE_CLAMP_TOLERANCE_PX: f64 = 0.0;
const EDGE_INSET: &str = "var(--slider-edge-inset)";

const RANGE_TEXT_WARNING: &str = "[Dialtone] DtSlider in range mode: provide getValueText to give each thumb a distinct screen-reader description.";
const NAME_WARNING: &str = "[Dialtone] DtSlider: provide a label prop (set showLabel to false to hide it visually) or aria-label for accessibility.";

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn warn_slider(message: &str) {
    leptos::logging::warn!("{message}");
}

/// Controlled slider value. `Single` is one thumb. `Range` is two (`modelValue`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DtSliderValue {
    Single(f64),
    Range(f64, f64),
}

/// `fillOrigin`. `At` is the numeric origin. It is clamped to `min`/`max` and ignored in range mode.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum DtSliderFill {
    #[default]
    Start,
    End,
    At(f64),
}

/// `readout`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DtSliderReadout {
    #[default]
    Tooltip,
    Always,
    Never,
}

/// `showTicks`. `EveryStep` is Vue `true`. `Interval` is a number.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum DtSliderTicks {
    #[default]
    Off,
    EveryStep,
    Interval(f64),
}

/// One `showMarks` entry. `text` is the optional label. `None` uses prefix and suffix.
#[derive(Clone, Debug, PartialEq)]
pub struct DtSliderMark {
    pub value: f64,
    pub text: Option<String>,
}

/// `showMarks`. `Ends` is Vue `true` (`[min, max]`).
#[derive(Clone, Debug, PartialEq, Default)]
pub enum DtSliderMarks {
    #[default]
    Off,
    Ends,
    At(Vec<DtSliderMark>),
}

/// `snapPoints`. Pointer drags only.
#[derive(Clone, Debug, PartialEq, Default)]
pub enum DtSliderSnaps {
    #[default]
    Off,
    Interval(f64),
    Points(Vec<f64>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum MarkEdge {
    Start,
    End,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Bounds {
    min: f64,
    max: f64,
    step: f64,
    gap: f64,
}

impl Bounds {
    fn new(min: f64, max: f64, step: f64, gap: f64) -> Self {
        Self {
            min: if min.is_finite() { min } else { 0.0 },
            max: if max.is_finite() { max } else { 100.0 },
            step: if step.is_finite() { step } else { 1.0 },
            gap: if gap.is_finite() { gap } else { 0.0 },
        }
    }

    fn midpoint(self) -> f64 {
        (self.min + self.max) / 2.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct BoxRect {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
}

#[derive(Clone, Copy, Debug)]
enum TrackSize {
    Missing,
    Zero,
    Px(f64),
}

#[derive(Clone, Debug)]
struct ThumbWrite {
    values: Vec<f64>,
    snaps: Vec<Option<f64>>,
    changed: bool,
    native: f64,
}

#[derive(Clone, Debug)]
struct SyncOut {
    values: Vec<f64>,
    committed: Vec<f64>,
    emit_input: Option<DtSliderValue>,
    emit_change: Option<DtSliderValue>,
}

struct MarkMeasure {
    index: usize,
    natural_left: f64,
    natural_right: f64,
    top: f64,
    bottom: f64,
}

/// JavaScript `String(number)`. Rust `Display` uses a different exponent cutoff.
fn js_num(n: f64) -> String {
    if n.is_nan() {
        return "NaN".to_string();
    }
    if n == 0.0 {
        return "0".to_string();
    }
    if n.is_infinite() {
        return if n.is_sign_negative() {
            "-Infinity".to_string()
        } else {
            "Infinity".to_string()
        };
    }
    let neg = n.is_sign_negative();
    let body = decimal_to_js(&format!("{}", n.abs()));
    if neg {
        format!("-{body}")
    } else {
        body
    }
}

fn decimal_to_js(dec: &str) -> String {
    if let Some((mantissa, exp)) = split_scientific(dec) {
        return format_scientific(&mantissa, exp);
    }
    let exp = scientific_exponent(dec);
    if exp < -6 || exp >= 21 {
        let digits: String = dec
            .chars()
            .filter(|c| c.is_ascii_digit())
            .skip_while(|c| *c == '0')
            .collect();
        let digits = digits.trim_end_matches('0');
        format_scientific(digits, exp)
    } else {
        dec.to_string()
    }
}

fn split_scientific(dec: &str) -> Option<(String, i32)> {
    let e = dec.find(['e', 'E'])?;
    let mantissa = dec[..e].trim_end_matches('0').trim_end_matches('.');
    let exp: i32 = dec[e + 1..].parse().ok()?;
    Some((mantissa.to_string(), exp))
}

fn scientific_exponent(dec: &str) -> i32 {
    if let Some(rest) = dec.strip_prefix("0.") {
        let zeros = rest.chars().take_while(|c| *c == '0').count();
        -((zeros as i32) + 1)
    } else {
        let int_digits = dec.find('.').unwrap_or(dec.len());
        (int_digits as i32) - 1
    }
}

fn format_scientific(digits: &str, exp: i32) -> String {
    let digits = if digits.is_empty() { "0" } else { digits };
    let mut chars = digits.chars();
    let first = chars.next().unwrap_or('0');
    let rest: String = chars.collect();
    let mantissa = if rest.is_empty() {
        first.to_string()
    } else {
        format!("{first}.{rest}")
    };
    if exp >= 0 {
        format!("{mantissa}e+{exp}")
    } else {
        format!("{mantissa}e{exp}")
    }
}

/// JavaScript `Math.round` (half toward +∞).
fn js_round(n: f64) -> f64 {
    if !n.is_finite() {
        return n;
    }
    (n + 0.5).floor()
}

/// JavaScript `Number#toFixed`. Half away from zero, via the absolute value.
fn js_to_fixed(n: f64, digits: i32) -> String {
    if !n.is_finite() {
        return js_num(n);
    }
    if n.abs() >= 1e21 {
        return js_num(n);
    }
    let neg = n.is_sign_negative() && n != 0.0;
    let digits = digits.max(0);
    let factor = 10f64.powi(digits);
    let rounded = js_round(n.abs() * factor);
    let mut body = format!("{rounded:.0}");
    if body == "-0" {
        body = "0".to_string();
    }
    if digits > 0 {
        let width = digits as usize;
        if body.len() <= width {
            body = format!("{body:0>width$}", width = width + 1);
        }
        let split = body.len() - width;
        body.insert(split, '.');
    }
    if neg {
        format!("-{body}")
    } else {
        body
    }
}

fn js_to_fixed_num(n: f64, digits: i32) -> f64 {
    js_to_fixed(n, digits).parse().unwrap_or(n)
}

fn decimal_places(n: f64) -> i32 {
    let str = js_num(n);
    if let Some(e_index) = str.find(['e', 'E']) {
        let mantissa = &str[..e_index];
        let exponent: i32 = str[e_index + 1..].parse().unwrap_or(0);
        let mantissa_decimals = mantissa
            .find('.')
            .map(|dot| mantissa.len() - dot - 1)
            .unwrap_or(0);
        (mantissa_decimals as i32 - exponent).max(0)
    } else {
        str.find('.')
            .map(|dot| (str.len() - dot - 1) as i32)
            .unwrap_or(0)
    }
}

fn clamp_to_range(val: f64, bounds: &Bounds) -> f64 {
    bounds.max.min(bounds.min.max(val))
}

fn snap_to_step(val: f64, bounds: &Bounds) -> f64 {
    if bounds.step <= 0.0 {
        return clamp_to_range(val, bounds);
    }
    let steps = js_round((val - bounds.min) / bounds.step);
    let dp = decimal_places(bounds.min).max(decimal_places(bounds.step));
    clamp_to_range(
        js_to_fixed_num(bounds.min + steps * bounds.step, dp),
        bounds,
    )
}

fn normalize_range(values: Vec<f64>) -> Vec<f64> {
    if values.len() == 2 && values[0] > values[1] {
        vec![values[1], values[0]]
    } else {
        values
    }
}

fn enforce_range_gap(values: Vec<f64>, bounds: &Bounds, held: Option<usize>) -> Vec<f64> {
    if values.len() != 2 {
        return values;
    }
    let gap = js_round(bounds.gap) * bounds.step;
    if gap <= 0.0 {
        return values;
    }
    let mut lo = values[0];
    let mut hi = values[1];
    if hi - lo >= gap {
        return values;
    }
    if held == Some(0) {
        hi = bounds.max.min(lo + gap);
    } else if held == Some(1) {
        lo = bounds.min.max(hi - gap);
    } else if lo + gap <= bounds.max {
        hi = lo + gap;
    } else if hi - gap >= bounds.min {
        lo = hi - gap;
    } else {
        hi = bounds.max;
        lo = bounds.min.max(hi - gap);
    }
    let held_dp = held
        .and_then(|index| values.get(index).copied())
        .map(decimal_places)
        .unwrap_or(0);
    let dp = decimal_places(bounds.min)
        .max(decimal_places(bounds.max))
        .max(decimal_places(bounds.step))
        .max(held_dp);
    if held != Some(0) {
        lo = clamp_to_range(js_to_fixed_num(lo, dp), bounds);
    }
    if held != Some(1) {
        hi = clamp_to_range(js_to_fixed_num(hi, dp), bounds);
    }
    vec![lo, hi]
}

fn apply_value_constraints(values: &[f64], bounds: &Bounds, held: Option<usize>) -> Vec<f64> {
    let mut out: Vec<f64> = values
        .iter()
        .enumerate()
        .map(|(index, value)| {
            if Some(index) == held {
                *value
            } else if value.is_finite() {
                *value
            } else {
                bounds.midpoint()
            }
        })
        .collect();
    out = out
        .iter()
        .enumerate()
        .map(|(index, value)| {
            if Some(index) == held {
                *value
            } else {
                clamp_to_range(*value, bounds)
            }
        })
        .collect();
    out = out
        .iter()
        .enumerate()
        .map(|(index, value)| {
            if Some(index) == held {
                *value
            } else {
                snap_to_step(*value, bounds)
            }
        })
        .collect();
    if out.len() == 2 && held.is_none() {
        out = normalize_range(out);
    }
    enforce_range_gap(out, bounds, held)
}

fn numbers_of(value: DtSliderValue) -> Vec<f64> {
    match value {
        DtSliderValue::Single(value) => vec![value],
        DtSliderValue::Range(lo, hi) => vec![lo, hi],
    }
}

fn is_range_value(value: DtSliderValue) -> bool {
    matches!(value, DtSliderValue::Range(_, _))
}

fn payload(values: &[f64], range: bool) -> DtSliderValue {
    if range {
        DtSliderValue::Range(
            values.first().copied().unwrap_or(0.0),
            values.get(1).copied().unwrap_or(0.0),
        )
    } else {
        DtSliderValue::Single(values.first().copied().unwrap_or(0.0))
    }
}

fn different(left: &[f64], right: &[f64]) -> bool {
    left.len() != right.len() || left.iter().zip(right).any(|(a, b)| a != b)
}

fn thumb_percent(val: f64, bounds: &Bounds) -> f64 {
    if bounds.max == bounds.min {
        0.0
    } else {
        ((val - bounds.min) / (bounds.max - bounds.min)) * 100.0
    }
}

fn to_grid_bound(val: f64, bounds: &Bounds, round_up: bool) -> f64 {
    if bounds.step <= 0.0 {
        return val;
    }
    let steps = (val - bounds.min) / bounds.step;
    let rounded = if round_up {
        (steps - 1e-9).ceil()
    } else {
        (steps + 1e-9).floor()
    };
    bounds.min + rounded * bounds.step
}

fn thumb_native_min(index: usize, range: bool, values: &[f64], bounds: &Bounds) -> f64 {
    if !range || index != 1 || values.len() < 2 {
        return bounds.min;
    }
    let gap = js_round(bounds.gap) * bounds.step;
    let raw = bounds.max.min(values[0] + gap);
    bounds.max.min(to_grid_bound(raw, bounds, true))
}

fn thumb_native_max(index: usize, range: bool, values: &[f64], bounds: &Bounds) -> f64 {
    if !range || index != 0 || values.len() < 2 {
        return bounds.max;
    }
    let gap = js_round(bounds.gap) * bounds.step;
    let raw = bounds.min.max(values[1] - gap);
    bounds.min.max(to_grid_bound(raw, bounds, false))
}

fn is_off_grid(val: f64, min: f64, step: f64) -> bool {
    if step <= 0.0 {
        return false;
    }
    let steps = (val - min) / step;
    (steps - js_round(steps)).abs() > 1e-9
}

fn thumb_native_step(index: usize, values: &[f64], bounds: &Bounds) -> String {
    if bounds.step <= 0.0 {
        return "any".to_string();
    }
    let Some(&val) = values.get(index) else {
        return js_num(bounds.step);
    };
    if is_off_grid(val, bounds.min, bounds.step) {
        "any".to_string()
    } else {
        js_num(bounds.step)
    }
}

fn format_value(
    value: f64,
    _index: usize,
    prefix: &str,
    suffix: &str,
    text: Option<Result<String, String>>,
    warned: &mut bool,
) -> String {
    if let Some(text) = text {
        match text {
            Ok(text) => return text,
            Err(error) => {
                if !*warned {
                    *warned = true;
                    warn_slider(&format!(
                        "[Dialtone] DtSlider: getValueText threw — falling back to the raw value. {error}"
                    ));
                }
            }
        }
    }
    format!("{prefix}{}{suffix}", js_num(value))
}

fn generate_interval(min: f64, max: f64, interval: f64, label: &str) -> (Vec<f64>, Option<String>) {
    let span = max - min;
    let natural = if span > 0.0 {
        (span / interval).floor() as usize + 1
    } else {
        1
    };
    let mut effective = interval;
    let mut warning = None;
    if natural > MAX_GENERATED_POINTS {
        effective = span / (MAX_GENERATED_POINTS as f64 - 1.0);
        warning = Some(format!(
            "[Dialtone] DtSlider: {label}={} would generate more than {MAX_GENERATED_POINTS} points over this range — using {} instead so coverage still spans the full range.",
            js_num(interval),
            js_num(effective)
        ));
    }
    let mut values = Vec::new();
    let mut value = min;
    while value <= max && values.len() < MAX_GENERATED_POINTS {
        values.push(value);
        value = js_to_fixed_num(value + effective, 10);
    }
    if let Some(&last) = values.last() {
        if last < max - 1e-9 {
            values.push(max);
        }
    }
    (values, warning)
}

fn generated_points(min: f64, max: f64, interval: f64, label: &str) -> Vec<f64> {
    let (values, warning) = generate_interval(min, max, interval, label);
    if let Some(warning) = warning {
        warn_slider(&warning);
    }
    values
}

fn edge_clamp_offset(
    left_edge: f64,
    right_edge: f64,
    control_left: f64,
    control_right: f64,
    tolerance: f64,
) -> f64 {
    let start_overflow = control_left - left_edge;
    let end_overflow = right_edge - control_right;
    if start_overflow > tolerance {
        start_overflow - tolerance
    } else if end_overflow > tolerance {
        -(end_overflow - tolerance)
    } else {
        0.0
    }
}

fn mirror_pct(pct: f64, rtl: bool) -> f64 {
    if rtl {
        100.0 - pct
    } else {
        pct
    }
}

fn rects_overlap(a: BoxRect, b: BoxRect, padding: f64) -> bool {
    !(a.right + padding < b.left
        || a.left - padding > b.right
        || a.bottom + padding < b.top
        || a.top - padding > b.bottom)
}

fn analytical_readout_rect(
    pct: f64,
    el: BoxRect,
    control: BoxRect,
    inset: f64,
    vertical: bool,
    rtl: bool,
) -> BoxRect {
    if vertical {
        let height = el.bottom - el.top;
        let total = control.bottom - control.top;
        let anchor = inset + (total - 2.0 * inset) * (pct / 100.0);
        let center = control.bottom - anchor;
        BoxRect {
            left: el.left,
            right: el.right,
            top: center - height / 2.0,
            bottom: center + height / 2.0,
        }
    } else {
        let width = el.right - el.left;
        let total = control.right - control.left;
        let anchor = inset + (total - 2.0 * inset) * (mirror_pct(pct, rtl) / 100.0);
        let center = control.left + anchor;
        BoxRect {
            left: center - width / 2.0,
            right: center + width / 2.0,
            top: el.top,
            bottom: el.bottom,
        }
    }
}

fn mark_collision_results(
    marks: &[MarkMeasure],
    count: usize,
    control: Option<(f64, f64)>,
    readouts: &[BoxRect],
) -> (Vec<bool>, Vec<f64>) {
    let mut hidden = vec![false; count];
    let mut offsets = vec![0.0; count];
    for mark in marks {
        if mark.index >= count {
            continue;
        }
        let offset = control
            .map(|(left, right)| {
                edge_clamp_offset(
                    mark.natural_left,
                    mark.natural_right,
                    left,
                    right,
                    EDGE_CLAMP_TOLERANCE_PX,
                )
            })
            .unwrap_or(0.0);
        offsets[mark.index] = offset;
        if readouts.is_empty() {
            continue;
        }
        let corrected = BoxRect {
            top: mark.top,
            bottom: mark.bottom,
            left: mark.natural_left + offset,
            right: mark.natural_right + offset,
        };
        hidden[mark.index] = readouts
            .iter()
            .any(|readout| rects_overlap(corrected, *readout, COLLISION_PADDING));
    }
    (hidden, offsets)
}

fn large_step_value(step: f64, large: f64) -> f64 {
    if step <= 0.0 {
        return large;
    }
    js_round(large / step).max(1.0) * step
}

fn large_step_delta(key: &str, shift: bool, rtl: bool) -> i32 {
    if key == "PageUp" {
        return 1;
    }
    if key == "PageDown" {
        return -1;
    }
    if !shift {
        return 0;
    }
    let increase = if rtl { "ArrowLeft" } else { "ArrowRight" };
    let decrease = if rtl { "ArrowRight" } else { "ArrowLeft" };
    if key == "ArrowUp" || key == increase {
        1
    } else if key == "ArrowDown" || key == decrease {
        -1
    } else {
        0
    }
}

fn is_nav_key(key: &str) -> bool {
    matches!(
        key,
        "ArrowUp"
            | "ArrowDown"
            | "ArrowLeft"
            | "ArrowRight"
            | "Home"
            | "End"
            | "PageUp"
            | "PageDown"
    )
}

fn nearest_thumb(val: f64, values: &[f64], range: bool, last_active: Option<usize>) -> usize {
    if !range || values.len() < 2 {
        return 0;
    }
    let lo = values[0];
    let hi = values[1];
    if lo == hi {
        return if last_active == Some(0) { 1 } else { 0 };
    }
    if (val - lo).abs() <= (val - hi).abs() {
        0
    } else {
        1
    }
}

fn coincident_handoff(index: usize, raw: f64, lo: f64, hi: f64, enabled: bool) -> usize {
    if !enabled || lo != hi {
        return index;
    }
    if index == 0 && raw > lo {
        return 1;
    }
    if index == 1 && raw < hi {
        return 0;
    }
    index
}

fn value_from_pointer(
    client_x: f64,
    client_y: f64,
    left: f64,
    bottom: f64,
    width: f64,
    height: f64,
    inset: f64,
    vertical: bool,
    rtl: bool,
    bounds: &Bounds,
) -> f64 {
    let size = if vertical { height } else { width };
    let span = size - 2.0 * inset;
    let mut pct = if vertical {
        let raw = bottom - client_y;
        if span > 0.0 {
            (raw - inset) / span
        } else {
            raw / size
        }
    } else {
        let raw = client_x - left;
        if span > 0.0 {
            (raw - inset) / span
        } else {
            raw / size
        }
    };
    if !vertical && rtl {
        pct = 1.0 - pct;
    }
    if !pct.is_finite() {
        pct = 0.0;
    }
    pct = pct.clamp(0.0, 1.0);
    bounds.min + pct * (bounds.max - bounds.min)
}

fn snap_point_list(snaps: &DtSliderSnaps, bounds: &Bounds) -> Vec<f64> {
    match snaps {
        DtSliderSnaps::Off => Vec::new(),
        DtSliderSnaps::Interval(interval) => {
            if !interval.is_finite() || *interval <= 0.0 {
                Vec::new()
            } else {
                generated_points(bounds.min, bounds.max, *interval, "snapPoints")
            }
        }
        DtSliderSnaps::Points(points) => points
            .iter()
            .copied()
            .filter(|point| point.is_finite() && *point >= bounds.min && *point <= bounds.max)
            .collect(),
    }
}

fn set_snap(snaps: &mut Vec<Option<f64>>, index: usize, value: Option<f64>) {
    if snaps.len() <= index {
        snaps.resize(index + 1, None);
    }
    snaps[index] = value;
}

fn update_thumb(
    values: &[f64],
    snaps: &[Option<f64>],
    index: usize,
    new_val: f64,
    allow_snap: bool,
    bounds: &Bounds,
    range: bool,
    points: &[f64],
    track: TrackSize,
) -> ThumbWrite {
    let mut snaps = snaps.to_vec();
    let mut clamped = clamp_to_range(new_val, bounds);
    let mut magnetic = None;
    if allow_snap {
        let clear = points.is_empty() || matches!(track, TrackSize::Missing);
        if clear {
            set_snap(&mut snaps, index, None);
        } else if matches!(track, TrackSize::Zero) {
            // A zero track cannot be measured. Keep the held point and skip the pull.
        } else if let TrackSize::Px(px) = track {
            let entry = (SNAP_THRESHOLD_PX / px) * (bounds.max - bounds.min);
            let held = snaps.get(index).copied().flatten();
            let kept = held.filter(|held| {
                points.contains(held) && (*held - clamped).abs() <= entry * SNAP_RELEASE_MULTIPLIER
            });
            if let Some(held) = kept {
                magnetic = Some(held);
            } else {
                let mut closest = None;
                let mut closest_dist = f64::INFINITY;
                for point in points {
                    let dist = (point - clamped).abs();
                    if dist <= entry && dist < closest_dist {
                        closest = Some(*point);
                        closest_dist = dist;
                    }
                }
                set_snap(&mut snaps, index, closest);
                magnetic = closest;
            }
        }
        if let Some(magnetic) = magnetic {
            clamped = magnetic;
        } else {
            // A zero track returns no magnetic point and does not clear the
            // hold. Step quantization still runs, matching Vue.
            clamped = snap_to_step(clamped, bounds);
        }
    } else {
        clamped = snap_to_step(clamped, bounds);
    }

    if range {
        let gap = (js_round(bounds.gap) * bounds.step).max(0.0);
        let next_hi = values.get(1).copied().unwrap_or(bounds.max);
        let next_lo = values.first().copied().unwrap_or(bounds.min);
        if index == 0 {
            clamped = clamped.min(next_hi - gap);
        } else {
            clamped = clamped.max(next_lo + gap);
        }
        let held_magnetic = magnetic.is_some_and(|magnetic| clamped == magnetic);
        if !held_magnetic {
            let dp = decimal_places(bounds.min).max(decimal_places(bounds.step));
            clamped = clamp_to_range(js_to_fixed_num(clamped, dp), bounds);
        }
        if magnetic.is_some_and(|magnetic| clamped != magnetic) {
            set_snap(&mut snaps, index, None);
        }
    }

    if values.get(index).copied() == Some(clamped) {
        return ThumbWrite {
            values: values.to_vec(),
            snaps,
            changed: false,
            native: clamped,
        };
    }
    let mut next = values.to_vec();
    if index >= next.len() {
        return ThumbWrite {
            values: next,
            snaps,
            changed: false,
            native: clamped,
        };
    }
    next[index] = clamped;
    ThumbWrite {
        values: next,
        snaps,
        changed: true,
        native: clamped,
    }
}

fn held_echo(values: &[f64], incoming: &[f64], snaps: &[Option<f64>]) -> Option<usize> {
    values.iter().enumerate().find_map(|(index, value)| {
        let same = incoming.get(index).copied() == Some(*value);
        let held = snaps.get(index).copied().flatten() == Some(*value);
        (same && held).then_some(index)
    })
}

fn held_in_bounds(values: &[f64], snaps: &[Option<f64>], bounds: &Bounds) -> Option<usize> {
    values.iter().enumerate().find_map(|(index, value)| {
        let held = snaps.get(index).copied().flatten() == Some(*value);
        (held && *value >= bounds.min && *value <= bounds.max).then_some(index)
    })
}

fn sync_slider(
    incoming: DtSliderValue,
    bounds: &Bounds,
    prev: Option<&(DtSliderValue, Bounds)>,
    values: &[f64],
    committed: &[f64],
    snaps: &[Option<f64>],
) -> SyncOut {
    let range = is_range_value(incoming);
    let raw = numbers_of(incoming);
    let Some((prev_value, prev_bounds)) = prev else {
        let next = apply_value_constraints(&raw, bounds, None);
        let emit_input = different(&next, &raw).then(|| payload(&next, range));
        return SyncOut {
            values: next.clone(),
            committed: next,
            emit_input,
            emit_change: None,
        };
    };

    let mut values = values.to_vec();
    let mut committed = committed.to_vec();
    let mut emit_input = None;
    let mut emit_change = None;
    let model_changed = *prev_value != incoming;
    let bounds_changed = *prev_bounds != *bounds;

    if model_changed {
        let held = held_echo(&values, &raw, snaps);
        let next = apply_value_constraints(&raw, bounds, held);
        if different(&next, &values) {
            let already = different(&values, &committed);
            values = next.clone();
            if !already {
                committed = next.clone();
            }
        }
        if different(&next, &raw) {
            emit_input = Some(payload(&next, range));
        }
    }

    if bounds_changed {
        let held = held_in_bounds(&values, snaps, bounds);
        let next = apply_value_constraints(&values, bounds, held);
        if different(&next, &values) {
            let already = different(&values, &committed);
            values = next.clone();
            emit_input = Some(payload(&next, range));
            if !already {
                committed = next.clone();
                emit_change = Some(payload(&next, range));
            }
        }
    }

    SyncOut {
        values,
        committed,
        emit_input,
        emit_change,
    }
}

fn should_restore_focus(
    prev_len: usize,
    new_len: usize,
    focused: Option<usize>,
    second_is_active: bool,
) -> bool {
    new_len == 1 && prev_len == 2 && (focused == Some(1) || second_is_active)
}

fn style_of(pairs: &[(&str, &str)]) -> String {
    pairs
        .iter()
        .filter(|(_, value)| !value.is_empty())
        .map(|(key, value)| format!("{key}: {value}"))
        .collect::<Vec<_>>()
        .join("; ")
}

fn anchor_pct(pct: f64) -> String {
    format!(
        "calc({EDGE_INSET} + (100% - {EDGE_INSET} * 2) * {})",
        js_num(pct / 100.0)
    )
}

fn anchor_span_pct(delta: f64) -> String {
    format!(
        "calc((100% - {EDGE_INSET} * 2) * {})",
        js_num(delta / 100.0)
    )
}

fn center_inline(rtl: bool) -> &'static str {
    if rtl {
        "50%"
    } else {
        "-50%"
    }
}

fn thumb_transform(vertical: bool, rtl: bool) -> String {
    let x = center_inline(rtl);
    if vertical {
        format!("translate({x}, 50%)")
    } else {
        format!("translate({x}, -50%)")
    }
}

fn tick_transform(vertical: bool, rtl: bool) -> String {
    if vertical {
        "translateY(50%)".to_string()
    } else {
        format!("translateX({})", center_inline(rtl))
    }
}

fn mark_edge_transform(edge: MarkEdge, rtl: bool) -> String {
    let inward = if rtl { -1 } else { 1 };
    if edge == MarkEdge::Start {
        format!("translateX({inward}px)")
    } else {
        let flush = if rtl { "100%" } else { "-100%" };
        format!("translateX(calc({flush} + {}px))", -inward)
    }
}

fn anchored_position(pct: f64, vertical: bool, transform: &str) -> String {
    let anchor = anchor_pct(pct);
    if vertical {
        style_of(&[("bottom", &anchor), ("transform", transform)])
    } else {
        style_of(&[("inset-inline-start", &anchor), ("transform", transform)])
    }
}

fn thumb_position_style(val: f64, bounds: &Bounds, vertical: bool, rtl: bool) -> String {
    let transform = thumb_transform(vertical, rtl);
    anchored_position(thumb_percent(val, bounds), vertical, &transform)
}

fn tick_position_style(val: f64, bounds: &Bounds, vertical: bool, rtl: bool) -> String {
    let transform = tick_transform(vertical, rtl);
    anchored_position(thumb_percent(val, bounds), vertical, &transform)
}

fn label_style(pct: f64, offset: f64, edge: Option<MarkEdge>, vertical: bool, rtl: bool) -> String {
    if let Some(edge) = edge {
        if !vertical {
            let transform = mark_edge_transform(edge, rtl);
            let pct_text = format!("{}%", js_num(pct));
            return style_of(&[("inset-inline-start", &pct_text), ("transform", &transform)]);
        }
    }
    let offset = if vertical { 0.0 } else { offset };
    if offset == 0.0 {
        let anchor = anchor_pct(pct);
        return if vertical {
            style_of(&[("bottom", &anchor)])
        } else {
            style_of(&[("inset-inline-start", &anchor)])
        };
    }
    let transform = format!(
        "translateX(calc({} + {}px))",
        center_inline(rtl),
        js_num(offset)
    );
    let anchor = anchor_pct(pct);
    if vertical {
        style_of(&[("bottom", &anchor), ("transform", &transform)])
    } else {
        style_of(&[("inset-inline-start", &anchor), ("transform", &transform)])
    }
}

fn readout_edge(val: f64, bounds: &Bounds) -> Option<MarkEdge> {
    if val == bounds.min {
        Some(MarkEdge::Start)
    } else if val == bounds.max {
        Some(MarkEdge::End)
    } else {
        None
    }
}

fn indicator_style(
    values: &[f64],
    bounds: &Bounds,
    fill: DtSliderFill,
    range: bool,
    vertical: bool,
) -> String {
    if range && values.len() == 2 {
        let lo = thumb_percent(values[0], bounds);
        let hi = thumb_percent(values[1], bounds);
        let start = anchor_pct(lo);
        let size = anchor_span_pct(hi - lo);
        return if vertical {
            style_of(&[("bottom", &start), ("height", &size)])
        } else {
            style_of(&[("inset-inline-start", &start), ("width", &size)])
        };
    }
    let pct = thumb_percent(values.first().copied().unwrap_or(bounds.min), bounds);
    if let DtSliderFill::At(origin) = fill {
        let origin = thumb_percent(clamp_to_range(origin, bounds), bounds);
        let start_pct = pct.min(origin);
        let size_pct = (pct - origin).abs();
        let start = anchor_pct(start_pct);
        let size = anchor_span_pct(size_pct);
        return if vertical {
            style_of(&[("bottom", &start), ("height", &size)])
        } else {
            style_of(&[("inset-inline-start", &start), ("width", &size)])
        };
    }
    let from_end = fill == DtSliderFill::End;
    let anchor = anchor_pct(pct);
    if vertical {
        if from_end {
            let height = format!("calc(100% - {anchor})");
            style_of(&[("top", "0"), ("height", &height)])
        } else {
            style_of(&[("bottom", "0"), ("height", &anchor)])
        }
    } else if from_end {
        let width = format!("calc(100% - {anchor})");
        style_of(&[("inset-inline-end", "0"), ("width", &width)])
    } else {
        style_of(&[("inset-inline-start", "0"), ("width", &anchor)])
    }
}

fn slider_root_class(
    extra: &str,
    disabled: bool,
    vertical: bool,
    inverted: bool,
    dragging: bool,
) -> String {
    join_class([
        "d-slider",
        extra,
        if disabled { "d-slider--disabled" } else { "" },
        if vertical { "d-slider--vertical" } else { "" },
        if inverted { "d-slider--inverted" } else { "" },
        if dragging { "d-slider--dragging" } else { "" },
    ])
}

fn reserves_annotation_space(vertical: bool, marks: usize, readout: DtSliderReadout) -> bool {
    !vertical && (marks > 0 || readout == DtSliderReadout::Always)
}

fn slider_body_class(extra: &str, reserve: bool) -> String {
    join_class([
        "d-slider__body",
        extra,
        if reserve {
            "d-slider__body--reserve-annotation-space"
        } else {
            ""
        },
    ])
}

fn thumb_visual_class(focused: bool, active: bool) -> String {
    join_class([
        "d-slider__thumb-visual",
        if focused {
            "d-slider__thumb-visual--focused"
        } else {
            ""
        },
        if active {
            "d-slider__thumb-visual--active"
        } else {
            ""
        },
    ])
}

fn mark_class(hidden: bool) -> String {
    if hidden {
        "d-slider__mark d-slider__mark--collision-hidden".to_string()
    } else {
        "d-slider__mark".to_string()
    }
}

fn readout_class_name(show: bool, extra: &str) -> String {
    join_class([
        "d-slider__readout",
        if show {
            "d-slider__readout--show"
        } else {
            "d-slider__readout--hide"
        },
        extra,
    ])
}

struct RenderedMark {
    text: String,
    pct: f64,
    edge: Option<MarkEdge>,
}

fn format_mark(value: f64, prefix: &str, suffix: &str) -> String {
    format!("{prefix}{}{suffix}", js_num(value))
}

fn build_marks(
    marks: &DtSliderMarks,
    bounds: &Bounds,
    prefix: &str,
    suffix: &str,
) -> Vec<RenderedMark> {
    let source: Vec<DtSliderMark> = match marks {
        DtSliderMarks::Off => return Vec::new(),
        DtSliderMarks::Ends => vec![
            DtSliderMark {
                value: bounds.min,
                text: None,
            },
            DtSliderMark {
                value: bounds.max,
                text: None,
            },
        ],
        DtSliderMarks::At(marks) => marks.clone(),
    };
    source
        .into_iter()
        .filter(|mark| mark.value.is_finite())
        .map(|mark| {
            let edge = readout_edge(mark.value, bounds);
            let text = mark
                .text
                .unwrap_or_else(|| format_mark(mark.value, prefix, suffix));
            RenderedMark {
                text,
                pct: thumb_percent(mark.value, bounds),
                edge,
            }
        })
        .collect()
}

fn tick_values(ticks: DtSliderTicks, bounds: &Bounds) -> Vec<f64> {
    let interval = match ticks {
        DtSliderTicks::Off => return Vec::new(),
        DtSliderTicks::EveryStep => bounds.step,
        DtSliderTicks::Interval(interval) => interval,
    };
    if !interval.is_finite() || interval <= 0.0 {
        return Vec::new();
    }
    generated_points(bounds.min, bounds.max, interval, "ticks")
}

fn fractional_gap_warning(gap: f64) -> Option<String> {
    let rounded = js_round(gap);
    if gap != rounded {
        Some(format!(
            "[Dialtone] DtSlider: minGapSteps ({}) is not a whole number — it will be rounded to {}.",
            js_num(gap),
            js_num(rounded)
        ))
    } else {
        None
    }
}

fn min_max_warning(min: f64, max: f64) -> Option<String> {
    if min > max {
        Some(format!(
            "[Dialtone] DtSlider: min ({}) is greater than max ({}) — every value will collapse to max.",
            js_num(min),
            js_num(max)
        ))
    } else {
        None
    }
}

fn finite_warning(name: &str, value: f64, fallback: f64) -> Option<String> {
    if value.is_finite() {
        None
    } else {
        Some(format!(
            "[Dialtone] DtSlider: {name} ({}) is not finite — using {}.",
            js_num(value),
            js_num(fallback)
        ))
    }
}

fn orientation_warning(value: &str) -> String {
    format!("[Dialtone] DtSlider: orientation \"{value}\" is not supported — using \"horizontal\".")
}

fn readout_open(
    readout: DtSliderReadout,
    index: usize,
    active: Option<usize>,
    focused: Option<usize>,
    hovered: Option<usize>,
    escape: Option<usize>,
) -> bool {
    if readout == DtSliderReadout::Always {
        return true;
    }
    if readout == DtSliderReadout::Never {
        return false;
    }
    if escape == Some(index) {
        return false;
    }
    active == Some(index) || focused == Some(index) || hovered == Some(index)
}

fn merged_readout_text(values: &[f64], prefix: &str, suffix: &str) -> String {
    if values.len() != 2 {
        return String::new();
    }
    let mut warned = false;
    let lo = format_value(values[0], 0, prefix, suffix, None, &mut warned);
    let hi = format_value(values[1], 1, prefix, suffix, None, &mut warned);
    format!("{lo}–{hi}")
}

#[derive(Clone, Copy)]
struct Handles {
    value: Signal<DtSliderValue>,
    values: RwSignal<Vec<f64>>,
    committed: RwSignal<Vec<f64>>,
    snaps: RwSignal<Vec<Option<f64>>>,
    dragging: RwSignal<bool>,
    active: RwSignal<Option<usize>>,
    last_active: RwSignal<Option<usize>>,
    focused: RwSignal<Option<usize>>,
    hovered: RwSignal<Option<usize>>,
    escape: RwSignal<Option<usize>>,
    rtl: RwSignal<bool>,
    min: MaybeProp<f64>,
    max: MaybeProp<f64>,
    step: MaybeProp<f64>,
    gap: MaybeProp<f64>,
    large: MaybeProp<f64>,
    disabled: MaybeProp<bool>,
    vertical: bool,
    readout: DtSliderReadout,
    on_input: StoredValue<Callback<DtSliderValue>>,
    on_change: StoredValue<Option<Callback<DtSliderValue>>>,
    on_focus: StoredValue<Option<Callback<web_sys::FocusEvent>>>,
    on_blur: StoredValue<Option<Callback<web_sys::FocusEvent>>>,
    prefix: StoredValue<String>,
    suffix: StoredValue<String>,
    text: StoredValue<Option<Callback<(f64, usize), String>>>,
    text_warned: StoredValue<bool>,
    snaps_src: StoredValue<DtSliderSnaps>,
    marks: StoredValue<DtSliderMarks>,
    ticks: StoredValue<DtSliderTicks>,
    readout_class: StoredValue<String>,
    control: NodeRef<html::Div>,
    thumbs: [NodeRef<html::Input>; 2],
    visuals: [NodeRef<html::Div>; 2],
    tooltips: [NodeRef<html::Div>; 2],
    merged: NodeRef<html::Div>,
    pointer: StoredValue<Option<i32>>,
    coincident: StoredValue<bool>,
    inset: StoredValue<f64>,
    pointer_focus: StoredValue<bool>,
    mark_hidden: RwSignal<Vec<bool>>,
    mark_offsets: RwSignal<Vec<f64>>,
    readout_offsets: RwSignal<Vec<f64>>,
    merged_offset: RwSignal<f64>,
    readout_merged: RwSignal<bool>,
    frame: StoredValue<u32>,
    resize_observer: StoredValue<Option<web_sys::ResizeObserver>>,
    dir_observer: StoredValue<Option<web_sys::MutationObserver>>,
}

impl Handles {
    fn bounds_from(self, tracked: bool) -> Bounds {
        let read = |prop: MaybeProp<f64>| {
            if tracked {
                prop.get()
            } else {
                prop.get_untracked()
            }
        };
        Bounds::new(
            read(self.min).unwrap_or(0.0),
            read(self.max).unwrap_or(100.0),
            read(self.step).unwrap_or(1.0),
            read(self.gap).unwrap_or(0.0),
        )
    }

    fn is_disabled(self) -> bool {
        self.disabled.get_untracked().unwrap_or(false)
    }

    fn range_now(self) -> bool {
        is_range_value(self.value.get_untracked())
    }

    fn emit_input(self, next: DtSliderValue) {
        self.on_input.get_value().run(next);
    }

    fn emit_change(self, next: DtSliderValue) {
        if let Some(callback) = self.on_change.get_value() {
            callback.run(next);
        }
    }

    fn payload_now(self, values: &[f64]) -> DtSliderValue {
        payload(values, self.range_now())
    }

    fn format_at(self, value: f64, index: usize) -> String {
        let provided = self
            .text
            .get_value()
            .map(|callback| Ok(callback.run((value, index))));
        let mut warned = self.text_warned.get_value();
        let text = format_value(
            value,
            index,
            &self.prefix.get_value(),
            &self.suffix.get_value(),
            provided,
            &mut warned,
        );
        self.text_warned.set_value(warned);
        text
    }

    fn track(self) -> TrackSize {
        let Some(el) = self.control.get() else {
            return TrackSize::Missing;
        };
        let rect = el.get_bounding_client_rect();
        let size = if self.vertical {
            rect.height()
        } else {
            rect.width()
        };
        if size == 0.0 {
            TrackSize::Zero
        } else {
            TrackSize::Px(size)
        }
    }

    fn force_native(self, index: usize, value: f64) {
        let Some(el) = self.thumbs[index].get() else {
            return;
        };
        let text = js_num(value);
        if el.value() != text {
            el.set_value(&text);
        }
    }

    fn apply_thumb(self, index: usize, raw: f64, allow_snap: bool) {
        let values = self.values.get_untracked();
        if index >= values.len() {
            return;
        }
        let bounds = self.bounds_from(false);
        let snaps = self.snaps.get_untracked();
        let points = if allow_snap {
            snap_point_list(&self.snaps_src.get_value(), &bounds)
        } else {
            Vec::new()
        };
        let track = if allow_snap {
            self.track()
        } else {
            TrackSize::Missing
        };
        let write = update_thumb(
            &values,
            &snaps,
            index,
            raw,
            allow_snap,
            &bounds,
            self.range_now(),
            &points,
            track,
        );
        if write.snaps != snaps {
            self.snaps.set(write.snaps);
        }
        if !write.changed {
            self.force_native(index, write.native);
            return;
        }
        self.values.set(write.values.clone());
        self.emit_input(self.payload_now(&write.values));
    }

    fn commit(self) {
        let values = self.values.get_untracked();
        let committed = self.committed.get_untracked();
        if different(&values, &committed) {
            self.committed.set(values.clone());
            self.emit_change(self.payload_now(&values));
        }
    }

    fn end_drag(self, event_id: Option<i32>) {
        if !self.dragging.get_untracked() {
            return;
        }
        if let Some(id) = event_id {
            if self.pointer.get_value() != Some(id) {
                return;
            }
        }
        self.dragging.set(false);
        self.pointer.set_value(None);
        if let Some(index) = self.active.get_untracked() {
            let values = self.values.get_untracked();
            if let Some(&value) = values.get(index) {
                let bounds = self.bounds_from(false);
                if is_off_grid(value, bounds.min, bounds.step) {
                    self.apply_thumb(index, value, false);
                }
            }
        }
        self.commit();
        self.active.set(None);
        self.snaps.set(Vec::new());
    }

    fn pointer_down(self, event: &web_sys::PointerEvent) {
        if self.is_disabled() {
            return;
        }
        if event.pointer_type() == "mouse" && event.button() != 0 {
            return;
        }
        if self.dragging.get_untracked() && self.pointer.get_value() != Some(event.pointer_id()) {
            return;
        }
        event.prevent_default();
        sync_rtl(self);
        let Some(el) = self.control.get() else {
            return;
        };
        self.inset.set_value(measure_edge_inset(&el));
        let rect = el.get_bounding_client_rect();
        let bounds = self.bounds_from(false);
        let raw = value_from_pointer(
            event.client_x() as f64,
            event.client_y() as f64,
            rect.left(),
            rect.bottom(),
            rect.width(),
            rect.height(),
            self.inset.get_value(),
            self.vertical,
            self.rtl.get_untracked(),
            &bounds,
        );
        let values = self.values.get_untracked();
        let index = nearest_thumb(
            raw,
            &values,
            self.range_now(),
            self.last_active.get_untracked(),
        );
        let coincident = self.range_now() && values.len() == 2 && values.first() == values.get(1);
        self.coincident.set_value(coincident);
        if self.escape.get_untracked() == Some(index) {
            self.escape.set(None);
        }
        self.active.set(Some(index));
        self.last_active.set(Some(index));
        self.dragging.set(true);
        self.pointer.set_value(Some(event.pointer_id()));
        let _ = el.set_pointer_capture(event.pointer_id());
        self.hovered.set(None);
        self.apply_thumb(index, raw, true);
        if let Some(thumb) = self.thumbs[index].get() {
            let active = web_sys::window()
                .and_then(|window| window.document())
                .and_then(|document| document.active_element());
            let already = active.is_some_and(|active| active == **thumb);
            if !already {
                self.pointer_focus.set_value(true);
                let _ = thumb.focus();
            } else if self.focused.get_untracked() == Some(index) {
                self.focused.set(None);
            }
        }
    }

    fn pointer_move(self, event: &web_sys::PointerEvent) {
        if !self.dragging.get_untracked() || self.active.get_untracked().is_none() {
            return;
        }
        if event.pointer_id() != self.pointer.get_value().unwrap_or(i32::MIN) {
            return;
        }
        if self.is_disabled() || event.buttons() == 0 {
            self.end_drag(None);
            return;
        }
        let Some(el) = self.control.get() else {
            return;
        };
        let rect = el.get_bounding_client_rect();
        let bounds = self.bounds_from(false);
        let raw = value_from_pointer(
            event.client_x() as f64,
            event.client_y() as f64,
            rect.left(),
            rect.bottom(),
            rect.width(),
            rect.height(),
            self.inset.get_value(),
            self.vertical,
            self.rtl.get_untracked(),
            &bounds,
        );
        let mut index = self.active.get_untracked().unwrap_or(0);
        if self.coincident.get_value() {
            let values = self.values.get_untracked();
            if values.len() == 2 {
                let next = coincident_handoff(index, raw, values[0], values[1], true);
                if next != index {
                    let mut snaps = self.snaps.get_untracked();
                    set_snap(&mut snaps, index, None);
                    self.snaps.set(snaps);
                    if self.escape.get_untracked() == Some(next) {
                        self.escape.set(None);
                    }
                    index = next;
                    self.active.set(Some(index));
                    self.last_active.set(Some(index));
                }
            }
        }
        self.apply_thumb(index, raw, true);
    }

    fn thumb_keydown(self, index: usize, event: &web_sys::KeyboardEvent) {
        if self.focused.get_untracked() != Some(index) {
            self.focused.set(Some(index));
        }
        let key = event.key();
        let delta = large_step_delta(&key, event.shift_key(), self.rtl.get_untracked());
        if delta != 0 {
            event.prevent_default();
            let bounds = self.bounds_from(false);
            let large = large_step_value(bounds.step, self.large.get_untracked().unwrap_or(10.0));
            let current = self
                .values
                .get_untracked()
                .get(index)
                .copied()
                .unwrap_or(bounds.min);
            self.apply_thumb(index, current + f64::from(delta) * large, false);
        }
        if key == "Escape" {
            self.escape.set(Some(index));
        } else if is_nav_key(&key) && self.escape.get_untracked() == Some(index) {
            self.escape.set(None);
        }
    }

    fn thumb_focus(self, index: usize, event: web_sys::FocusEvent) {
        if self.pointer_focus.get_value() {
            self.pointer_focus.set_value(false);
        } else {
            if self.escape.get_untracked() == Some(index) {
                self.escape.set(None);
            }
            self.focused.set(Some(index));
        }
        if let Some(callback) = self.on_focus.get_value() {
            callback.run(event);
        }
    }

    fn thumb_blur(self, event: web_sys::FocusEvent) {
        self.focused.set(None);
        self.commit();
        if let Some(callback) = self.on_blur.get_value() {
            callback.run(event);
        }
    }

    fn thumb_enter(self, index: usize) {
        if self.escape.get_untracked() == Some(index) {
            self.escape.set(None);
        }
        self.hovered.set(Some(index));
    }

    fn thumb_leave(self, index: usize) {
        if self.hovered.get_untracked() == Some(index) {
            self.hovered.set(None);
        }
    }

    fn schedule(self) {
        let id = self.frame.get_value().wrapping_add(1);
        self.frame.set_value(id);
        request_frame(move || {
            if self.frame.get_value() != id {
                return;
            }
            if self.readout == DtSliderReadout::Tooltip {
                position_tooltips(self);
            }
            let again = measure_collisions(self);
            if again {
                self.schedule();
            }
        });
    }
}

fn request_frame(task: impl FnOnce() + 'static) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let closure = Closure::once(task);
    if window
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .is_ok()
    {
        closure.forget();
    }
}

fn measure_edge_inset(control: &web_sys::HtmlElement) -> f64 {
    let Some(document) = control.owner_document() else {
        return 0.0;
    };
    let Ok(probe) = document.create_element("div") else {
        return 0.0;
    };
    let Ok(probe) = probe.dyn_into::<web_sys::HtmlElement>() else {
        return 0.0;
    };
    let style = probe.style();
    let _ = style.set_property("position", "absolute");
    let _ = style.set_property("visibility", "hidden");
    let _ = style.set_property("pointer-events", "none");
    let _ = style.set_property("inline-size", EDGE_INSET);
    let _ = style.set_property("block-size", "0px");
    if control.append_child(&probe).is_err() {
        return 0.0;
    }
    let px = probe.get_bounding_client_rect().width();
    probe.remove();
    px
}

fn sync_rtl(handles: Handles) {
    let Some(el) = handles.control.get() else {
        return;
    };
    let rtl = web_sys::window()
        .and_then(|window| window.get_computed_style(&el).ok().flatten())
        .and_then(|style| style.get_property_value("direction").ok())
        .is_some_and(|direction| direction == "rtl");
    if handles.rtl.get_untracked() != rtl {
        handles.rtl.set(rtl);
    }
}

fn install_observers(handles: Handles) {
    let Some(el) = handles.control.get() else {
        return;
    };
    if handles.resize_observer.get_value().is_some() {
        return;
    }
    sync_rtl(handles);
    let resize_callback = Closure::wrap(Box::new(
        move |_entries: js_sys::Array, _observer: web_sys::ResizeObserver| {
            handles.schedule();
        },
    )
        as Box<dyn FnMut(js_sys::Array, web_sys::ResizeObserver)>);
    if let Ok(observer) = web_sys::ResizeObserver::new(resize_callback.as_ref().unchecked_ref()) {
        observer.observe(&el);
        resize_callback.forget();
        handles.resize_observer.set_value(Some(observer));
    }
    let Some(window) = web_sys::window() else {
        return;
    };
    let direction_callback = Closure::wrap(Box::new(
        move |_records: js_sys::Array, _observer: web_sys::MutationObserver| {
            sync_rtl(handles);
        },
    )
        as Box<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>);
    let Ok(observer) = web_sys::MutationObserver::new(direction_callback.as_ref().unchecked_ref())
    else {
        return;
    };
    let options = web_sys::MutationObserverInit::new();
    options.set_attributes(true);
    let filter = js_sys::Array::new();
    let _ = filter.push(&JsValue::from_str("dir"));
    let _ = filter.push(&JsValue::from_str("style"));
    let _ = filter.push(&JsValue::from_str("class"));
    options.set_attribute_filter(&filter);
    let mut node = Some(el.unchecked_into::<web_sys::Element>());
    while let Some(current) = node {
        let _ = observer.observe_with_options(&current, &options);
        node = current.parent_element();
    }
    if let Some(root) = window
        .document()
        .and_then(|document| document.document_element())
    {
        let _ = observer.observe_with_options(&root, &options);
    }
    direction_callback.forget();
    handles.dir_observer.set_value(Some(observer));
}

fn box_from_dom(rect: &web_sys::DomRect) -> BoxRect {
    BoxRect {
        left: rect.left(),
        right: rect.right(),
        top: rect.top(),
        bottom: rect.bottom(),
    }
}

fn readout_is_open(handles: Handles, index: usize) -> bool {
    readout_open(
        handles.readout,
        index,
        handles.active.get_untracked(),
        handles.focused.get_untracked(),
        handles.hovered.get_untracked(),
        handles.escape.get_untracked(),
    )
}

fn measure_collisions(handles: Handles) -> bool {
    let Some(control) = handles.control.get() else {
        return false;
    };
    let inset = measure_edge_inset(&control);
    let control_box = box_from_dom(&control.get_bounding_client_rect());
    let values = handles.values.get_untracked();
    let bounds = handles.bounds_from(false);
    let rtl = handles.rtl.get_untracked();
    let was_merged = handles.readout_merged.get_untracked();
    let mut merged = false;
    if handles.readout != DtSliderReadout::Tooltip
        && handles.range_now()
        && values.len() == 2
        && readout_is_open(handles, 0)
        && readout_is_open(handles, 1)
    {
        if let (Some(a), Some(b)) = (readout_element(&control, 0), readout_element(&control, 1)) {
            let rect_a = analytical_readout_rect(
                thumb_percent(values[0], &bounds),
                box_from_dom(&a.get_bounding_client_rect()),
                control_box,
                inset,
                handles.vertical,
                rtl,
            );
            let rect_b = analytical_readout_rect(
                thumb_percent(values[1], &bounds),
                box_from_dom(&b.get_bounding_client_rect()),
                control_box,
                inset,
                handles.vertical,
                rtl,
            );
            merged = rects_overlap(rect_a, rect_b, COLLISION_PADDING);
        }
    }
    if merged != was_merged {
        handles.readout_merged.set(merged);
        return true;
    }
    update_mark_collisions(handles, &control, control_box, &values, &bounds, inset, rtl);
    update_readout_offsets(handles, &control, control_box);
    false
}

fn readout_element(control: &web_sys::HtmlElement, index: usize) -> Option<web_sys::Element> {
    control
        .query_selector(&format!("[data-readout-index=\"{index}\"]"))
        .ok()
        .flatten()
}

fn update_mark_collisions(
    handles: Handles,
    control: &web_sys::HtmlElement,
    control_box: BoxRect,
    values: &[f64],
    bounds: &Bounds,
    inset: f64,
    rtl: bool,
) {
    let Ok(nodes) = control.query_selector_all("[data-mark-index]") else {
        return;
    };
    if nodes.length() == 0 {
        return;
    }
    let marks = build_marks(
        &handles.marks.get_value(),
        bounds,
        &handles.prefix.get_value(),
        &handles.suffix.get_value(),
    );
    let prev = handles.mark_offsets.get_untracked();
    let mut measures = Vec::new();
    for index in 0..nodes.length() {
        let Some(node) = nodes.item(index) else {
            continue;
        };
        let Ok(el) = node.dyn_into::<web_sys::Element>() else {
            continue;
        };
        let Some(mark_index) = el
            .get_attribute("data-mark-index")
            .and_then(|value| value.parse().ok())
        else {
            continue;
        };
        let rect = el.get_bounding_client_rect();
        let prev_offset = prev.get(mark_index).copied().unwrap_or(0.0);
        measures.push(MarkMeasure {
            index: mark_index,
            natural_left: rect.left() - prev_offset,
            natural_right: rect.right() - prev_offset,
            top: rect.top(),
            bottom: rect.bottom(),
        });
    }
    let readouts = collect_readout_rects(handles, control, control_box, values, bounds, inset, rtl);
    let clamp = (!handles.vertical).then_some((control_box.left, control_box.right));
    let (hidden, offsets) = mark_collision_results(&measures, marks.len(), clamp, &readouts);
    if hidden != handles.mark_hidden.get_untracked() {
        handles.mark_hidden.set(hidden);
    }
    if offsets != handles.mark_offsets.get_untracked() {
        handles.mark_offsets.set(offsets);
    }
}

fn collect_readout_rects(
    handles: Handles,
    control: &web_sys::HtmlElement,
    control_box: BoxRect,
    values: &[f64],
    bounds: &Bounds,
    inset: f64,
    rtl: bool,
) -> Vec<BoxRect> {
    if handles.readout_merged.get_untracked() {
        let Some(el) = handles.merged.get() else {
            return Vec::new();
        };
        if values.len() != 2 {
            return Vec::new();
        }
        let pct = (thumb_percent(values[0], bounds) + thumb_percent(values[1], bounds)) / 2.0;
        return vec![analytical_readout_rect(
            pct,
            box_from_dom(&el.get_bounding_client_rect()),
            control_box,
            inset,
            handles.vertical,
            rtl,
        )];
    }
    values
        .iter()
        .enumerate()
        .filter_map(|(index, value)| {
            if !readout_is_open(handles, index) {
                return None;
            }
            if handles.readout == DtSliderReadout::Tooltip {
                return handles.tooltips[index]
                    .get()
                    .map(|el| box_from_dom(&el.get_bounding_client_rect()));
            }
            let el = readout_element(control, index)?;
            Some(analytical_readout_rect(
                thumb_percent(*value, bounds),
                box_from_dom(&el.get_bounding_client_rect()),
                control_box,
                inset,
                handles.vertical,
                rtl,
            ))
        })
        .collect()
}

fn update_readout_offsets(handles: Handles, control: &web_sys::HtmlElement, control_box: BoxRect) {
    let can_clamp = handles.readout == DtSliderReadout::Always && !handles.vertical;
    if !can_clamp {
        if handles.merged_offset.get_untracked() != 0.0 {
            handles.merged_offset.set(0.0);
        }
        let zeros = vec![0.0; handles.values.with_untracked(|values| values.len())];
        if handles.readout_offsets.get_untracked() != zeros {
            handles.readout_offsets.set(zeros);
        }
        return;
    }
    if handles.readout_merged.get_untracked() {
        let offset = handles
            .merged
            .get()
            .map(|el| {
                let rect = el.get_bounding_client_rect();
                let prev = handles.merged_offset.get_untracked();
                edge_clamp_offset(
                    rect.left() - prev,
                    rect.right() - prev,
                    control_box.left,
                    control_box.right,
                    READOUT_EDGE_CLAMP_TOLERANCE_PX,
                )
            })
            .unwrap_or(0.0);
        if offset != handles.merged_offset.get_untracked() {
            handles.merged_offset.set(offset);
        }
        let zeros = vec![0.0; handles.values.with_untracked(|values| values.len())];
        if handles
            .readout_offsets
            .with_untracked(|offsets| offsets.iter().any(|offset| *offset != 0.0))
        {
            handles.readout_offsets.set(zeros);
        }
        return;
    }
    if handles.merged_offset.get_untracked() != 0.0 {
        handles.merged_offset.set(0.0);
    }
    let prev = handles.readout_offsets.get_untracked();
    let offsets: Vec<f64> = handles.values.with_untracked(|values| {
        values
            .iter()
            .enumerate()
            .map(|(index, _)| {
                if !readout_is_open(handles, index) {
                    return 0.0;
                }
                let Some(el) = readout_element(control, index) else {
                    return 0.0;
                };
                let rect = el.get_bounding_client_rect();
                let previous = prev.get(index).copied().unwrap_or(0.0);
                edge_clamp_offset(
                    rect.left() - previous,
                    rect.right() - previous,
                    control_box.left,
                    control_box.right,
                    READOUT_EDGE_CLAMP_TOLERANCE_PX,
                )
            })
            .collect()
    });
    if offsets != prev {
        handles.readout_offsets.set(offsets);
    }
}

fn invoke(el: &web_sys::HtmlElement, method: &str) -> bool {
    let Ok(value) = js_sys::Reflect::get(el, &JsValue::from_str(method)) else {
        return false;
    };
    let Ok(func) = value.dyn_into::<js_sys::Function>() else {
        return false;
    };
    func.call0(el).is_ok()
}

fn anchor_visible(el: &web_sys::HtmlElement) -> bool {
    let rect = el.get_bounding_client_rect();
    if rect.width() == 0.0 && rect.height() == 0.0 {
        return false;
    }
    let Some(window) = web_sys::window() else {
        return false;
    };
    if window
        .get_computed_style(el)
        .ok()
        .flatten()
        .and_then(|style| style.get_property_value("visibility").ok())
        .as_deref()
        == Some("hidden")
    {
        return false;
    }
    let mut current = Some(el.clone().unchecked_into::<web_sys::Element>());
    while let Some(node) = current {
        if let Some(style) = window.get_computed_style(&node).ok().flatten() {
            if style.get_property_value("opacity").ok().as_deref() == Some("0") {
                return false;
            }
            if style
                .get_property_value("content-visibility")
                .ok()
                .as_deref()
                == Some("hidden")
            {
                return false;
            }
        }
        current = node.parent_element();
    }
    true
}

fn tooltip_placement(vertical: bool, rtl: bool) -> Placement {
    if vertical {
        Placement {
            side: if rtl { Side::Left } else { Side::Right },
            alignment: Alignment::Center,
        }
    } else {
        Placement {
            side: Side::Top,
            alignment: Alignment::Center,
        }
    }
}

fn position_tooltips(handles: Handles) {
    let len = handles.values.with_untracked(|values| values.len());
    for index in 0..len.min(2) {
        let Some(node) = handles.tooltips[index].get() else {
            continue;
        };
        // `HtmlDivElement` picks `ElementExt::style` over `HtmlElement::style`.
        let tooltip: web_sys::HtmlElement = node.into();
        let open = !handles.is_disabled() && readout_is_open(handles, index);
        let Some(anchor) = handles.visuals[index].get() else {
            close_tooltip(&tooltip);
            continue;
        };
        if !open {
            close_tooltip(&tooltip);
            continue;
        }
        let showed = invoke(&tooltip, "showPopover");
        if !showed {
            let _ = tooltip.style().set_property("display", "block");
        }
        let gap = web_sys::window()
            .and_then(|window| window.get_computed_style(&tooltip).ok().flatten())
            .and_then(|style| style.get_property_value("padding-block-start").ok())
            .and_then(|value| parse_css_px(&value))
            .unwrap_or(0.0);
        let anchor_rect = element_rect(&anchor);
        let floating = element_rect(&tooltip);
        let preferred = tooltip_placement(handles.vertical, handles.rtl.get_untracked());
        let computed = compute_position(
            anchor_rect,
            floating,
            preferred,
            &PopperOptions {
                offset: gap,
                padding: gap * 2.0,
                viewport: viewport_rect(),
                fallbacks: vec![preferred.opposite()],
                auto: false,
                tether: false,
            },
        );
        let style = tooltip.style();
        let _ = style.set_property("position", "fixed");
        let _ = style.set_property("left", &format!("{}px", js_num(computed.x)));
        let _ = style.set_property("top", &format!("{}px", js_num(computed.y)));
        let visible = !computed.hidden && anchor_visible(&anchor);
        let _ = style.set_property("visibility", if visible { "visible" } else { "hidden" });
        let _ = tooltip.set_attribute("data-side", computed.placement.as_str());
        let placed = tooltip.get_bounding_client_rect();
        let side = computed.placement.side;
        if side == Side::Top || side == Side::Bottom {
            let x = js_clamp(
                anchor_rect.x + anchor_rect.width / 2.0 - computed.x,
                gap,
                placed.width() - gap,
            );
            let _ = style.set_property("--slider-tooltip-arrow-x", &format!("{}px", js_num(x)));
        } else {
            let y = js_clamp(
                anchor_rect.y + anchor_rect.height / 2.0 - computed.y,
                gap,
                placed.height() - gap,
            );
            let _ = style.set_property("--slider-tooltip-arrow-y", &format!("{}px", js_num(y)));
        }
    }
}

fn close_tooltip(el: &web_sys::HtmlElement) {
    if el.matches(":popover-open").unwrap_or(false) {
        let _ = invoke(el, "hidePopover");
    }
    let _ = el.style().set_property("display", "none");
    let _ = el.style().set_property("visibility", "hidden");
}

fn element_rect(el: &web_sys::HtmlElement) -> Rect {
    let rect = el.get_bounding_client_rect();
    Rect {
        x: rect.x(),
        y: rect.y(),
        width: rect.width(),
        height: rect.height(),
    }
}

fn viewport_rect() -> Rect {
    let Some(window) = web_sys::window() else {
        return Rect::from_size(0.0, 0.0);
    };
    Rect {
        x: 0.0,
        y: 0.0,
        width: window
            .inner_width()
            .ok()
            .and_then(|value| value.as_f64())
            .unwrap_or(0.0),
        height: window
            .inner_height()
            .ok()
            .and_then(|value| value.as_f64())
            .unwrap_or(0.0),
    }
}

fn parse_css_px(value: &str) -> Option<f64> {
    let number: String = value
        .trim()
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.' || *c == '-' || *c == '+')
        .collect();
    if number.is_empty() {
        None
    } else {
        number.parse().ok()
    }
}

fn js_clamp(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}

fn disconnect_observers(handles: Handles) {
    let _ = handles.resize_observer.try_update_value(|slot| {
        if let Some(observer) = slot.take() {
            observer.disconnect();
        }
    });
    let _ = handles.dir_observer.try_update_value(|slot| {
        if let Some(observer) = slot.take() {
            observer.disconnect();
        }
    });
}

fn opt_text(value: &str) -> Option<String> {
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

/// Dialtone Vue `DtSlider`.
///
/// Source is tag `dialtone/v10.11.0`, commit
/// `4b56a42e5c5eeead57181a961060dd2ea8eaa053`
/// (`packages/dialtone-vue/components/Slider`). The Vue sources and
/// `slider.less` match staging `84923549`. It is not in tag `dialtone/v10.7.0`.
/// `d-slider*` is in the vendored Dialtone 10.11.0 CSS. `d-slider--inverted`
/// is still emitted. The tag leaves the rule empty, so it is not in the
/// compiled CSS.
///
/// `value` is `modelValue`. [`DtSliderValue::Single`] is one thumb.
/// [`DtSliderValue::Range`] is two. Vue's omitted value becomes the midpoint.
/// This port is controlled: the caller owns the signal. `on_input` is
/// `update:modelValue` and runs for every change, including a correction on
/// mount. The parent writes the signal or the next effect snaps the thumb
/// back. `on_change` is `change` (pointer up, blur, or a min / max / step /
/// `minGapSteps` correction outside a drag).
///
/// `fill` is `fillOrigin`. `readout` is `readout`. `ticks` is `showTicks`.
/// `marks` is `showMarks`. `snaps` is `snapPoints`. `min_gap_steps` is
/// `minGapSteps`. `large_step` is `largeStep`. `show_label` is `showLabel`.
/// `label_size` is `labelSize`. `label_strength` is `labelStrength`.
/// `get_value_text` is `getValueText` and receives `(value, thumb index)`.
/// `body_class`, `label_class`, `start_class`, `end_class`, and
/// `readout_class` are those Vue class props. `min`, `max`, `step`,
/// `min_gap_steps`, `large_step`, and `disabled` stay reactive.
///
/// `label_slot` is the `label` slot. It does not receive Vue's scoped
/// `{ value }`. A present slot counts as the accessible name. This port does
/// not probe the slot for text. `start` and `end` are those slots.
/// `defineExpose` `focus()` / `blur()` are not ported.
///
/// The tooltip readout uses [`popper::compute_position`] (flip and shift,
/// `tether` false). Vue uses `@floating-ui/dom` with `offset`, `flip`,
/// `shift`, `size`, and `hide`, plus a per-frame anchor check. The `size`
/// middleware is not ported. `showPopover` is called when the browser has it.
/// Without that API the readout is `position: fixed` with `display: block`,
/// and an ancestor `overflow: hidden` can clip it. Direction follows
/// `getComputedStyle` on mount and pointer down, and a `MutationObserver` on
/// the ancestor chain plus `documentElement`. Collision measurement runs on
/// `requestAnimationFrame` instead of Vue's `nextTick`.
#[component]
pub fn DtSlider(
    /// `modelValue`.
    #[prop(into)]
    value: Signal<DtSliderValue>,
    /// `update:modelValue`.
    on_input: Callback<DtSliderValue>,
    /// `change`.
    #[prop(optional, into)]
    on_change: Option<Callback<DtSliderValue>>,
    /// `focus`.
    #[prop(optional, into)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// `blur`.
    #[prop(optional, into)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    /// `min`. Default 0.
    #[prop(optional, into)]
    min: MaybeProp<f64>,
    /// `max`. Default 100.
    #[prop(optional, into)]
    max: MaybeProp<f64>,
    /// `step`. Default 1.
    #[prop(optional, into)]
    step: MaybeProp<f64>,
    /// `minGapSteps`. Default 0.
    #[prop(optional, into)]
    min_gap_steps: MaybeProp<f64>,
    /// `largeStep`. Default 10.
    #[prop(optional, into)]
    large_step: MaybeProp<f64>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    /// `orientation`. Unknown values warn and use horizontal.
    #[prop(optional, default = "horizontal")]
    orientation: &'static str,
    /// `fillOrigin`.
    #[prop(optional)]
    fill: DtSliderFill,
    /// `readout`.
    #[prop(optional)]
    readout: DtSliderReadout,
    /// `showTicks`.
    #[prop(optional)]
    ticks: DtSliderTicks,
    /// `showMarks`.
    #[prop(optional)]
    marks: DtSliderMarks,
    /// `snapPoints`.
    #[prop(optional)]
    snaps: DtSliderSnaps,
    #[prop(optional, into)] label: String,
    /// `showLabel`. Default true.
    #[prop(optional, default = true)]
    show_label: bool,
    /// `labelSize`. Default `300`.
    #[prop(optional, default = "300")]
    label_size: &'static str,
    /// `labelStrength`. Empty omits the strength class.
    #[prop(optional, default = "")]
    label_strength: &'static str,
    /// `getValueText`.
    #[prop(optional, into)]
    get_value_text: Option<Callback<(f64, usize), String>>,
    #[prop(optional, into)] prefix: String,
    #[prop(optional, into)] suffix: String,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] class: String,
    /// `bodyClass`.
    #[prop(optional, into)]
    body_class: String,
    /// `labelClass`.
    #[prop(optional, into)]
    label_class: String,
    /// `startClass`.
    #[prop(optional, into)]
    start_class: String,
    /// `endClass`.
    #[prop(optional, into)]
    end_class: String,
    /// `readoutClass`.
    #[prop(optional, into)]
    readout_class: String,
    #[prop(optional, into)] style: String,
    #[prop(optional, into)] id: String,
    /// `aria-label` on each thumb when the visible label is not the name.
    #[prop(optional, into)]
    aria_label: String,
    /// `aria-labelledby` on each thumb when the visible label is not the name.
    #[prop(optional, into)]
    aria_labelledby: String,
    /// `aria-describedby` on each thumb.
    #[prop(optional, into)]
    aria_describedby: String,
    /// `aria-errormessage` on each thumb.
    #[prop(optional, into)]
    aria_errormessage: String,
    /// `aria-details` on each thumb.
    #[prop(optional, into)]
    aria_details: String,
    /// `aria-invalid` on each thumb.
    #[prop(optional, into)]
    aria_invalid: String,
    /// `start` slot.
    #[prop(optional)]
    start: Option<ViewFn>,
    /// `end` slot.
    #[prop(optional)]
    end: Option<ViewFn>,
    /// `label` slot. No scoped `value`.
    #[prop(optional)]
    label_slot: Option<ViewFn>,
) -> impl IntoView {
    let vertical = orientation == "vertical";
    if orientation != "horizontal" && orientation != "vertical" {
        warn_slider(&orientation_warning(orientation));
    }
    let orientation_attr = if vertical { "vertical" } else { "horizontal" };
    let label_id = unique_id("slider-label");
    let visible_name = show_label && (label_slot.is_some() || !label.trim().is_empty());
    let resolved_label = if visible_name {
        String::new()
    } else if !show_label && !label.is_empty() {
        label.clone()
    } else {
        aria_label.clone()
    };
    let resolved_labelledby = if visible_name {
        label_id.clone()
    } else {
        aria_labelledby.clone()
    };
    let has_name =
        visible_name || !resolved_label.trim().is_empty() || !resolved_labelledby.trim().is_empty();
    if !has_name {
        warn_slider(NAME_WARNING);
    }
    let has_value_text = get_value_text.is_some();
    let range_warned = StoredValue::new(false);
    Effect::new(move |_| {
        let range = is_range_value(value.get());
        let missing = range && !has_value_text;
        if missing && !range_warned.get_value() {
            warn_slider(RANGE_TEXT_WARNING);
        }
        range_warned.set_value(missing);
    });
    let gap_warned = StoredValue::new(false);
    let inverted_warned = StoredValue::new(false);
    let finite_warned = StoredValue::new([false; 4]);
    Effect::new(move |_| {
        let min_raw = min.get().unwrap_or(0.0);
        let max_raw = max.get().unwrap_or(100.0);
        let step_raw = step.get().unwrap_or(1.0);
        let gap_raw = min_gap_steps.get().unwrap_or(0.0);
        let flags = [
            finite_warning("min", min_raw, 0.0),
            finite_warning("max", max_raw, 100.0),
            finite_warning("step", step_raw, 1.0),
            finite_warning("minGapSteps", gap_raw, 0.0),
        ];
        finite_warned.update_value(|seen| {
            for (index, warning) in flags.iter().enumerate() {
                let bad = warning.is_some();
                if bad && !seen[index] {
                    if let Some(warning) = warning {
                        warn_slider(warning);
                    }
                }
                seen[index] = bad;
            }
        });
        let fractional = fractional_gap_warning(gap_raw);
        let bad_gap = fractional.is_some();
        if bad_gap && !gap_warned.get_value() {
            if let Some(fractional) = fractional {
                warn_slider(&fractional);
            }
        }
        gap_warned.set_value(bad_gap);
        let inverted = min_max_warning(min_raw, max_raw);
        let bad_bounds = inverted.is_some();
        if bad_bounds && !inverted_warned.get_value() {
            if let Some(inverted) = inverted {
                warn_slider(&inverted);
            }
        }
        inverted_warned.set_value(bad_bounds);
    });

    let initial_bounds = Bounds::new(
        min.get_untracked().unwrap_or(0.0),
        max.get_untracked().unwrap_or(100.0),
        step.get_untracked().unwrap_or(1.0),
        min_gap_steps.get_untracked().unwrap_or(0.0),
    );
    let initial =
        apply_value_constraints(&numbers_of(value.get_untracked()), &initial_bounds, None);
    let values = RwSignal::new(initial.clone());
    let committed = RwSignal::new(initial);
    let handles = Handles {
        value,
        values,
        committed,
        snaps: RwSignal::new(Vec::new()),
        dragging: RwSignal::new(false),
        active: RwSignal::new(None),
        last_active: RwSignal::new(None),
        focused: RwSignal::new(None),
        hovered: RwSignal::new(None),
        escape: RwSignal::new(None),
        rtl: RwSignal::new(false),
        min,
        max,
        step,
        gap: min_gap_steps,
        large: large_step,
        disabled,
        vertical,
        readout,
        on_input: StoredValue::new(on_input),
        on_change: StoredValue::new(on_change),
        on_focus: StoredValue::new(on_focus),
        on_blur: StoredValue::new(on_blur),
        prefix: StoredValue::new(prefix),
        suffix: StoredValue::new(suffix),
        text: StoredValue::new(get_value_text),
        text_warned: StoredValue::new(false),
        snaps_src: StoredValue::new(snaps),
        marks: StoredValue::new(marks),
        ticks: StoredValue::new(ticks),
        readout_class: StoredValue::new(readout_class),
        control: NodeRef::new(),
        thumbs: [NodeRef::new(), NodeRef::new()],
        visuals: [NodeRef::new(), NodeRef::new()],
        tooltips: [NodeRef::new(), NodeRef::new()],
        merged: NodeRef::new(),
        pointer: StoredValue::new(None),
        coincident: StoredValue::new(false),
        inset: StoredValue::new(0.0),
        pointer_focus: StoredValue::new(false),
        mark_hidden: RwSignal::new(Vec::new()),
        mark_offsets: RwSignal::new(Vec::new()),
        readout_offsets: RwSignal::new(Vec::new()),
        merged_offset: RwSignal::new(0.0),
        readout_merged: RwSignal::new(false),
        frame: StoredValue::new(0),
        resize_observer: StoredValue::new(None),
        dir_observer: StoredValue::new(None),
    };
    let seen = StoredValue::new(None::<(DtSliderValue, Bounds)>);
    Effect::new(move |_| {
        let incoming = handles.value.get();
        let bounds = handles.bounds_from(true);
        let current = handles.values.get_untracked();
        let committed_now = handles.committed.get_untracked();
        let snaps_now = handles.snaps.get_untracked();
        let prev_len = current.len();
        let out = sync_slider(
            incoming,
            &bounds,
            seen.get_value().as_ref(),
            &current,
            &committed_now,
            &snaps_now,
        );
        seen.set_value(Some((incoming, bounds)));
        if out.values != current {
            handles.values.set(out.values.clone());
        }
        if out.committed != committed_now {
            handles.committed.set(out.committed);
        }
        if let Some(next) = out.emit_input {
            handles.emit_input(next);
        }
        if let Some(next) = out.emit_change {
            handles.emit_change(next);
        }
        let new_len = out.values.len();
        let second_active = handles.thumbs[1].get().is_some_and(|thumb| {
            web_sys::window()
                .and_then(|window| window.document())
                .and_then(|document| document.active_element())
                .is_some_and(|active| active == **thumb)
        });
        if should_restore_focus(
            prev_len,
            new_len,
            handles.focused.get_untracked(),
            second_active,
        ) {
            request_frame(move || {
                if let Some(thumb) = handles.thumbs[0].get() {
                    let _ = thumb.focus();
                }
            });
        }
        if handles
            .active
            .get_untracked()
            .is_some_and(|index| index >= new_len)
        {
            handles.active.set(None);
        }
    });
    Effect::new(move |_| {
        if handles.disabled.get().unwrap_or(false) {
            if handles.focused.get_untracked().is_some() {
                handles.focused.set(None);
            }
            if handles.hovered.get_untracked().is_some() {
                handles.hovered.set(None);
            }
        }
    });
    Effect::new(move |_| {
        let _ = handles.control.get();
        install_observers(handles);
    });
    Effect::new(move |_| {
        let _ = handles.control.get();
        let _ = handles.values.get();
        let _ = handles.rtl.get();
        let _ = handles.disabled.get();
        let _ = handles.focused.get();
        let _ = handles.hovered.get();
        let _ = handles.active.get();
        let _ = handles.escape.get();
        let _ = handles.bounds_from(true);
        handles.schedule();
    });
    on_cleanup(move || disconnect_observers(handles));

    let root_class = class;
    let inverted = fill == DtSliderFill::End;
    let mark_count = build_marks(
        &handles.marks.get_value(),
        &initial_bounds,
        &handles.prefix.get_value(),
        &handles.suffix.get_value(),
    )
    .len();
    let body = slider_body_class(
        &body_class,
        reserves_annotation_space(vertical, mark_count, readout),
    );
    let label_class = join_class(["d-slider__label", &label_class]);
    let start_class = join_class(["d-slider__start", &start_class]);
    let end_class = join_class(["d-slider__end", &end_class]);
    let id_attr = opt_text(&id);
    let style_attr = opt_text(&style);
    let name_attr = opt_text(&name);
    let aria_label_attr = opt_text(&resolved_label);
    let aria_labelledby_attr = opt_text(&resolved_labelledby);
    let aria_describedby_attr = opt_text(&aria_describedby);
    let aria_errormessage_attr = opt_text(&aria_errormessage);
    let aria_details_attr = opt_text(&aria_details);
    let aria_invalid_attr = opt_text(&aria_invalid);
    let aria_orientation = vertical.then(|| "vertical".to_string());
    let label_text = label;
    let touch = if vertical {
        "touch-action: pan-x"
    } else {
        "touch-action: pan-y"
    };

    view! {
        <div
            class=move || slider_root_class(&root_class, handles.disabled.get().unwrap_or(false), vertical, inverted, handles.dragging.get())
            style=style_attr
            id=id_attr
            data-disabled=move || handles.disabled.get().unwrap_or(false).then_some("true")
            data-orientation=orientation_attr
            data-dragging=move || handles.dragging.get().then_some("true")
            data-qa="dt-slider"
        >
            {show_label.then(|| view! {
                <div id=label_id.clone() class=label_class.clone() data-qa="dt-slider-label">
                    {if let Some(slot) = label_slot {
                        slot.run()
                    } else if !label_text.trim().is_empty() {
                        let label_text = label_text.clone();
                        view! {
                            {move || {
                                let tone = if handles.disabled.get().unwrap_or(false) {
                                    "disabled"
                                } else {
                                    "secondary"
                                };
                                let label_text = label_text.clone();
                                view! {
                                    <DtText kind="label" size=label_size strength=label_strength tone=tone>
                                        {label_text}
                                    </DtText>
                                }
                            }}
                        }.into_any()
                    } else {
                        ().into_any()
                    }}
                </div>
            })}
            <div class=body data-qa="dt-slider-body">
                {start.map(|slot| view! {
                    <div class=start_class.clone() data-qa="dt-slider-start">{slot.run()}</div>
                })}
                <div
                    node_ref=handles.control
                    class="d-slider__control"
                    style=touch
                    data-qa="dt-slider-control"
                    on:pointerdown=move |event: web_sys::PointerEvent| handles.pointer_down(&event)
                    on:pointermove=move |event: web_sys::PointerEvent| handles.pointer_move(&event)
                    on:pointerup=move |event: web_sys::PointerEvent| handles.end_drag(Some(event.pointer_id()))
                    on:pointercancel=move |event: web_sys::PointerEvent| handles.end_drag(Some(event.pointer_id()))
                >
                    <div class="d-slider__track" data-qa="dt-slider-track"></div>
                    <div
                        class="d-slider__indicator"
                        style=move || indicator_style(&handles.values.get(), &handles.bounds_from(true), fill, is_range_value(handles.value.get()), vertical)
                        data-qa="dt-slider-indicator"
                    ></div>
                    {move || {
                        let bounds = handles.bounds_from(true);
                        let rtl = handles.rtl.get();
                        tick_values(handles.ticks.get_value(), &bounds)
                            .into_iter()
                            .map(|tick| {
                                let style = tick_position_style(tick, &bounds, vertical, rtl);
                                view! { <div class="d-slider__tick" style=style data-qa="dt-slider-tick"></div> }
                            })
                            .collect_view()
                    }}
                    {thumb_view(handles, 0, name_attr.clone(), aria_label_attr.clone(), aria_labelledby_attr.clone(), aria_describedby_attr.clone(), aria_errormessage_attr.clone(), aria_details_attr.clone(), aria_invalid_attr.clone(), aria_orientation.clone(), touch)}
                    <Show when=move || handles.values.with(|values| values.len() > 1)>
                        {thumb_view(handles, 1, name_attr.clone(), aria_label_attr.clone(), aria_labelledby_attr.clone(), aria_describedby_attr.clone(), aria_errormessage_attr.clone(), aria_details_attr.clone(), aria_invalid_attr.clone(), aria_orientation.clone(), touch)}
                    </Show>
                    {move || {
                        let bounds = handles.bounds_from(true);
                        let rendered = build_marks(&handles.marks.get_value(), &bounds, &handles.prefix.get_value(), &handles.suffix.get_value());
                        rendered.into_iter().enumerate().map(|(index, mark)| {
                            let text = mark.text;
                            let pct = mark.pct;
                            let edge = mark.edge;
                            view! {
                                <div
                                    class=move || mark_class(handles.mark_hidden.with(|hidden| hidden.get(index).copied().unwrap_or(false)))
                                    style=move || {
                                        let offset = handles.mark_offsets.with(|offsets| offsets.get(index).copied().unwrap_or(0.0));
                                        label_style(pct, offset, edge, vertical, handles.rtl.get())
                                    }
                                    data-mark-index=index.to_string()
                                    data-qa="dt-slider-mark"
                                >
                                    {text}
                                </div>
                            }
                        }).collect_view()
                    }}
                    {if readout == DtSliderReadout::Always {
                        view! {
                            {always_readout(handles, 0, touch)}
                            <Show when=move || handles.values.with(|values| values.len() > 1)>
                                {always_readout(handles, 1, touch)}
                            </Show>
                            <Show when=move || handles.readout_merged.get()>
                                <div
                                    node_ref=handles.merged
                                    class=move || readout_class_name(true, &handles.readout_class.get_value())
                                    style=move || {
                                        let values = handles.values.get();
                                        let bounds = handles.bounds_from(true);
                                        let pct = if values.len() == 2 {
                                            (thumb_percent(values[0], &bounds) + thumb_percent(values[1], &bounds)) / 2.0
                                        } else {
                                            0.0
                                        };
                                        label_style(pct, handles.merged_offset.get(), None, vertical, handles.rtl.get())
                                    }
                                    aria-hidden="true"
                                    data-qa="dt-slider-thumb-readout-merged"
                                >
                                    {move || {
                                        let values = handles.values.get();
                                        merged_readout_text(&values, &handles.prefix.get_value(), &handles.suffix.get_value())
                                    }}
                                </div>
                            </Show>
                        }.into_any()
                    } else {
                        ().into_any()
                    }}
                    {if readout == DtSliderReadout::Tooltip {
                        view! {
                            <div
                                node_ref=handles.tooltips[0]
                                class="d-tooltip d-slider__readout-tooltip"
                                popover="manual"
                                inert
                                aria-hidden="true"
                                data-qa="dt-slider-thumb-readout-tooltip"
                            >
                                <span class="d-slider__tooltip-content">
                                    {move || handles.format_at(handles.values.with(|values| values.first().copied().unwrap_or(0.0)), 0)}
                                </span>
                            </div>
                            <Show when=move || handles.values.with(|values| values.len() > 1)>
                                <div
                                    node_ref=handles.tooltips[1]
                                    class="d-tooltip d-slider__readout-tooltip"
                                    popover="manual"
                                    inert
                                    aria-hidden="true"
                                    data-qa="dt-slider-thumb-readout-tooltip"
                                >
                                    <span class="d-slider__tooltip-content">
                                        {move || handles.format_at(handles.values.with(|values| values.get(1).copied().unwrap_or(0.0)), 1)}
                                    </span>
                                </div>
                            </Show>
                        }.into_any()
                    } else {
                        ().into_any()
                    }}
                </div>
                {end.map(|slot| view! {
                    <div class=end_class.clone() data-qa="dt-slider-end">{slot.run()}</div>
                })}
            </div>
        </div>
    }
}

fn always_readout(handles: Handles, index: usize, _touch: &'static str) -> impl IntoView {
    view! {
        <div
            class=move || readout_class_name(!handles.readout_merged.get(), &handles.readout_class.get_value())
            style=move || {
                let values = handles.values.get();
                let bounds = handles.bounds_from(true);
                let value = values.get(index).copied().unwrap_or(bounds.min);
                let offset = handles.readout_offsets.with(|offsets| offsets.get(index).copied().unwrap_or(0.0));
                label_style(thumb_percent(value, &bounds), offset, readout_edge(value, &bounds), handles.vertical, handles.rtl.get())
            }
            data-readout-index=index.to_string()
            aria-hidden="true"
            data-qa="dt-slider-thumb-readout"
        >
            {move || {
                let values = handles.values.get();
                let value = values.get(index).copied().unwrap_or(0.0);
                handles.format_at(value, index)
            }}
        </div>
    }
}

fn thumb_view(
    handles: Handles,
    index: usize,
    name_attr: Option<String>,
    aria_label_attr: Option<String>,
    aria_labelledby_attr: Option<String>,
    aria_describedby_attr: Option<String>,
    aria_errormessage_attr: Option<String>,
    aria_details_attr: Option<String>,
    aria_invalid_attr: Option<String>,
    aria_orientation: Option<String>,
    touch: &'static str,
) -> impl IntoView {
    view! {
        <div
            node_ref=handles.visuals[index]
            class=move || thumb_visual_class(handles.focused.get() == Some(index), handles.active.get() == Some(index))
            style=move || {
                let values = handles.values.get();
                let bounds = handles.bounds_from(true);
                let value = values.get(index).copied().unwrap_or(bounds.min);
                thumb_position_style(value, &bounds, handles.vertical, handles.rtl.get())
            }
            data-qa="dt-slider-thumb-visual"
        ></div>
        <div
            class="d-slider__thumb-hit"
            style=move || {
                let values = handles.values.get();
                let bounds = handles.bounds_from(true);
                let value = values.get(index).copied().unwrap_or(bounds.min);
                let position = thumb_position_style(value, &bounds, handles.vertical, handles.rtl.get());
                format!("{position}; {touch}")
            }
            data-qa="dt-slider-thumb-hit"
            on:pointerenter=move |_| handles.thumb_enter(index)
            on:pointerleave=move |_| handles.thumb_leave(index)
        ></div>
        <input
            node_ref=handles.thumbs[index]
            type="range"
            class="d-slider__thumb"
            min=move || {
                let bounds = handles.bounds_from(true);
                js_num(thumb_native_min(index, is_range_value(handles.value.get()), &handles.values.get(), &bounds))
            }
            max=move || {
                let bounds = handles.bounds_from(true);
                js_num(thumb_native_max(index, is_range_value(handles.value.get()), &handles.values.get(), &bounds))
            }
            step=move || thumb_native_step(index, &handles.values.get(), &handles.bounds_from(true))
            prop:value=move || {
                let values = handles.values.get();
                js_num(values.get(index).copied().unwrap_or(0.0))
            }
            prop:disabled=move || handles.disabled.get().unwrap_or(false)
            name=name_attr
            aria-labelledby=aria_labelledby_attr
            aria-label=aria_label_attr
            aria-describedby=aria_describedby_attr
            aria-errormessage=aria_errormessage_attr
            aria-details=aria_details_attr
            aria-invalid=aria_invalid_attr
            aria-valuetext=move || {
                let values = handles.values.get();
                let value = values.get(index).copied().unwrap_or(0.0);
                handles.format_at(value, index)
            }
            aria-orientation=aria_orientation
            data-qa="dt-slider-thumb"
            on:input=move |event| {
                let Ok(raw) = event_target_value(&event).parse::<f64>() else {
                    return;
                };
                handles.apply_thumb(index, raw, false);
            }
            on:keydown=move |event: web_sys::KeyboardEvent| handles.thumb_keydown(index, &event)
            on:focus=move |event: web_sys::FocusEvent| handles.thumb_focus(index, event)
            on:blur=move |event: web_sys::FocusEvent| handles.thumb_blur(event)
        />
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bounds(min: f64, max: f64, step: f64, gap: f64) -> Bounds {
        Bounds::new(min, max, step, gap)
    }

    #[test]
    fn js_string_matches_number_tostring() {
        assert_eq!(js_num(5e-7), "5e-7");
        assert_eq!(js_num(0.1), "0.1");
        assert_eq!(js_num(1.0), "1");
        assert_eq!(js_num(0.96), "0.96");
        assert_eq!(js_num(1e-6), "0.000001");
        assert_eq!(js_num(1e-7), "1e-7");
        assert_eq!(js_num(1e20), "100000000000000000000");
        assert_eq!(js_num(1e21), "1e+21");
        assert_eq!(decimal_places(5e-7), 7);
        assert_eq!(decimal_places(0.1), 1);
        assert_eq!(decimal_places(1.0), 0);
        assert_eq!(decimal_places(0.96), 2);
        assert_eq!(js_to_fixed(1.005, 2), "1.00");
        assert_eq!(js_to_fixed(-1.5, 0), "-2");
        assert_eq!(js_round(-1.5), -1.0);
    }

    #[test]
    fn snap_gap_and_native_bounds_match_vue() {
        let unit = bounds(0.0, 100.0, 1.0, 0.0);
        assert_eq!(snap_to_step(1.4, &unit), 1.0);
        let coarse = bounds(0.0, 100.0, 25.0, 0.0);
        assert_eq!(snap_to_step(42.0, &coarse), 50.0);
        let gapped = bounds(0.0, 100.0, 10.0, 1.0);
        assert_eq!(
            apply_value_constraints(&[80.0, 85.0], &gapped, None),
            vec![80.0, 90.0]
        );
        assert_eq!(
            apply_value_constraints(&[f64::NAN], &unit, None),
            vec![50.0]
        );
        let pair = [20.0, 80.0];
        assert_eq!(thumb_native_min(1, true, &pair, &unit), 20.0);
        assert_eq!(thumb_native_max(0, true, &pair, &unit), 80.0);
        let wide = bounds(0.0, 100.0, 10.0, 2.0);
        assert_eq!(thumb_native_min(1, true, &pair, &wide), 40.0);
        assert_eq!(thumb_native_step(0, &[42.0], &coarse), "any");
        assert_eq!(thumb_native_step(0, &[50.0], &coarse), "25");
        assert_eq!(large_step_value(25.0, 10.0), 25.0);
        assert_eq!(
            apply_value_constraints(&[50.0], &bounds(200.0, 100.0, 1.0, 0.0), None),
            vec![100.0]
        );
        assert_eq!(
            apply_value_constraints(&[70.0, 30.0], &unit, None),
            vec![30.0, 70.0]
        );
    }

    #[test]
    fn interval_marks_and_magnetic_snap() {
        let (ticks, warning) = generate_interval(0.0, 100.0, 25.0, "ticks");
        assert!(warning.is_none());
        assert_eq!(ticks, vec![0.0, 25.0, 50.0, 75.0, 100.0]);
        let (wide, warning) = generate_interval(0.0, 100.0, 0.001, "ticks");
        let warning = warning.expect("widened interval");
        assert!(warning.contains("ticks=0.001"));
        assert!(warning.contains("0.1001001001001001"));
        assert_eq!(wide.first().copied(), Some(0.0));
        // Vue's 1e-9 end guard does not append max: toFixed(10) lands 1e-10 short.
        assert_eq!(js_num(wide.last().copied().unwrap()), "99.9999999999");
        assert_eq!(wide.len(), MAX_GENERATED_POINTS);

        let step = bounds(0.0, 100.0, 25.0, 0.0);
        let pulled = update_thumb(
            &[75.0],
            &[None],
            0,
            42.0,
            true,
            &step,
            false,
            &[42.0],
            TrackSize::Px(200.0),
        );
        assert_eq!(pulled.values, vec![42.0]);
        assert_eq!(pulled.snaps, vec![Some(42.0)]);
        let released = update_thumb(
            &pulled.values,
            &pulled.snaps,
            0,
            42.0,
            false,
            &step,
            false,
            &[],
            TrackSize::Missing,
        );
        assert_eq!(released.values, vec![50.0]);
        let sticky = update_thumb(
            &[42.0],
            &[Some(42.0)],
            0,
            48.0,
            true,
            &step,
            false,
            &[42.0],
            TrackSize::Px(200.0),
        );
        assert_eq!(sticky.values, vec![42.0]);
        let dropped = update_thumb(
            &[42.0],
            &[Some(42.0)],
            0,
            42.0,
            true,
            &step,
            false,
            &[],
            TrackSize::Missing,
        );
        assert_eq!(dropped.snaps, vec![None]);
        let kept = update_thumb(
            &[42.0],
            &[Some(42.0)],
            0,
            42.0,
            true,
            &step,
            false,
            &[42.0],
            TrackSize::Zero,
        );
        assert_eq!(kept.snaps, vec![Some(42.0)]);
        assert_eq!(kept.values, vec![50.0]);
    }

    #[test]
    fn sync_preserves_a_held_echo_and_commits_bounds() {
        let step = bounds(0.0, 100.0, 25.0, 0.0);
        let mount = sync_slider(
            DtSliderValue::Single(1.4),
            &bounds(0.0, 100.0, 1.0, 0.0),
            None,
            &[],
            &[],
            &[],
        );
        assert_eq!(mount.values, vec![1.0]);
        assert_eq!(mount.emit_input, Some(DtSliderValue::Single(1.0)));
        assert!(mount.emit_change.is_none());

        let prev = (DtSliderValue::Single(75.0), step);
        let echo = sync_slider(
            DtSliderValue::Single(42.0),
            &step,
            Some(&prev),
            &[42.0],
            &[75.0],
            &[Some(42.0)],
        );
        assert_eq!(echo.values, vec![42.0]);
        assert_eq!(echo.committed, vec![75.0]);
        assert!(echo.emit_input.is_none());
        assert!(echo.emit_change.is_none());

        let idle = (DtSliderValue::Single(80.0), bounds(0.0, 100.0, 1.0, 0.0));
        let shrunk = sync_slider(
            DtSliderValue::Single(80.0),
            &bounds(0.0, 40.0, 1.0, 0.0),
            Some(&idle),
            &[80.0],
            &[80.0],
            &[],
        );
        assert_eq!(shrunk.values, vec![40.0]);
        assert_eq!(shrunk.emit_input, Some(DtSliderValue::Single(40.0)));
        assert_eq!(shrunk.emit_change, Some(DtSliderValue::Single(40.0)));

        let dragging = sync_slider(
            DtSliderValue::Single(80.0),
            &bounds(0.0, 40.0, 1.0, 0.0),
            Some(&idle),
            &[80.0],
            &[50.0],
            &[],
        );
        assert_eq!(dragging.values, vec![40.0]);
        assert_eq!(dragging.committed, vec![50.0]);
        assert!(dragging.emit_change.is_none());
        assert!(should_restore_focus(2, 1, Some(1), false));
        assert!(!should_restore_focus(2, 1, Some(0), false));
        assert!(should_restore_focus(2, 1, None, true));
    }

    #[test]
    fn geometry_strings_match_the_vue_indicator_assertions() {
        let bounds = bounds(0.0, 100.0, 1.0, 0.0);
        let start = indicator_style(&[0.0], &bounds, DtSliderFill::Start, false, false);
        assert!(start.contains("inset-inline-start: 0;"));
        assert!(start.contains(
            "width: calc(var(--slider-edge-inset) + (100% - var(--slider-edge-inset) * 2) * 0)"
        ));
        let end = indicator_style(&[25.0], &bounds, DtSliderFill::End, false, false);
        assert!(end.contains("inset-inline-end: 0;"));
        assert!(end.contains(
            "width: calc(100% - calc(var(--slider-edge-inset) + (100% - var(--slider-edge-inset) * 2) * 0.25))"
        ));
        let origin = indicator_style(&[75.0], &bounds, DtSliderFill::At(50.0), false, false);
        assert!(origin.contains(
            "inset-inline-start: calc(var(--slider-edge-inset) + (100% - var(--slider-edge-inset) * 2) * 0.5)"
        ));
        assert!(origin.contains("width: calc((100% - var(--slider-edge-inset) * 2) * 0.25)"));
        let below = indicator_style(&[25.0], &bounds, DtSliderFill::At(50.0), false, false);
        assert!(below.contains(
            "inset-inline-start: calc(var(--slider-edge-inset) + (100% - var(--slider-edge-inset) * 2) * 0.25)"
        ));
        assert!(below.contains("width: calc((100% - var(--slider-edge-inset) * 2) * 0.25)"));
        let clamped = indicator_style(&[75.0], &bounds, DtSliderFill::At(200.0), false, false);
        assert!(clamped.contains("* 0.75)"));
        let range = indicator_style(&[20.0, 80.0], &bounds, DtSliderFill::End, true, false);
        assert!(range.contains(
            "inset-inline-start: calc(var(--slider-edge-inset) + (100% - var(--slider-edge-inset) * 2) * 0.2)"
        ));
        assert!(range.contains("width: calc((100% - var(--slider-edge-inset) * 2) * 0.6)"));
        let vertical = indicator_style(&[25.0], &bounds, DtSliderFill::Start, false, true);
        assert!(vertical.contains("bottom: 0;"));
        assert!(vertical.contains("height: calc(var(--slider-edge-inset)"));
        let vertical_end = indicator_style(&[25.0], &bounds, DtSliderFill::End, false, true);
        assert!(vertical_end.contains("top: 0;"));

        assert_eq!(
            mark_edge_transform(MarkEdge::Start, false),
            "translateX(1px)"
        );
        assert_eq!(
            mark_edge_transform(MarkEdge::End, false),
            "translateX(calc(-100% + -1px))"
        );
        let readout = label_style(0.0, 0.0, Some(MarkEdge::Start), false, false);
        assert!(readout.contains("inset-inline-start: 0%;"));
        assert!(readout.contains("transform: translateX(1px)"));
        assert!(!readout.contains(EDGE_INSET));
        let thumb = thumb_position_style(0.0, &bounds, false, true);
        assert!(thumb.contains("translate(50%, -50%)"));
        let vertical_thumb = thumb_position_style(0.0, &bounds, true, false);
        assert!(vertical_thumb.contains("translate(-50%, 50%)"));
    }

    #[test]
    fn classes_pointer_math_and_warnings() {
        assert_eq!(
            slider_root_class("extra", true, true, true, true),
            "d-slider extra d-slider--disabled d-slider--vertical d-slider--inverted d-slider--dragging"
        );
        assert_eq!(
            slider_root_class("", false, false, false, false),
            "d-slider"
        );
        assert!(!reserves_annotation_space(
            false,
            0,
            DtSliderReadout::Tooltip
        ));
        assert!(reserves_annotation_space(
            false,
            1,
            DtSliderReadout::Tooltip
        ));
        assert!(reserves_annotation_space(false, 0, DtSliderReadout::Always));
        assert!(!reserves_annotation_space(true, 3, DtSliderReadout::Always));
        assert_eq!(
            slider_body_class("extra", true),
            "d-slider__body extra d-slider__body--reserve-annotation-space"
        );
        assert_eq!(large_step_delta("ArrowRight", true, true), -1);
        assert_eq!(large_step_delta("ArrowLeft", true, true), 1);
        assert_eq!(large_step_delta("PageUp", false, false), 1);
        let unit = bounds(0.0, 100.0, 1.0, 0.0);
        let at_min = value_from_pointer(8.0, 0.0, 0.0, 0.0, 200.0, 20.0, 8.0, false, false, &unit);
        assert!((at_min - 0.0).abs() < 1e-9);
        assert_eq!(nearest_thumb(40.0, &[20.0, 80.0], true, None), 0);
        assert_eq!(nearest_thumb(10.0, &[40.0, 40.0], true, Some(0)), 1);
        assert_eq!(coincident_handoff(0, 41.0, 40.0, 40.0, true), 1);
        assert_eq!(coincident_handoff(0, 41.0, 40.0, 40.0, false), 0);
        assert_eq!(edge_clamp_offset(0.0, 10.0, 8.0, 100.0, 32.0), 0.0);
        assert_eq!(edge_clamp_offset(-10.0, 10.0, 0.0, 100.0, 0.0), 10.0);
        let rect = analytical_readout_rect(
            0.0,
            BoxRect {
                left: 0.0,
                right: 20.0,
                top: 0.0,
                bottom: 10.0,
            },
            BoxRect {
                left: 0.0,
                right: 200.0,
                top: 0.0,
                bottom: 20.0,
            },
            10.0,
            false,
            false,
        );
        assert!((rect.left - 0.0).abs() < 1e-9);
        assert!((rect.right - 20.0).abs() < 1e-9);
        let (hidden, offsets) = mark_collision_results(
            &[MarkMeasure {
                index: 0,
                natural_left: -40.0,
                natural_right: 20.0,
                top: 0.0,
                bottom: 10.0,
            }],
            1,
            Some((0.0, 100.0)),
            &[],
        );
        assert!(!hidden[0]);
        assert_eq!(offsets[0], 8.0);
        let warning = fractional_gap_warning(1.2).expect("fraction");
        assert!(warning.contains("minGapSteps (1.2)"));
        assert!(warning.contains("rounded to 1"));
        assert!(min_max_warning(2.0, 1.0)
            .unwrap()
            .contains("collapse to max"));
        assert_eq!(format_value(8.0, 0, "$", "", None, &mut false), "$8");
        let mut warned = false;
        assert_eq!(
            format_value(8.0, 0, "$", "", Some(Err("nope".to_string())), &mut warned),
            "$8"
        );
        assert!(warned);
        assert_eq!(merged_readout_text(&[20.0, 80.0], "", "%"), "20%–80%");
        assert!(readout_open(DtSliderReadout::Tooltip, 0, Some(0), None, None, Some(0)) == false);
        assert!(readout_open(
            DtSliderReadout::Always,
            0,
            None,
            None,
            None,
            Some(0)
        ));
    }
}
