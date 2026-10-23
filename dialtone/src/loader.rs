use leptos::prelude::*;

/// Dialtone Vue `DtLoader`.
///
/// `size` is the icon token. This port defaults to `"200"` so existing
/// `<DtLoader />` call sites stay the same. Vue's default is `"500"`.
#[component]
pub fn DtLoader(
    /// Icon size token (`size`).
    #[prop(optional, default = "200")]
    size: &'static str,
    /// Extra class on the root, for example `d-btn__loader`.
    #[prop(optional, into)]
    class: String,
    /// Screen-reader label (`ariaLabel`). Empty uses `loading`.
    #[prop(optional, into)]
    aria_label: String,
    /// Hides the loader from assistive tech.
    #[prop(optional)]
    aria_hidden: bool,
) -> impl IntoView {
    let aria_label = if aria_label.is_empty() {
        "loading".to_string()
    } else {
        aria_label
    };
    let root = if class.is_empty() {
        "d-loader".to_string()
    } else {
        format!("d-loader {class}")
    };
    let track = format!("d-icon d-icon--size-{size} d-loader__icon-track");
    let icon = format!("d-icon d-icon--size-{size} d-loader__icon");
    view! {
        <span
            class=root
            aria-label=aria_label
            aria-hidden=aria_hidden.then_some("true")
            data-qa="dt-loader"
        >
            <svg
                class=track
                viewBox="0 0 24 24"
                fill="currentColor"
                xmlns="http://www.w3.org/2000/svg"
                aria-hidden="true"
            >
                <path d="M12 .018C18.618.018 23.982 5.383 23.982 12S18.618 23.982 12 23.982C5.383 23.982 .018 18.618 .018 12S5.383 .018 12 .018Zm0 3.25C7.178 3.268 3.268 7.178 3.268 12S7.178 20.732 12 20.732 20.732 16.823 20.732 12 16.822 3.268 12 3.268Z" />
            </svg>
            <svg
                class=icon
                data-qa="dt-loader-icon"
                viewBox="0 0 24 24"
                fill="currentColor"
                xmlns="http://www.w3.org/2000/svg"
                aria-hidden="true"
            >
                <path d="M.018 12C.018 5.383 5.383 .018 12 .018c.897 0 1.625 .728 1.625 1.625S12.897 3.268 12 3.268C7.178 3.268 3.268 7.178 3.268 12c0 4.822 3.91 8.732 8.732 8.732 4.822 0 8.732-3.91 8.732-8.732 0-.897 .728-1.625 1.625-1.625S23.982 11.103 23.982 12C23.982 18.618 18.618 23.982 12 23.982S.018 18.618 .018 12Z" />
            </svg>
        </span>
    }
}
