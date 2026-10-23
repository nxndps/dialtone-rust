use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use leptos::html;
use leptos::leptos_dom::helpers::{set_timeout_with_handle, TimeoutHandle};
use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Vue `completeAnimation` waits this long before `reset` and `start`.
const MOTION_TEXT_LOOP_GAP_MS: u64 = 500;

static NEXT_MOTION_TEXT: AtomicU64 = AtomicU64::new(1);

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct MotionTiming {
    character_delay: u64,
    word_delay: u64,
    duration: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MotionWord {
    text: String,
    chars: Vec<char>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MotionFrame {
    words: Vec<MotionWord>,
    visible_words: usize,
    visible_chars: Vec<usize>,
    animating: bool,
    paused: bool,
    key: u32,
}

impl Default for MotionFrame {
    fn default() -> Self {
        Self {
            words: Vec::new(),
            visible_words: 0,
            visible_chars: Vec::new(),
            animating: false,
            paused: false,
            key: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MotionWait {
    Word,
    Char(usize),
    Finish,
    Restart,
}

#[derive(Clone, Debug, PartialEq)]
enum MotionNotice {
    Start,
    Pause,
    Resume,
    Complete,
    Progress {
        words_complete: usize,
        total_words: usize,
        progress: f64,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Action {
    Start,
    Pause,
    Resume,
    Reset,
    Skip,
    Replace,
}

struct MotionPlayback {
    words: Vec<MotionWord>,
    visible_words: usize,
    visible_chars: Vec<usize>,
    animating: bool,
    paused: bool,
    looped: bool,
    mode: &'static str,
    reduced: bool,
    key: u32,
    wait: Option<MotionWait>,
    pending_words: Vec<MotionWord>,
}

impl MotionPlayback {
    fn new(looped: bool) -> Self {
        Self {
            words: Vec::new(),
            visible_words: 0,
            visible_chars: Vec::new(),
            animating: false,
            paused: false,
            looped,
            mode: "gradient-in",
            reduced: false,
            key: 0,
            wait: None,
            pending_words: Vec::new(),
        }
    }

    fn frame(&self) -> MotionFrame {
        MotionFrame {
            words: self.words.clone(),
            visible_words: self.visible_words,
            visible_chars: self.visible_chars.clone(),
            animating: self.animating,
            paused: self.paused,
            key: self.key,
        }
    }

    fn set_words(&mut self, words: Vec<MotionWord>) {
        self.words = words;
        self.visible_words = 0;
        self.visible_chars = vec![0; self.words.len()];
    }

    fn reset_counts(&mut self) {
        self.wait = None;
        self.animating = false;
        self.paused = false;
        self.visible_words = 0;
        self.visible_chars = vec![0; self.words.len()];
        self.key = self.key.wrapping_add(1);
    }

    fn apply(&mut self, action: Action) -> Vec<MotionNotice> {
        match action {
            Action::Start => self.start(),
            Action::Pause => self.pause(),
            Action::Resume => self.resume(),
            Action::Reset => {
                let looped = self.looped;
                self.reset_counts();
                if looped {
                    self.start()
                } else {
                    Vec::new()
                }
            }
            Action::Skip => self.skip(),
            Action::Replace => {
                let words = std::mem::take(&mut self.pending_words);
                self.reset_counts();
                self.set_words(words);
                Vec::new()
            }
        }
    }

    fn start(&mut self) -> Vec<MotionNotice> {
        if self.animating {
            return Vec::new();
        }
        self.animating = true;
        self.paused = false;
        let mut notes = vec![MotionNotice::Start];
        if self.reduced || self.mode == "none" {
            self.show_all();
            return notes;
        }
        if is_static(self.mode) {
            return notes;
        }
        notes.extend(self.show_next_word());
        notes
    }

    fn pause(&mut self) -> Vec<MotionNotice> {
        if !self.animating || self.paused {
            return Vec::new();
        }
        self.paused = true;
        self.wait = None;
        vec![MotionNotice::Pause]
    }

    fn resume(&mut self) -> Vec<MotionNotice> {
        if !self.paused {
            return Vec::new();
        }
        self.paused = false;
        let mut notes = vec![MotionNotice::Resume];
        notes.extend(self.show_next_word());
        notes
    }

    fn skip(&mut self) -> Vec<MotionNotice> {
        self.show_all();
        Vec::new()
    }

    fn show_all(&mut self) {
        self.visible_words = self.words.len();
        self.visible_chars = self.words.iter().map(|word| word.chars.len()).collect();
        self.wait = Some(MotionWait::Finish);
    }

    fn show_next_word(&mut self) -> Vec<MotionNotice> {
        if self.paused || self.visible_words >= self.words.len() {
            if self.visible_words >= self.words.len() {
                return self.complete_animation();
            }
            return Vec::new();
        }
        self.wait = Some(MotionWait::Word);
        Vec::new()
    }

    fn begin_word(&mut self) -> Vec<MotionNotice> {
        self.visible_words += 1;
        let total_words = self.words.len();
        let words_complete = self.visible_words;
        let progress = words_complete as f64 / total_words as f64;
        let mut notes = vec![MotionNotice::Progress {
            words_complete,
            total_words,
            progress,
        }];
        notes.extend(self.animate_chars(self.visible_words - 1));
        notes
    }

    fn animate_chars(&mut self, word_idx: usize) -> Vec<MotionNotice> {
        if self.paused || word_idx >= self.words.len() {
            return Vec::new();
        }
        if word_idx >= self.visible_chars.len() {
            return Vec::new();
        }
        self.visible_chars[word_idx] = 0;
        self.reveal_char(word_idx)
    }

    fn reveal_char(&mut self, word_idx: usize) -> Vec<MotionNotice> {
        if self.paused || word_idx >= self.words.len() || word_idx >= self.visible_chars.len() {
            return Vec::new();
        }
        let total = self.words[word_idx].chars.len();
        if self.visible_chars[word_idx] >= total {
            return self.show_next_word();
        }
        self.visible_chars[word_idx] += 1;
        self.wait = Some(MotionWait::Char(word_idx));
        Vec::new()
    }

    fn complete_animation(&mut self) -> Vec<MotionNotice> {
        self.animating = false;
        self.wait = None;
        if self.looped {
            self.wait = Some(MotionWait::Restart);
        }
        vec![MotionNotice::Complete]
    }

    fn on_timer(&mut self) -> Vec<MotionNotice> {
        match self.wait.take() {
            Some(MotionWait::Finish) => {
                self.animating = false;
                vec![MotionNotice::Complete]
            }
            Some(MotionWait::Restart) => {
                self.reset_counts();
                self.start()
            }
            Some(MotionWait::Word) => self.begin_word(),
            Some(MotionWait::Char(index)) => self.reveal_char(index),
            None => Vec::new(),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Load {
    Fresh,
    Words(String),
    AwaitingSlot,
    Slot,
}

/// Progress payload for `progress`.
///
/// `progress` is `words_complete / total_words`, the same ratio Vue emits.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DtMotionTextProgress {
    pub words_complete: usize,
    pub total_words: usize,
    pub progress: f64,
}

#[derive(Clone)]
struct MotionOps {
    token: u64,
    start: Callback<()>,
    pause: Callback<()>,
    resume: Callback<()>,
    reset: Callback<()>,
    skip_to_end: Callback<()>,
}

/// Calls `start`, `pause`, `resume`, `reset`, and `skipToEnd` on one `DtMotionText`.
///
/// Create it with [`DtMotionTextHandle::new`] during component setup and pass it
/// as `handle`. Methods do nothing before the component registers and after it
/// unmounts.
#[derive(Clone, Copy)]
pub struct DtMotionTextHandle {
    ops: StoredValue<Option<MotionOps>>,
}

impl DtMotionTextHandle {
    pub fn new() -> Self {
        Self {
            ops: StoredValue::new(None),
        }
    }

    /// `start`.
    pub fn start(&self) {
        self.call(|ops| &ops.start);
    }

    /// `pause`.
    pub fn pause(&self) {
        self.call(|ops| &ops.pause);
    }

    /// `resume`.
    pub fn resume(&self) {
        self.call(|ops| &ops.resume);
    }

    /// `reset`.
    pub fn reset(&self) {
        self.call(|ops| &ops.reset);
    }

    /// `skipToEnd`.
    pub fn skip_to_end(&self) {
        self.call(|ops| &ops.skip_to_end);
    }

    fn call(&self, pick: impl Fn(&MotionOps) -> &Callback<()>) {
        if let Some(ops) = self.ops.try_get_value().flatten() {
            pick(&ops).run(());
        }
    }
}

/// Words from `String.match(/\S+\s*/g)`. Leading space is skipped. Trailing
/// space stays on the word.
fn motion_words(text: &str) -> Vec<MotionWord> {
    let chars: Vec<char> = text.chars().collect();
    let mut words = Vec::new();
    let mut index = 0;
    while index < chars.len() {
        if chars[index].is_whitespace() {
            index += 1;
            continue;
        }
        let start = index;
        while index < chars.len() && !chars[index].is_whitespace() {
            index += 1;
        }
        while index < chars.len() && chars[index].is_whitespace() {
            index += 1;
        }
        let slice = &chars[start..index];
        words.push(MotionWord {
            text: slice.iter().collect(),
            chars: slice.to_vec(),
        });
    }
    words
}

/// `MOTION_TEXT_TIMING_PRESETS`. An unknown `speed` warns and uses `300`.
fn motion_timing(speed: &str) -> MotionTiming {
    match speed {
        "100" => MotionTiming {
            character_delay: 10,
            word_delay: 15,
            duration: 300,
        },
        "200" => MotionTiming {
            character_delay: 20,
            word_delay: 30,
            duration: 600,
        },
        "300" => MotionTiming {
            character_delay: 30,
            word_delay: 50,
            duration: 1000,
        },
        "400" => MotionTiming {
            character_delay: 50,
            word_delay: 100,
            duration: 1500,
        },
        "500" => MotionTiming {
            character_delay: 80,
            word_delay: 180,
            duration: 2100,
        },
        _ => {
            leptos::logging::warn!(
                "DtMotionText speed \"{speed}\" is not 100, 200, 300, 400, or 500. Using 300."
            );
            MotionTiming {
                character_delay: 30,
                word_delay: 50,
                duration: 1000,
            }
        }
    }
}

fn motion_style(timing: MotionTiming) -> String {
    format!(
        "--d-motion-text-duration: {}ms; --d-motion-text-char-duration: {}ms; --d-motion-text-word-duration: {}ms;",
        timing.duration,
        timing.duration,
        timing.duration * 2
    )
}

/// Known `animationMode`. Anything else warns and uses `gradient-in`.
fn resolve_mode(mode: &str) -> &'static str {
    match mode {
        "gradient-in" => "gradient-in",
        "fade-in" => "fade-in",
        "slide-in" => "slide-in",
        "gradient-sweep" => "gradient-sweep",
        "shimmer" => "shimmer",
        "none" => "none",
        _ => {
            leptos::logging::warn!(
                "DtMotionText animation_mode \"{mode}\" is not gradient-in, fade-in, slide-in, gradient-sweep, shimmer, or none. Using gradient-in."
            );
            "gradient-in"
        }
    }
}

fn mode_class(mode: &str) -> &'static str {
    match mode {
        "gradient-in" => "d-motion-text--gradient-in",
        "fade-in" => "d-motion-text--fade-in",
        "slide-in" => "d-motion-text--slide-in",
        "gradient-sweep" => "d-motion-text--gradient-sweep",
        "shimmer" => "d-motion-text--shimmer",
        "none" => "d-motion-text--none",
        _ => "",
    }
}

fn is_static(mode: &str) -> bool {
    mode == "gradient-sweep" || mode == "shimmer"
}

fn motion_classes(
    mode_class_name: &str,
    animating: bool,
    paused: bool,
    looped: bool,
    extra: &str,
) -> String {
    join_class([
        "d-motion-text",
        mode_class_name,
        if animating {
            "d-motion-text--animating"
        } else {
            ""
        },
        if paused { "d-motion-text--paused" } else { "" },
        if looped { "d-motion-text--looped" } else { "" },
        extra,
    ])
}

fn word_enter_class(mode: &str) -> &'static str {
    match mode {
        "gradient-in" => "d-motion-text-word-gradient-in-enter-active",
        "fade-in" => "d-motion-text-word-fade-in-enter-active",
        "slide-in" => "d-motion-text-word-slide-in-enter-active",
        _ => "",
    }
}

fn char_enter_class(mode: &str) -> &'static str {
    match mode {
        "gradient-in" => "d-motion-text-char-gradient-in-enter-active",
        "fade-in" => "d-motion-text-char-fade-in-enter-active",
        "slide-in" => "d-motion-text-char-slide-in-enter-active",
        _ => "",
    }
}

fn wait_ms(wait: MotionWait, timing: MotionTiming) -> u64 {
    match wait {
        MotionWait::Word => timing.word_delay,
        MotionWait::Char(_) => timing.character_delay,
        MotionWait::Finish => 0,
        MotionWait::Restart => MOTION_TEXT_LOOP_GAP_MS,
    }
}

fn prefers_reduced_motion() -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };
    match window.match_media("(prefers-reduced-motion: reduce)") {
        Ok(Some(query)) => query.matches(),
        _ => false,
    }
}

fn clear_timer(timer: StoredValue<Option<TimeoutHandle>>) {
    let _ = timer.try_update_value(|slot| {
        if let Some(handle) = slot.take() {
            handle.clear();
        }
    });
}

fn publish(
    playback: StoredValue<MotionPlayback>,
    frame: RwSignal<MotionFrame>,
    generation: RwSignal<u32>,
) {
    if let Some(snapshot) = playback.try_with_value(|playback| playback.frame()) {
        let key = snapshot.key;
        frame.set(snapshot);
        if generation.get_untracked() != key {
            generation.set(key);
        }
    }
}

/// Park or restart the root CSS animation.
///
/// Gradient sweep and shimmer run once on the root. Clearing `animation-name`
/// and restoring it after a reflow starts that animation at the first frame.
/// Leaving it cleared holds the first frame.
fn rewind_motion(root: NodeRef<html::Span>, park: bool) {
    let Some(node) = root.get_untracked() else {
        return;
    };
    let element: &web_sys::HtmlElement = node.unchecked_ref();
    let style = web_sys::HtmlElement::style(element);
    if park {
        let _ = style.set_property("animation-name", "none");
        let _ = style.set_property("-webkit-animation-name", "none");
        let _ = style.set_property("--d-motion-text-mask-pos", "100%");
        let _ = web_sys::HtmlElement::offset_height(element);
    } else {
        let _ = web_sys::HtmlElement::offset_height(element);
        let _ = style.remove_property("animation-name");
        let _ = style.remove_property("-webkit-animation-name");
        let _ = style.remove_property("--d-motion-text-mask-pos");
    }
}

fn emit(listeners: &Listeners, notes: &[MotionNotice]) {
    for note in notes {
        match note {
            MotionNotice::Start => {
                if let Some(callback) = &listeners.start {
                    callback.run(());
                }
            }
            MotionNotice::Pause => {
                if let Some(callback) = &listeners.pause {
                    callback.run(());
                }
            }
            MotionNotice::Resume => {
                if let Some(callback) = &listeners.resume {
                    callback.run(());
                }
            }
            MotionNotice::Complete => {
                if let Some(callback) = &listeners.complete {
                    callback.run(());
                }
            }
            MotionNotice::Progress {
                words_complete,
                total_words,
                progress,
            } => {
                if let Some(callback) = &listeners.progress {
                    callback.run(DtMotionTextProgress {
                        words_complete: *words_complete,
                        total_words: *total_words,
                        progress: *progress,
                    });
                }
            }
        }
    }
}

#[derive(Clone)]
struct Listeners {
    start: Option<Callback<()>>,
    pause: Option<Callback<()>>,
    resume: Option<Callback<()>>,
    complete: Option<Callback<()>>,
    progress: Option<Callback<DtMotionTextProgress>>,
}

/// Dialtone Vue `DtMotionText` at `dialtone/v10.7.0`.
///
/// `text` is the animated copy. An empty `text` uses the default slot. The
/// slot is read after its children mount, then revealed like `text`.
/// `animation_mode` is `animationMode`. `speed` is `100` to `500`.
/// `auto_start` is `autoStart`. `loop` restarts after a natural complete.
/// `reset` returns the reveal to the first frame. It stays there unless `loop`
/// is set, which starts again from that frame. `respects_reduced_motion` is
/// `respectsReducedMotion`. `screen_reader_text` is `screenReaderText`.
/// `on_start` is `start`, `on_pause` is `pause`, `on_resume` is `resume`,
/// `on_complete` is `complete`, and `on_progress` is `progress`.
/// [`DtMotionTextHandle`] is the template ref.
///
/// `gradient-in`, `fade-in`, and `slide-in` insert one span per character.
/// `loop` on those modes repeats that insert for the life of the component
/// and lays the line out on every tick. Leave them unlooped. `gradient-sweep`
/// and `shimmer` animate a CSS mask, so they can stay looped without layout.
#[component]
pub fn DtMotionText(
    /// Animated copy. Empty uses the default slot.
    #[prop(optional, into)]
    text: Signal<String>,
    /// `animationMode`. Default `gradient-in`.
    #[prop(optional, default = "gradient-in")]
    animation_mode: &'static str,
    /// `speed`. Default `300`.
    #[prop(optional, default = "300")]
    speed: &'static str,
    /// `autoStart`. Default true.
    #[prop(optional, default = true)]
    auto_start: bool,
    /// `loop`. Default false. On a character mode this repeats a DOM insert
    /// per tick. Use `shimmer` or `gradient-sweep` for a continuous loop.
    #[prop(optional, default = false)]
    r#loop: bool,
    /// `respectsReducedMotion`. Default true.
    #[prop(optional, default = true)]
    respects_reduced_motion: bool,
    /// `screenReaderText`. Empty omits the screen-reader span and `aria-label`.
    #[prop(optional, into)]
    screen_reader_text: String,
    /// Extra classes on the root. Vue merges `class` onto the root.
    #[prop(optional, into)]
    class: String,
    /// Template ref for `start`, `pause`, `resume`, `reset`, and `skipToEnd`.
    #[prop(optional)]
    handle: Option<DtMotionTextHandle>,
    /// `start`.
    #[prop(optional, into)]
    on_start: Option<Callback<()>>,
    /// `pause`.
    #[prop(optional, into)]
    on_pause: Option<Callback<()>>,
    /// `resume`.
    #[prop(optional, into)]
    on_resume: Option<Callback<()>>,
    /// `complete`.
    #[prop(optional, into)]
    on_complete: Option<Callback<()>>,
    /// `progress`.
    #[prop(optional, into)]
    on_progress: Option<Callback<DtMotionTextProgress>>,
    /// Default slot. Used when `text` is empty.
    #[prop(optional)]
    children: Option<ChildrenFn>,
) -> impl IntoView {
    let mode = resolve_mode(animation_mode);
    let mode_class_name = mode_class(mode);
    let timing = motion_timing(speed);
    let style = motion_style(timing);
    let reduced = respects_reduced_motion && prefers_reduced_motion();
    let static_mode = is_static(mode);
    let playback = StoredValue::new(MotionPlayback::new(r#loop));
    playback.update_value(|playback| {
        playback.mode = mode;
        playback.reduced = reduced;
    });
    let frame = RwSignal::new(MotionFrame::default());
    let generation = RwSignal::new(0u32);
    let timer: StoredValue<Option<TimeoutHandle>> = StoredValue::new(None);
    let schedule: StoredValue<Option<Callback<MotionWait>>> = StoredValue::new(None);
    let listeners = StoredValue::new(Listeners {
        start: on_start,
        pause: on_pause,
        resume: on_resume,
        complete: on_complete,
        progress: on_progress,
    });
    let root_ref = NodeRef::<html::Span>::new();
    let fallback_ref = NodeRef::<html::Span>::new();
    let loaded = StoredValue::new(Load::Fresh);
    let slot_ticket = StoredValue::new(0u32);
    let schedule_slot = schedule;
    let tick = Callback::new(move |wait: MotionWait| {
        clear_timer(timer);
        let delay = wait_ms(wait, timing);
        match set_timeout_with_handle(
            move || {
                let _ = timer.try_update_value(|slot| *slot = None);
                let notes = playback
                    .try_update_value(|playback| playback.on_timer())
                    .unwrap_or_default();
                publish(playback, frame, generation);
                if let Some(listeners) = listeners.try_get_value() {
                    emit(&listeners, &notes);
                }
                if let Some(next) = playback.try_with_value(|playback| playback.wait).flatten() {
                    if let Some(again) = schedule_slot.try_get_value().flatten() {
                        again.run(next);
                    }
                }
            },
            Duration::from_millis(delay),
        ) {
            Ok(handle) => {
                let _ = timer.try_set_value(Some(handle));
            }
            Err(_) => {}
        }
    });
    schedule.set_value(Some(tick));

    let run = move |action: Action| {
        let was_playing = playback
            .try_with_value(|playback| playback.animating)
            .unwrap_or(false);
        let rewind = static_mode
            && (matches!(action, Action::Reset)
                || (matches!(action, Action::Start) && !was_playing));
        if rewind {
            rewind_motion(root_ref, true);
        }
        clear_timer(timer);
        let notes = playback
            .try_update_value(|playback| playback.apply(action))
            .unwrap_or_default();
        publish(playback, frame, generation);
        if let Some(listeners) = listeners.try_get_value() {
            emit(&listeners, &notes);
        }
        if rewind && (matches!(action, Action::Start) || r#loop) {
            rewind_motion(root_ref, false);
        }
        if let Some(wait) = playback.try_with_value(|playback| playback.wait).flatten() {
            if let Some(schedule) = schedule.get_value() {
                schedule.run(wait);
            }
        }
    };

    if static_mode && !auto_start && !r#loop {
        let did_park = StoredValue::new(false);
        Effect::new(move |_| {
            if root_ref.get().is_none() || did_park.get_value() {
                return;
            }
            did_park.set_value(true);
            rewind_motion(root_ref, true);
        });
    }

    let token = NEXT_MOTION_TEXT.fetch_add(1, Ordering::Relaxed);
    if let Some(handle) = handle {
        handle.ops.set_value(Some(MotionOps {
            token,
            start: Callback::new(move |_| run(Action::Start)),
            pause: Callback::new(move |_| run(Action::Pause)),
            resume: Callback::new(move |_| run(Action::Resume)),
            reset: Callback::new(move |_| run(Action::Reset)),
            skip_to_end: Callback::new(move |_| run(Action::Skip)),
        }));
        on_cleanup(move || {
            let _ = handle.ops.try_update_value(|slot| {
                if slot.as_ref().is_some_and(|ops| ops.token == token) {
                    *slot = None;
                }
            });
        });
    }
    on_cleanup(move || {
        clear_timer(timer);
        let _ = slot_ticket.try_update_value(|ticket| *ticket = ticket.wrapping_add(1));
    });

    let adopt_slot = Callback::new(move |_: ()| {
        if static_mode || !text.get_untracked().is_empty() {
            return;
        }
        let Some(node) = fallback_ref.get_untracked() else {
            return;
        };
        let words = motion_words(&node.text_content().unwrap_or_default());
        let same = matches!(loaded.get_value(), Load::Slot)
            && playback
                .try_with_value(|playback| playback.words == words)
                .unwrap_or(false);
        if same {
            return;
        }
        let start = auto_start && !words.is_empty();
        loaded.set_value(Load::Slot);
        playback.update_value(|playback| playback.pending_words = words);
        run(Action::Replace);
        if start {
            run(Action::Start);
        }
    });

    Effect::new(move |_| {
        let value = text.get();
        // Resubscribe when the fallback mounts. Its children are not in the
        // node yet, so a non-empty read can adopt immediately and an empty
        // read waits one turn.
        let _attached = fallback_ref.get().is_some();
        if static_mode || !value.is_empty() {
            let _ = slot_ticket.try_update_value(|ticket| {
                *ticket = ticket.wrapping_add(1);
            });
            let state = loaded.get_value();
            if matches!(state, Load::Words(ref previous) if previous == &value) {
                return;
            }
            let first = matches!(state, Load::Fresh);
            loaded.set_value(Load::Words(value.clone()));
            let words = if static_mode {
                Vec::new()
            } else {
                motion_words(&value)
            };
            if first {
                playback.update_value(|playback| playback.set_words(words.clone()));
                publish(playback, frame, generation);
            } else {
                playback.update_value(|playback| playback.pending_words = words.clone());
                run(Action::Replace);
            }
            if auto_start && (static_mode || !words.is_empty()) {
                run(Action::Start);
            }
            return;
        }
        if matches!(loaded.get_value(), Load::Fresh | Load::Words(_)) {
            loaded.set_value(Load::AwaitingSlot);
            playback.update_value(|playback| playback.pending_words.clear());
            run(Action::Replace);
        }
        let ready = fallback_ref
            .get_untracked()
            .and_then(|node| node.text_content())
            .is_some_and(|raw| raw.chars().any(|ch| !ch.is_whitespace()));
        if ready {
            adopt_slot.run(());
            return;
        }
        let ticket = slot_ticket
            .try_update_value(|ticket| {
                *ticket = ticket.wrapping_add(1);
                *ticket
            })
            .unwrap_or(0);
        let adopt_later = adopt_slot;
        let _ = set_timeout_with_handle(
            move || {
                if slot_ticket.try_get_value() != Some(ticket) {
                    return;
                }
                adopt_later.run(());
            },
            Duration::ZERO,
        );
    });

    let screen_reader = (!screen_reader_text.is_empty()).then_some(screen_reader_text.clone());
    let word_class = join_class(["d-motion-text__word", word_enter_class(mode)]);
    let char_class = join_class(["d-motion-text__char", char_enter_class(mode)]);
    let character_delay = timing.character_delay;
    let body = if static_mode {
        let shown = text;
        view! {
            {move || {
                let value = shown.get();
                if value.is_empty() {
                    match children.as_ref() {
                        Some(slot) => slot().into_any(),
                        None => ().into_any(),
                    }
                } else {
                    value.into_any()
                }
            }}
        }
        .into_any()
    } else {
        let word_class_for_body = word_class.clone();
        let char_class_for_body = char_class.clone();
        view! {
            {move || {
                let _generation = generation.get();
                let word_class = word_class_for_body.clone();
                let char_class = char_class_for_body.clone();
                view! {
                    <span
                        class="d-motion-text__content"
                        aria-hidden=move || frame.get().animating.then_some("true")
                    >
                        <For
                            each=move || {
                                let snapshot = frame.get();
                                (0..snapshot.visible_words)
                                    .map(|index| (snapshot.key, index))
                                    .collect::<Vec<_>>()
                            }
                            key=|item| *item
                            let:item
                        >
                            <MotionWord
                                index=item.1
                                frame=frame
                                word_class=word_class.clone()
                                char_class=char_class.clone()
                                character_delay=character_delay
                            />
                        </For>
                    </span>
                }
            }}
            <Show when=move || {
                let snapshot = frame.get();
                text.get().is_empty() && snapshot.words.is_empty()
            }>
                <span class="d-motion-text__fallback" node_ref=fallback_ref>
                    {children.as_ref().map(|slot| slot())}
                </span>
            </Show>
        }
        .into_any()
    };

    view! {
        <span
            class=move || {
                let snapshot = frame.get();
                motion_classes(
                    mode_class_name,
                    snapshot.animating,
                    snapshot.paused,
                    r#loop,
                    &class,
                )
            }
            style=style
            aria-live=move || if frame.get().animating { "polite" } else { "off" }
            aria-label=screen_reader.clone()
            data-text-content=move || static_mode.then(|| text.get())
            node_ref=root_ref
        >
            {
                screen_reader.clone().map(|label| view! {
                    <span class="d-motion-text__sr-only">{label}</span>
                })
            }
            {body}
        </span>
    }
}

#[component]
fn MotionWord(
    index: usize,
    frame: RwSignal<MotionFrame>,
    word_class: String,
    char_class: String,
    character_delay: u64,
) -> impl IntoView {
    let word_style = format!("--word-index: {index};");
    let label = frame
        .get_untracked()
        .words
        .get(index)
        .map(|word| word.text.clone())
        .unwrap_or_default();
    view! {
        <span class=word_class style=word_style data-text-content=label>
            <For
                each=move || {
                    let snapshot = frame.get();
                    let shown = snapshot.visible_chars.get(index).copied().unwrap_or(0);
                    (0..shown)
                        .map(|char_index| (snapshot.key, char_index))
                        .collect::<Vec<_>>()
                }
                key=|item| *item
                let:item
            >
                <MotionChar
                    word_index=index
                    char_index=item.1
                    frame=frame
                    char_class=char_class.clone()
                    character_delay=character_delay
                />
            </For>
        </span>
    }
}

#[component]
fn MotionChar(
    word_index: usize,
    char_index: usize,
    frame: RwSignal<MotionFrame>,
    char_class: String,
    character_delay: u64,
) -> impl IntoView {
    let label = frame
        .get_untracked()
        .words
        .get(word_index)
        .and_then(|word| word.chars.get(char_index).copied())
        .map(|ch| ch.to_string())
        .unwrap_or_default();
    let style = format!(
        "--char-index: {char_index}; --char-delay: {}ms;",
        char_index as u64 * character_delay
    );
    view! { <span class=char_class style=style>{label}</span> }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn texts(words: &[MotionWord]) -> Vec<&str> {
        words.iter().map(|word| word.text.as_str()).collect()
    }

    fn names(notes: &[MotionNotice]) -> Vec<&'static str> {
        notes
            .iter()
            .map(|note| match note {
                MotionNotice::Start => "start",
                MotionNotice::Pause => "pause",
                MotionNotice::Resume => "resume",
                MotionNotice::Complete => "complete",
                MotionNotice::Progress { .. } => "progress",
            })
            .collect()
    }

    fn playback(text: &str, looped: bool) -> MotionPlayback {
        let mut playback = MotionPlayback::new(looped);
        playback.mode = "gradient-in";
        playback.set_words(motion_words(text));
        playback
    }

    #[test]
    fn words_keep_the_trailing_space() {
        let words = motion_words("Test text content");
        assert_eq!(texts(&words), ["Test ", "text ", "content"]);
        assert_eq!(words[0].chars.len(), 5);
        assert!(motion_words("   ").is_empty());
        assert_eq!(texts(&motion_words("  hello  world")), ["hello  ", "world"]);
    }

    #[test]
    fn timing_and_style_follow_the_presets() {
        let fast = motion_timing("200");
        assert_eq!(fast.character_delay, 20);
        assert_eq!(fast.word_delay, 30);
        assert_eq!(fast.duration, 600);
        let medium = motion_timing("300");
        assert_eq!(medium.character_delay, 30);
        assert_eq!(medium.word_delay, 50);
        assert_eq!(medium.duration, 1000);
        let slow = motion_timing("400");
        assert_eq!(slow.character_delay, 50);
        assert_eq!(slow.word_delay, 100);
        assert_eq!(slow.duration, 1500);
        assert_eq!(motion_timing("nope"), medium);
        let style = motion_style(medium);
        assert!(style.contains("--d-motion-text-duration: 1000ms"));
        assert!(style.contains("--d-motion-text-char-duration: 1000ms"));
        assert!(style.contains("--d-motion-text-word-duration: 2000ms"));
        assert!(motion_style(fast).contains("--d-motion-text-word-duration: 1200ms"));
    }

    #[test]
    fn classes_follow_the_vue_array() {
        assert_eq!(
            motion_classes("d-motion-text--gradient-in", false, false, false, ""),
            "d-motion-text d-motion-text--gradient-in"
        );
        assert_eq!(
            motion_classes(
                "d-motion-text--fade-in",
                true,
                true,
                true,
                "d-headline--lg"
            ),
            "d-motion-text d-motion-text--fade-in d-motion-text--animating d-motion-text--paused d-motion-text--looped d-headline--lg"
        );
        assert_eq!(mode_class("shimmer"), "d-motion-text--shimmer");
        assert_eq!(mode_class("none"), "d-motion-text--none");
        assert_eq!(mode_class("nope"), "");
        assert_eq!(resolve_mode("slide-in"), "slide-in");
        assert_eq!(resolve_mode("nope"), "gradient-in");
        assert!(is_static("gradient-sweep"));
        assert!(is_static("shimmer"));
        assert!(!is_static("gradient-in"));
        assert_eq!(
            word_enter_class("gradient-in"),
            "d-motion-text-word-gradient-in-enter-active"
        );
        assert_eq!(
            char_enter_class("fade-in"),
            "d-motion-text-char-fade-in-enter-active"
        );
        assert_eq!(char_enter_class("none"), "");
    }

    #[test]
    fn a_word_reveals_one_character_then_waits() {
        let mut playback = playback("Hi", false);
        assert_eq!(names(&playback.start()), ["start"]);
        assert_eq!(playback.wait, Some(MotionWait::Word));
        let notes = playback.on_timer();
        assert_eq!(names(&notes), ["progress"]);
        assert_eq!(playback.visible_words, 1);
        assert_eq!(playback.visible_chars, vec![1]);
        assert_eq!(playback.wait, Some(MotionWait::Char(0)));
        assert!(names(&playback.on_timer()).is_empty());
        assert_eq!(playback.visible_chars, vec![2]);
        assert_eq!(names(&playback.on_timer()), ["complete"]);
        assert!(!playback.animating);
        assert!(playback.wait.is_none());
    }

    #[test]
    fn pause_then_resume_schedules_the_next_word() {
        let mut playback = playback("Ab Cd", false);
        playback.start();
        playback.on_timer();
        assert_eq!(playback.visible_chars[0], 1);
        assert_eq!(names(&playback.pause()), ["pause"]);
        assert!(playback.paused);
        assert!(playback.wait.is_none());
        assert!(names(&playback.pause()).is_empty());
        assert_eq!(names(&playback.resume()), ["resume"]);
        assert_eq!(playback.wait, Some(MotionWait::Word));
        assert_eq!(playback.visible_chars[0], 1);
        assert!(names(&playback.start()).is_empty());
    }

    #[test]
    fn reduced_motion_and_skip_finish_without_looping() {
        let mut reduced = playback("Hi", true);
        reduced.reduced = true;
        assert_eq!(names(&reduced.start()), ["start"]);
        assert_eq!(reduced.visible_words, 1);
        assert_eq!(reduced.visible_chars, vec![2]);
        assert_eq!(reduced.wait, Some(MotionWait::Finish));
        assert_eq!(names(&reduced.on_timer()), ["complete"]);
        assert!(reduced.wait.is_none());

        let mut skipped = playback("Hi", true);
        skipped.start();
        assert!(names(&skipped.skip()).is_empty());
        assert_eq!(skipped.visible_chars, vec![2]);
        assert_eq!(skipped.wait, Some(MotionWait::Finish));
        assert_eq!(names(&skipped.on_timer()), ["complete"]);
        assert!(skipped.wait.is_none());
    }

    #[test]
    fn a_natural_complete_restarts_when_looped() {
        let mut playback = playback("Z", true);
        playback.start();
        playback.on_timer();
        assert_eq!(names(&playback.on_timer()), ["complete"]);
        assert_eq!(playback.wait, Some(MotionWait::Restart));
        let notes = playback.on_timer();
        assert_eq!(names(&notes), ["start"]);
        assert_eq!(playback.visible_words, 0);
        assert_eq!(playback.wait, Some(MotionWait::Word));
        assert!(playback.animating);
    }

    #[test]
    fn static_and_none_and_empty_follow_vue() {
        let mut shimmer = MotionPlayback::new(true);
        shimmer.mode = "shimmer";
        assert_eq!(names(&shimmer.start()), ["start"]);
        assert!(shimmer.animating);
        assert!(shimmer.wait.is_none());
        assert!(shimmer.words.is_empty());

        let mut none = playback("Hi", true);
        none.mode = "none";
        assert_eq!(names(&none.start()), ["start"]);
        assert_eq!(none.visible_chars, vec![2]);
        assert_eq!(none.wait, Some(MotionWait::Finish));

        let mut empty = MotionPlayback::new(true);
        empty.mode = "fade-in";
        assert_eq!(names(&empty.start()), ["start", "complete"]);
        assert_eq!(empty.wait, Some(MotionWait::Restart));
    }

    #[test]
    fn reset_stops_at_the_start_unless_looped() {
        let mut once = playback("Hi", false);
        once.start();
        once.on_timer();
        assert_eq!(once.visible_chars, vec![1]);
        let notes = once.apply(Action::Reset);
        assert!(notes.is_empty());
        assert!(!once.animating);
        assert!(!once.paused);
        assert_eq!(once.visible_words, 0);
        assert_eq!(once.visible_chars, vec![0]);
        assert!(once.wait.is_none());
        assert_eq!(once.key, 1);
        assert!(once.on_timer().is_empty());
        assert!(!once.animating);

        let mut looped = playback("Hi", true);
        looped.start();
        looped.on_timer();
        let notes = looped.apply(Action::Reset);
        assert_eq!(names(&notes), ["start"]);
        assert!(looped.animating);
        assert_eq!(looped.visible_words, 0);
        assert_eq!(looped.visible_chars, vec![0]);
        assert_eq!(looped.wait, Some(MotionWait::Word));
        assert_eq!(looped.key, 1);

        let mut shimmer = MotionPlayback::new(false);
        shimmer.mode = "shimmer";
        shimmer.start();
        assert!(names(&shimmer.apply(Action::Reset)).is_empty());
        assert!(!shimmer.animating);
        assert!(shimmer.wait.is_none());

        let mut looped_shimmer = MotionPlayback::new(true);
        looped_shimmer.mode = "shimmer";
        looped_shimmer.start();
        assert_eq!(names(&looped_shimmer.apply(Action::Reset)), ["start"]);
        assert!(looped_shimmer.animating);
        assert!(looped_shimmer.wait.is_none());
    }
}
