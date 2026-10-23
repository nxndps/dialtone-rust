use super::*;

const TRANSITIONS: &[(&str, &str)] = &[
    ("", "None"),
    ("fade", "Fade"),
    ("slide-down", "Slide down"),
    ("pop", "Pop"),
    ("shake", "Shake"),
];

#[component]
pub(crate) fn LazyShowPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("lazy_show.rs")));
    let shown = RwSignal::new(false);
    let transition = RwSignal::new("");
    let appear = RwSignal::new(false);
    let slot_text = RwSignal::new("I'm Lazy!".to_string());
    let root_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let transition_name = transition.get().to_string();
                let appear_now = appear.get();
                let class_name = root_class.get();
                let label = slot_text.get();
                view! {
                    <DtButton on_click=move || shown.set(!shown.get_untracked())>
                        "Toggle"
                    </DtButton>
                    <DtLazyShow
                        show=shown
                        transition=transition_name
                        appear=appear_now
                        class=class_name
                    >
                        {label.clone()}
                    </DtLazyShow>
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingCheck label="Show" value=shown />
            <SettingSelect label="Transition" value=transition options=TRANSITIONS.to_vec() />
            <SettingCheck label="Appear" value=appear />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_expr(&mut props, "show", "shown");
        push_str(&mut props, "transition", transition.get());
        push_bool(&mut props, "appear", appear.get());
        push_str(&mut props, "class", &root_class.get());
        let child = slot_text.get();
        let child = (!child.is_empty()).then(|| rust_string(&child));
        view_of(&element("DtLazyShow", &props, child.as_deref()))
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingText
                label="Default"
                hint="Slot for main content. It stays unmounted until show is true the first time."
                value=slot_text
            />
        }
    });
    let classes = class_panel(vec![ClassField {
        label: "Class",
        hint: ROOT_CLASS,
        value: root_class,
        off: None,
    }]);
    let shown_once = Signal::derive(|| true);
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">
                "The default slot is the main content. It stays unmounted until show is true the first time. After that it stays mounted and hides with display none. fade, slide-down, pop, and shake use the Dialtone transition classes. Appear runs the enter transition only when show starts true."
            </p>
            <Play preview=preview settings=settings slots=slots classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Shown">
                    <DtLazyShow show=shown_once>"I'm Lazy!"</DtLazyShow>
                </Variant>
            </div>
        </section>
    }
}
