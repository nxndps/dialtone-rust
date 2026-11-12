use super::*;

const ICON_NAMES: &[(&str, &str)] = &[
    ("close", "Close"),
    ("info", "Info"),
    ("bell", "Bell"),
    ("alert", "Alert"),
    ("chevron", "Down"),
    ("left", "Left"),
    ("right", "Right"),
    ("sparkle", "Sparkle"),
    ("plus", "Plus"),
    ("check", "Check"),
    ("user", "User"),
];

fn icon_caption(name: &str) -> &'static str {
    ICON_NAMES
        .iter()
        .find(|pair| pair.0 == name)
        .map(|pair| pair.1)
        .unwrap_or("Close")
}

fn named_icon(name: &str, size: &'static str, class_name: String) -> AnyView {
    if class_name.is_empty() {
        match name {
            "info" => view! { <DtIconInfo size=size /> }.into_any(),
            "bell" => view! { <DtIconBell size=size /> }.into_any(),
            "alert" => view! { <DtIconAlertCircle size=size /> }.into_any(),
            "chevron" => view! { <DtIconChevronDown size=size /> }.into_any(),
            "left" => view! { <DtIconChevronLeft size=size /> }.into_any(),
            "right" => view! { <DtIconChevronRight size=size /> }.into_any(),
            "sparkle" => view! { <DtIconSparkle size=size /> }.into_any(),
            "plus" => view! { <DtIconPlus size=size /> }.into_any(),
            "check" => view! { <DtIconCheck size=size /> }.into_any(),
            "user" => view! { <DtIconUser size=size /> }.into_any(),
            _ => view! { <DtIconClose size=size /> }.into_any(),
        }
    } else {
        match name {
            "info" => view! { <DtIconInfo size=size class=class_name /> }.into_any(),
            "bell" => view! { <DtIconBell size=size class=class_name /> }.into_any(),
            "alert" => view! { <DtIconAlertCircle size=size class=class_name /> }.into_any(),
            "chevron" => view! { <DtIconChevronDown size=size class=class_name /> }.into_any(),
            "left" => view! { <DtIconChevronLeft size=size class=class_name /> }.into_any(),
            "right" => view! { <DtIconChevronRight size=size class=class_name /> }.into_any(),
            "sparkle" => view! { <DtIconSparkle size=size class=class_name /> }.into_any(),
            "plus" => view! { <DtIconPlus size=size class=class_name /> }.into_any(),
            "check" => view! { <DtIconCheck size=size class=class_name /> }.into_any(),
            "user" => view! { <DtIconUser size=size class=class_name /> }.into_any(),
            _ => view! { <DtIconClose size=size class=class_name /> }.into_any(),
        }
    }
}

