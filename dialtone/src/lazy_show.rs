use std::time::Duration;

use leptos::leptos_dom::helpers::set_timeout;
use leptos::prelude::*;

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Vue keeps the slot mounted after the first `show`.
fn keep_mounted(initialized: bool, show: bool) -> bool {
    initialized || show
}

/// `v-show` hides the wrapper. Leave transitions stay visible until they end.
fn hidden_style(visible: bool) -> &'static str {
    if visible {
        ""
    } else {
        "display: none"
    }
}

/// Vue `<Transition>` class pair for `name` and the current phase.
///
/// Phases are `enter-from`, `enter-to`, `leave-from`, `leave-to`, and `idle`.
fn motion_classes(name: &str, phase: &str) -> String {
    let suffix = match phase {
        "enter-from" => "enter-from",
        "enter-to" => "enter-to",
        "leave-from" => "leave-from",
        "leave-to" => "leave-to",
        _ => return String::new(),
    };
    if name.is_empty() {
        return String::new();
    }
    let active = if suffix.starts_with("enter") {
        "enter-active"
    } else {
        "leave-active"
    };
    format!("{name}-{suffix} {name}-{active}")
}

/// Durations from the vendored `.fade`, `.slide-down`, `.pop`, and `.shake` rules.
fn transition_duration_ms(name: &str) -> u64 {
    match name {
        "fade" => 180,
        "slide-down" => 280,
        "pop" => 500,
        "shake" => 820,
        _ => 300,
    }
}

fn schedule_motion(
    entering: bool,
    name: String,
    phase: RwSignal<&'static str>,
    visible: RwSignal<bool>,
    generation: StoredValue<u32>,
) {
    if name.is_empty() {
        phase.set("idle");
        visible.set(entering);
        return;
    }
    let token = generation.get_value().wrapping_add(1);
    generation.set_value(token);
    visible.set(true);
    phase.set(if entering { "enter-from" } else { "leave-from" });
    let ms = transition_duration_ms(&name);
    // Vue waits two frames so the from-class paints before the to-class.
    request_animation_frame(move || {
        request_animation_frame(move || {
            if generation.get_value() != token {
                return;
            }
            phase.set(if entering { "enter-to" } else { "leave-to" });
            set_timeout(
                move || {
                    if generation.get_value() != token {
                        return;
                    }
                    phase.set("idle");
                    if !entering {
                        visible.set(false);
                    }
                },
                Duration::from_millis(ms),
            );
        });
    });
}

/// Dialtone Vue `DtLazyShow` (`dialtone/v10.7.0`).
///
/// The root is a `div`. The default slot stays unmounted until `show` is true
/// the first time (`initialized`). After that the slot stays in the DOM and
/// `show` false hides the div with `display: none`, which is Vue `v-show`.
/// `transition` is the Vue transition `name` (`fade`, `slide-down`, `pop`,
/// `shake`, or any other class prefix). An empty name does not add classes.
/// `appear` runs the enter transition when `show` starts true. `class` is
/// Vue `$attrs.class` on the div. Vue Router is not involved.
#[component]
pub fn DtLazyShow(
    /// Whether the slot is shown (`show`). False hides the wrapper.
    #[prop(into)]
    show: Signal<bool>,
    /// Vue transition name (`transition`). Empty skips the classes.
    #[prop(optional, into)]
    transition: String,
    /// Enter transition on the first render when `show` starts true (`appear`).
    #[prop(optional)]
    appear: bool,
    /// Extra class on the wrapper (`$attrs.class`).
    #[prop(optional, into)]
    class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let initial = show.get_untracked();
    let initialized = RwSignal::new(initial);
    let visible = RwSignal::new(initial);
    let phase = RwSignal::new("idle");
    let generation = StoredValue::new(0u32);
    let started = StoredValue::new(false);
    let class_name = StoredValue::new(class);
    let transition_name = StoredValue::new(transition);

    Effect::new(move |_| {
        let next = show.get();
        let name = transition_name.get_value();
        let first = !started.get_value();
        started.set_value(true);
        initialized.set(keep_mounted(initialized.get_untracked(), next));
        if first {
            if next && appear && !name.is_empty() {
                schedule_motion(true, name, phase, visible, generation);
            }
            return;
        }
        if next == visible.get_untracked() && phase.get_untracked() == "idle" {
            return;
        }
        schedule_motion(next, name, phase, visible, generation);
    });

    view! {
        <div
            class=move || {
                join_class([
                    motion_classes(&transition_name.get_value(), phase.get()).as_str(),
                    class_name.get_value().as_str(),
                ])
            }
            style=move || hidden_style(visible.get())
        >
            <Show when=move || initialized.get()>
                {children()}
            </Show>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slot_stays_mounted_after_the_first_show() {
        assert!(!keep_mounted(false, false));
        assert!(keep_mounted(false, true));
        assert!(keep_mounted(true, false));
        assert!(keep_mounted(true, true));
    }

    #[test]
    fn hidden_style_matches_v_show() {
        assert_eq!(hidden_style(true), "");
        assert_eq!(hidden_style(false), "display: none");
    }

    #[test]
    fn motion_classes_follow_the_vue_transition_names() {
        assert_eq!(motion_classes("", "enter-from"), "");
        assert_eq!(motion_classes("fade", "idle"), "");
        assert_eq!(
            motion_classes("fade", "enter-from"),
            "fade-enter-from fade-enter-active"
        );
        assert_eq!(
            motion_classes("fade", "enter-to"),
            "fade-enter-to fade-enter-active"
        );
        assert_eq!(
            motion_classes("slide-down", "leave-from"),
            "slide-down-leave-from slide-down-leave-active"
        );
        assert_eq!(
            motion_classes("pop", "leave-to"),
            "pop-leave-to pop-leave-active"
        );
        assert_eq!(
            motion_classes("shake", "enter-to"),
            "shake-enter-to shake-enter-active"
        );
    }

    #[test]
    fn durations_follow_the_vendored_css() {
        assert_eq!(transition_duration_ms("fade"), 180);
        assert_eq!(transition_duration_ms("slide-down"), 280);
        assert_eq!(transition_duration_ms("pop"), 500);
        assert_eq!(transition_duration_ms("shake"), 820);
        assert_eq!(transition_duration_ms("other"), 300);
    }
}
