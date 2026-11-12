use super::*;

#[component]
pub(crate) fn MotionTextPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("motion_text.rs")));
    let text = RwSignal::new("Welcome to Dialtone Motion Text".to_string());
    let animation_mode = RwSignal::new("gradient-in");
    let speed = RwSignal::new("300");
    let auto_start = RwSignal::new(true);
    let looped = RwSignal::new(false);
    let respects = RwSignal::new(true);
    let screen_reader = RwSignal::new(String::new());
    let slot_text = RwSignal::new(String::new());
    let root_class = RwSignal::new("d-headline--lg".to_string());
    let motion = DtMotionTextHandle::new();
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let copy = text.get();
                let slotted = slot_text.get();
                let mode = animation_mode.get();
                let rate = speed.get();
                let start = auto_start.get();
                let repeat = looped.get();
                let reduced = respects.get();
                let reader = screen_reader.get();
                let class_name = root_class.get();
                view! {
                    <DtStack gap="100">
                        <DtStack direction="row" gap="100">
                            <DtButton size="xs" importance="outlined" kind="muted" on_click=move |_| motion.start()>"Start"</DtButton>
                            <DtButton size="xs" importance="outlined" kind="muted" on_click=move |_| motion.pause()>"Pause"</DtButton>
                            <DtButton size="xs" importance="outlined" kind="muted" on_click=move |_| motion.resume()>"Resume"</DtButton>
                            <DtButton size="xs" importance="outlined" kind="muted" on_click=move |_| motion.reset()>"Reset"</DtButton>
                            <DtButton size="xs" importance="outlined" kind="muted" on_click=move |_| motion.skip_to_end()>"Skip to end"</DtButton>
                        </DtStack>
                        <DtMotionText
                            handle=motion
                            text=copy
                            animation_mode=mode
                            speed=rate
                            auto_start=start
                            r#loop=repeat
                            respects_reduced_motion=reduced
                            screen_reader_text=reader
                            class=class_name
                        >
                            {slotted.clone()}
                        </DtMotionText>
                    </DtStack>
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingText label="Text" value=text />
            <SettingSelect
                label="Animation mode"
                value=animation_mode
                options=vec![
                    ("gradient-in", "Gradient in"),
                    ("fade-in", "Fade in"),
                    ("slide-in", "Slide in"),
                    ("gradient-sweep", "Gradient sweep"),
                    ("shimmer", "Shimmer"),
                    ("none", "None"),
                ]
            />
            <SettingSelect
                label="Speed"
                value=speed
                options=vec![
                    ("100", "100 Near-instant"),
                    ("200", "200 Fast"),
                    ("300", "300 Medium"),
                    ("400", "400 Slow"),
                    ("500", "500 Very slow"),
                ]
            />
            <SettingCheck label="Auto start" value=auto_start />
            <SettingCheck label="Loop" value=looped />
            <SettingCheck label="Respects reduced motion" value=respects />
            <SettingText label="Screen reader text" value=screen_reader />
        }
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingText
                label="Default"
                hint="Used when Text is empty. The words are revealed the same way as the text prop."
                value=slot_text
            />
        }
    });
    let code = example_panel(move || {
        let mut lines = vec!["let motion = DtMotionTextHandle::new();".to_string()];
        lines.push("<DtButton on_click=move |_| motion.start()>\"Start\"</DtButton>".to_string());
        let mut props = Vec::new();
        push_expr(&mut props, "handle", "motion");
        let copy = text.get();
        push_str(&mut props, "text", &copy);
        let child = if copy.is_empty() {
            let slotted = slot_text.get();
            if slotted.is_empty() {
                None
            } else {
                Some(rust_string(&slotted))
            }
        } else {
            None
        };
        let mode = animation_mode.get();
        if mode != "gradient-in" {
            push_str(&mut props, "animation_mode", mode);
        }
        let rate = speed.get();
        if rate != "300" {
            push_str(&mut props, "speed", rate);
        }
        if !auto_start.get() {
            push_flag(&mut props, "auto_start", false);
        }
        if looped.get() {
            push_bool(&mut props, "r#loop", true);
        }
        if !respects.get() {
            push_flag(&mut props, "respects_reduced_motion", false);
        }
        push_str(&mut props, "screen_reader_text", &screen_reader.get());
        push_str(&mut props, "class", &root_class.get());
        lines.push(view_of(&element("DtMotionText", &props, child.as_deref())));
        lines.join("\n")
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"Gradient in, fade in, and slide in insert a character span on each tick. Leave them unlooped. A loop on those modes lays the line out for as long as the component is mounted. Gradient sweep and shimmer animate a CSS mask and can loop without layout. Reset returns the animation to the start and leaves it there. The default slot is used when Text is empty. slide-in-gradient is in the vendored CSS and is not a mode here."</p>
            <Play
                preview=preview
                settings=settings
                slots=slots
                classes=class_panel(vec![ClassField {
                    label: "Class",
                    hint: ROOT_CLASS,
                    value: root_class,
                    off: None,
                }])
                code=code
            />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Modes">
                    <DtStack gap="200">
                        <ModeLine mode="gradient-in" text="Gradient in" />
                        <ModeLine mode="fade-in" text="Fade in" />
                        <ModeLine mode="slide-in" text="Slide in" />
                        <ModeLine mode="gradient-sweep" text="Gradient sweep" looped=true />
                        <ModeLine mode="shimmer" text="Shimmer" looped=true />
                        <ModeLine mode="none" text="No animation" />
                    </DtStack>
                </Variant>
                <Variant title="Loop">
                    <DtMotionText
                        text="This text loops continuously".to_string()
                        animation_mode="shimmer"
                        r#loop=true
                        class="d-headline--md"
                    />
                </Variant>
                <Variant title="Slot">
                    <DtMotionText animation_mode="fade-in" class="d-headline--md">
                        <span>"Animated "</span>
                        <strong>"text"</strong>
                    </DtMotionText>
                </Variant>
            </div>
        </section>
    }
}

#[component]
fn ModeLine(
    mode: &'static str,
    text: &'static str,
    #[prop(optional)] looped: bool,
) -> impl IntoView {
    view! {
        <DtStack gap="50">
            <p class="d-body--sm d-fc-tertiary">{mode}</p>
            <DtMotionText
                text=text.to_string()
                animation_mode=mode
                r#loop=looped
                class="d-headline--md"
            />
        </DtStack>
    }
}
