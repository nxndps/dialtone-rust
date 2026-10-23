use leptos::prelude::*;

fn alignment_class(alignment: &str) -> &'static str {
    match alignment {
        "end" => "d-btn-group--end",
        "space-between" => "d-btn-group--space-between",
        "start" => "d-btn-group--start",
        _ => "",
    }
}

fn join_class(parts: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    parts
        .into_iter()
        .map(|part| part.as_ref().trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn button_group_class(alignment: &str) -> String {
    join_class(["d-btn-group", alignment_class(alignment)])
}

/// Dialtone Vue `DtButtonGroup` (`dialtone/v9.190.0`).
///
/// `alignment` is `start` (default), `end`, or `space-between`.
#[component]
pub fn DtButtonGroup(
    /// Button alignment (`alignment`).
    #[prop(optional, default = "start")]
    alignment: &'static str,
    children: Children,
) -> impl IntoView {
    let class_name = button_group_class(alignment);
    view! {
        <div class=class_name role="group">
            {children()}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alignment_classes_match_vue() {
        assert_eq!(
            button_group_class("start"),
            "d-btn-group d-btn-group--start"
        );
        assert_eq!(button_group_class("end"), "d-btn-group d-btn-group--end");
        assert_eq!(
            button_group_class("space-between"),
            "d-btn-group d-btn-group--space-between"
        );
        assert_eq!(button_group_class("center"), "d-btn-group");
    }
}
