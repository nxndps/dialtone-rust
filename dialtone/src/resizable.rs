//! Dialtone Vue `DtResizable`, `DtResizablePanel`, and `DtResizableHandle`.
//!
//! Tag `dialtone/v10.7.0`. Panels share one inline axis. `direction` `column`
//! uses `writing-mode: vertical-lr` so the same insets stack vertically.
//!
//! Child panels register the layout. The Vue `panels` prop is accepted and not
//! applied. `spaceAllocationStrategy` is accepted and not read by the layout
//! engine. A custom `storage` adapter is not ported. `storageKey` uses
//! `localStorage` and does not keep the in-memory drag ratio. Slot props from
//! the Vue default slot are not forwarded.

use std::cell::RefCell;
use std::collections::HashMap;

use leptos::context::Provider;
use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, KeyboardEvent, PointerEvent};

/// `DEFAULT_PANEL_SIZE`. Used when a panel sets no `initialSize`.
pub const DEFAULT_PANEL_SIZE: &str = "50p";

/// `MIN_PANEL_SIZE_PX`. A resize below this on either panel is rejected.
pub const MIN_PANEL_SIZE_PX: f64 = 10.0;

/// `RESIZABLE_HANDLE_CENTER_OFFSET_PX`. The handle is 4px and sits on the boundary.
pub const RESIZABLE_HANDLE_CENTER_OFFSET_PX: f64 = 2.0;

const LAYOUT_BASE_PX: f64 = 64.0;

/// Dialtone layout tokens accepted by size props. Percentages (`50p`) are separate.
const LAYOUT_SIZE_VALUES: &[&str] = &[
    "0", "1px", "2px", "8px", "25", "20px", "24px", "50", "75", "100", "125", "150", "175", "200",
    "250", "300", "350", "400", "450", "500", "550", "600", "650", "700", "750", "800", "850",
    "900", "950", "1000", "1050", "1100", "1150", "1200", "1250", "1300", "1350", "1400", "1450",
    "1500", "1550", "1600",
];

const KEYBOARD_FINE: f64 = 1.0;
const KEYBOARD_NORMAL: f64 = 8.0;
const KEYBOARD_LARGE: f64 = 24.0;

const DEFAULT_RESIZE_ANNOUNCEMENT: &str = "{beforeId}: {beforePx}px, {afterId}: {afterPx}px";
const DEFAULT_COLLAPSE_ANNOUNCEMENT: &str = "{panelId} collapsed";
const DEFAULT_EXPAND_ANNOUNCEMENT: &str = "{panelId} expanded";
const DEFAULT_RESET_ANNOUNCEMENT: &str = "{beforeId} and {afterId} reset";
const DEFAULT_ARIA_VALUE_TEXT: &str = "{panelId}: {pixels}px";
const DEFAULT_HANDLE_ARIA_LABEL: &str = "Resize handle between {before} and {after} panels";

thread_local! {
    static TOKEN_CACHE: RefCell<HashMap<String, f64>> = RefCell::new(HashMap::new());
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static ROOT_FONT_PX: RefCell<Option<f64>> = const { RefCell::new(None) };
}

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// `buildHandleId`. Handles use `{before}:{after}`.
pub fn build_handle_id(before_id: &str, after_id: &str) -> String {
    format!("{before_id}:{after_id}")
}

/// Clear the layout-token cache. Vue `invalidateTokenCache`.
pub fn invalidate_token_cache() {
    TOKEN_CACHE.with(|cache| cache.borrow_mut().clear());
    #[cfg(target_arch = "wasm32")]
    ROOT_FONT_PX.with(|font| *font.borrow_mut() = None);
}

/// Vue `isValidSizing`. A layout token or a `0`–`100` percentage with a `p` suffix.
pub fn is_valid_sizing(value: &str) -> bool {
    is_size_token(value) || parse_percentage(value).is_some()
}

/// Vue `isPercentageValue`.
pub fn is_percentage_value(value: &str) -> bool {
    parse_percentage(value).is_some()
}

/// Vue `isCSSValue`. True for a layout token, including `8px`.
pub fn is_css_value(value: &str) -> bool {
    is_size_token(value)
}

/// Vue `hasPercentageMinSize`.
pub fn has_percentage_min_size(user_min_size: Option<&str>) -> bool {
    user_min_size.is_some_and(is_percentage_value)
}

/// Vue `pixelsToPercentage`.
pub fn pixels_to_percentage(pixels: f64, container_size: f64) -> f64 {
    (pixels / container_size) * 100.0
}

/// Vue `validateContainerSize`.
pub fn validate_container_size(container_size: f64) -> f64 {
    if !container_size.is_finite() || container_size < 0.0 {
        leptos::logging::warn!(
            "[resizable] Invalid containerSize: {container_size}. Using fallback value of 1000px."
        );
        return 1000.0;
    }
    if container_size > 10000.0 {
        leptos::logging::warn!(
            "[resizable] Unusually large containerSize: {container_size}px. Capping at 10000px."
        );
        return 10000.0;
    }
    container_size
}

/// Vue `parseSizeToPixels`.
///
/// Layout tokens use `--dt-layout-*` when that value is `Nrem` or `Npx`.
/// `calc(...)` tokens fall through to `(stop * 64) / 100`, so `500` is 320.
/// `clamp_to_container` defaults to true. `collapseSize` passes false.
pub fn parse_size_to_pixels(value: &str, container_size: f64, clamp_to_container: bool) -> f64 {
    let container_size = validate_container_size(container_size);
    if container_size == 0.0 && value == "0" {
        return 0.0;
    }
    let calculation_container = if container_size == 0.0 {
        1000.0
    } else {
        container_size
    };
    let Some(result) = parse_sizing_to_pixels(value, calculation_container) else {
        leptos::logging::warn!(
            "[resizable] Invalid ResizableSizeValue: {value}. Expected a size token or percentage with 'p' suffix."
        );
        return 0.0;
    };
    if !result.is_finite() || result < 0.0 {
        leptos::logging::warn!(
            "[resizable] Invalid pixel calculation result: {result} for value: {value}, containerSize: {container_size}"
        );
        return 0.0;
    }
    if clamp_to_container && container_size > 0.0 && result > container_size {
        leptos::logging::warn!(
            "[resizable] Size value '{value}' ({result}px) exceeds container ({container_size}px). Clamping to container."
        );
        return container_size;
    }
    result
}

fn is_size_token(value: &str) -> bool {
    LAYOUT_SIZE_VALUES.contains(&value)
}

fn parse_percentage(value: &str) -> Option<f64> {
    let number = value.strip_suffix('p')?;
    let number = number.parse::<f64>().ok()?;
    (number.is_finite() && (0.0..=100.0).contains(&number)).then_some(number)
}

fn parse_sizing_to_pixels(value: &str, container_size: f64) -> Option<f64> {
    if let Some(pixels) = resolve_token_pixels(value) {
        return Some(pixels);
    }
    parse_percentage(value).map(|percentage| (percentage / 100.0) * container_size)
}

fn resolve_token_pixels(token: &str) -> Option<f64> {
    if !is_size_token(token) {
        return None;
    }
    if let Some(cached) = TOKEN_CACHE.with(|cache| cache.borrow().get(token).copied()) {
        return Some(cached);
    }
    let pixels = css_token_pixels(token).or_else(|| fallback_layout_token_pixels(token))?;
    TOKEN_CACHE.with(|cache| cache.borrow_mut().insert(token.to_string(), pixels));
    Some(pixels)
}

fn fallback_layout_token_pixels(token: &str) -> Option<f64> {
    if token == "0" {
        return Some(0.0);
    }
    if let Some(pixels) = token.strip_suffix("px") {
        return pixels.parse::<f64>().ok();
    }
    let stop = token.parse::<f64>().ok()?;
    stop.is_finite().then_some((stop * LAYOUT_BASE_PX) / 100.0)
}

fn css_token_pixels(token: &str) -> Option<f64> {
    #[cfg(target_arch = "wasm32")]
    {
        return css_token_pixels_dom(token);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = token;
        None
    }
}

#[cfg(target_arch = "wasm32")]
fn css_token_pixels_dom(token: &str) -> Option<f64> {
    let window = web_sys::window()?;
    let document = window.document()?;
    let root = document.document_element()?;
    let style = window.get_computed_style(&root).ok().flatten()?;
    let css_value = style
        .get_property_value(&format!("--dt-layout-{token}"))
        .ok()?;
    let css_value = css_value.trim();
    if let Some(rem) = css_value.strip_suffix("rem") {
        let rem = rem.trim().parse::<f64>().ok()?;
        return Some(rem * root_font_px());
    }
    css_value
        .strip_suffix("px")
        .and_then(|px| px.trim().parse::<f64>().ok())
}

#[cfg(target_arch = "wasm32")]
fn root_font_px() -> f64 {
    if let Some(cached) = ROOT_FONT_PX.with(|font| *font.borrow()) {
        return cached;
    }
    let parsed = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.document_element())
        .and_then(|root| window_font_px(&root))
        .unwrap_or(10.0);
    ROOT_FONT_PX.with(|font| *font.borrow_mut() = Some(parsed));
    parsed
}

#[cfg(target_arch = "wasm32")]
fn window_font_px(root: &web_sys::Element) -> Option<f64> {
    let window = web_sys::window()?;
    let style = window.get_computed_style(root).ok().flatten()?;
    let font = style.get_property_value("font-size").ok()?;
    font.trim().strip_suffix("px")?.trim().parse::<f64>().ok()
}

/// One panel's size props. Vue `ResizablePanelConfig`.
#[derive(Clone, Debug, PartialEq)]
pub struct DtResizablePanelConfig {
    pub id: String,
    /// `initialSize`.
    pub initial_size: Option<String>,
    /// `userMinSize`.
    pub user_min_size: Option<String>,
    /// `userMaxSize`.
    pub user_max_size: Option<String>,
    /// `systemMinSize`.
    pub system_min_size: Option<String>,
    /// `systemMaxSize`.
    pub system_max_size: Option<String>,
    /// `collapseSize`. Compared with the container, not the panel.
    pub collapse_size: Option<String>,
    pub resizable: bool,
    pub collapsible: bool,
    pub collapsed: bool,
}

impl DtResizablePanelConfig {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            initial_size: None,
            user_min_size: None,
            user_max_size: None,
            system_min_size: None,
            system_max_size: None,
            collapse_size: None,
            resizable: true,
            collapsible: false,
            collapsed: false,
        }
    }
}

/// `CollapseRule`. Lower `priority` collapses first.
#[derive(Clone, Debug, PartialEq)]
pub struct DtResizableCollapseRule {
    pub panel_id: String,
    pub priority: i32,
    /// `minSizeBeforeCollapse`. Falls back to the panel `userMinSize`.
    pub min_size_before_collapse: Option<String>,
}

/// `messages` overrides. Empty strings keep the English default.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DtResizableMessages {
    pub resize_announcement: String,
    pub collapse_announcement: String,
    pub expand_announcement: String,
    pub reset_announcement: String,
    pub aria_value_text: String,
    pub handle_aria_label: String,
}

/// Persisted panel. `manualTargetRatio` stays in memory and is not written.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DtResizableSavedPanel {
    pub id: String,
    #[serde(rename = "pixelSize")]
    pub pixel_size: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collapsed: Option<bool>,
    #[serde(
        rename = "autoCollapsed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_collapsed: Option<bool>,
    #[serde(
        rename = "manualTargetRatio",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub manual_target_ratio: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
