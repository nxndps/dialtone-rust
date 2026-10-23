//! Headless Popper/Tippy placement: flip, shift, and hide.
//!
//! Mirrors `@popperjs/core` modifiers used by Dialtone Vue (`flip`,
//! `preventOverflow`, `hide`) without bundling Tippy.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn right(self) -> f64 {
        self.x + self.width
    }

    pub fn bottom(self) -> f64 {
        self.y + self.height
    }

    pub fn from_size(width: f64, height: f64) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width,
            height,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

impl Side {
    pub fn opposite(self) -> Self {
        match self {
            Self::Top => Self::Bottom,
            Self::Bottom => Self::Top,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    pub fn is_vertical(self) -> bool {
        matches!(self, Self::Top | Self::Bottom)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alignment {
    Start,
    Center,
    End,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Placement {
    pub side: Side,
    pub alignment: Alignment,
}

impl Placement {
    pub fn parse(value: &str) -> Self {
        let (side, alignment) = match value {
            "top-start" | "auto-start" => (Side::Top, Alignment::Start),
            "top-end" | "auto-end" => (Side::Top, Alignment::End),
            "bottom" => (Side::Bottom, Alignment::Center),
            "bottom-start" => (Side::Bottom, Alignment::Start),
            "bottom-end" => (Side::Bottom, Alignment::End),
            "left" => (Side::Left, Alignment::Center),
            "left-start" => (Side::Left, Alignment::Start),
            "left-end" => (Side::Left, Alignment::End),
            "right" => (Side::Right, Alignment::Center),
            "right-start" => (Side::Right, Alignment::Start),
            "right-end" => (Side::Right, Alignment::End),
            "auto" => (Side::Top, Alignment::Center),
            _ => (Side::Top, Alignment::Center),
        };
        Self { side, alignment }
    }

    pub fn is_auto(value: &str) -> bool {
        value == "auto" || value.starts_with("auto-")
    }

    pub fn opposite(self) -> Self {
        Self {
            side: self.side.opposite(),
            alignment: self.alignment,
        }
    }

    pub fn as_str(self) -> &'static str {
        match (self.side, self.alignment) {
            (Side::Top, Alignment::Center) => "top",
            (Side::Top, Alignment::Start) => "top-start",
            (Side::Top, Alignment::End) => "top-end",
            (Side::Bottom, Alignment::Center) => "bottom",
            (Side::Bottom, Alignment::Start) => "bottom-start",
            (Side::Bottom, Alignment::End) => "bottom-end",
            (Side::Left, Alignment::Center) => "left",
            (Side::Left, Alignment::Start) => "left-start",
            (Side::Left, Alignment::End) => "left-end",
            (Side::Right, Alignment::Center) => "right",
            (Side::Right, Alignment::Start) => "right-start",
            (Side::Right, Alignment::End) => "right-end",
        }
    }

    /// Dialtone CSS class that draws the arrow on the tooltip edge facing the anchor.
    pub fn arrow_class(self) -> &'static str {
        match (self.side, self.alignment) {
            (Side::Top, Alignment::Start) => "d-tooltip__arrow--bottom-left",
            (Side::Top, Alignment::End) => "d-tooltip__arrow--bottom-right",
            (Side::Top, Alignment::Center) => "d-tooltip__arrow--bottom-center",
            (Side::Bottom, Alignment::Start) => "d-tooltip__arrow--top-left",
            (Side::Bottom, Alignment::End) => "d-tooltip__arrow--top-right",
            (Side::Bottom, Alignment::Center) => "d-tooltip__arrow--top-center",
            (Side::Left, Alignment::Start) => "d-tooltip__arrow--right-top",
            (Side::Left, Alignment::End) => "d-tooltip__arrow--right-bottom",
            (Side::Left, Alignment::Center) => "d-tooltip__arrow--right-center",
            (Side::Right, Alignment::Start) => "d-tooltip__arrow--left-top",
            (Side::Right, Alignment::End) => "d-tooltip__arrow--left-bottom",
            (Side::Right, Alignment::Center) => "d-tooltip__arrow--left-center",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Options {
    pub offset: f64,
    pub padding: f64,
    pub viewport: Rect,
    pub fallbacks: Vec<Placement>,
    pub auto: bool,
    /// Popper `preventOverflow.tether`. True keeps the raw placement and clips.
    /// False shifts the popup into the viewport, which can cover the anchor.
    pub tether: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComputedPosition {
    pub x: f64,
    pub y: f64,
    pub placement: Placement,
    pub hidden: bool,
    pub arrow_offset: f64,
}

pub fn compute_position(
    reference: Rect,
    floating: Rect,
    preferred: Placement,
    options: &Options,
) -> ComputedPosition {
    let size = Rect::from_size(floating.width, floating.height);
    let mut candidates = Vec::new();
    if options.auto || Placement::is_auto(preferred.as_str()) {
        candidates.extend(auto_placements(
            reference,
            options.viewport,
            preferred.alignment,
        ));
    } else {
        candidates.push(preferred);
        let opposite = preferred.opposite();
        if opposite != preferred {
            candidates.push(opposite);
        }
        for fallback in &options.fallbacks {
            if !candidates.contains(fallback) {
                candidates.push(*fallback);
            }
        }
        if options.fallbacks.is_empty() {
            for extra in auto_placements(reference, options.viewport, preferred.alignment) {
                if !candidates.contains(&extra) {
                    candidates.push(extra);
                }
            }
        }
    }

    let mut best = candidates.first().copied().unwrap_or(preferred);
    let mut best_score = f64::INFINITY;
    let mut best_xy = (reference.x, reference.y);
    for placement in candidates {
        let (x, y) = raw_coords(reference, size, placement, options.offset);
        let score = overflow_score(x, y, size, options.viewport, options.padding);
        if score < best_score {
            best_score = score;
            best = placement;
            best_xy = (x, y);
            if score == 0.0 {
                break;
            }
        }
    }

    let (x, y) = if options.tether {
        best_xy
    } else {
        shift(
            best_xy.0,
            best_xy.1,
            size,
            options.viewport,
            options.padding,
        )
    };
    let arrow_offset = arrow_offset(reference, x, y, size, best);
    ComputedPosition {
        x,
        y,
        placement: best,
        hidden: reference_hidden(reference, options.viewport),
        arrow_offset,
    }
}

fn auto_placements(reference: Rect, viewport: Rect, alignment: Alignment) -> Vec<Placement> {
    let mut sides = [
        (Side::Top, (reference.y - viewport.y).max(0.0)),
        (
            Side::Bottom,
            (viewport.bottom() - reference.bottom()).max(0.0),
        ),
        (Side::Right, (viewport.right() - reference.right()).max(0.0)),
        (Side::Left, (reference.x - viewport.x).max(0.0)),
    ];
    sides.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    sides
        .into_iter()
        .map(|(side, _)| Placement { side, alignment })
        .collect()
}

fn raw_coords(reference: Rect, size: Rect, placement: Placement, offset: f64) -> (f64, f64) {
    let align_x = match placement.alignment {
        Alignment::Start => reference.x,
        Alignment::Center => reference.x + reference.width / 2.0 - size.width / 2.0,
        Alignment::End => reference.right() - size.width,
    };
    let align_y = match placement.alignment {
        Alignment::Start => reference.y,
        Alignment::Center => reference.y + reference.height / 2.0 - size.height / 2.0,
        Alignment::End => reference.bottom() - size.height,
    };
    match placement.side {
        Side::Top => (align_x, reference.y - size.height - offset),
        Side::Bottom => (align_x, reference.bottom() + offset),
        Side::Left => (reference.x - size.width - offset, align_y),
        Side::Right => (reference.right() + offset, align_y),
    }
}

fn overflow_score(x: f64, y: f64, size: Rect, viewport: Rect, padding: f64) -> f64 {
    let top = (viewport.y + padding) - y;
    let left = (viewport.x + padding) - x;
    let bottom = (y + size.height) - (viewport.bottom() - padding);
    let right = (x + size.width) - (viewport.right() - padding);
    top.max(0.0) + left.max(0.0) + bottom.max(0.0) + right.max(0.0)
}

fn shift(x: f64, y: f64, size: Rect, viewport: Rect, padding: f64) -> (f64, f64) {
    let min_x = viewport.x + padding;
    let max_x = viewport.right() - padding - size.width;
    let min_y = viewport.y + padding;
    let max_y = viewport.bottom() - padding - size.height;
    (
        x.clamp(min_x.min(max_x), max_x.max(min_x)),
        y.clamp(min_y.min(max_y), max_y.max(min_y)),
    )
}

fn arrow_offset(reference: Rect, x: f64, y: f64, size: Rect, placement: Placement) -> f64 {
    let min = 8.0;
    if placement.side.is_vertical() {
        let center = reference.x + reference.width / 2.0;
        (center - x).clamp(min, (size.width - min).max(min))
    } else {
        let center = reference.y + reference.height / 2.0;
        (center - y).clamp(min, (size.height - min).max(min))
    }
}

fn reference_hidden(reference: Rect, viewport: Rect) -> bool {
    reference.right() < viewport.x
        || reference.x > viewport.right()
        || reference.bottom() < viewport.y
        || reference.y > viewport.bottom()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn viewport() -> Rect {
        Rect {
            x: 0.0,
            y: 0.0,
            width: 1000.0,
            height: 800.0,
        }
    }

    fn options() -> Options {
        Options {
            offset: 12.0,
            padding: 8.0,
            viewport: viewport(),
            fallbacks: Vec::new(),
            auto: false,
            tether: false,
        }
    }

    #[test]
    fn places_above_when_there_is_room() {
        let reference = Rect {
            x: 400.0,
            y: 400.0,
            width: 40.0,
            height: 20.0,
        };
        let floating = Rect::from_size(80.0, 32.0);
        let result = compute_position(reference, floating, Placement::parse("top"), &options());
        assert_eq!(result.placement.side, Side::Top);
        assert!(result.y + floating.height <= reference.y);
        assert!(!result.hidden);
    }

    #[test]
    fn flips_to_bottom_when_top_overflows() {
        let reference = Rect {
            x: 400.0,
            y: 10.0,
            width: 40.0,
            height: 20.0,
        };
        let floating = Rect::from_size(80.0, 40.0);
        let result = compute_position(reference, floating, Placement::parse("top"), &options());
        assert_eq!(result.placement.side, Side::Bottom);
        assert!(result.y >= reference.bottom());
    }

    #[test]
    fn shifts_horizontally_to_stay_on_screen() {
        let reference = Rect {
            x: 5.0,
            y: 400.0,
            width: 20.0,
            height: 20.0,
        };
        let floating = Rect::from_size(200.0, 32.0);
        let result = compute_position(reference, floating, Placement::parse("top"), &options());
        assert!(result.x >= 8.0);
        assert!(result.x + floating.width <= 992.0);
    }

    #[test]
    fn tether_keeps_the_placement_when_it_overflows() {
        let reference = Rect {
            x: 5.0,
            y: 400.0,
            width: 20.0,
            height: 20.0,
        };
        let floating = Rect::from_size(200.0, 32.0);
        let mut options = options();
        options.tether = true;
        options.fallbacks = vec![Placement::parse("top")];
        let result = compute_position(reference, floating, Placement::parse("top"), &options);
        assert!(result.x < 0.0);
    }

    #[test]
    fn hides_when_reference_is_offscreen() {
        let reference = Rect {
            x: -200.0,
            y: -200.0,
            width: 40.0,
            height: 20.0,
        };
        let result = compute_position(
            reference,
            Rect::from_size(80.0, 32.0),
            Placement::parse("top"),
            &options(),
        );
        assert!(result.hidden);
    }
}
