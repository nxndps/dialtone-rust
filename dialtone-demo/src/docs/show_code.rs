//! "Show Code" control for catalog examples and component variants.
//!
//! The button opens a card with the Leptos source, a Copy control, and a
//! Close icon. Copy uses the document listener in `copy`.

use dialtone::DtIconClose;
use leptos::prelude::*;

use super::copy::copy_button_markup;

#[component]
pub(crate) fn ShowCode(code: ViewFn) -> impl IntoView {
    let open = RwSignal::new(false);
    view! {
        <div class="docs-code-reveal">
            <div class="docs-code-reveal-bar">
                <button
                    type="button"
                    class="docs-show-code"
                    aria-expanded=move || if open.get() { "true" } else { "false" }
                    on:click=move |_| open.update(|value| *value = !*value)
                >
                    "Show Code"
                </button>
            </div>
            {move || {
                if open.get() {
                    view! {
                        <div class="docs-code-card docs-code-shell">
                            <div class="docs-code-card-bar">
                                <p class="docs-settings-title">"Leptos"</p>
                                <button
                                    type="button"
                                    class="docs-code-close"
                                    aria-label="Close"
                                    on:click=move |_| open.set(false)
                                >
                                    <DtIconClose size="200" />
                                </button>
                            </div>
                            <div class="docs-code-body">{code.run()}</div>
                            <div class="docs-copy-slot" inner_html=copy_button_markup()></div>
                        </div>
                    }
                    .into_any()
                } else {
                    ().into_any()
                }
            }}
        </div>
    }
}

/// Static Leptos snippet. The playground passes a live [`ShowCode`] view instead.
#[component]
pub(crate) fn ShowCodeText(#[prop(into)] text: String) -> impl IntoView {
    let code = ViewFn::from(move || {
        let text = text.clone();
        let html = super::highlight::code(&text);
        view! { <pre><code class="d-codeblock" inner_html=html></code></pre> }
    });
    view! { <ShowCode code=code /> }
}