struct ConstraintHierarchy {
    user_min: Option<f64>,
    user_max: Option<f64>,
    system_min: Option<f64>,
    system_max: Option<f64>,
    collapse_size: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
struct PanelPosition {
    id: String,
    left: f64,
    right: f64,
    width: f64,
    collapsed: bool,
    constraints: ConstraintHierarchy,
}

#[derive(Clone, Debug, PartialEq)]
struct HandlePosition {
    id: String,
    before_panel_id: String,
    after_panel_id: String,
    left: f64,
    disabled: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct LayoutResult {
    panels: HashMap<String, PanelPosition>,
    handles: Vec<HandlePosition>,
}

impl LayoutResult {
    fn empty() -> Self {
        Self {
            panels: HashMap::new(),
            handles: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
struct WorkingPanel {
    config: DtResizablePanelConfig,
    constraints: ConstraintHierarchy,
    width: f64,
    collapsed: bool,
    is_fixed: bool,
    manual_target_ratio: Option<f64>,
}

#[derive(Clone, Debug)]
struct PanelRuntime {
    id: String,
    pixel_size: f64,
    resizable: bool,
    collapsible: bool,
    collapsed: bool,
    user_min: Option<f64>,
    user_max: Option<f64>,
    initial_size: Option<String>,
    auto_collapsed: bool,
}

struct ResizeMove {
    cursor: f64,
    before_size: f64,
    after_size: f64,
    valid: bool,
}

#[derive(Clone, Debug)]
struct DragPreview {
    handle_id: String,
    before_id: String,
    after_id: String,
    original_before: f64,
    original_after: f64,
    before_left: f64,
    proposed_before: f64,
    proposed_after: f64,
    before_end: f64,
    after_start: f64,
    after_end: f64,
    handle_start: f64,
    origin_x: f64,
    origin_y: f64,
}

struct PreCollapse {
    sizes: HashMap<String, f64>,
    container_size: f64,
}

fn clamp_size(value: f64, min: Option<f64>, max: Option<f64>) -> f64 {
    let mut result = value;
    if let Some(min) = min {
        result = result.max(min);
    }
    if let Some(max) = max {
        result = result.min(max);
    }
    result
}

fn clamp_to_tier(value: f64, constraints: &ConstraintHierarchy, user_tier: bool) -> f64 {
    let (min, max) = if user_tier {
        (constraints.user_min, constraints.user_max)
    } else {
        (
            constraints.system_min.or(constraints.user_min),
            constraints.system_max.or(constraints.user_max),
        )
    };
    clamp_size(value, min, max)
}

fn constraint_hierarchy(
    config: &DtResizablePanelConfig,
    container_size: f64,
) -> ConstraintHierarchy {
    let user_min = config
        .user_min_size
        .as_deref()
        .map(|value| parse_size_to_pixels(value, container_size, true));
    let user_max = config
        .user_max_size
        .as_deref()
        .map(|value| parse_size_to_pixels(value, container_size, true));
    let raw_system_min = config
        .system_min_size
        .as_deref()
        .map(|value| parse_size_to_pixels(value, container_size, true))
        .or(user_min);
    let raw_system_max = config
        .system_max_size
        .as_deref()
        .map(|value| parse_size_to_pixels(value, container_size, true))
        .or(user_max);
    let system_min = match (raw_system_min, user_min) {
        (Some(system), Some(user)) => Some(system.max(user)),
        (system, _) => system,
    };
    let system_max = match (raw_system_max, user_max) {
        (Some(system), Some(user)) => Some(system.min(user)),
        (system, _) => system,
    };
    let collapse_size = config
        .collapse_size
        .as_deref()
        .map(|value| parse_size_to_pixels(value, container_size, false));
    ConstraintHierarchy {
        user_min,
        user_max,
        system_min,
        system_max,
        collapse_size,
    }
}

fn compute_layout(
    panels: &[DtResizablePanelConfig],
    container_size: f64,
    saved: Option<&[DtResizableSavedPanel]>,
) -> LayoutResult {
    if panels.is_empty() {
        return LayoutResult::empty();
    }
    let saved_index: HashMap<&str, &DtResizableSavedPanel> = saved
        .into_iter()
        .flatten()
        .map(|panel| (panel.id.as_str(), panel))
        .collect();
    let mut working = build_working_panels(panels, container_size, &saved_index);
    distribute_space(&mut working, container_size);
    round_widths(&mut working, container_size);
    let positions = compute_positions(&working, container_size);
    let handles = compute_handles(panels, &positions);
    LayoutResult {
        panels: positions,
        handles,
    }
}

fn build_working_panels(
    panels: &[DtResizablePanelConfig],
    container_size: f64,
    saved_index: &HashMap<&str, &DtResizableSavedPanel>,
) -> Vec<WorkingPanel> {
    panels
        .iter()
        .map(|config| {
            let constraints = constraint_hierarchy(config, container_size);
            let saved = saved_index.get(config.id.as_str()).copied();
            let collapsed = saved
                .and_then(|panel| panel.collapsed)
                .unwrap_or(config.collapsed);
            let saved_ratio = saved.and_then(|panel| panel.manual_target_ratio);
            let width = if collapsed {
                0.0
            } else if let Some(ratio) = saved_ratio {
                if container_size > 0.0 {
                    ratio * container_size
                } else {
                    resolve_raw_size(config, container_size, saved)
                }
            } else {
                resolve_raw_size(config, container_size, saved)
            };
            WorkingPanel {
                config: config.clone(),
                constraints,
                width,
                collapsed,
                is_fixed: !config.resizable,
                manual_target_ratio: saved_ratio,
            }
        })
        .collect()
}

fn resolve_raw_size(
    config: &DtResizablePanelConfig,
    container_size: f64,
    saved: Option<&DtResizableSavedPanel>,
) -> f64 {
    if let Some(saved) = saved {
        let size = saved.pixel_size;
        if !size.is_finite() || size < 0.0 || (container_size > 0.0 && size > container_size * 2.0)
        {
            return parse_size_to_pixels(
                config.initial_size.as_deref().unwrap_or(DEFAULT_PANEL_SIZE),
                container_size,
                true,
            );
        }
        return size;
    }
    parse_size_to_pixels(
        config.initial_size.as_deref().unwrap_or(DEFAULT_PANEL_SIZE),
        container_size,
        true,
    )
}

fn distribute_space(working: &mut [WorkingPanel], container_size: f64) {
    if container_size <= 0.0 {
        for panel in working.iter_mut() {
            if !panel.collapsed {
                panel.width = 0.0;
            }
        }
        return;
    }
    let visible: Vec<usize> = working
        .iter()
        .enumerate()
        .filter(|(_, panel)| !panel.collapsed)
        .map(|(index, _)| index)
        .collect();
    if visible.is_empty() {
        return;
    }
    let mut reserved = 0.0;
    for index in &visible {
        let panel = &mut working[*index];
        if panel.is_fixed {
            panel.width = clamp_to_tier(panel.width, &panel.constraints, false);
            reserved += panel.width;
        }
    }
    for index in &visible {
        let panel = &working[*index];
        if panel.is_fixed || panel.manual_target_ratio.is_none() {
            continue;
        }
        let target = panel.manual_target_ratio.unwrap_or(0.0) * container_size;
        let clamped = clamp_to_tier(target, &panel.constraints, true);
        reserved += clamped;
        working[*index].width = clamped;
    }
    let prop: Vec<usize> = visible
        .iter()
        .copied()
        .filter(|index| {
            let panel = &working[*index];
            !panel.is_fixed && panel.manual_target_ratio.is_none()
        })
        .collect();
    if prop.is_empty() {
        return;
    }
    let remaining = (container_size - reserved).max(0.0);
    let weight: f64 = prop
        .iter()
        .map(|index| working[*index].width.max(0.0))
        .sum();
    for index in &prop {
        working[*index].width = if weight > 0.0 {
            (working[*index].width / weight) * remaining
        } else {
            remaining / prop.len() as f64
        };
    }
    constrain_and_redistribute(working, &prop);
    let allocated: f64 = visible.iter().map(|index| working[*index].width).sum();
    let gap = container_size - allocated;
    if gap > 1.0 {
        if let Some(index) = prop.iter().rev().find(|index| !working[**index].is_fixed) {
            working[*index].width += gap;
        }
    }
}

fn constrain_and_redistribute(working: &mut [WorkingPanel], prop: &[usize]) {
    let max_passes = prop.len() + 1;
    for _ in 0..max_passes {
        let mut total_delta = 0.0;
        let mut clamped = Vec::new();
        for index in prop {
            let width = working[*index].width;
            let constraints = working[*index].constraints.clone();
            let id = working[*index].config.id.clone();
            let next = clamp_to_tier(width, &constraints, false);
            let delta = next - width;
            if delta != 0.0 {
                working[*index].width = next;
                clamped.push(id);
                total_delta += delta;
            }
        }
        if total_delta == 0.0 {
            break;
        }
        let free: Vec<usize> = prop
            .iter()
            .copied()
            .filter(|index| !clamped.iter().any(|id| id == &working[*index].config.id))
            .collect();
        if free.is_empty() {
            break;
        }
        let change = -total_delta / free.len() as f64;
        for index in free {
            working[index].width += change;
        }
    }
    for index in prop {
        let panel = &working[*index];
        working[*index].width = clamp_to_tier(panel.width, &panel.constraints, false);
    }
}

fn round_widths(working: &mut [WorkingPanel], container_size: f64) {
    if container_size <= 0.0 {
        return;
    }
    let visible: Vec<usize> = working
        .iter()
        .enumerate()
        .filter(|(_, panel)| !panel.collapsed)
        .map(|(index, _)| index)
        .collect();
    if visible.is_empty() {
        return;
    }
    let mut remainders = Vec::new();
    for index in &visible {
        let width = working[*index].width;
        let floored = width.floor();
        remainders.push((index, width - floored));
        working[*index].width = floored;
    }
    let current: f64 = visible.iter().map(|index| working[*index].width).sum();
    let mut remaining = container_size - current;
    remainders.sort_by(|left, right| {
        right
            .1
            .partial_cmp(&left.1)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    for (index, _) in remainders {
        if remaining <= 0.0 {
            break;
        }
        let panel = &working[*index];
        let max = panel
            .constraints
            .system_max
            .or(panel.constraints.user_max)
            .unwrap_or(f64::INFINITY);
        if panel.width < max && !panel.is_fixed {
            working[*index].width += 1.0;
            remaining -= 1.0;
        }
    }
}

fn compute_positions(
    working: &[WorkingPanel],
    container_size: f64,
) -> HashMap<String, PanelPosition> {
    let mut cursor = 0.0;
    let mut result = HashMap::new();
    for panel in working {
        let width = if panel.collapsed { 0.0 } else { panel.width };
        let left = cursor;
        let right = (container_size - left - width).max(0.0);
        result.insert(
            panel.config.id.clone(),
            PanelPosition {
                id: panel.config.id.clone(),
                left,
                right,
                width,
                collapsed: panel.collapsed,
                constraints: panel.constraints.clone(),
            },
        );
        cursor += width;
    }
    result
}

fn compute_handles(
    panels: &[DtResizablePanelConfig],
    positions: &HashMap<String, PanelPosition>,
) -> Vec<HandlePosition> {
    let mut handles = Vec::new();
    for pair in panels.windows(2) {
        let before = &pair[0];
        let after = &pair[1];
        if !before.resizable || !after.resizable {
            continue;
        }
        let Some(before_pos) = positions.get(&before.id) else {
            continue;
        };
        let Some(after_pos) = positions.get(&after.id) else {
            continue;
        };
        handles.push(HandlePosition {
            id: build_handle_id(&before.id, &after.id),
            before_panel_id: before.id.clone(),
            after_panel_id: after.id.clone(),
            left: before_pos.left + before_pos.width,
            disabled: before_pos.collapsed || after_pos.collapsed,
        });
    }
    handles
}

fn process_resize_move(
    raw_cursor: f64,
    before: &PanelRuntime,
    after: &PanelRuntime,
    container_size: f64,
    panels: &[PanelRuntime],
    before_left: f64,
) -> ResizeMove {
    if !before.resizable || !after.resizable {
        return ResizeMove {
            cursor: raw_cursor,
            before_size: before.pixel_size,
            after_size: after.pixel_size,
            valid: false,
        };
    }
    let proposed_before = raw_cursor - before_left;
    let mut before_size = clamp_size(proposed_before, before.user_min, before.user_max);
    let max_cursor = max_cursor_for_after(after, container_size, panels);
    let before_cursor = before_left + before_size;
    let effective = before_cursor.min(max_cursor);
    let cursor = if effective < before_cursor {
        before_size = effective - before_left;
        effective
    } else {
        before_cursor
    };
    let after_size = clamp_size(
        after_size_at(cursor, container_size, after, panels),
        after.user_min,
        after.user_max,
    );
    ResizeMove {
        cursor,
        before_size,
        after_size,
        valid: before_size >= MIN_PANEL_SIZE_PX && after_size >= MIN_PANEL_SIZE_PX,
    }
}

fn max_cursor_for_after(after: &PanelRuntime, container_size: f64, panels: &[PanelRuntime]) -> f64 {
    let Some(min) = after.user_min else {
        return container_size;
    };
    let Some(index) = panels.iter().position(|panel| panel.id == after.id) else {
        return container_size;
    };
    let after_fixed: f64 = panels
        .iter()
        .skip(index + 1)
        .map(|panel| panel.pixel_size)
        .sum();
    container_size - min - after_fixed
}

fn after_size_at(
    cursor: f64,
    container_size: f64,
    after: &PanelRuntime,
    panels: &[PanelRuntime],
) -> f64 {
    let Some(index) = panels.iter().position(|panel| panel.id == after.id) else {
        return container_size - cursor;
    };
    let after_fixed: f64 = panels
        .iter()
        .skip(index + 1)
        .map(|panel| panel.pixel_size)
        .sum();
    container_size - cursor - after_fixed
}

fn keyboard_increment(meta: bool, ctrl: bool, shift: bool) -> f64 {
    if meta || ctrl {
        KEYBOARD_FINE
    } else if shift {
        KEYBOARD_LARGE
    } else {
        KEYBOARD_NORMAL
    }
}

fn keyboard_direction(key: &str, layout_direction: &str) -> Option<bool> {
    let increase = if layout_direction == "row" {
        match key {
            "ArrowLeft" => Some(false),
            "ArrowRight" => Some(true),
            _ => None,
        }
    } else {
        match key {
            "ArrowUp" => Some(false),
            "ArrowDown" => Some(true),
            _ => None,
        }
    };
    increase
}

fn check_auto_collapse_rules(
    panels: &[PanelRuntime],
    rules: &[DtResizableCollapseRule],
    container_size: f64,
) -> Vec<String> {
    if panels.is_empty() || rules.is_empty() {
        return Vec::new();
    }
    let mut indexed: Vec<(usize, &DtResizableCollapseRule)> = rules.iter().enumerate().collect();
    indexed.sort_by(|left, right| {
        left.1
            .priority
            .cmp(&right.1.priority)
            .then(left.0.cmp(&right.0))
    });
    indexed
        .into_iter()
        .filter_map(|(_, rule)| {
            let panel = panels.iter().find(|panel| panel.id == rule.panel_id)?;
            if panel.collapsed {
                return None;
            }
            let threshold = rule
                .min_size_before_collapse
                .as_deref()
                .map(|value| parse_size_to_pixels(value, container_size, true))
                .or(panel.user_min)?;
            (panel.pixel_size <= threshold).then(|| panel.id.clone())
        })
        .collect()
}

fn reset_pair_sizes(
    before_px: f64,
    after_px: f64,
    before_initial: f64,
    after_initial: f64,
) -> (f64, f64) {
    let combined = before_px + after_px;
    let total_initial = before_initial + after_initial;
    let before = if total_initial > 0.0 {
        (combined * (before_initial / total_initial)).round()
    } else {
        (combined / 2.0).round()
    };
    (before, combined - before)
}

fn resizable_class(direction: &str, resizing: bool, extra: &str) -> String {
    join_class([
        "d-resizable",
        &format!("d-resizable--{direction}"),
        if resizing {
            "d-resizable--resizing"
        } else {
            ""
        },
        extra,
    ])
}

fn panel_class(extra: &str, collapsed: bool, fixed: bool) -> String {
    join_class([
        "d-resizable-panel",
        extra,
        if collapsed {
            "d-resizable-panel--collapsed"
        } else {
            ""
        },
        if fixed {
            "d-resizable-panel--fixed"
        } else {
            ""
        },
    ])
}

fn handle_class(direction: &str, extra: &str, active: bool, disabled: bool) -> String {
    join_class([
        "d-resizable-handle",
        &format!("d-resizable-handle--{direction}"),
        extra,
        if active {
            "d-resizable-handle--active"
        } else {
            ""
        },
        if disabled {
            "d-resizable-handle--disabled"
        } else {
            ""
        },
    ])
}

fn panel_style(position: Option<&PanelPosition>, drag: Option<&DragPreview>, id: &str) -> String {
    if let Some(drag) = drag {
        if drag.before_id == id {
            return format!(
                "inset-inline-start:{}px;inset-inline-end:{}px",
                drag.before_left, drag.before_end
            );
        }
        if drag.after_id == id {
            return format!(
                "inset-inline-start:{}px;inset-inline-end:{}px",
                drag.after_start, drag.after_end
            );
        }
    }
    match position {
        None => "inset-inline-start:0px;inline-size:0px;pointer-events:none".to_string(),
        Some(position) if position.collapsed => format!(
            "inset-inline-start:{}px;inline-size:0px;overflow:hidden;pointer-events:none",
            position.left
        ),
        Some(position) => format!(
            "inset-inline-start:{}px;inset-inline-end:{}px",
            position.left, position.right
        ),
    }
}

fn handle_style(
    position: Option<&HandlePosition>,
    drag: Option<&DragPreview>,
    offset: &str,
) -> String {
    let Some(position) = position else {
        return "visibility:hidden".to_string();
    };
    let start = if drag.is_some_and(|drag| drag.handle_id == position.id) {
        drag.unwrap().handle_start
    } else {
        (position.left - RESIZABLE_HANDLE_CENTER_OFFSET_PX).max(0.0)
    };
    let mut style = format!("inset-inline-start:{start}px");
    if !offset.is_empty() {
        style.push(';');
        style.push_str(offset);
    }
    style
}

fn offset_style(kind: &str, amount: f64, edge: &str) -> String {
    if amount <= 0.0 {
        return String::new();
    }
    let px = format!("{amount}px");
    let (start_name, end_name) = if kind == "handle" {
        ("inset-block-start", "inset-block-end")
    } else {
        ("padding-block-start", "padding-block-end")
    };
    match edge {
        "end" => format!("{end_name}:{px}"),
        "both" => format!("{start_name}:{px};{end_name}:{px}"),
        _ => format!("{start_name}:{px}"),
    }
}

fn parse_stored(json: &str) -> Option<Vec<DtResizableSavedPanel>> {
    let panels: Vec<DtResizableSavedPanel> = serde_json::from_str(json).ok()?;
    panels
        .iter()
        .all(|panel| {
            !panel.id.is_empty() && panel.pixel_size.is_finite() && panel.pixel_size >= 0.0
        })
        .then_some(panels)
}

fn serialize_stored(panels: &[DtResizableSavedPanel]) -> String {
    let slim: Vec<DtResizableSavedPanel> = panels
        .iter()
        .map(|panel| DtResizableSavedPanel {
            manual_target_ratio: None,
            ..panel.clone()
        })
        .collect();
    serde_json::to_string(&slim).unwrap_or_else(|_| "[]".to_string())
}

fn replace_tokens(template: &str, pairs: &[(&str, &str)]) -> String {
    let mut text = template.to_string();
    for (key, value) in pairs {
        text = text.replace(key, value);
    }
    text
}

fn message<'a>(custom: &'a str, fallback: &'a str) -> &'a str {
    if custom.is_empty() {
        fallback
    } else {
        custom
    }
}

#[derive(Clone, Copy)]
struct ResizableCtx {
    registered: RwSignal<Vec<DtResizablePanelConfig>>,
    saved: RwSignal<Option<Vec<DtResizableSavedPanel>>>,
    container_size: RwSignal<f64>,
    direction: Signal<String>,
    is_resizing: RwSignal<bool>,
    active_handle: RwSignal<Option<String>>,
    is_initializing: RwSignal<bool>,
    container: NodeRef<html::Div>,
    layout: Signal<LayoutResult>,
    drag: RwSignal<Option<DragPreview>>,
    pre_collapse: StoredValue<HashMap<String, PreCollapse>>,
    collapse_rules: StoredValue<Vec<DtResizableCollapseRule>>,
    messages: StoredValue<DtResizableMessages>,
    storage_key: StoredValue<String>,
    on_panel_resize: Option<Callback<(String, f64)>>,
    on_panel_collapse: Option<Callback<(String, bool)>>,
    on_resize_start: Option<Callback<String>>,
    on_resize_end: Option<Callback<String>>,
    listening: StoredValue<bool>,
    move_fn: StoredValue<Option<js_sys::Function>>,
    up_fn: StoredValue<Option<js_sys::Function>>,
    key_fn: StoredValue<Option<js_sys::Function>>,
    offset_px: RwSignal<f64>,
    offset_edge: StoredValue<String>,
}

fn synced_panels(ctx: ResizableCtx) -> Vec<PanelRuntime> {
    let layout = ctx.layout.get_untracked();
    let saved = ctx.saved.get_untracked();
    ctx.registered
        .get_untracked()
        .into_iter()
        .map(|config| {
            let position = layout.panels.get(&config.id);
            let saved = saved
                .as_ref()
                .and_then(|panels| panels.iter().find(|panel| panel.id == config.id));
            PanelRuntime {
                id: config.id,
                pixel_size: position.map(|position| position.width).unwrap_or(0.0),
                resizable: config.resizable,
                collapsible: config.collapsible,
                collapsed: position
                    .map(|position| position.collapsed)
                    .unwrap_or(config.collapsed),
                user_min: position.and_then(|position| position.constraints.user_min),
                user_max: position.and_then(|position| position.constraints.user_max),
                initial_size: config.initial_size,
                auto_collapsed: saved
                    .and_then(|panel| panel.auto_collapsed)
                    .unwrap_or(false),
            }
        })
        .collect()
}

fn write_saved(ctx: ResizableCtx, id: &str, edit: impl FnOnce(&mut DtResizableSavedPanel)) {
    let current = ctx
        .layout
        .get_untracked()
        .panels
        .get(id)
        .map(|position| position.width)
        .unwrap_or(0.0);
    ctx.saved.try_update(|slot| {
        let list = slot.get_or_insert_with(Vec::new);
        if let Some(item) = list.iter_mut().find(|panel| panel.id == id) {
            edit(item);
        } else {
            let mut item = DtResizableSavedPanel {
                id: id.to_string(),
                pixel_size: current,
                collapsed: None,
                auto_collapsed: None,
                manual_target_ratio: None,
            };
            edit(&mut item);
            list.push(item);
        }
    });
    persist_saved(ctx);
}

fn persist_saved(ctx: ResizableCtx) {
    let key = ctx.storage_key.try_get_value().unwrap_or_default();
    if key.is_empty() {
        return;
    }
    let Some(panels) = ctx.saved.try_get_untracked() else {
        return;
    };
    let Some(json) = panels.as_ref().map(|panels| serialize_stored(panels)) else {
        storage_remove(&key);
        return;
    };
    storage_set(&key, &json);
}

fn clear_saved(ctx: ResizableCtx) {
    let key = ctx.storage_key.try_get_value().unwrap_or_default();
    if !key.is_empty() {
        storage_remove(&key);
    }
    ctx.saved.try_set(None);
}

fn commit_panel_size(ctx: ResizableCtx, id: &str, pixels: f64) {
    let rounded = pixels.round();
    let container = ctx.container_size.get_untracked();
    let ratio = (container > 0.0).then_some(rounded / container);
    write_saved(ctx, id, |panel| {
        panel.pixel_size = rounded;
        panel.manual_target_ratio = ratio;
    });
}

fn emit_resize(ctx: ResizableCtx, id: &str, size: f64) {
    if let Some(callback) = ctx.on_panel_resize {
        callback.run((id.to_string(), size));
    }
}

fn collapse_panel(ctx: ResizableCtx, id: &str, collapsed: bool, auto: bool) {
    let panels = synced_panels(ctx);
    let Some(panel) = panels.iter().find(|panel| panel.id == id) else {
        return;
    };
    if collapsed == panel.collapsed {
        return;
    }
    if collapsed && !panel.collapsible {
        return;
    }
    if collapsed {
        let mut sizes = HashMap::new();
        for item in &panels {
            sizes.insert(item.id.clone(), item.pixel_size);
        }
        let snapshot = PreCollapse {
            sizes,
            container_size: ctx.container_size.get_untracked(),
        };
        ctx.pre_collapse
            .try_update_value(|map| map.insert(id.to_string(), snapshot));
        write_saved(ctx, id, |item| {
            item.collapsed = Some(true);
            item.auto_collapsed = Some(auto);
        });
        for item in panels
            .iter()
            .filter(|item| item.id != id && !item.collapsed)
        {
            write_saved(ctx, &item.id, |saved| saved.manual_target_ratio = None);
        }
    } else {
        expand_panel(ctx, panel);
    }
    if let Some(callback) = ctx.on_panel_collapse {
        callback.run((id.to_string(), collapsed));
    }
}

fn expand_panel(ctx: ResizableCtx, panel: &PanelRuntime) {
    let was_auto = panel.auto_collapsed;
    let snapshot = ctx
        .pre_collapse
        .try_update_value(|map| map.remove(&panel.id))
        .flatten();
    if let Some(snapshot) = snapshot {
        let changed = if snapshot.container_size > 0.0 {
            (ctx.container_size.get_untracked() - snapshot.container_size).abs()
                / snapshot.container_size
                > 0.1
        } else {
            false
        };
        if !changed || !was_auto {
            if let Some(size) = snapshot.sizes.get(&panel.id) {
                let size = *size;
                write_saved(ctx, &panel.id, |item| {
                    item.pixel_size = size;
                    item.collapsed = Some(false);
                    item.auto_collapsed = None;
                });
            } else {
                write_saved(ctx, &panel.id, |item| {
                    item.collapsed = Some(false);
                    item.auto_collapsed = None;
                });
            }
            for (id, size) in &snapshot.sizes {
                if id == &panel.id {
                    continue;
                }
                let size = *size;
                write_saved(ctx, id, |item| item.pixel_size = size);
            }
        } else {
            write_saved(ctx, &panel.id, |item| {
                item.collapsed = Some(false);
                item.auto_collapsed = None;
            });
        }
    } else {
        let container = ctx.container_size.get_untracked();
        let initial = parse_size_to_pixels(
            panel.initial_size.as_deref().unwrap_or(DEFAULT_PANEL_SIZE),
            container,
            true,
        );
        let constraints = ctx
            .layout
            .get_untracked()
            .panels
            .get(&panel.id)
            .map(|position| position.constraints.clone())
            .unwrap_or_else(|| ConstraintHierarchy {
                user_min: panel.user_min,
                user_max: panel.user_max,
                system_min: panel.user_min,
                system_max: panel.user_max,
                collapse_size: None,
            });
        let size = clamp_to_tier(initial, &constraints, false);
        write_saved(ctx, &panel.id, |item| {
            item.pixel_size = size;
            item.collapsed = Some(false);
            item.auto_collapsed = None;
        });
    }
}

fn reset_panels(ctx: ResizableCtx, before_id: &str, after_id: &str, behavior: &str) {
    match behavior {
        "all" => {
            reset_all_pairs(ctx);
            clear_saved(ctx);
        }
        "before" => reset_single(ctx, before_id),
        "after" => reset_single(ctx, after_id),
        "both" => reset_specific_pair(ctx, before_id, after_id),
        _ => {
            leptos::logging::warn!(
                "[resizable] Invalid resetBehavior: {behavior}. Expected 'both', 'before', 'after', or 'all'."
            );
            reset_specific_pair(ctx, before_id, after_id);
        }
    }
    if behavior != "all" {
        if !before_id.is_empty() {
            write_saved(ctx, before_id, |panel| panel.manual_target_ratio = None);
        }
        if !after_id.is_empty() {
            write_saved(ctx, after_id, |panel| panel.manual_target_ratio = None);
        }
    }
}

fn reset_all_pairs(ctx: ResizableCtx) {
    let count = synced_panels(ctx).len();
    for index in 0..count.saturating_sub(1) {
        let panels = synced_panels(ctx);
        let Some(before) = panels.get(index) else {
            continue;
        };
        let Some(after) = panels.get(index + 1) else {
            continue;
        };
        if can_reset(before, after) {
            let before_id = before.id.clone();
            let after_id = after.id.clone();
            reset_adjacent(ctx, &before_id, &after_id);
        }
    }
}

fn reset_specific_pair(ctx: ResizableCtx, before_id: &str, after_id: &str) {
    let panels = synced_panels(ctx);
    let before = panels.iter().find(|panel| panel.id == before_id);
    let after = panels.iter().find(|panel| panel.id == after_id);
    if let (Some(before), Some(after)) = (before, after) {
        if can_reset(before, after) {
            reset_adjacent(ctx, before_id, after_id);
        }
    }
}

fn can_reset(before: &PanelRuntime, after: &PanelRuntime) -> bool {
    before.resizable && !before.collapsed && after.resizable && !after.collapsed
}

fn reset_adjacent(ctx: ResizableCtx, before_id: &str, after_id: &str) {
    let panels = synced_panels(ctx);
    let Some(before) = panels.iter().find(|panel| panel.id == before_id) else {
        return;
    };
    let Some(after) = panels.iter().find(|panel| panel.id == after_id) else {
        return;
    };
    let container = ctx.container_size.get_untracked();
    let before_initial = parse_size_to_pixels(
        before.initial_size.as_deref().unwrap_or(DEFAULT_PANEL_SIZE),
        container,
        true,
    );
    let after_initial = parse_size_to_pixels(
        after.initial_size.as_deref().unwrap_or(DEFAULT_PANEL_SIZE),
        container,
        true,
    );
    let (before_size, after_size) = reset_pair_sizes(
        before.pixel_size,
        after.pixel_size,
        before_initial,
        after_initial,
    );
    write_saved(ctx, before_id, |panel| {
        panel.pixel_size = before_size;
        panel.manual_target_ratio = None;
    });
    write_saved(ctx, after_id, |panel| {
        panel.pixel_size = after_size;
        panel.manual_target_ratio = None;
    });
    emit_resize(ctx, before_id, before_size);
    emit_resize(ctx, after_id, after_size);
}

fn reset_single(ctx: ResizableCtx, id: &str) {
    if id.is_empty() {
        return;
    }
    let panels = synced_panels(ctx);
    let Some(index) = panels.iter().position(|panel| panel.id == id) else {
        return;
    };
    let panel = &panels[index];
    if panel.collapsed || !panel.resizable {
        return;
    }
    let container = ctx.container_size.get_untracked();
    let initial = parse_size_to_pixels(
        panel.initial_size.as_deref().unwrap_or(DEFAULT_PANEL_SIZE),
        container,
        true,
    );
    let delta = initial - panel.pixel_size;
    let Some(adjacent) = panels
        .iter()
        .enumerate()
        .find_map(|(at, item)| (at != index && !item.collapsed && item.resizable).then_some(item))
    else {
        return;
    };
    let adjacent_id = adjacent.id.clone();
    let adjacent_size = adjacent.pixel_size - delta;
    write_saved(ctx, id, |item| {
        item.pixel_size = initial;
        item.manual_target_ratio = None;
    });
    write_saved(ctx, &adjacent_id, |item| item.pixel_size = adjacent_size);
    emit_resize(ctx, id, initial);
    emit_resize(ctx, &adjacent_id, adjacent_size);
}

fn process_auto(ctx: ResizableCtx) {
    let container = ctx.container_size.get_untracked();
    for panel in synced_panels(ctx) {
        if panel.collapsed && panel.auto_collapsed {
            if let Some(threshold) = ctx
                .layout
                .get_untracked()
                .panels
                .get(&panel.id)
                .and_then(|position| position.constraints.collapse_size)
            {
                if container >= threshold {
                    collapse_panel(ctx, &panel.id, false, true);
                }
            }
        }
    }
    for panel in synced_panels(ctx) {
        if panel.collapsed || !panel.collapsible {
            continue;
        }
        if let Some(threshold) = ctx
            .layout
            .get_untracked()
            .panels
            .get(&panel.id)
            .and_then(|position| position.constraints.collapse_size)
        {
            if container < threshold {
                collapse_panel(ctx, &panel.id, true, true);
            }
        }
    }
    let rules = ctx.collapse_rules.try_get_value().unwrap_or_default();
    let hits = check_auto_collapse_rules(&synced_panels(ctx), &rules, container);
    if let Some(id) = hits.first() {
        collapse_panel(ctx, id, true, true);
    }
}

fn register_panel(ctx: ResizableCtx, config: DtResizablePanelConfig) {
    if config.id.contains(':') {
        leptos::logging::warn!(
            "[resizable] Panel id \"{}\" contains ':' which is reserved as the handle separator. Use kebab-case.",
            config.id
        );
    }
    ctx.registered.try_update(|list| {
        if let Some(slot) = list.iter_mut().find(|panel| panel.id == config.id) {
            *slot = config;
        } else {
            list.push(config);
        }
    });
}

fn unregister_panel(ctx: ResizableCtx, id: &str) {
    ctx.registered
        .try_update(|list| list.retain(|panel| panel.id != id));
}

fn sort_registered(ctx: ResizableCtx) {
    let Some(container) = ctx.container.get_untracked() else {
        return;
    };
    let container: HtmlElement = container.into();
    let Ok(nodes) = container.query_selector_all(".d-resizable-panel[data-panel-id]") else {
        return;
    };
    if nodes.length() == 0 {
        return;
    }
    let mut order = HashMap::new();
    for index in 0..nodes.length() {
        let Some(node) = nodes.item(index) else {
            continue;
        };
        let Ok(element) = node.dyn_into::<web_sys::Element>() else {
            continue;
        };
        if let Some(id) = element.get_attribute("data-panel-id") {
            order.insert(id, index);
        }
    }
    let Some(current) = ctx.registered.try_get_untracked() else {
        return;
    };
    let mut sorted = current.clone();
    sorted.sort_by_key(|panel| order.get(&panel.id).copied().unwrap_or(u32::MAX));
    let same = current
        .iter()
        .map(|panel| panel.id.as_str())
        .eq(sorted.iter().map(|panel| panel.id.as_str()));
    if !same {
        ctx.registered.try_set(sorted);
    }
}

fn apply_keyboard_resize(
    ctx: ResizableCtx,
    before_id: &str,
    after_id: &str,
    increase: bool,
    amount: f64,
) {
    let panels = synced_panels(ctx);
    let Some(before) = panels.iter().find(|panel| panel.id == before_id) else {
        return;
    };
    let Some(after) = panels.iter().find(|panel| panel.id == after_id) else {
        return;
    };
    let delta = if increase { amount } else { -amount };
    let result = process_resize_move(
        before.pixel_size + delta,
        before,
        after,
        ctx.container_size.get_untracked(),
        &panels,
        0.0,
    );
    if !result.valid {
        return;
    }
    let before_size = result.before_size.round();
    let after_size = result.after_size.round();
    commit_panel_size(ctx, before_id, before_size);
    commit_panel_size(ctx, after_id, after_size);
    let messages = ctx.messages.try_get_value().unwrap_or_default();
    let template = message(&messages.resize_announcement, DEFAULT_RESIZE_ANNOUNCEMENT);
    announce(&replace_tokens(
        template,
        &[
            ("{beforeId}", before_id),
            ("{afterId}", after_id),
            ("{beforePx}", &before_size.round().to_string()),
            ("{afterPx}", &after_size.round().to_string()),
        ],
    ));
}

fn start_drag(ctx: ResizableCtx, handle_id: &str) {
    if handle_id.is_empty() || ctx.drag.get_untracked().is_some() {
        return;
    }
    let sep = handle_id.find(':');
    let Some(sep) = sep else {
        return;
    };
    let before_id = &handle_id[..sep];
    let after_id = &handle_id[sep + 1..];
    let panels = synced_panels(ctx);
    let Some(before) = panels.iter().find(|panel| panel.id == before_id) else {
        leptos::logging::warn!("[resizable] No panel pair found for handle \"{handle_id}\"");
        return;
    };
    let Some(after) = panels.iter().find(|panel| panel.id == after_id) else {
        leptos::logging::warn!("[resizable] No panel pair found for handle \"{handle_id}\"");
        return;
    };
    let Some(container) = ctx.container.get_untracked() else {
        return;
    };
    let container: HtmlElement = container.into();
    let rect = container.get_bounding_client_rect();
    let layout = ctx.layout.get_untracked();
    let before_left = layout
        .panels
        .get(before_id)
        .map(|position| position.left)
        .unwrap_or(0.0);
    ctx.drag.set(Some(DragPreview {
        handle_id: handle_id.to_string(),
        before_id: before_id.to_string(),
        after_id: after_id.to_string(),
        original_before: before.pixel_size,
        original_after: after.pixel_size,
        before_left,
        proposed_before: before.pixel_size,
        proposed_after: after.pixel_size,
        before_end: layout
            .panels
            .get(before_id)
            .map(|position| position.right)
            .unwrap_or(0.0),
        after_start: layout
            .panels
            .get(after_id)
            .map(|position| position.left)
            .unwrap_or(before_left + before.pixel_size),
        after_end: layout
            .panels
            .get(after_id)
            .map(|position| position.right)
            .unwrap_or(0.0),
        handle_start: (before_left + before.pixel_size - RESIZABLE_HANDLE_CENTER_OFFSET_PX)
            .max(0.0),
        origin_x: rect.left(),
        origin_y: rect.top(),
    }));
    ctx.is_resizing.set(true);
    ctx.active_handle.set(Some(handle_id.to_string()));
    set_body_drag_style(&ctx.direction.get_untracked(), true);
    bind_drag_listeners(ctx);
    if let Some(callback) = ctx.on_resize_start {
        callback.run(handle_id.to_string());
    }
}

fn drag_move(ctx: ResizableCtx, event: &PointerEvent) {
    let Some(preview) = ctx.drag.get_untracked() else {
        return;
    };
    let panels = synced_panels(ctx);
    let Some(before) = panels.iter().find(|panel| panel.id == preview.before_id) else {
        return;
    };
    let Some(after) = panels.iter().find(|panel| panel.id == preview.after_id) else {
        return;
    };
    let direction = ctx.direction.get_untracked();
    let cursor = if direction == "row" {
        event.client_x() as f64 - preview.origin_x
    } else {
        event.client_y() as f64 - preview.origin_y
    };
    let result = process_resize_move(
        cursor,
        before,
        after,
        ctx.container_size.get_untracked(),
        &panels,
        preview.before_left,
    );
    if !result.valid {
        return;
    }
    let container = ctx.container_size.get_untracked();
    ctx.drag.set(Some(DragPreview {
        proposed_before: result.before_size,
        proposed_after: result.after_size,
        before_end: container - result.cursor,
        after_start: result.cursor,
        after_end: (container - result.cursor - result.after_size).max(0.0),
        handle_start: (result.cursor - RESIZABLE_HANDLE_CENTER_OFFSET_PX).max(0.0),
        ..preview
    }));
}

fn commit_drag(ctx: ResizableCtx) {
    let Some(preview) = ctx.drag.try_get_untracked().flatten() else {
        return;
    };
    ctx.drag.try_set(None);
    ctx.is_resizing.try_set(false);
    ctx.active_handle.try_set(None);
    set_body_drag_style("", false);
    unbind_drag_listeners(ctx);
    let changed = preview.proposed_before != preview.original_before
        || preview.proposed_after != preview.original_after;
    if !changed {
        return;
    }
    commit_panel_size(ctx, &preview.before_id, preview.proposed_before);
    commit_panel_size(ctx, &preview.after_id, preview.proposed_after);
    if let Some(callback) = ctx.on_resize_end {
        callback.run(preview.handle_id);
    }
    process_auto(ctx);
}

fn cancel_drag(ctx: ResizableCtx) {
    if ctx.drag.try_get_untracked().flatten().is_none() {
        return;
    }
    ctx.drag.try_set(None);
    ctx.is_resizing.try_set(false);
    ctx.active_handle.try_set(None);
    set_body_drag_style("", false);
    unbind_drag_listeners(ctx);
}

fn bind_drag_listeners(ctx: ResizableCtx) {
    if ctx.listening.try_get_value().unwrap_or(false) {
        return;
    }
    let Some(document) = owner_document() else {
        return;
    };
    let Some(move_fn) = ctx.move_fn.try_get_value().flatten() else {
        return;
    };
    let Some(up_fn) = ctx.up_fn.try_get_value().flatten() else {
        return;
    };
    let Some(key_fn) = ctx.key_fn.try_get_value().flatten() else {
        return;
    };
    let _ = document.add_event_listener_with_callback("pointermove", move_fn.unchecked_ref());
    let _ = document.add_event_listener_with_callback("pointerup", up_fn.unchecked_ref());
    let _ = document.add_event_listener_with_callback("pointercancel", up_fn.unchecked_ref());
    let _ = document.add_event_listener_with_callback("keydown", key_fn.unchecked_ref());
    ctx.listening.try_set_value(true);
}

fn unbind_drag_listeners(ctx: ResizableCtx) {
    if !ctx.listening.try_get_value().unwrap_or(false) {
        return;
    }
    if let Some(document) = owner_document() {
        if let Some(move_fn) = ctx.move_fn.try_get_value().flatten() {
            let _ = document
                .remove_event_listener_with_callback("pointermove", move_fn.unchecked_ref());
        }
        if let Some(up_fn) = ctx.up_fn.try_get_value().flatten() {
            let _ =
                document.remove_event_listener_with_callback("pointerup", up_fn.unchecked_ref());
            let _ = document
                .remove_event_listener_with_callback("pointercancel", up_fn.unchecked_ref());
        }
        if let Some(key_fn) = ctx.key_fn.try_get_value().flatten() {
            let _ = document.remove_event_listener_with_callback("keydown", key_fn.unchecked_ref());
        }
    }
    ctx.listening.try_set_value(false);
}

fn install_drag_closures(ctx: ResizableCtx) {
    let move_closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
        if let Some(pointer) = event.dyn_ref::<PointerEvent>() {
            drag_move(ctx, pointer);
        }
    }) as Box<dyn FnMut(web_sys::Event)>);
    let up_closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
        commit_drag(ctx);
    }) as Box<dyn FnMut(web_sys::Event)>);
    let key_closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
        let Some(key) = event.dyn_ref::<KeyboardEvent>() else {
            return;
        };
        if key.key() == "Escape" {
            key.prevent_default();
            cancel_drag(ctx);
        }
    }) as Box<dyn FnMut(web_sys::Event)>);
    ctx.move_fn.try_set_value(Some(
        move_closure
            .as_ref()
            .unchecked_ref::<js_sys::Function>()
            .clone(),
    ));
    ctx.up_fn.try_set_value(Some(
        up_closure
            .as_ref()
            .unchecked_ref::<js_sys::Function>()
            .clone(),
    ));
    ctx.key_fn.try_set_value(Some(
        key_closure
            .as_ref()
            .unchecked_ref::<js_sys::Function>()
            .clone(),
    ));
    // `Function::clone` does not keep the Rust closure alive. Forget them for
    // the page lifetime. Cleanup removes the document listeners.
    move_closure.forget();
    up_closure.forget();
    key_closure.forget();
}

