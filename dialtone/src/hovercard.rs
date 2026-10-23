use std::time::Duration;

use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use super::popover::DtPopover;
use super::unique_id;

/// Matches Dialtone Vue `TOOLTIP_DELAY_MS`, the hovercard enter and leave default.
const HOVERCARD_DELAY_MS: u64 = 300;
/// Vue `offset: [0, 16]`. The skid stays 0.
const HOVERCARD_OFFSET: f64 = 16.0;

const PLACEMENTS: &[&str] = &[
    "top",
    "top-start",
    "top-end",
    "right",
    "right-start",
    "right-end",
    "left",
    "left-start",
    "left-end",
    "bottom",
    "bottom-start",
    "bottom-end",
];

const PADDINGS: &[&str] = &["none", "small", "medium", "large"];

/// `show` and `hide` from Vue `defineExpose`. Create it during setup.
///
/// `show` is `onMouseEnter`. `hide` is `onMouseLeave`. Both no-op when `open`
/// is set, matching the Vue timers.
#[derive(Clone, Copy)]
pub struct DtHovercardHandle {
    ops: StoredValue<Option<HoverOps>>,
}

#[derive(Clone, Copy)]
struct HoverOps {
    show: Callback<()>,
    hide: Callback<()>,
}

impl DtHovercardHandle {
    pub fn new() -> Self {
        Self {
            ops: StoredValue::new(None),
        }
    }

    /// `show`.
    pub fn show(&self) {
        self.call(|ops| ops.show);
    }

    /// `hide`.
    pub fn hide(&self) {
        self.call(|ops| ops.hide);
    }

    fn call(&self, pick: impl Fn(HoverOps) -> Callback<()>) {
        if let Some(ops) = self.ops.try_get_value().flatten() {
            pick(ops).run(());
        }
    }
}

impl Default for DtHovercardHandle {
    fn default() -> Self {
        Self::new()
    }
}

