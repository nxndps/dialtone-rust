//! Copy control for playground, Show Code cards, and article code samples.
//!
//! The button matches the Dialtone docs code card: a copy icon and "Copy"
//! at the bottom right, then "Copied!" after the clipboard write succeeds.

use std::sync::Once;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

const COPY_LABEL: &str = "Copy";
const COPIED_LABEL: &str = "Copied!";

pub(super) fn copy_button_markup() -> &'static str {
    r#"<div class="docs-copy-row"><button type="button" class="docs-copy" data-docs-copy aria-label="Copy code"><svg class="docs-copy-icon docs-copy-icon--copy" viewBox="0 0 12 12" width="12" height="12" aria-hidden="true" focusable="false"><path fill="currentColor" fill-rule="evenodd" d="M2 1.5a.5.5 0 0 0-.5.5v4.5A.5.5 0 0 0 2 7h.5a.5.5 0 0 1 0 1H2A1.5 1.5 0 0 1 .5 6.5V2A1.5 1.5 0 0 1 2 .5h4.5A1.5 1.5 0 0 1 8 2v.5a.5.5 0 0 1-1 0V2a.5.5 0 0 0-.5-.5H2ZM5.5 5a.5.5 0 0 0-.5.5V10a.5.5 0 0 0 .5.5H10a.5.5 0 0 0 .5-.5V5.5A.5.5 0 0 0 10 5H5.5ZM4 5.5A1.5 1.5 0 0 1 5.5 4H10a1.5 1.5 0 0 1 1.5 1.5V10a1.5 1.5 0 0 1-1.5 1.5H5.5A1.5 1.5 0 0 1 4 10V5.5Z" clip-rule="evenodd"></path></svg><svg class="docs-copy-icon docs-copy-icon--check" viewBox="0 0 12 12" width="12" height="12" aria-hidden="true" focusable="false"><path fill="currentColor" fill-rule="evenodd" d="M10.354 2.646a.5.5 0 0 1 0 .708l-5.5 5.5a.5.5 0 0 1-.708 0l-2.5-2.5a.5.5 0 1 1 .708-.708L4.5 7.793l5.146-5.147a.5.5 0 0 1 .708 0Z" clip-rule="evenodd"></path></svg><span class="docs-copy-label">Copy</span></button></div>"#
}

pub(crate) fn install_copy_listener() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let Some(document) = web_sys::window().and_then(|window| window.document()) else {
            return;
        };
        let closure = Closure::<dyn FnMut(web_sys::Event)>::new(|event: web_sys::Event| {
            on_copy_click(&event);
        });
        let _ =
            document.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
        closure.forget();
    });
}

fn on_copy_click(event: &web_sys::Event) {
    let Some(button) = copy_button(event) else {
        return;
    };
    let Some(text) = code_text(&button) else {
        return;
    };
    let Some(promise) = clipboard_write(&text) else {
        leptos::logging::warn!("Clipboard is not available");
        return;
    };
    let button_ok = button.clone();
    let copied = Closure::<dyn FnMut(JsValue)>::new(move |_| show_copied(&button_ok));
    let failed = Closure::<dyn FnMut(JsValue)>::new(|_| {
        leptos::logging::warn!("Copy failed");
    });
    let _ = promise.then2(&copied, &failed);
    copied.forget();
    failed.forget();
}

fn copy_button(event: &web_sys::Event) -> Option<web_sys::Element> {
    let target = event.target()?;
    let element = target.dyn_ref::<web_sys::Element>().cloned().or_else(|| {
        target
            .dyn_ref::<web_sys::Node>()
            .and_then(|node| node.parent_element())
    })?;
    element.closest("[data-docs-copy]").ok().flatten()
}

fn code_text(button: &web_sys::Element) -> Option<String> {
    let shell = button.closest(".docs-code-shell").ok().flatten()?;
    let code = shell.query_selector("code").ok().flatten()?;
    let text = code.text_content().unwrap_or_default();
    if text.trim().is_empty() {
        None
    } else {
        Some(text)
    }
}

fn clipboard_write(text: &str) -> Option<js_sys::Promise> {
    let window = web_sys::window()?;
    let navigator = js_sys::Reflect::get(window.as_ref(), &JsValue::from_str("navigator")).ok()?;
    let clipboard = js_sys::Reflect::get(&navigator, &JsValue::from_str("clipboard")).ok()?;
    if clipboard.is_null() || clipboard.is_undefined() {
        return None;
    }
    let write_text = js_sys::Reflect::get(&clipboard, &JsValue::from_str("writeText")).ok()?;
    let function = write_text.dyn_into::<js_sys::Function>().ok()?;
    function
        .call1(&clipboard, &JsValue::from_str(text))
        .ok()?
        .dyn_into()
        .ok()
}

fn show_copied(button: &web_sys::Element) {
    let _ = button.class_list().add_1("docs-copy--done");
    set_label(button, COPIED_LABEL, "Copied");
    let generation = button
        .get_attribute("data-copy-gen")
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(0)
        .wrapping_add(1);
    let _ = button.set_attribute("data-copy-gen", &generation.to_string());
    let Some(window) = web_sys::window() else {
        return;
    };
    let button = button.clone();
    let reset = Closure::once(move || {
        let current = button
            .get_attribute("data-copy-gen")
            .and_then(|value| value.parse::<u32>().ok());
        if current != Some(generation) {
            return;
        }
        let _ = button.class_list().remove_1("docs-copy--done");
        set_label(&button, COPY_LABEL, "Copy code");
    });
    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
        reset.as_ref().unchecked_ref(),
        2000,
    );
    reset.forget();
}

fn set_label(button: &web_sys::Element, text: &str, aria_label: &str) {
    if let Ok(Some(label)) = button.query_selector(".docs-copy-label") {
        label.set_text_content(Some(text));
    }
    let _ = button.set_attribute("aria-label", aria_label);
}
