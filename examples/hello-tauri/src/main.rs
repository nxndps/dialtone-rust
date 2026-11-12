use dialtone::{
    DtButton, DtCard, DtIconClose, DtIconVerySatisfied, DtIllustration, DtStack, DtText,
};
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let open = RwSignal::new(false);
    view! {
        <main class="grid min-h-dvh place-items-center">
            {move || {
                if open.get() {
                    let close = Callback::new(move |_| open.set(false));
                    view! { <Howdy on_close=close /> }.into_any()
                } else {
                    let hello_icon = ViewFn::from(|| view! { <DtIconVerySatisfied size="300" /> });
                    view! {
                        <DtButton
                            on_click=move |_| open.set(true)
                            start_icon=Some(hello_icon)
                        >
                            "Say Hello"
                        </DtButton>
                    }
                    .into_any()
                }
            }}
        </main>
    }
}

#[component]
fn Howdy(on_close: Callback<()>) -> impl IntoView {
    let on_close = StoredValue::new(on_close);
    let header = ViewFn::from(move || {
        let bye_icon = ViewFn::from(|| view! { <DtIconClose size="300" /> });
        view! {
            <DtButton
                importance="clear"
                circle=true
                aria_label="Bye"
                on_click=move |_| on_close.get_value().run(())
                icon=bye_icon
            />
        }
    });
    view! {
        <DtCard header_class="d-jc-end" header=header>
            <DtStack gap="400" align="center">
                <DtIllustration name="new-meeting" />
                <DtText r#as="h1" variant="headline-lg" align="center">
                    "Howdy!"
                </DtText>
            </DtStack>
        </DtCard>
    }
}
