use leptos::prelude::*;

const TRACK_PATH: &str = "M12 0.0180664C18.6175 0.0180664 23.9819 5.38305 23.9819 12.0005C23.9819 18.6177 18.6175 23.9819 12 23.9819C5.38281 23.9819 0.0180664 18.6177 0.0180664 12.0005C0.0180664 5.38305 5.38281 0.0180664 12 0.0180664ZM12 3.26807C7.17773 3.26807 3.26807 7.17797 3.26807 12.0005C3.26807 16.8228 7.17773 20.7319 12 20.7319C16.8223 20.7319 20.7319 16.8228 20.7319 12.0005C20.7319 7.17797 16.8223 3.26807 12 3.26807Z";

const ICON_PATH: &str = "M0.0180664 12.0005C0.0180664 5.38305 5.38305 0.0180664 12.0005 0.0180664C12.8977 0.0183302 13.6255 0.745766 13.6255 1.64307C13.6255 2.54037 12.8977 3.2678 12.0005 3.26807C7.17797 3.26807 3.26807 7.17797 3.26807 12.0005C3.26833 16.8228 7.17814 20.7319 12.0005 20.7319C16.8226 20.7317 20.7317 16.8226 20.7319 12.0005C20.7319 11.103 21.4595 10.3755 22.3569 10.3755C23.2544 10.3755 23.9819 11.103 23.9819 12.0005C23.9817 18.6175 18.6175 23.9817 12.0005 23.9819C5.38321 23.9819 0.0183303 18.6177 0.0180664 12.0005Z";

/// Rest positions from `Loader.vue` at `dialtone/v10.11.0`. The keyframes add
/// the outward spread on top of these values.
const LOADER_DOTS: [(&str, &str, &str, &str); 4] = [
    ("d-loader__dot d-loader__dot--1", "14.843", "9.471", "2.410"),
    ("d-loader__dot d-loader__dot--2", "9.156", "15.199", "2.410"),
    (
        "d-loader__dot d-loader__dot--3",
        "15.469",
        "15.743",
        "3.647",
    ),
    ("d-loader__dot d-loader__dot--4", "8.539", "8.915", "3.647"),
];

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

/// `dots` or `spinner`. Anything else warns and uses `dots`.
pub(crate) fn loader_kind(kind: &str) -> &'static str {
    match kind {
        "dots" => "dots",
        "spinner" => "spinner",
        _ => {
            leptos::logging::warn!("DtLoader kind `{kind}` is not dots or spinner; using dots");
            "dots"
        }
    }
}

/// `d-icon d-icon--size-{size} {role}` for a size `loader_size_token` already resolved.
pub(crate) fn loader_icon_class(size: &str, role: &str) -> String {
    format!("d-icon d-icon--size-{size} {role}")
}

/// Root class. `kind` is resolved by [`loader_kind`]. `spinner` adds
/// `d-loader--spinner`. An extra class is appended.
pub(crate) fn loader_root_class(kind: &str, class: &str) -> String {
    let mut root = String::from("d-loader");
    if loader_kind(kind) == "spinner" {
        root.push_str(" d-loader--spinner");
    }
    if !class.is_empty() {
        root.push(' ');
        root.push_str(class);
    }
    root
}

/// Screen-reader name. `none` and `presentation` omit it. An empty label is `loading`.
pub(crate) fn loader_accessible_name(role: &str, aria_label: &str) -> Option<String> {
    if matches!(role, "none" | "presentation") {
        return None;
    }
    Some(if aria_label.is_empty() {
        "loading".to_string()
    } else {
        aria_label.to_string()
    })
}