pub(crate) fn hovercard_placement(value: &'static str) -> &'static str {
    if PLACEMENTS.contains(&value) {
        return value;
    }
    leptos::logging::warn!(
        "DtHovercard placement `{value}` is not a tooltip direction; using top-start"
    );
    "top-start"
}

pub(crate) fn hovercard_padding(value: &'static str) -> &'static str {
    if PADDINGS.contains(&value) {
        return value;
    }
    leptos::logging::warn!(
        "DtHovercard padding `{value}` is not none, small, medium, or large; using large"
    );
    "large"
}

pub(crate) fn hovercard_transition(enabled: bool) -> &'static str {
    if enabled {
        "fade"
    } else {
        ""
    }
}

fn clear_timer(slot: StoredValue<Option<TimeoutHandle>>) {
    let _ = slot.try_update_value(|timer| {
        if let Some(handle) = timer.take() {
            handle.clear();
        }
    });
}

/// Dialtone Vue `DtHovercard` (`dialtone/v10.7.0`).
///
/// The card composes [`DtPopover`](crate::DtPopover). Unset `open` shows on
/// pointer enter after `enter_delay` (`enterDelay`, default 300) and hides
/// after `leave_delay` (`leaveDelay`). A set `open` is controlled and the
/// timers do nothing. Focus inside the content keeps it open. The anchor
/// click does not toggle. Escape and an outside click close it and restore
/// focus. `offset` is the distance in px (`offset` `[0, distance]`, default
/// 16). `append_to` (`appendTo`) `root` and an `HTMLElement` are not ported.
/// `external_anchor_id` is an element id. An `HTMLElement` anchor is not
/// ported. [`DtHovercardHandle`](crate::DtHovercardHandle) is Vue `show` / `hide`.
#[component]
pub fn DtHovercard(
    /// Controlled visibility (`open`). Unset hovers.
    #[prop(optional)]
    open: MaybeProp<bool>,
    /// `update:open` when `open` is set. Escape and an outside click pass `false`.
    #[prop(optional, into)]
    on_open: Option<Callback<bool>>,
    /// Fired after open or close (`opened`).
    #[prop(optional, into)]
    on_opened: Option<Callback<bool>>,
    /// `top-start` by default (`placement`).
    #[prop(optional, default = "top-start")]
    placement: &'static str,
    /// Comma-separated fallbacks (`fallbackPlacements`). Default `auto`.
    #[prop(optional, default = "auto")]
    fallback_placements: &'static str,
    /// `none`, `small`, `medium`, or `large` (`padding`). Default `large`.
    #[prop(optional, default = "large")]
    padding: &'static str,
    /// Distance in px. Vue `offset` `[0, distance]`. Default 16.
    #[prop(optional, default = HOVERCARD_OFFSET)]
    offset: f64,
    /// Fade when true (`transition`). Default false.
    #[prop(optional)]
    transition: bool,
    /// `light`, `dark`, or `invert` (`contentMode`).
    #[prop(optional, default = "")]
    content_mode: &'static str,
    /// Popover id. Generated when empty.
    #[prop(optional, into)]
    id: String,
    /// Extra class on the anchor wrapper (`anchorClass`).
    #[prop(optional, into)]
    anchor_class: String,
    /// Extra class on the header (`headerClass`).
    #[prop(optional, into)]
    header_class: String,
    /// Extra class on the footer (`footerClass`).
    #[prop(optional, into)]
    footer_class: String,
    /// Extra class on the dialog (`dialogClass`).
    #[prop(optional, into)]
    dialog_class: String,
    /// Extra class on the content (`contentClass`).
    #[prop(optional, into)]
    content_class: String,
    /// `body` or `parent` (`appendTo`). Default `body`.
    #[prop(optional, default = "body")]
    append_to: &'static str,
    /// Delay before show, in ms (`enterDelay`).
    #[prop(optional, default = HOVERCARD_DELAY_MS)]
    enter_delay: u64,
    /// Delay before hide, in ms (`leaveDelay`).
    #[prop(optional, default = HOVERCARD_DELAY_MS)]
    leave_delay: u64,
    /// Element id to position against (`externalAnchor` id).
    #[prop(optional, into)]
    external_anchor_id: String,
    /// Anchor slot.
    #[prop(optional)]
    anchor: Option<ViewFn>,
    /// Header slot (`headerContent`).
    #[prop(optional)]
    header: Option<ViewFn>,
    /// Footer slot (`footerContent`).
    #[prop(optional)]
    footer: Option<ViewFn>,
    /// `show` / `hide`.
    #[prop(optional)]
    handle: Option<DtHovercardHandle>,
    /// Content slot.
    children: ChildrenFn,
) -> impl IntoView {
    let internal = RwSignal::new(false);
    let shown = Signal::derive(move || open.get().unwrap_or_else(|| internal.get()));
    let pointer_over = RwSignal::new(false);
    let content_focused = RwSignal::new(false);
    let in_timer = StoredValue::new(None::<TimeoutHandle>);
    let out_timer = StoredValue::new(None::<TimeoutHandle>);
    let placement = hovercard_placement(placement);
    let padding = hovercard_padding(padding);
    let transition_name = hovercard_transition(transition);
    let dialog_id = if id.is_empty() {
        unique_id("dt-hovercard")
    } else {
        id
    };
    let append_to = if matches!(append_to, "body" | "parent") {
        append_to
    } else {
        leptos::logging::warn!(
            "DtHovercard append_to `{append_to}` is not body or parent; using body"
        );
        "body"
    };

    let schedule_open = Callback::new(move |_: ()| {
        if open.get_untracked().is_some() {
            return;
        }
        clear_timer(out_timer);
        clear_timer(in_timer);
        match set_timeout_with_handle(
            move || {
                let _ = in_timer.try_update_value(|timer| *timer = None);
                internal.set(true);
            },
            Duration::from_millis(enter_delay),
        ) {
            Ok(timer) => in_timer.set_value(Some(timer)),
            Err(_) => internal.set(true),
        }
    });
    let schedule_close = Callback::new(move |_: ()| {
        if open.get_untracked().is_some() {
            return;
        }
        clear_timer(in_timer);
        clear_timer(out_timer);
        match set_timeout_with_handle(
            move || {
                let _ = out_timer.try_update_value(|timer| *timer = None);
                internal.set(false);
            },
            Duration::from_millis(leave_delay),
        ) {
            Ok(timer) => out_timer.set_value(Some(timer)),
            Err(_) => internal.set(false),
        }
    });
    let on_enter = Callback::new(move |_: ()| {
        pointer_over.set(true);
        schedule_open.run(());
    });
    let on_leave = Callback::new(move |_: ()| {
        pointer_over.set(false);
        if content_focused.get_untracked() {
            return;
        }
        schedule_close.run(());
    });
    let apply_shell = Callback::new(move |next: bool| {
        clear_timer(in_timer);
        clear_timer(out_timer);
        if open.get_untracked().is_some() {
            if let Some(callback) = on_open {
                callback.run(next);
            }
        } else {
            internal.set(next);
        }
    });

    if let Some(handle) = handle {
        handle.ops.set_value(Some(HoverOps {
            show: on_enter,
            hide: on_leave,
        }));
        on_cleanup(move || handle.ops.set_value(None));
    }

    let saw_anchor = StoredValue::new(false);
    let watch_id = dialog_id.clone();
    Effect::new(move |_| {
        let watch_id = watch_id.clone();
        let Some(body) = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.body())
        else {
            return;
        };
        let closure = Closure::wrap(Box::new(
            move |_records: js_sys::Array, _observer: web_sys::MutationObserver| {
                let Some(document) = web_sys::window().and_then(|window| window.document()) else {
                    return;
                };
                let selector = format!("[aria-controls=\"{watch_id}\"]");
                let found = document.query_selector(&selector).ok().flatten();
                let lost = match &found {
                    Some(element) => {
                        saw_anchor.set_value(true);
                        !element.is_connected()
                    }
                    None => saw_anchor.get_value(),
                };
                if lost {
                    saw_anchor.set_value(false);
                    apply_shell.run(false);
                }
            },
        )
            as Box<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>);
        let Ok(observer) = web_sys::MutationObserver::new(closure.as_ref().unchecked_ref()) else {
            return;
        };
        let options = web_sys::MutationObserverInit::new();
        options.set_child_list(true);
        options.set_subtree(true);
        if observer.observe_with_options(&body, &options).is_err() {
            return;
        }
        closure.forget();
        on_cleanup(move || observer.disconnect());
    });

    view! {
        <DtPopover
            id=dialog_id
            open=shown
            on_open_change=apply_shell
            nostrip:on_opened=on_opened
            placement=placement
            fallback_placements=fallback_placements
            padding=padding
            offset=offset
            modal=false
            tether=true
            toggle_on_click=false
            focus_trap=true
            data_qa="dt-hovercard"
            initial_focus="none"
            content_mode=content_mode
            append_to=append_to
            transition=transition_name
            anchor_class=anchor_class
            header_class=header_class
            footer_class=footer_class
            dialog_class=dialog_class
            content_class=content_class
            external_anchor_id=external_anchor_id
            nostrip:anchor=anchor
            nostrip:header=header
            nostrip:footer=footer
            on_pointer_enter=on_enter
            on_pointer_leave=on_leave
        >
            <div
                on:focusin=move |_| {
                    content_focused.set(true);
                    schedule_open.run(());
                }
                on:focusout=move |_| {
                    content_focused.set(false);
                    if !pointer_over.get_untracked() {
                        schedule_close.run(());
                    }
                }
            >
                {children()}
            </div>
        </DtPopover>
    }
}

#[cfg(test)]
mod tests {
    use super::{hovercard_padding, hovercard_placement, hovercard_transition};

    #[test]
    fn placement_and_padding_fall_back() {
        assert_eq!(hovercard_placement("bottom-end"), "bottom-end");
        assert_eq!(hovercard_placement("auto"), "top-start");
        assert_eq!(hovercard_padding("small"), "small");
        assert_eq!(hovercard_padding("xl"), "large");
    }

    #[test]
    fn transition_is_fade_only_when_enabled() {
        assert_eq!(hovercard_transition(false), "");
        assert_eq!(hovercard_transition(true), "fade");
    }
}
