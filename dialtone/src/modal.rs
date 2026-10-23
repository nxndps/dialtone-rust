use leptos::prelude::*;
use web_sys::MouseEvent;

use super::icon::DtIconClose;

const DIALOG_CLASS: &str =
    "d-modal__dialog d-modal__dialog--scrollable d-hmx764 w-max min-w-[80rem] max-w-[110rem]";

/// Dialtone `DtModal`: overlay, labeled header, close button, optional footer.
#[component]
pub fn DtModal(
    /// Whether the modal is visible.
    #[prop(into)]
    show: Signal<bool>,
    /// Called when the overlay or close button is clicked.
    #[prop(into)]
    on_close: Callback<()>,
    /// `id` of the title element (`aria-labelledby`).
    labelled_by: &'static str,
    /// Extra classes appended to the dialog.
    #[prop(optional, into)]
    dialog_class: String,
    /// Header title.
    title: &'static str,
    /// Optional subtitle under the title.
    #[prop(optional, into)]
    subtitle: MaybeProp<String>,
    /// Optional footer slot. Omit to hide the footer.
    #[prop(optional, into)]
    footer: Option<ViewFn>,
    /// Extra overlay content (outside the dialog, e.g. a tooltip).
    #[prop(optional, into)]
    overlay: Option<ViewFn>,
    children: ChildrenFn,
) -> impl IntoView {
    let dialog_class = if dialog_class.is_empty() {
        DIALOG_CLASS.to_string()
    } else {
        format!("{DIALOG_CLASS} {dialog_class}")
    };

    view! {
        <Show when=move || show.get()>
            <div
                class="d-modal"
                role="dialog"
                aria-modal="true"
                aria-labelledby=labelled_by
                aria-hidden="false"
                on:click=move |_| on_close.run(())
            >
                <div
                    class=dialog_class.clone()
                    role="document"
                    on:click=move |ev: MouseEvent| ev.stop_propagation()
                >
                    <h2 id=labelled_by class="d-modal__header">
                        {title}
                    </h2>
                    {move || {
                        subtitle.get().filter(|text| !text.is_empty()).map(|text| {
                            view! { <p class="m-0 text-xs text-fg-muted">{text}</p> }
                        })
                    }}
                    <button
                        type="button"
                        class="d-modal__close d-btn d-btn--circle"
                        aria-label="Close"
                        on:click=move |_| on_close.run(())
                    >
                        <DtIconClose />
                    </button>
                    <div class="d-modal__content max-w-none">{children()}</div>
                    {footer.as_ref().map(|slot| {
                        view! { <footer class="d-modal__footer">{slot.run()}</footer> }
                    })}
                </div>
                {overlay.as_ref().map(|slot| slot.run())}
            </div>
        </Show>
    }
}