fn set_body_drag_style(direction: &str, active: bool) {
    let Some(body) = owner_document().and_then(|document| document.body()) else {
        return;
    };
    let style = body.style();
    if active {
        let cursor = if direction == "row" {
            "ew-resize"
        } else {
            "ns-resize"
        };
        let _ = style.set_property("user-select", "none");
        let _ = style.set_property("cursor", cursor);
    } else {
        let _ = style.set_property("user-select", "");
        let _ = style.set_property("cursor", "");
    }
}

fn owner_document() -> Option<web_sys::Document> {
    web_sys::window()?.document()
}

fn storage_set(key: &str, json: &str) {
    if let Some(storage) = local_storage() {
        let _ = storage.set_item(key, json);
    }
}

fn storage_remove(key: &str) {
    if let Some(storage) = local_storage() {
        let _ = storage.remove_item(key);
    }
}

fn storage_load(key: &str) -> Option<Vec<DtResizableSavedPanel>> {
    if key.is_empty() {
        return None;
    }
    let storage = local_storage()?;
    let json = storage.get_item(key).ok().flatten()?;
    match parse_stored(&json) {
        Some(panels) => Some(panels),
        None => {
            let _ = storage.remove_item(key);
            None
        }
    }
}

fn local_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

fn measure_container(element: &HtmlElement, direction: &str) -> f64 {
    let raw = if direction == "row" {
        element.client_width()
    } else {
        element.client_height()
    };
    validate_container_size(raw as f64)
}