#[component]
pub(crate) fn IconPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("icon.rs")));
    let size = RwSignal::new("500");
    let name = RwSignal::new("close");
    let root_class = RwSignal::new(String::new());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let size = size.get();
                let name = name.get();
                let class_name = root_class.get();
                let caption = icon_caption(name);
                view! {
                    <div class="docs-icon-grid">
                        <div>
                            <p class="docs-note">"Sized"</p>
                            <div class="demo-icons">
                                <div class="demo-icon">
                                    {named_icon(name, size, class_name)}
                                    <span>{caption}</span>
                                </div>
                                <div class="demo-icon"><DtIconInfo size=size /><span>"Info"</span></div>
                                <div class="demo-icon"><DtIconBell size=size /><span>"Bell"</span></div>
                                <div class="demo-icon"><DtIconAlertCircle size=size /><span>"Alert"</span></div>
                                <div class="demo-icon"><DtIconChevronDown size=size /><span>"Down"</span></div>
                                <div class="demo-icon"><DtIconChevronLeft size=size /><span>"Left"</span></div>
                                <div class="demo-icon"><DtIconChevronRight size=size /><span>"Right"</span></div>
                            </div>
                        </div>
                        <div>
                            <p class="docs-note">"Default 500"</p>
                            <div class="demo-icons">
                                <div class="demo-icon"><DtIconPlus /><span>"Plus"</span></div>
                                <div class="demo-icon"><DtIconCheck /><span>"Check"</span></div>
                                <div class="demo-icon"><DtIconUser /><span>"User"</span></div>
                            </div>
                        </div>
                    </div>
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingSelect label="Name" value=name options=ICON_NAMES.to_vec() />
            <SettingSelect
                label="Size"
                value=size
                options=vec![
                    ("100", "100"),
                    ("200", "200"),
                    ("300", "300"),
                    ("400", "400"),
                    ("500", "500"),
                    ("600", "600"),
                    ("700", "700"),
                    ("800", "800"),
                ]
            />
        }
    });
    let code = example_panel(move || {
        let token = rust_string(size.get());
        let class_name = root_class.get();
        let chosen = name.get();
        if chosen == "close" && class_name.is_empty() {
            let sized = [
                "DtIconClose",
                "DtIconInfo",
                "DtIconBell",
                "DtIconAlertCircle",
                "DtIconChevronDown",
                "DtIconChevronLeft",
                "DtIconChevronRight",
            ];
            let mut lines: Vec<String> = sized
                .iter()
                .map(|component| element(component, &[("size", token.clone())], None))
                .collect();
            for component in ["DtIconPlus", "DtIconCheck", "DtIconUser"] {
                lines.push(element(component, &[], None));
            }
            return view_of(&lines.join("\n"));
        }
        let first = match chosen {
            "info" => "DtIconInfo",
            "bell" => "DtIconBell",
            "alert" => "DtIconAlertCircle",
            "chevron" => "DtIconChevronDown",
            "left" => "DtIconChevronLeft",
            "right" => "DtIconChevronRight",
            "sparkle" => "DtIconSparkle",
            "plus" => "DtIconPlus",
            "check" => "DtIconCheck",
            "user" => "DtIconUser",
            _ => "DtIconClose",
        };
        let first_line = if class_name.is_empty() {
            element(first, &[("size", token.clone())], None)
        } else {
            element(
                first,
                &[("size", token.clone()), ("class", rust_string(&class_name))],
                None,
            )
        };
        let mut lines = vec![first_line];
        for component in [
            "DtIconInfo",
            "DtIconBell",
            "DtIconAlertCircle",
            "DtIconChevronDown",
            "DtIconChevronLeft",
            "DtIconChevronRight",
            "DtIconSparkle",
        ] {
            if component != first {
                lines.push(element(component, &[("size", token.clone())], None));
            }
        }
        if first != "DtIconPlus" {
            lines.push(element("DtIconPlus", &[], None));
        }
        if first != "DtIconCheck" {
            lines.push(element("DtIconCheck", &[], None));
        }
        if first != "DtIconUser" {
            lines.push(element("DtIconUser", &[], None));
        }
        view_of(&lines.join("\n"))
    });
    let classes = class_panel(vec![ClassField {
        label: "Class",
        hint: "Extra class on the svg.",
        value: root_class,
        off: None,
    }]);
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">"The first row uses the size token. An omitted size is 500."</p>
            <Play stack=true preview=preview settings=settings classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="500" row=true>
                    <DtIconBell size="500" />
                    <DtIconInfo size="500" />
                    <DtIconChevronDown size="500" />
                </Variant>
                <Variant title="More sized icons">
                    <div class="demo-icons">
                        <div class="demo-icon"><DtIconSearch /><span>"Search"</span></div>
                        <div class="demo-icon"><DtIconLaptop2 /><span>"Laptop"</span></div>
                        <div class="demo-icon"><DtIconSun /><span>"Sun"</span></div>
                        <div class="demo-icon"><DtIconMoon /><span>"Moon"</span></div>
                        <div class="demo-icon"><DtIconChevronsLeft /><span>"Chevrons left"</span></div>
                        <div class="demo-icon"><DtIconChevronsRight /><span>"Chevrons right"</span></div>
                        <div class="demo-icon"><DtIconAlertTriangle /><span>"Warning"</span></div>
                        <div class="demo-icon"><DtIconCheckCircle /><span>"Success"</span></div>
                        <div class="demo-icon"><DtIconSparkle /><span>"Sparkle"</span></div>
                        <div class="demo-icon"><DtIconMoreHorizontal /><span>"More horizontal"</span></div>
                    </div>
                </Variant>
                <Variant title="Default 500">
                    <div class="demo-icons">
                        <div class="demo-icon"><DtIconRefresh /><span>"Refresh"</span></div>
                        <div class="demo-icon"><DtIconSave /><span>"Save"</span></div>
                        <div class="demo-icon"><DtIconMoreVertical /><span>"More"</span></div>
                        <div class="demo-icon"><DtIconArrowLeftRight /><span>"Swap"</span></div>
                        <div class="demo-icon"><DtIconConnectionApi /><span>"API"</span></div>
                        <div class="demo-icon"><DtIconUsers /><span>"Users"</span></div>
                        <div class="demo-icon"><DtIconTrash /><span>"Trash"</span></div>
                    </div>
                </Variant>
            </div>
        </section>
    }
}
