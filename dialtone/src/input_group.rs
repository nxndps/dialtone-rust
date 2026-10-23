use leptos::prelude::*;

/// Dialtone Vue `DtInputGroup`.
#[component]
pub fn DtInputGroup(
    #[prop(optional, into)] legend: String,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let class = if class.is_empty() {
        "d-input-group__fieldset w-full".to_string()
    } else {
        format!("d-input-group__fieldset w-full {class}")
    };
    view! {
        <fieldset class=class data-qa="input-group">
            {(!legend.is_empty()).then(|| {
                view! {
                    <legend class="d-label" data-qa="input-group-legend">
                        {legend}
                    </legend>
                }
            })}
            {children()}
        </fieldset>
    }
}