fn measure_offset(selector: &str, amount: Option<f64>, direction: &str) -> f64 {
    if let Some(amount) = amount {
        if amount > 0.0 {
            return amount;
        }
    }
    if selector.is_empty() {
        return 0.0;
    }
    let Some(document) = owner_document() else {
        return 0.0;
    };
    let Ok(Some(element)) = document.query_selector(selector) else {
        return 0.0;
    };
    let rect = element.get_bounding_client_rect();
    if direction == "row" {
        rect.height()
    } else {
        rect.width()
    }
}

fn announce(message: &str) {
    if message.trim().is_empty() {
        return;
    }
    let Some(document) = owner_document() else {
        return;
    };
    let element = document
        .get_element_by_id("d-resizable-announcements")
        .or_else(|| {
            let element = document.create_element("div").ok()?;
            element.set_id("d-resizable-announcements");
            element.set_attribute("aria-live", "polite").ok()?;
            element.set_attribute("aria-atomic", "true").ok()?;
            let Some(html) = element.dyn_ref::<HtmlElement>() else {
                return None;
            };
            let style = html.style();
            let _ = style.set_property("position", "absolute");
            let _ = style.set_property("left", "-10000px");
            let _ = style.set_property("width", "1px");
            let _ = style.set_property("height", "1px");
            let _ = style.set_property("overflow", "hidden");
            document.body()?.append_child(&element).ok()?;
            Some(element)
        });
    if let Some(element) = element {
        element.set_text_content(Some(message));
    }
}

