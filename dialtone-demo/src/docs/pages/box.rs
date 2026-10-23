use super::*;

const SURFACES: &[(&str, &str)] = &[
    ("", "None"),
    ("primary", "Primary"),
    ("secondary", "Secondary"),
    ("moderate", "Moderate"),
    ("bold", "Bold"),
    ("strong", "Strong"),
    ("contrast", "Contrast"),
    ("brand", "Brand"),
    ("info", "Info"),
    ("positive", "Positive"),
    ("warning", "Warning"),
    ("critical", "Critical"),
];

const PADDINGS: &[(&str, &str)] = &[
    ("", "None"),
    ("100", "100"),
    ("200", "200"),
    ("300", "300"),
    ("400", "400"),
    ("500", "500"),
];

const BORDER_WIDTHS: &[(&str, &str)] = &[
    ("", "None"),
    ("0", "0"),
    ("100", "100"),
    ("200", "200"),
    ("400", "400"),
];

const BORDER_COLORS: &[(&str, &str)] = &[
    ("default", "Default"),
    ("subtle", "Subtle"),
    ("moderate", "Moderate"),
    ("bold", "Bold"),
    ("brand", "Brand"),
    ("positive", "Positive"),
    ("critical", "Critical"),
    ("transparent", "Transparent"),
];

const RADII: &[(&str, &str)] = &[
    ("", "None"),
    ("0", "0"),
    ("200", "200"),
    ("300", "300"),
    ("400", "400"),
    ("pill", "Pill"),
    ("circle", "Circle"),
];

const SHADOWS: &[(&str, &str)] = &[
    ("", "None"),
    ("small", "Small"),
    ("medium", "Medium"),
    ("large", "Large"),
    ("extra-large", "Extra large"),
    ("card", "Card"),
];

const INLINE_SIZES: &[(&str, &str)] = &[
    ("", "Auto"),
    ("200", "200"),
    ("300", "300"),
    ("500", "500"),
    ("50p", "50%"),
    ("100p", "100%"),
];

const ELEMENTS: &[(&str, &str)] = &[
    ("div", "div"),
    ("section", "section"),
    ("article", "article"),
    ("nav", "nav"),
    ("aside", "aside"),
    ("span", "span"),
];

const SCROLLBARS: &[(&str, &str)] = &[
    ("", "Off"),
    ("leave", "Leave"),
    ("scroll", "Scroll"),
    ("move", "Move"),
    ("always", "Always"),
    ("true", "true (leave)"),
];

const SIDE_WIDTHS: &[(&str, &str)] = &[
    ("", "None"),
    ("0", "0"),
    ("50", "50"),
    ("100", "100"),
    ("150", "150"),
    ("200", "200"),
    ("300", "300"),
    ("400", "400"),
];

const SIDE_SPACING: &[(&str, &str)] = &[
    ("", "None"),
    ("0", "0"),
    ("1", "1"),
    ("25", "25"),
    ("50", "50"),
    ("75", "75"),
    ("100", "100"),
    ("125", "125"),
    ("150", "150"),
    ("175", "175"),
    ("200", "200"),
    ("250", "250"),
    ("300", "300"),
];

const SIDE_SIZES: &[(&str, &str)] = &[
    ("", "None"),
    ("0", "0"),
    ("1px", "1px"),
    ("2px", "2px"),
    ("8px", "8px"),
    ("25", "25"),
    ("20px", "20px"),
    ("24px", "24px"),
    ("50", "50"),
    ("75", "75"),
    ("100", "100"),
    ("125", "125"),
    ("150", "150"),
];

const POSITIONS: &[(&str, &str)] = &[
    ("", "None"),
    ("static", "Static"),
    ("relative", "Relative"),
    ("absolute", "Absolute"),
    ("fixed", "Fixed"),
    ("sticky", "Sticky"),
];

const Z_INDEXES: &[(&str, &str)] = &[
    ("", "None"),
    ("hide", "Hide"),
    ("base", "Base"),
    ("base1", "Base 1"),
    ("selected", "Selected"),
    ("active", "Active"),
    ("navigation", "Navigation"),
    ("navigation-fixed", "Navigation fixed"),
    ("dropdown", "Dropdown"),
    ("popover", "Popover"),
    ("tooltip", "Tooltip"),
    ("drawer", "Drawer"),
    ("modal", "Modal"),
];

const OVERFLOWS: &[(&str, &str)] = &[
    ("", "None"),
    ("hidden", "Hidden"),
    ("scroll", "Scroll"),
    ("auto", "Auto"),
    ("clip", "Clip"),
    ("visible", "Visible"),
];

