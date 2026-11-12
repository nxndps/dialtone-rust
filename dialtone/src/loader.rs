use leptos::prelude::*;

const TRACK_PATH: &str = "M12 0.0180664C18.6175 0.0180664 23.9819 5.38305 23.9819 12.0005C23.9819 18.6177 18.6175 23.9819 12 23.9819C5.38281 23.9819 0.0180664 18.6177 0.0180664 12.0005C0.0180664 5.38305 5.38281 0.0180664 12 0.0180664ZM12 3.26807C7.17773 3.26807 3.26807 7.17797 3.26807 12.0005C3.26807 16.8228 7.17773 20.7319 12 20.7319C16.8223 20.7319 20.7319 16.8228 20.7319 12.0005C20.7319 7.17797 16.8223 3.26807 12 3.26807Z";

const ICON_PATH: &str = "M0.0180664 12.0005C0.0180664 5.38305 5.38305 0.0180664 12.0005 0.0180664C12.8977 0.0183302 13.6255 0.745766 13.6255 1.64307C13.6255 2.54037 12.8977 3.2678 12.0005 3.26807C7.17797 3.26807 3.26807 7.17797 3.26807 12.0005C3.26833 16.8228 7.17814 20.7319 12.0005 20.7319C16.8226 20.7317 20.7317 16.8226 20.7319 12.0005C20.7319 11.103 21.4595 10.3755 22.3569 10.3755C23.2544 10.3755 23.9819 11.103 23.9819 12.0005C23.9817 18.6175 18.6175 23.9817 12.0005 23.9819C5.38321 23.9819 0.0183303 18.6177 0.0180664 12.0005Z";

/// Icon size token. Unknown sizes warn and use `500`, Vue's default.
pub(crate) fn loader_size_token(size: &str) -> &'static str {
    match size {
        "100" => "100",
        "200" => "200",
        "300" => "300",
        "400" => "400",
        "500" => "500",
        "600" => "600",
        "700" => "700",
        "800" => "800",
        _ => {
            leptos::logging::warn!("DtLoader size `{size}` is not an icon size; using 500");
            "500"
        }
    }
}

/// `d-icon d-icon--size-{size} {role}` for a size `loader_size_token` already resolved.
pub(crate) fn loader_icon_class(size: &str, role: &str) -> String {
    format!("d-icon d-icon--size-{size} {role}")
}

/// Dialtone Vue `DtLoader` (`dialtone/v10.7.0`).
///
/// The root is `div.d-loader` (`data-qa="dt-loader"`). The spinning icon is
/// `data-qa="dt-loader-icon"`. `size` defaults to `"500"`. Valid sizes are
/// `100` through `800`. An unknown size warns and uses `500`. `aria_label`
/// (`ariaLabel`) empty uses `loading`. `class` and `aria_hidden` stand in for
/// fallthrough attributes. `aria_hidden` false omits the attribute.
#[component]
pub fn DtLoader(
    /// Icon size token (`size`). Default `"500"`.
    #[prop(optional, default = "500")]
    size: &'static str,
    /// Extra class on the root, for example `d-btn__loader`.
    #[prop(optional, into)]
    class: String,
    /// Screen-reader label (`ariaLabel`). Empty uses `loading`.
    #[prop(optional, into)]
    aria_label: String,
    /// Hides the loader from assistive tech. False omits the attribute.
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
    let size = loader_size_token(size);
    let track = loader_icon_class(size, "d-loader__icon-track");
    let icon = loader_icon_class(size, "d-loader__icon");
    view! {
        <div
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
                <path d=TRACK_PATH />
            </svg>
            <svg
                class=icon
                data-qa="dt-loader-icon"
                viewBox="0 0 24 24"
                fill="currentColor"
                xmlns="http://www.w3.org/2000/svg"
                aria-hidden="true"
            >
                <path d=ICON_PATH />
            </svg>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_sizes_stay_and_unknown_sizes_use_500() {
        assert_eq!(loader_size_token("200"), "200");
        assert_eq!(loader_size_token("500"), "500");
        assert_eq!(loader_size_token("800"), "800");
        assert_eq!(loader_size_token("999"), "500");
        assert_eq!(loader_size_token("xs"), "500");
    }

    #[test]
    fn icon_class_uses_the_size_and_the_role() {
        assert_eq!(
            loader_icon_class("500", "d-loader__icon"),
            "d-icon d-icon--size-500 d-loader__icon"
        );
        assert_eq!(
            loader_icon_class("200", "d-loader__icon-track"),
            "d-icon d-icon--size-200 d-loader__icon-track"
        );
    }
}