fn opt_size(value: String) -> Option<String> {
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

fn warn_size(name: &str, value: &str) {
    if !value.is_empty() && !is_valid_sizing(value) {
        leptos::logging::warn!("[DtResizablePanel] Invalid {name}: \"{value}\".");
    }
}

/// Dialtone Vue `DtResizable`.
///
/// `direction` is `row` or `column`. `storageKey` restores sizes from
/// `localStorage`. `collapseRules` collapse the first matching panel when its
/// size is at the threshold. `on_panel_resize` runs for a programmatic resize
/// and a reset. A drag or arrow key commits sizes and does not emit that
/// callback. `on_resize_end` runs when a drag changes sizes.
#[component]
pub fn DtResizable(
    /// `direction`.
    #[prop(into, default = Signal::stored(String::from("row")))]
    direction: Signal<String>,
    /// `storageKey`.
    #[prop(optional, into)]
    storage_key: String,
    /// `class`.
    #[prop(optional, into)]
    class: String,
    /// `panels`. Accepted and not applied. Children register the layout.
    #[prop(optional)]
    panels: Vec<DtResizablePanelConfig>,
    /// `spaceAllocationStrategy`. Accepted. The engine uses proportional space
    /// plus a drag ratio.
    #[prop(optional, default = "proportional")]
    space_allocation_strategy: &'static str,
    /// `collapseRules`.
    #[prop(optional)]
    collapse_rules: Vec<DtResizableCollapseRule>,
    /// `offsetElement`.
    #[prop(optional, into)]
    offset_element: String,
    /// `offsetAmount`. Wins over `offsetElement` when set.
    #[prop(optional)]
    offset_amount: Option<f64>,
    /// `offsetDirection`. `start`, `end`, or `both`.
    #[prop(optional, default = "start")]
    offset_direction: &'static str,
    /// `messages`.
    #[prop(optional)]
    messages: DtResizableMessages,
    /// `panel-resize`.
    #[prop(optional)]
    on_panel_resize: Option<Callback<(String, f64)>>,
    /// `panel-collapse`.
    #[prop(optional)]
    on_panel_collapse: Option<Callback<(String, bool)>>,
    /// `resize-start`.
    #[prop(optional)]
    on_resize_start: Option<Callback<String>>,
    /// `resize-end`.
    #[prop(optional)]
    on_resize_end: Option<Callback<String>>,
    children: Children,
) -> impl IntoView {
    if !panels.is_empty() {
        leptos::logging::warn!(
            "[resizable] The panels prop is accepted and not applied. Register panels with DtResizablePanel."
        );
    }
    if space_allocation_strategy != "proportional" && space_allocation_strategy != "preserve-manual"
    {
        leptos::logging::warn!(
            "[resizable] Invalid spaceAllocationStrategy: {space_allocation_strategy}. Expected 'proportional' or 'preserve-manual'."
        );
    }
    if !matches!(offset_direction, "start" | "end" | "both") {
        leptos::logging::warn!(
            "[resizable] Invalid offsetDirection: {offset_direction}. Expected 'start', 'end', or 'both'."
        );
    }

    let registered = RwSignal::new(Vec::<DtResizablePanelConfig>::new());
    let saved = RwSignal::new(storage_load(&storage_key));
    let container_size = RwSignal::new(0.0);
    let is_resizing = RwSignal::new(false);
    let active_handle = RwSignal::new(None::<String>);
    let is_initializing = RwSignal::new(true);
    let container = NodeRef::<html::Div>::new();
    let drag = RwSignal::new(None::<DragPreview>);
    let offset_px = RwSignal::new(0.0);
    let layout = Signal::derive(move || {
        let panels = registered.get();
        let size = container_size.get();
        if panels.is_empty() || size <= 0.0 {
            return LayoutResult::empty();
        }
        compute_layout(&panels, size, saved.get().as_deref())
    });
    let ctx = ResizableCtx {
        registered,
        saved,
        container_size,
        direction,
        is_resizing,
        active_handle,
        is_initializing,
        container,
        layout,
        drag,
        pre_collapse: StoredValue::new(HashMap::new()),
        collapse_rules: StoredValue::new(collapse_rules),
        messages: StoredValue::new(messages),
        storage_key: StoredValue::new(storage_key),
        on_panel_resize,
        on_panel_collapse,
        on_resize_start,
        on_resize_end,
        listening: StoredValue::new(false),
        move_fn: StoredValue::new(None),
        up_fn: StoredValue::new(None),
        key_fn: StoredValue::new(None),
        offset_px,
        offset_edge: StoredValue::new(offset_direction.to_string()),
    };
    install_drag_closures(ctx);
    let observer = StoredValue::new(None::<web_sys::ResizeObserver>);

    Effect::new(move |_| {
        let layout = ctx.layout.get();
        if !layout.panels.is_empty() && ctx.is_initializing.get_untracked() {
            ctx.is_initializing.set(false);
        }
    });
    Effect::new(move |_| {
        let _ = ctx.container_size.get();
        let _ = ctx.layout.get();
        if ctx.is_initializing.get() {
            return;
        }
        process_auto(ctx);
    });
    Effect::new(move |_| {
        let _ = ctx.registered.get();
        if ctx.container.get().is_some() {
            sort_registered(ctx);
        }
    });
    Effect::new(move |_| {
        let direction = ctx.direction.get();
        offset_px.set(measure_offset(&offset_element, offset_amount, &direction));
    });
    Effect::new(move |_| {
        let Some(element) = container.get() else {
            return;
        };
        let element: HtmlElement = element.into();
        let direction = ctx.direction.get_untracked();
        container_size.set(measure_container(&element, &direction).round());
        let installed = observer
            .try_with_value(|slot| slot.is_some())
            .unwrap_or(true);
        if installed {
            return;
        }
        let callback = Closure::wrap(Box::new(
            move |entries: js_sys::Array, _observer: web_sys::ResizeObserver| {
                let Ok(entry) = entries.get(0).dyn_into::<web_sys::ResizeObserverEntry>() else {
                    return;
                };
                let rect = entry.content_rect();
                let direction = ctx.direction.get_untracked();
                let raw = if direction == "row" {
                    rect.width()
                } else {
                    rect.height()
                };
                ctx.container_size
                    .try_set(validate_container_size(raw.round()));
            },
        )
            as Box<dyn FnMut(js_sys::Array, web_sys::ResizeObserver)>);
        if let Ok(resize_observer) = web_sys::ResizeObserver::new(callback.as_ref().unchecked_ref())
        {
            resize_observer.observe(&element);
            callback.forget();
            let _ = observer.try_set_value(Some(resize_observer));
        }
    });
    Effect::new(move |_| {
        let direction = ctx.direction.get();
        let Some(element) = container.get() else {
            return;
        };
        let element: HtmlElement = element.into();
        container_size.set(measure_container(&element, &direction).round());
    });
    on_cleanup(move || {
        cancel_drag(ctx);
        let _ = observer.try_update_value(|slot| {
            if let Some(resize_observer) = slot.take() {
                resize_observer.disconnect();
            }
        });
    });

    let storage_attr = ctx.storage_key.get_value();
    let storage_attr = (!storage_attr.is_empty()).then_some(storage_attr);
    view! {
        <div
            node_ref=container
            class=move || resizable_class(&direction.get(), is_resizing.get(), &class)
            data-storage-key=storage_attr
        >
            <Provider value=ctx>
                {children()}
            </Provider>
        </div>
    }
}

/// Dialtone Vue `DtResizablePanel`.
///
/// `id` is required and must be unique in the parent. Size props take a layout
/// token (`500`) or a percentage (`25p`). The default slot is the panel body.
/// Vue slot props `panel`, `isCollapsed`, and `isResizing` are not forwarded.
#[component]
pub fn DtResizablePanel(
    /// `id`.
    #[prop(into)]
    id: String,
    /// `initialSize`.
    #[prop(optional, into)]
    initial_size: String,
    /// `userMinSize`.
    #[prop(optional, into)]
    user_min_size: String,
    /// `userMaxSize`.
    #[prop(optional, into)]
    user_max_size: String,
    /// `systemMinSize`.
    #[prop(optional, into)]
    system_min_size: String,
    /// `systemMaxSize`.
    #[prop(optional, into)]
    system_max_size: String,
    /// `collapseSize`.
    #[prop(optional, into)]
    collapse_size: String,
    /// `resizable`.
    #[prop(optional, default = true)]
    resizable: bool,
    /// `collapsible`.
    #[prop(optional)]
    collapsible: bool,
    /// `collapsed`. A signal updates the panel after the first layout.
    #[prop(into, default = Signal::stored(false))]
    collapsed: Signal<bool>,
    /// `class`.
    #[prop(optional, into)]
    class: String,
    /// `contentClass`.
    #[prop(optional, into)]
    content_class: String,
    children: Children,
) -> impl IntoView {
    warn_size("initialSize", &initial_size);
    warn_size("userMinSize", &user_min_size);
    warn_size("userMaxSize", &user_max_size);
    warn_size("systemMinSize", &system_min_size);
    warn_size("systemMaxSize", &system_max_size);
    warn_size("collapseSize", &collapse_size);
    let ctx = use_context::<ResizableCtx>();
    if ctx.is_none() {
        leptos::logging::warn!("[resizable] DtResizablePanel requires a DtResizable parent.");
    }
    let panel_id = id.clone();
    let dom_id = format!("dt-resizable-panel-{id}");
    let seen = StoredValue::new(false);
    if let Some(ctx) = ctx {
        Effect::new(move |_| {
            let config = DtResizablePanelConfig {
                id: panel_id.clone(),
                initial_size: opt_size(initial_size.clone()),
                user_min_size: opt_size(user_min_size.clone()),
                user_max_size: opt_size(user_max_size.clone()),
                system_min_size: opt_size(system_min_size.clone()),
                system_max_size: opt_size(system_max_size.clone()),
                collapse_size: opt_size(collapse_size.clone()),
                resizable,
                collapsible,
                collapsed: collapsed.get(),
            };
            register_panel(ctx, config);
        });
        let cleanup_id = id.clone();
        on_cleanup(move || unregister_panel(ctx, &cleanup_id));
        let collapsed_id = id.clone();
        Effect::new(move |_| {
            let next = collapsed.get();
            if !seen.try_get_value().unwrap_or(true) {
                let _ = seen.try_set_value(true);
                return;
            }
            if ctx.is_initializing.get_untracked() {
                return;
            }
            collapse_panel(ctx, &collapsed_id, next, false);
        });
    }
    let class_name = class;
    let content_name = join_class(["d-resizable-panel__content", &content_class]);
    let panel_key = StoredValue::new(id);
    view! {
        <div
            id=dom_id
            class=move || {
                let id = panel_key.get_value();
                let layout = ctx.map(|ctx| ctx.layout.get());
                let position = layout.as_ref().and_then(|layout| layout.panels.get(&id));
                let collapsed = position.map(|position| position.collapsed).unwrap_or(false);
                panel_class(&class_name, collapsed, !resizable)
            }
            style=move || {
                let id = panel_key.get_value();
                let drag = ctx.and_then(|ctx| ctx.drag.get());
                let layout = ctx.map(|ctx| ctx.layout.get());
                let position = layout.as_ref().and_then(|layout| layout.panels.get(&id));
                panel_style(position, drag.as_ref(), &id)
            }
            data-panel-id=panel_key.get_value()
            data-qa="d-resizable-panel"
        >
            <div
                class=content_name
                style=move || {
                    let Some(ctx) = ctx else {
                        return String::new();
                    };
                    offset_style(
                        "content",
                        ctx.offset_px.get(),
                        &ctx.offset_edge.get_value(),
                    )
                }
            >
                {children()}
            </div>
        </div>
    }
}

/// Dialtone Vue `DtResizableHandle`.
///
/// Sits between two panels. Arrow keys resize by 8px, Shift by 24px, and
/// Ctrl or Meta by 1px. Enter collapses the panel before the handle when that
/// panel is `collapsible`. Home and End use the user min and max. R resets the
/// two panels. Escape blurs the handle, and Escape during a drag cancels it.
/// Double-click resets using `resetBehavior`.
#[component]
pub fn DtResizableHandle(
    /// `beforePanelId`. Taken from layout order when empty.
    #[prop(optional, into)]
    before_panel_id: String,
    /// `afterPanelId`.
    #[prop(optional, into)]
    after_panel_id: String,
    /// `disabled`.
    #[prop(optional)]
    disabled: bool,
    /// `class`.
    #[prop(optional, into)]
    class: String,
    /// `disableResetOnDoubleClick`.
    #[prop(optional)]
    disable_reset_on_double_click: bool,
    /// `resetBehavior`. `both`, `before`, `after`, or `all`.
    #[prop(optional, default = "both")]
    reset_behavior: &'static str,
    /// `ariaLabel`.
    #[prop(optional, into)]
    aria_label: String,
) -> impl IntoView {
    let ctx = use_context::<ResizableCtx>();
    if ctx.is_none() {
        leptos::logging::warn!("[resizable] DtResizableHandle requires a DtResizable parent.");
    }
    let handle_ref = NodeRef::<html::Div>::new();
    let dom_index = RwSignal::new(0usize);
    if let Some(ctx) = ctx {
        Effect::new(move |_| {
            let _ = ctx.layout.get();
            let Some(element) = handle_ref.get() else {
                return;
            };
            let element: HtmlElement = element.into();
            let Some(parent) = element.parent_element() else {
                return;
            };
            let Ok(nodes) = parent.query_selector_all(".d-resizable-handle") else {
                return;
            };
            for index in 0..nodes.length() {
                let Some(node) = nodes.item(index) else {
                    continue;
                };
                if element.is_same_node(Some(&node)) {
                    let next = index as usize;
                    if dom_index.get_untracked() != next {
                        dom_index.set(next);
                    }
                    break;
                }
            }
        });
    }

    let before_id = StoredValue::new(before_panel_id);
    let after_id = StoredValue::new(after_panel_id);

    view! {
        <div
            node_ref=handle_ref
            data-qa="dt-resizable-handle"
            class=move || {
                let direction = ctx
                    .map(|ctx| ctx.direction.get())
                    .unwrap_or_else(|| "row".to_string());
                let position = placed_handle(ctx, before_id, after_id, dom_index.get(), true);
                let active = position.as_ref().is_some_and(|position| {
                    ctx.map(|ctx| ctx.active_handle.get())
                        .is_some_and(|active| active.as_deref() == Some(position.id.as_str()))
                });
                let layout_disabled = position.as_ref().map(|position| position.disabled).unwrap_or(false);
                handle_class(&direction, &class, active, disabled || layout_disabled)
            }
            style=move || {
                let position = placed_handle(ctx, before_id, after_id, dom_index.get(), true);
                let drag = ctx.and_then(|ctx| ctx.drag.get());
                let offset = ctx
                    .map(|ctx| {
                        offset_style("handle", ctx.offset_px.get(), &ctx.offset_edge.get_value())
                    })
                    .unwrap_or_default();
                handle_style(position.as_ref(), drag.as_ref(), &offset)
            }
            data-handle-id=move || placed_handle(ctx, before_id, after_id, dom_index.get(), true).map(|position| position.id).unwrap_or_default()
            tabindex=move || {
                let layout_disabled = placed_handle(ctx, before_id, after_id, dom_index.get(), true).map(|position| position.disabled).unwrap_or(false);
                if disabled || layout_disabled { "-1" } else { "0" }
            }
            role="separator"
            aria-orientation=move || {
                let direction = ctx
                    .map(|ctx| ctx.direction.get())
                    .unwrap_or_else(|| "row".to_string());
                if direction == "row" { "vertical" } else { "horizontal" }
            }
            aria-label=move || {
                if !aria_label.is_empty() {
                    return aria_label.clone();
                }
                let position = placed_handle(ctx, before_id, after_id, dom_index.get(), true);
                let before = position
                    .as_ref()
                    .map(|position| position.before_panel_id.clone())
                    .filter(|id| !id.is_empty())
                    .unwrap_or_else(|| "first".to_string());
                let after = position
                    .as_ref()
                    .map(|position| position.after_panel_id.clone())
                    .filter(|id| !id.is_empty())
                    .unwrap_or_else(|| "second".to_string());
                let template = ctx
                    .and_then(|ctx| ctx.messages.try_get_value())
                    .map(|messages| messages.handle_aria_label)
                    .unwrap_or_default();
                let template = if template.is_empty() {
                    DEFAULT_HANDLE_ARIA_LABEL.to_string()
                } else {
                    template
                };
                replace_tokens(&template, &[("{before}", before.as_str()), ("{after}", after.as_str())])
            }
            aria-valuenow=move || aria_value_now(ctx, &placed_handle(ctx, before_id, after_id, dom_index.get(), true))
            aria-valuemin=move || aria_value_min(ctx, &placed_handle(ctx, before_id, after_id, dom_index.get(), true))
            aria-valuemax=move || aria_value_max(ctx, &placed_handle(ctx, before_id, after_id, dom_index.get(), true))
            aria-controls=move || {
                let before = placed_handle(ctx, before_id, after_id, dom_index.get(), true)
                    .map(|position| position.before_panel_id)
                    .unwrap_or_default();
                format!("dt-resizable-panel-{before}")
            }
            aria-valuetext=move || aria_value_text(ctx, &placed_handle(ctx, before_id, after_id, dom_index.get(), true))
            aria-disabled=move || {
                let layout_disabled = placed_handle(ctx, before_id, after_id, dom_index.get(), true).map(|position| position.disabled).unwrap_or(false);
                (disabled || layout_disabled).then_some("true")
            }
            on:pointerdown=move |event: PointerEvent| {
                let layout_disabled = placed_handle(ctx, before_id, after_id, dom_index.get_untracked(), false)
                    .map(|position| position.disabled)
                    .unwrap_or(false);
                if disabled || layout_disabled {
                    return;
                }
                event.prevent_default();
                if let Some(ctx) = ctx {
                    if let Some(position) = placed_handle(
                        Some(ctx),
                        before_id,
                        after_id,
                        dom_index.get_untracked(),
                        false,
                    ) {
                        start_drag(ctx, &position.id);
                    }
                }
            }
            on:dblclick=move |_| {
                if disable_reset_on_double_click {
                    return;
                }
                let Some(position) =
                    placed_handle(ctx, before_id, after_id, dom_index.get_untracked(), false)
                else {
                    return;
                };
                if disabled || position.disabled {
                    return;
                }
                if let Some(ctx) = ctx {
                    reset_panels(
                        ctx,
                        &position.before_panel_id,
                        &position.after_panel_id,
                        reset_behavior,
                    );
                    let messages = ctx.messages.try_get_value().unwrap_or_default();
                    let template = message(&messages.reset_announcement, DEFAULT_RESET_ANNOUNCEMENT);
                    announce(&replace_tokens(
                        template,
                        &[
                            ("{beforeId}", position.before_panel_id.as_str()),
                            ("{afterId}", position.after_panel_id.as_str()),
                        ],
                    ));
                }
            }
            on:keydown=move |event: KeyboardEvent| {
                let Some(ctx) = ctx else {
                    return;
                };
                let Some(position) = placed_handle(
                    Some(ctx),
                    before_id,
                    after_id,
                    dom_index.get_untracked(),
                    false,
                ) else {
                    return;
                };
                let key = event.key();
                match key.as_str() {
                    "Enter" => {
                        let panels = synced_panels(ctx);
                        let Some(before) = panels.iter().find(|panel| panel.id == position.before_panel_id) else {
                            return;
                        };
                        if !before.collapsible {
                            return;
                        }
                        event.prevent_default();
                        let next = !before.collapsed;
                        collapse_panel(ctx, &before.id, next, false);
                        let messages = ctx.messages.try_get_value().unwrap_or_default();
                        let template = if next {
                            message(&messages.collapse_announcement, DEFAULT_COLLAPSE_ANNOUNCEMENT)
                        } else {
                            message(&messages.expand_announcement, DEFAULT_EXPAND_ANNOUNCEMENT)
                        };
                        announce(&replace_tokens(template, &[("{panelId}", before.id.as_str())]));
                    }
                    "Home" => {
                        event.prevent_default();
                        nudge_to_edge(ctx, &position, true);
                    }
                    "End" => {
                        event.prevent_default();
                        nudge_to_edge(ctx, &position, false);
                    }
                    "r" | "R" => {
                        if event.ctrl_key() || event.meta_key() || event.alt_key() || event.shift_key() {
                            return;
                        }
                        event.prevent_default();
                        reset_panels(
                            ctx,
                            &position.before_panel_id,
                            &position.after_panel_id,
                            "both",
                        );
                    }
                    "Escape" => {
                        event.prevent_default();
                        if let Some(element) = handle_ref.get_untracked() {
                            let element: HtmlElement = element.into();
                            let _ = element.blur();
                        }
                    }
                    _ => {
                        let Some(increase) = keyboard_direction(&key, &ctx.direction.get_untracked()) else {
                            return;
                        };
                        event.prevent_default();
                        let amount = keyboard_increment(event.meta_key(), event.ctrl_key(), event.shift_key());
                        apply_keyboard_resize(
                            ctx,
                            &position.before_panel_id,
                            &position.after_panel_id,
                            increase,
                            amount,
                        );
                    }
                }
            }
        >
            <div class="d-resizable-handle__indicator"></div>
        </div>
    }
}

fn placed_handle(
    ctx: Option<ResizableCtx>,
    before_id: StoredValue<String>,
    after_id: StoredValue<String>,
    index: usize,
    track: bool,
) -> Option<HandlePosition> {
    let ctx = ctx?;
    let layout = if track {
        ctx.layout.get()
    } else {
        ctx.layout.get_untracked()
    };
    let before_id = before_id.get_value();
    let after_id = after_id.get_value();
    if !before_id.is_empty() && !after_id.is_empty() {
        let id = build_handle_id(&before_id, &after_id);
        return layout.handles.into_iter().find(|handle| handle.id == id);
    }
    layout.handles.get(index).cloned()
}

fn aria_value_now(ctx: Option<ResizableCtx>, position: &Option<HandlePosition>) -> i32 {
    let Some(ctx) = ctx else {
        return 0;
    };
    let Some(position) = position else {
        return 0;
    };
    let container = ctx.container_size.get();
    if container == 0.0 {
        return 0;
    }
    let Some(panel) = synced_panels(ctx)
        .into_iter()
        .find(|panel| panel.id == position.before_panel_id)
    else {
        return 0;
    };
    let pixels = if panel.collapsed {
        0.0
    } else {
        panel.pixel_size
    };
    pixels_to_percentage(pixels, container).floor() as i32
}

fn aria_value_min(ctx: Option<ResizableCtx>, position: &Option<HandlePosition>) -> i32 {
    let Some(ctx) = ctx else {
        return 0;
    };
    let Some(position) = position else {
        return 0;
    };
    let container = ctx.container_size.get();
    if container == 0.0 {
        return 0;
    }
    let Some(panel) = synced_panels(ctx)
        .into_iter()
        .find(|panel| panel.id == position.before_panel_id)
    else {
        return 0;
    };
    pixels_to_percentage(panel.user_min.unwrap_or(0.0), container).floor() as i32
}

fn aria_value_max(ctx: Option<ResizableCtx>, position: &Option<HandlePosition>) -> i32 {
    let Some(ctx) = ctx else {
        return 100;
    };
    let Some(position) = position else {
        return 100;
    };
    let container = ctx.container_size.get();
    if container == 0.0 {
        return 100;
    }
    let Some(panel) = synced_panels(ctx)
        .into_iter()
        .find(|panel| panel.id == position.before_panel_id)
    else {
        return 100;
    };
    pixels_to_percentage(panel.user_max.unwrap_or(container), container).floor() as i32
}

fn aria_value_text(ctx: Option<ResizableCtx>, position: &Option<HandlePosition>) -> String {
    let Some(ctx) = ctx else {
        return String::new();
    };
    let Some(position) = position else {
        return String::new();
    };
    let Some(panel) = synced_panels(ctx)
        .into_iter()
        .find(|panel| panel.id == position.before_panel_id)
    else {
        return String::new();
    };
    let messages = ctx.messages.try_get_value().unwrap_or_default();
    let template = message(&messages.aria_value_text, DEFAULT_ARIA_VALUE_TEXT);
    replace_tokens(
        template,
        &[
            ("{panelId}", panel.id.as_str()),
            ("{pixels}", &panel.pixel_size.round().to_string()),
        ],
    )
}

fn nudge_to_edge(ctx: ResizableCtx, position: &HandlePosition, home: bool) {
    let panels = synced_panels(ctx);
    let Some(before) = panels
        .iter()
        .find(|panel| panel.id == position.before_panel_id)
    else {
        return;
    };
    let Some(after) = panels
        .iter()
        .find(|panel| panel.id == position.after_panel_id)
    else {
        return;
    };
    let target = if home {
        before.user_min.unwrap_or(MIN_PANEL_SIZE_PX)
    } else {
        let after_min = after.user_min.unwrap_or(MIN_PANEL_SIZE_PX);
        let container = ctx.container_size.get_untracked();
        let max = before
            .user_max
            .unwrap_or(container - after_min)
            .min(container - after_min);
        max
    };
    let delta = target - before.pixel_size;
    if delta == 0.0 {
        return;
    }
    apply_keyboard_resize(
        ctx,
        &position.before_panel_id,
        &position.after_panel_id,
        delta > 0.0,
        delta.abs(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn panel(id: &str) -> DtResizablePanelConfig {
        DtResizablePanelConfig::new(id)
    }

    fn width(layout: &LayoutResult, id: &str) -> f64 {
        layout
            .panels
            .get(id)
            .map(|position| position.width)
            .unwrap_or(-1.0)
    }

    #[test]
    fn size_tokens_and_percentages() {
        assert_eq!(parse_size_to_pixels("50p", 1000.0, true), 500.0);
        assert_eq!(parse_size_to_pixels("500", 1000.0, true), 320.0);
        assert_eq!(parse_size_to_pixels("8px", 1000.0, true), 8.0);
        assert_eq!(parse_size_to_pixels("0", 1000.0, true), 0.0);
        assert_eq!(parse_size_to_pixels("nope", 1000.0, true), 0.0);
        assert_eq!(parse_size_to_pixels("1600", 1000.0, true), 1000.0);
        assert!(is_valid_sizing("25p"));
        assert!(is_valid_sizing("200"));
        assert!(!is_valid_sizing("101p"));
        assert!(is_percentage_value("30p"));
        assert!(is_css_value("8px"));
        assert!(has_percentage_min_size(Some("50p")));
        assert!(!has_percentage_min_size(Some("500")));
        assert_eq!(pixels_to_percentage(250.0, 1000.0), 25.0);
        assert_eq!(validate_container_size(-5.0), 1000.0);
        assert_eq!(validate_container_size(20_000.0), 10_000.0);
        assert_eq!(build_handle_id("left", "right"), "left:right");
    }

    #[test]
    fn two_panels_split_the_container() {
        let layout = compute_layout(&[panel("left"), panel("right")], 1000.0, None);
        assert_eq!(width(&layout, "left"), 500.0);
        assert_eq!(width(&layout, "right"), 500.0);
        assert_eq!(layout.handles.len(), 1);
        assert_eq!(layout.handles[0].id, "left:right");
        assert_eq!(layout.handles[0].left, 500.0);
        assert!(!layout.handles[0].disabled);
    }

    #[test]
    fn collapsed_panel_has_no_width_and_disables_the_handle() {
        let mut left = panel("left");
        left.collapsed = true;
        left.collapsible = true;
        let layout = compute_layout(&[left, panel("right")], 1000.0, None);
        assert_eq!(width(&layout, "left"), 0.0);
        assert_eq!(width(&layout, "right"), 1000.0);
        assert!(layout.handles[0].disabled);
    }

    #[test]
    fn fixed_panel_keeps_its_token_and_skips_the_handle() {
        let mut left = panel("left");
        left.resizable = false;
        left.initial_size = Some("200".to_string());
        let layout = compute_layout(&[left, panel("right")], 1000.0, None);
        assert_eq!(width(&layout, "left"), 128.0);
        assert_eq!(width(&layout, "right"), 872.0);
        assert!(layout.handles.is_empty());
    }

    #[test]
    fn saved_ratio_reserves_that_share() {
        let saved = [DtResizableSavedPanel {
            id: "left".to_string(),
            pixel_size: 100.0,
            collapsed: Some(false),
            auto_collapsed: None,
            manual_target_ratio: Some(0.25),
        }];
        let layout = compute_layout(&[panel("left"), panel("right")], 1000.0, Some(&saved));
        assert_eq!(width(&layout, "left"), 250.0);
        assert_eq!(width(&layout, "right"), 750.0);
    }

    #[test]
    fn odd_container_gives_the_extra_pixel_to_the_first_panel() {
        let layout = compute_layout(&[panel("left"), panel("right")], 1001.0, None);
        assert_eq!(width(&layout, "left") + width(&layout, "right"), 1001.0);
        assert_eq!(width(&layout, "left"), 501.0);
        assert_eq!(width(&layout, "right"), 500.0);
    }

    #[test]
    fn system_limits_stay_inside_user_limits() {
        let mut config = panel("main");
        config.user_min_size = Some("100".to_string());
        config.system_min_size = Some("50".to_string());
        config.user_max_size = Some("200".to_string());
        config.system_max_size = Some("500".to_string());
        let constraints = constraint_hierarchy(&config, 1000.0);
        assert_eq!(constraints.user_min, Some(64.0));
        assert_eq!(constraints.system_min, Some(64.0));
        assert_eq!(constraints.user_max, Some(128.0));
        assert_eq!(constraints.system_max, Some(128.0));
    }

    #[test]
    fn resize_move_rejects_a_panel_under_ten_pixels() {
        let before = runtime("left", 200.0);
        let after = runtime("right", 200.0);
        let panels = vec![before.clone(), after.clone()];
        let ok = process_resize_move(250.0, &before, &after, 400.0, &panels, 0.0);
        assert!(ok.valid);
        assert_eq!(ok.before_size, 250.0);
        assert_eq!(ok.after_size, 150.0);
        let too_small = process_resize_move(5.0, &before, &after, 400.0, &panels, 0.0);
        assert!(!too_small.valid);
    }

    #[test]
    fn keyboard_steps_follow_the_modifiers_and_the_axis() {
        assert_eq!(keyboard_increment(true, false, true), 1.0);
        assert_eq!(keyboard_increment(false, true, false), 1.0);
        assert_eq!(keyboard_increment(false, false, true), 24.0);
        assert_eq!(keyboard_increment(false, false, false), 8.0);
        assert_eq!(keyboard_direction("ArrowRight", "row"), Some(true));
        assert_eq!(keyboard_direction("ArrowLeft", "row"), Some(false));
        assert_eq!(keyboard_direction("ArrowDown", "column"), Some(true));
        assert_eq!(keyboard_direction("ArrowLeft", "column"), None);
    }

    #[test]
    fn classes_follow_the_vue_order() {
        assert_eq!(
            resizable_class("row", true, "qa"),
            "d-resizable d-resizable--row d-resizable--resizing qa"
        );
        assert_eq!(
            panel_class("qa", true, true),
            "d-resizable-panel qa d-resizable-panel--collapsed d-resizable-panel--fixed"
        );
        assert_eq!(
            handle_class("column", "qa", true, true),
            "d-resizable-handle d-resizable-handle--column qa d-resizable-handle--active d-resizable-handle--disabled"
        );
    }

    #[test]
    fn handle_style_centers_on_the_boundary() {
        let position = HandlePosition {
            id: "a:b".to_string(),
            before_panel_id: "a".to_string(),
            after_panel_id: "b".to_string(),
            left: 10.0,
            disabled: false,
        };
        assert_eq!(
            handle_style(Some(&position), None, ""),
            "inset-inline-start:8px"
        );
        assert_eq!(
            offset_style("content", 12.0, "both"),
            "padding-block-start:12px;padding-block-end:12px"
        );
        assert_eq!(offset_style("handle", 0.0, "start"), "");
    }

    #[test]
    fn collapse_rules_run_low_priority_first() {
        let panels = vec![runtime("side", 40.0), runtime("main", 400.0)];
        let rules = vec![
            DtResizableCollapseRule {
                panel_id: "main".to_string(),
                priority: 2,
                min_size_before_collapse: Some("800".to_string()),
            },
            DtResizableCollapseRule {
                panel_id: "side".to_string(),
                priority: 1,
                min_size_before_collapse: Some("100".to_string()),
            },
        ];
        let hits = check_auto_collapse_rules(&panels, &rules, 1000.0);
        assert_eq!(hits, vec!["side".to_string(), "main".to_string()]);
    }

    #[test]
    fn stored_json_drops_the_drag_ratio_and_rejects_a_bad_payload() {
        let panels = vec![DtResizableSavedPanel {
            id: "left".to_string(),
            pixel_size: 320.0,
            collapsed: Some(false),
            auto_collapsed: None,
            manual_target_ratio: Some(0.4),
        }];
        let json = serialize_stored(&panels);
        assert!(!json.contains("manualTargetRatio"));
        let parsed = parse_stored(&json).unwrap();
        assert_eq!(parsed[0].pixel_size, 320.0);
        assert!(parsed[0].manual_target_ratio.is_none());
        assert!(parse_stored("{\"nope\":true}").is_none());
    }

    #[test]
    fn reset_pair_keeps_the_combined_width() {
        let (before, after) = reset_pair_sizes(300.0, 700.0, 200.0, 200.0);
        assert_eq!(before + after, 1000.0);
        assert_eq!(before, 500.0);
        assert_eq!(after, 500.0);
    }

    fn runtime(id: &str, pixel_size: f64) -> PanelRuntime {
        PanelRuntime {
            id: id.to_string(),
            pixel_size,
            resizable: true,
            collapsible: true,
            collapsed: false,
            user_min: None,
            user_max: None,
            initial_size: None,
            auto_collapsed: false,
        }
    }
}
