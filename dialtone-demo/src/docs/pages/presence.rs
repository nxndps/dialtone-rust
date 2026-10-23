use super::*;

/// Component page for `DtPresence` at `dialtone/v10.7.0`.
#[component]
pub(crate) fn PresencePage() -> impl IntoView {
    provide_context(VariantSource(include_str!("presence.rs")));
    let presence = RwSignal::new("active");
    let show_icon = RwSignal::new(true);
    let sr_text = RwSignal::new(String::new());
    let aria_live = RwSignal::new("off");
    let root_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let presence = presence.get();
                let show_icon = show_icon.get();
                let sr_text = sr_text.get();
                let aria_live = aria_live.get();
                let class_name = root_class.get();
                view! {
                    <DtPresence
                        presence=presence
                        show_icon=show_icon
                        sr_text=sr_text
                        aria_live=aria_live
                        class=class_name
                    />
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingSelect
                label="Presence"
                value=presence
                options=vec![
                    ("active", "Active"),
                    ("away", "Away"),
                    ("busy", "Busy"),
                    ("offline", "Offline"),
                    ("dnd", "Do not disturb"),
                ]
            />
            <SettingCheck label="Show icon" value=show_icon />
            <SettingText
                label="Screen reader text"
                hint="srText. Empty hides the span."
                value=sr_text
            />
            <SettingSelect
                label="Aria live"
                value=aria_live
                options=vec![("off", "Off"), ("polite", "Polite"), ("assertive", "Assertive")]
            />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        push_str(&mut props, "presence", presence.get());
        if !show_icon.get() {
            push_flag(&mut props, "show_icon", false);
        }
        push_str(&mut props, "sr_text", &sr_text.get());
        if aria_live.get() != "off" {
            push_str(&mut props, "aria_live", aria_live.get());
        }
        push_str(&mut props, "class", &root_class.get());
        view_of(&element("DtPresence", &props, None))
    });
    let classes = class_panel(vec![ClassField {
        label: "Class",
        hint: ROOT_CLASS,
        value: root_class,
        off: None,
    }]);
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">
                "Active draws a check. Busy and do not disturb draw a minus. Away draws an empty icon. Offline draws no icon."
            </p>
            <Play preview=preview settings=settings classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="States" row=true>
                    <DtPresence presence="active" />
                    <DtPresence presence="away" />
                    <DtPresence presence="busy" />
                    <DtPresence presence="dnd" />
                    <DtPresence presence="offline" />
                </Variant>
                <Variant title="Without icon" row=true>
                    <DtPresence presence="active" show_icon=false />
                    <DtPresence presence="away" show_icon=false />
                    <DtPresence presence="busy" show_icon=false />
                    <DtPresence presence="dnd" show_icon=false />
                    <DtPresence presence="offline" show_icon=false />
                </Variant>
            </div>
        </section>
    }
}
