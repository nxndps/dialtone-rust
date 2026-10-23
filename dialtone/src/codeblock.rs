use leptos::prelude::*;

/// Dialtone Vue `DtCodeblock` (`dialtone/v9.190.0`).
///
/// `text` is the source shown in the block.
#[component]
pub fn DtCodeblock(
    /// Source text (`text`).
    #[prop(into)]
    text: String,
) -> impl IntoView {
    view! {
        <pre><code class="d-codeblock">{text}</code></pre>
    }
}