/// Dialtone Vue `DtLoader` (`dialtone/v10.11.0`).
///
/// The root is `div.d-loader` (`data-qa="dt-loader"`, `role="img"`). `kind`
/// is `dots` (default) or `spinner`. An unknown kind warns and uses `dots`.
/// `dots` is one `svg.d-loader__icon` with four `circle.d-loader__dot`.
/// `spinner` adds `d-loader--spinner` and the two-path mark. `size` defaults
/// to `"500"`. Valid sizes are `100` through `800`. An unknown size warns and
/// uses `500`. `aria_label` (`ariaLabel`) empty uses `loading`. `role` `none`
/// or `presentation` omits `aria-label`. `class` and `aria_hidden` stand in
/// for fallthrough attributes. `aria_hidden` false omits the attribute.
#[component]
pub fn DtLoader(
    /// `dots` or `spinner` (`kind`). Default `dots`.
    #[prop(optional, default = "dots")]
    kind: &'static str,
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
    /// Fallthrough `role`. Empty uses `img`.
    #[prop(optional, default = "img")]
    role: &'static str,
) -> impl IntoView {
    let kind = loader_kind(kind);
    let role = if role.is_empty() { "img" } else { role };
    let root = loader_root_class(kind, &class);
    let name = loader_accessible_name(role, &aria_label);
    let size = loader_size_token(size);
    let mark = if kind == "spinner" {
        let track = loader_icon_class(size, "d-loader__icon-track");
        let icon = loader_icon_class(size, "d-loader__icon");
        view! {
            <svg
                class=track
                data-qa="dt-loader-icon-track"
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
        }
        .into_any()
    } else {
        let icon = loader_icon_class(size, "d-loader__icon");
        let (dot_1_class, dot_1_x, dot_1_y, dot_1_r) = LOADER_DOTS[0];
        let (dot_2_class, dot_2_x, dot_2_y, dot_2_r) = LOADER_DOTS[1];
        let (dot_3_class, dot_3_x, dot_3_y, dot_3_r) = LOADER_DOTS[2];
        let (dot_4_class, dot_4_x, dot_4_y, dot_4_r) = LOADER_DOTS[3];
        view! {
            <svg
                class=icon
                data-qa="dt-loader-icon"
                viewBox="0 0 24 24"
                xmlns="http://www.w3.org/2000/svg"
                aria-hidden="true"
            >
                <circle class=dot_1_class cx=dot_1_x cy=dot_1_y r=dot_1_r data-qa="dt-loader-dot" />
                <circle class=dot_2_class cx=dot_2_x cy=dot_2_y r=dot_2_r data-qa="dt-loader-dot" />
                <circle class=dot_3_class cx=dot_3_x cy=dot_3_y r=dot_3_r data-qa="dt-loader-dot" />
                <circle class=dot_4_class cx=dot_4_x cy=dot_4_y r=dot_4_r data-qa="dt-loader-dot" />
            </svg>
        }
        .into_any()
    };
    view! {
        <div
            class=root
            role=role
            aria-label=name
            aria-hidden=aria_hidden.then_some("true")
            data-qa="dt-loader"
        >
            {mark}
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

    #[test]
    fn dots_have_no_kind_class_and_spinner_adds_the_modifier() {
        assert_eq!(loader_root_class("dots", ""), "d-loader");
        assert_eq!(
            loader_root_class("spinner", "d-btn__loader"),
            "d-loader d-loader--spinner d-btn__loader"
        );
        assert_eq!(loader_root_class("pulse", ""), "d-loader");
    }

    #[test]
    fn accessible_name_defaults_to_loading_and_hides_for_presentational_roles() {
        assert_eq!(
            loader_accessible_name("img", ""),
            Some("loading".to_string())
        );
        assert_eq!(
            loader_accessible_name("img", "Loading schedules"),
            Some("Loading schedules".to_string())
        );
        assert_eq!(loader_accessible_name("none", "Loading schedules"), None);
        assert_eq!(loader_accessible_name("presentation", ""), None);
    }

    #[test]
    fn dot_geometry_matches_the_vue_rest_positions() {
        assert_eq!(
            LOADER_DOTS,
            [
                ("d-loader__dot d-loader__dot--1", "14.843", "9.471", "2.410"),
                ("d-loader__dot d-loader__dot--2", "9.156", "15.199", "2.410"),
                (
                    "d-loader__dot d-loader__dot--3",
                    "15.469",
                    "15.743",
                    "3.647"
                ),
                ("d-loader__dot d-loader__dot--4", "8.539", "8.915", "3.647"),
            ]
        );
    }
}