#[component]
pub(crate) fn BoxPage() -> impl IntoView {
    provide_context(VariantSource(include_str!("box.rs")));
    let surface = RwSignal::new("primary");
    let padding = RwSignal::new("300");
    let border_width = RwSignal::new("100");
    let border_color = RwSignal::new("default");
    let border_radius = RwSignal::new("400");
    let shadow = RwSignal::new("card");
    let inline_size = RwSignal::new("");
    let tag = RwSignal::new("div");
    let scrollbar = RwSignal::new("");
    let border_width_block = RwSignal::new("");
    let border_width_block_end = RwSignal::new("");
    let border_width_block_start = RwSignal::new("");
    let border_width_inline = RwSignal::new("");
    let border_width_inline_end = RwSignal::new("");
    let border_width_inline_start = RwSignal::new("");
    let padding_block = RwSignal::new("");
    let padding_block_end = RwSignal::new("");
    let padding_block_start = RwSignal::new("");
    let padding_inline = RwSignal::new("");
    let padding_inline_end = RwSignal::new("");
    let padding_inline_start = RwSignal::new("");
    let block_size = RwSignal::new("");
    let max_block_size = RwSignal::new("");
    let min_block_size = RwSignal::new("");
    let max_inline_size = RwSignal::new("");
    let min_inline_size = RwSignal::new("");
    let position = RwSignal::new("");
    let inset = RwSignal::new("");
    let inset_block = RwSignal::new("");
    let inset_block_start = RwSignal::new("");
    let inset_block_end = RwSignal::new("");
    let inset_inline = RwSignal::new("");
    let inset_inline_start = RwSignal::new("");
    let inset_inline_end = RwSignal::new("");
    let z_index = RwSignal::new("");
    let overflow = RwSignal::new("");
    let body = RwSignal::new("A surface with padding, a border, and a shadow.".to_string());
    let root_class = RwSignal::new(String::new());
    let scrollbar_content_class = RwSignal::new(String::new());
    let scrollbar_off = Signal::derive(move || scrollbar.get().is_empty());
    let preview = ViewFn::from(move || {
        view! {
            {move || {
                let surface = surface.get();
                let padding = padding.get();
                let border_width = border_width.get();
                let border_color = border_color.get();
                let border_radius = border_radius.get();
                let shadow = shadow.get();
                let inline_size = inline_size.get();
                let tag = tag.get();
                let scrollbar = scrollbar.get();
                let class_name = root_class.get();
                let body = body.get();
                let scrollbar_content_class = scrollbar_content_class.get();
                view! {
                    <DtBox
                        r#as=tag
                        surface=surface
                        padding=padding
                        padding_block=padding_block.get()
                        padding_block_end=padding_block_end.get()
                        padding_block_start=padding_block_start.get()
                        padding_inline=padding_inline.get()
                        padding_inline_end=padding_inline_end.get()
                        padding_inline_start=padding_inline_start.get()
                        border_width=border_width
                        border_width_block=border_width_block.get()
                        border_width_block_end=border_width_block_end.get()
                        border_width_block_start=border_width_block_start.get()
                        border_width_inline=border_width_inline.get()
                        border_width_inline_end=border_width_inline_end.get()
                        border_width_inline_start=border_width_inline_start.get()
                        border_color=border_color
                        border_radius=border_radius
                        shadow=shadow
                        block_size=block_size.get()
                        inline_size=inline_size
                        max_block_size=max_block_size.get()
                        min_block_size=min_block_size.get()
                        max_inline_size=max_inline_size.get()
                        min_inline_size=min_inline_size.get()
                        position=position.get()
                        inset=inset.get()
                        inset_block=inset_block.get()
                        inset_block_start=inset_block_start.get()
                        inset_block_end=inset_block_end.get()
                        inset_inline=inset_inline.get()
                        inset_inline_start=inset_inline_start.get()
                        inset_inline_end=inset_inline_end.get()
                        z_index=z_index.get()
                        overflow=overflow.get()
                        scrollbar=scrollbar
                        scrollbar_content_class=scrollbar_content_class
                        class=class_name
                    >
                        <p class="m-0 text-sm">{body}</p>
                    </DtBox>
                }
            }}
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingSelect label="Surface" value=surface options=SURFACES.to_vec() />
            <SettingSelect label="Padding" value=padding options=PADDINGS.to_vec() />
            <SettingSelect label="Padding block" value=padding_block options=SIDE_SPACING.to_vec() />
            <SettingSelect label="Padding block end" value=padding_block_end options=SIDE_SPACING.to_vec() />
            <SettingSelect label="Padding block start" value=padding_block_start options=SIDE_SPACING.to_vec() />
            <SettingSelect label="Padding inline" value=padding_inline options=SIDE_SPACING.to_vec() />
            <SettingSelect label="Padding inline end" value=padding_inline_end options=SIDE_SPACING.to_vec() />
            <SettingSelect label="Padding inline start" value=padding_inline_start options=SIDE_SPACING.to_vec() />
            <SettingSelect label="Border width" value=border_width options=BORDER_WIDTHS.to_vec() />
            <SettingSelect label="Border width block" value=border_width_block options=SIDE_WIDTHS.to_vec() />
            <SettingSelect label="Border width block end" value=border_width_block_end options=SIDE_WIDTHS.to_vec() />
            <SettingSelect label="Border width block start" value=border_width_block_start options=SIDE_WIDTHS.to_vec() />
            <SettingSelect label="Border width inline" value=border_width_inline options=SIDE_WIDTHS.to_vec() />
            <SettingSelect label="Border width inline end" value=border_width_inline_end options=SIDE_WIDTHS.to_vec() />
            <SettingSelect label="Border width inline start" value=border_width_inline_start options=SIDE_WIDTHS.to_vec() />
            <SettingSelect label="Border color" value=border_color options=BORDER_COLORS.to_vec() />
            <SettingSelect label="Radius" value=border_radius options=RADII.to_vec() />
            <SettingSelect label="Shadow" value=shadow options=SHADOWS.to_vec() />
            <SettingSelect label="Inline size" value=inline_size options=INLINE_SIZES.to_vec() />
            <SettingSelect label="Block size" value=block_size options=SIDE_SIZES.to_vec() />
            <SettingSelect label="Max block size" value=max_block_size options=SIDE_SIZES.to_vec() />
            <SettingSelect label="Min block size" value=min_block_size options=SIDE_SIZES.to_vec() />
            <SettingSelect label="Max inline size" value=max_inline_size options=SIDE_SIZES.to_vec() />
            <SettingSelect label="Min inline size" value=min_inline_size options=SIDE_SIZES.to_vec() />
            <SettingSelect label="Position" value=position options=POSITIONS.to_vec() />
            <SettingSelect label="Inset" value=inset options=SIDE_SPACING.to_vec() />
            <SettingSelect label="Inset block" value=inset_block options=SIDE_SPACING.to_vec() />
            <SettingSelect label="Inset block start" value=inset_block_start options=SIDE_SPACING.to_vec() />
            <SettingSelect label="Inset block end" value=inset_block_end options=SIDE_SPACING.to_vec() />
            <SettingSelect label="Inset inline" value=inset_inline options=SIDE_SPACING.to_vec() />
            <SettingSelect label="Inset inline start" value=inset_inline_start options=SIDE_SPACING.to_vec() />
            <SettingSelect label="Inset inline end" value=inset_inline_end options=SIDE_SPACING.to_vec() />
            <SettingSelect label="Z index" value=z_index options=Z_INDEXES.to_vec() />
            <SettingSelect label="Overflow" value=overflow options=OVERFLOWS.to_vec() />
            <SettingSelect label="As" value=tag options=ELEMENTS.to_vec() />
            <SettingSelect label="Scrollbar" value=scrollbar options=SCROLLBARS.to_vec() />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        if tag.get() != "div" {
            push_str(&mut props, "r#as", tag.get());
        }
        push_str(&mut props, "surface", surface.get());
        push_str(&mut props, "padding", padding.get());
        push_str(&mut props, "padding_block", padding_block.get());
        push_str(&mut props, "padding_block_end", padding_block_end.get());
        push_str(&mut props, "padding_block_start", padding_block_start.get());
        push_str(&mut props, "padding_inline", padding_inline.get());
        push_str(&mut props, "padding_inline_end", padding_inline_end.get());
        push_str(
            &mut props,
            "padding_inline_start",
            padding_inline_start.get(),
        );
        push_str(&mut props, "border_width", border_width.get());
        push_str(&mut props, "border_width_block", border_width_block.get());
        push_str(
            &mut props,
            "border_width_block_end",
            border_width_block_end.get(),
        );
        push_str(
            &mut props,
            "border_width_block_start",
            border_width_block_start.get(),
        );
        push_str(&mut props, "border_width_inline", border_width_inline.get());
        push_str(
            &mut props,
            "border_width_inline_end",
            border_width_inline_end.get(),
        );
        push_str(
            &mut props,
            "border_width_inline_start",
            border_width_inline_start.get(),
        );
        if border_color.get() != "default" {
            push_str(&mut props, "border_color", border_color.get());
        }
        push_str(&mut props, "border_radius", border_radius.get());
        push_str(&mut props, "shadow", shadow.get());
        push_str(&mut props, "block_size", block_size.get());
        push_str(&mut props, "inline_size", inline_size.get());
        push_str(&mut props, "max_block_size", max_block_size.get());
        push_str(&mut props, "min_block_size", min_block_size.get());
        push_str(&mut props, "max_inline_size", max_inline_size.get());
        push_str(&mut props, "min_inline_size", min_inline_size.get());
        push_str(&mut props, "position", position.get());
        push_str(&mut props, "inset", inset.get());
        push_str(&mut props, "inset_block", inset_block.get());
        push_str(&mut props, "inset_block_start", inset_block_start.get());
        push_str(&mut props, "inset_block_end", inset_block_end.get());
        push_str(&mut props, "inset_inline", inset_inline.get());
        push_str(&mut props, "inset_inline_start", inset_inline_start.get());
        push_str(&mut props, "inset_inline_end", inset_inline_end.get());
        push_str(&mut props, "z_index", z_index.get());
        push_str(&mut props, "overflow", overflow.get());
        push_str(&mut props, "scrollbar", scrollbar.get());
        push_str(
            &mut props,
            "scrollbar_content_class",
            &scrollbar_content_class.get(),
        );
        push_str(&mut props, "class", &root_class.get());
        let body = element(
            "p",
            &[("class", rust_string("m-0 text-sm"))],
            Some(&rust_string(&body.get())),
        );
        view_of(&element("DtBox", &props, Some(&body)))
    });
    view! {
        <section id="playground" class="docs-section">
            <h2>"Playground"</h2>
            <p class="docs-note">
                "DtBox is the layout primitive for surface, border, spacing, sizing, and position. Border color defaults to default and stays invisible until a border width is set. An invalid token warns and is omitted. Scrollbar inserts a wrapper that scrolls with overflow auto. v-dt-scrollbar is not ported, so leave, scroll, move, and always share that shell. ul, ol, span, and fieldset warn when scrollbar is set."
            </p>
            <Play
                preview=preview
                settings=settings
                slots=ViewFn::from(move || {
                    view! {
                        <SettingText label="Default" hint="Box content." value=body />
                    }
                })
                classes=class_panel(vec![
                    ClassField {
                        label: "Class",
                        hint: ROOT_CLASS,
                        value: root_class,
                        off: None,
                    },
                    ClassField {
                        label: "Scrollbar content class",
                        hint: "Extra class on the scrollbar content.",
                        value: scrollbar_content_class,
                        off: Some(scrollbar_off),
                    },
                ])
                code=code
            />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Surfaces">
                    <DtStack direction="row" gap="200">
                        <DtBox padding="200" surface="primary" border_width="100" border_radius="300">
                            "primary"
                        </DtBox>
                        <DtBox padding="200" surface="secondary" border_width="100" border_radius="300">
                            "secondary"
                        </DtBox>
                        <DtBox padding="200" surface="moderate" border_width="100" border_radius="300">
                            "moderate"
                        </DtBox>
                        <DtBox padding="200" surface="bold" border_width="100" border_radius="300">
                            "bold"
                        </DtBox>
                        <DtBox padding="200" surface="brand" border_width="100" border_radius="300">
                            "brand"
                        </DtBox>
                        <DtBox padding="200" surface="info" border_width="100" border_radius="300">
                            "info"
                        </DtBox>
                        <DtBox padding="200" surface="positive" border_width="100" border_radius="300">
                            "positive"
                        </DtBox>
                        <DtBox padding="200" surface="warning" border_width="100" border_radius="300">
                            "warning"
                        </DtBox>
                        <DtBox padding="200" surface="critical" border_width="100" border_radius="300">
                            "critical"
                        </DtBox>
                    </DtStack>
                </Variant>
                <Variant title="Card">
                    <DtBox
                        padding="300"
                        surface="primary"
                        border_width="100"
                        border_radius="400"
                        shadow="card"
                    >
                        <p class="m-0 text-sm">"Card body content with supporting text."</p>
                    </DtBox>
                </Variant>
                <Variant title="Scrollbar">
                    <DtBox
                        padding="200"
                        border_width="100"
                        border_radius="300"
                        scrollbar="always"
                        block_size="300"
                    >
                        {(1..=12).map(|index| {
                            view! {
                                <p class="m-0 text-sm">{format!("Scrollable item {index}")}</p>
                            }
                        }).collect_view()}
                    </DtBox>
                </Variant>
            </div>
        </section>
    }
}
