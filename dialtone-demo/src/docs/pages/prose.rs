use super::*;
use wasm_bindgen::JsCast;

const SIZES: &[(&str, &str)] = &[("100", "100"), ("200", "200"), ("300", "300")];

const DENSITIES: &[(&str, &str)] = &[("100", "100"), ("200", "200"), ("300", "300")];

const DEFAULT_SLOT: &str = "<h2>Release notes</h2>\n<p><a href=\"https://dialtone.dialpad.com\">Dialtone</a> covers <strong>buttons</strong> and lists.</p>\n<ul>\n<li>Voice</li>\n<li>Meetings</li>\n</ul>";

fn write_prose_slot(host: NodeRef<html::Div>, html: &str) -> bool {
    let Some(host) = host.get_untracked() else {
        return false;
    };
    let Ok(Some(prose)) = host.query_selector("[data-qa='dt-prose']") else {
        return false;
    };
    prose.set_inner_html(html);
    true
}

#[component]
pub(crate) fn ProsePage() -> impl IntoView {
    provide_context(VariantSource(include_str!("prose.rs")));
    let size = RwSignal::new("300");
    let density = RwSignal::new("200");
    let root_class = RwSignal::new(String::new());
    let slot_html = RwSignal::new(DEFAULT_SLOT.to_string());
    let host = NodeRef::<html::Div>::new();
    Effect::new(move |_| {
        let html = slot_html.get();
        let _ = (size.get(), density.get(), root_class.get());
        if write_prose_slot(host, &html) {
            return;
        }
        let host = host;
        let html = html.clone();
        let Some(window) = web_sys::window() else {
            return;
        };
        let script = wasm_bindgen::closure::Closure::once_into_js(move || {
            let _ = write_prose_slot(host, &html);
        });
        let Some(function) = script.dyn_ref::<js_sys::Function>() else {
            return;
        };
        let _ = window.request_animation_frame(function);
    });
    let preview = ViewFn::from(move || {
        view! {
            <div node_ref=host style="display: contents">
                {move || {
                    let size = size.get();
                    let density = density.get();
                    let class_name = root_class.get();
                    view! { <DtProse size=size density=density class=class_name /> }
                }}
            </div>
        }
    });
    let settings = ViewFn::from(move || {
        view! {
            <SettingSelect label="Size" value=size options=SIZES.to_vec() />
            <SettingSelect label="Density" value=density options=DENSITIES.to_vec() />
        }
    });
    let code = example_panel(move || {
        let mut props = Vec::new();
        if size.get() != "300" {
            push_str(&mut props, "size", size.get());
        }
        if density.get() != "200" {
            push_str(&mut props, "density", density.get());
        }
        push_str(&mut props, "class", &root_class.get());
        let body = slot_html.get();
        let body = (!body.is_empty()).then_some(body);
        view_of(&element("DtProse", &props, body.as_deref()))
    });
    let slots = ViewFn::from(move || {
        view! {
            <SettingArea
                label="Default"
                hint="Default slot for raw HTML content."
                value=slot_html
            />
        }
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
                "The default slot is raw HTML. Size 300 and density 200 add no modifier. Descendants lose class, style, and data attributes. Unsafe href, src, and cite values are removed. A debug build warns for form controls and custom elements. A checkbox inside a list item is allowed. The root is not sanitized."
            </p>
            <Play preview=preview settings=settings slots=slots classes=classes code=code />
        </section>
        <section id="variants" class="docs-section">
            <h2>"Variants"</h2>
            <div class="docs-variants">
                <Variant title="Sizes">
                    <DtStack gap="200">
                        <DtProse size="100">
                            <p>"Size 100"</p>
                        </DtProse>
                        <DtProse size="200">
                            <p>"Size 200"</p>
                        </DtProse>
                        <DtProse>
                            <p>"Size 300"</p>
                        </DtProse>
                    </DtStack>
                </Variant>
                <Variant title="Densities">
                    <DtStack gap="200">
                        <DtProse density="100">
                            <p>"Density 100"</p>
                        </DtProse>
                        <DtProse>
                            <p>"Density 200"</p>
                        </DtProse>
                        <DtProse density="300">
                            <p>"Density 300"</p>
                        </DtProse>
                    </DtStack>
                </Variant>
                <Variant title="Inline formatting">
                    <DtProse>
                        <p>
                            <strong>"Strong"</strong>
                            ", "
                            <em>"emphasis"</em>
                            ", and "
                            <a href="https://dialtone.dialpad.com">"a link"</a>
                            "."
                        </p>
                    </DtProse>
                </Variant>
                <Variant title="Task list">
                    <DtProse class="d-w100p">
                        <ul>
                            <li>
                                <input type="checkbox" checked=true disabled=true />
                                " Voice"
                            </li>
                            <li>
                                <input type="checkbox" disabled=true />
                                " Meetings"
                            </li>
                        </ul>
                    </DtProse>
                </Variant>
            </div>
        </section>
    }
}
