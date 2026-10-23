use leptos::html;
use leptos::prelude::*;

use super::text::{text_classes, TextStyle};
use super::unique_id;

/// Dialtone `INPUT_SIZE_CLASSES.input`. `300` and `md` add no modifier.
pub(crate) fn input_size_class(size: &str) -> &'static str {
    match size {
        "100" | "xs" => "d-input--xs",
        "200" | "sm" => "d-input--sm",
        "400" | "lg" => "d-input--lg",
        "500" | "xl" => "d-input--xl",
        "300" | "md" | "" => "",
        _ => {
            leptos::logging::warn!("DtInput size `{size}` is not 100–500 or xs–xl");
            ""
        }
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

/// Vue `resolvedLabelSize`. `xl` and `500` cap at the label maximum.
fn label_size_token(size: &'static str) -> &'static str {
    match size {
        "xl" => "lg",
        "500" => "400",
        "" => "300",
        other => other,
    }
}

/// Vue `resolvedDescriptionSize`.
fn description_size_token(size: &'static str) -> &'static str {
    match size {
        "100" | "200" | "xs" | "sm" => "xs",
        "500" | "xl" => "md",
        _ => "sm",
    }
}

fn text_style<'a>(kind: &'a str, size: &'a str, tone: &'a str, density: &'a str) -> TextStyle<'a> {
    TextStyle {
        kind,
        variant: "",
        size,
        family: "",
        italic: false,
        tone,
        align: "",
        truncate: false,
        max_lines: 0,
        numeric: false,
        wrap: "",
        text_box_trim: "",
        strength: "",
        density,
    }
}

pub(crate) fn input_label_class(size: &'static str) -> String {
    format!(
        "{} d-input__label-text",
        text_classes(&text_style(
            "label",
            label_size_token(size),
            "secondary",
            ""
        ))
    )
}

pub(crate) fn input_description_class(size: &'static str) -> String {
    let density = match size {
        "500" | "xl" => "300",
        _ => "",
    };
    format!(
        "{} d-input__description d-description",
        text_classes(&text_style(
            "body",
            description_size_token(size),
            "tertiary",
            density
        ))
    )
}

/// Dialtone Vue `DtInput`.
///
/// `show_label` is `showLabel`. `size` is `100`–`500` or `xs`–`xl` (`size`).
/// `300` and `md` are the default and add no size class.
#[component]
pub fn DtInput(
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] label: String,
    /// `showLabel`. Default true. False omits the visible label and sets `aria-label`.
    #[prop(optional, default = true)]
    show_label: bool,
    #[prop(optional, into)] description: String,
    /// `100`–`500`, or `xs` / `sm` / `md` / `lg` / `xl` (`size`).
    #[prop(optional, default = "300")]
    size: &'static str,
    #[prop(optional, default = "text")] input_type: &'static str,
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_input: Callback<String>,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] autocomplete: String,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional)] required: bool,
    #[prop(optional)] node_ref: NodeRef<html::Input>,
    #[prop(optional, into)] input_wrapper_class: String,
) -> impl IntoView {
    let id = if id.is_empty() {
        unique_id("dt-input")
    } else {
        id
    };
    let autocomplete = if autocomplete.is_empty() {
        "off".to_string()
    } else {
        autocomplete
    };
    let wrapper_class = if input_wrapper_class.is_empty() {
        "d-input__wrapper".to_string()
    } else {
        format!("d-input__wrapper {input_wrapper_class}")
    };
    let for_id = id.clone();
    let size_class = input_size_class(size);
    let input_class = join_class(["d-input__input", "d-input", size_class]);
    let label_class = input_label_class(size);
    let description_class = input_description_class(size);
    let aria_label = (!show_label && !label.is_empty()).then(|| label.clone());
    view! {
        <div class="d-input__root w-full" data-qa="dt-input">
            <label class="d-input__label" for=for_id data-qa="dt-input-label-wrapper">
                {(show_label && !label.is_empty()).then(|| {
                    view! {
                        <span class=label_class data-qa="dt-input-label">
                            {label}
                        </span>
                    }
                })}
                {(!description.is_empty()).then(|| {
                    view! {
                        <div class=description_class data-qa="dt-input-description">
                            {description}
                        </div>
                    }
                })}
                <div class=wrapper_class>
                    <input
                        id=id
                        name=name
                        class=input_class
                        type=input_type
                        node_ref=node_ref
                        autocomplete=autocomplete
                        placeholder=placeholder
                        required=required
                        aria-label=aria_label
                        prop:value=move || value.get()
                        prop:disabled=move || disabled.get().unwrap_or(false)
                        data-qa="dt-input-input"
                        on:input=move |ev| on_input.run(event_target_value(&ev))
                    />
                </div>
            </label>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_size_classes_match_vue() {
        assert_eq!(input_size_class("100"), "d-input--xs");
        assert_eq!(input_size_class("xs"), "d-input--xs");
        assert_eq!(input_size_class("200"), "d-input--sm");
        assert_eq!(input_size_class("sm"), "d-input--sm");
        assert_eq!(input_size_class("300"), "");
        assert_eq!(input_size_class("md"), "");
        assert_eq!(input_size_class(""), "");
        assert_eq!(input_size_class("400"), "d-input--lg");
        assert_eq!(input_size_class("lg"), "d-input--lg");
        assert_eq!(input_size_class("500"), "d-input--xl");
        assert_eq!(input_size_class("xl"), "d-input--xl");
        assert_eq!(input_size_class("nope"), "");
        assert_eq!(
            input_label_class("md"),
            "d-text d-text-label--md d-text--tone-secondary d-input__label-text"
        );
        assert_eq!(
            input_label_class("300"),
            "d-text d-text-label--md d-text--tone-secondary d-input__label-text"
        );
        assert_eq!(
            input_label_class("xl"),
            "d-text d-text-label--lg d-text--tone-secondary d-input__label-text"
        );
        assert_eq!(
            input_label_class("500"),
            "d-text d-text-label--lg d-text--tone-secondary d-input__label-text"
        );
        assert_eq!(
            input_description_class("md"),
            "d-text d-text-body--sm d-text--tone-tertiary d-input__description d-description"
        );
        assert_eq!(
            input_description_class("xl"),
            "d-text d-text-body--md d-text--tone-tertiary d-text--lh-300 d-input__description d-description"
        );
    }
}
