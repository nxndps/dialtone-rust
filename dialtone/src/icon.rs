use leptos::prelude::*;

fn icon_classes(name: &str, size: &str, class: &str) -> String {
    let base = format!("d-icon d-icon--{name} d-icon--size-{size}");
    let extra = class.trim();
    if extra.is_empty() {
        base
    } else {
        format!("{base} {extra}")
    }
}

/// Dialtone `DtIcon`: 12×12 viewBox, fill-based paths, `currentColor`.
/// Sizing uses the Dialtone scale (`d-icon--size-300` in buttons).
/// Vue icons default `data-qa` to `dt-icon`.
#[component]
pub fn DtIcon(
    /// CSS name, e.g. `plus` → `d-icon--plus`
    name: &'static str,
    /// Accessible `data-name`, e.g. `Plus`
    data_name: &'static str,
    /// Dialtone size token without the prefix, e.g. `"300"`.
    #[prop(optional, default = "300")]
    size: &'static str,
    /// Extra class on the svg. `DtKeyboardShortcut` passes `d-keyboard-shortcut__icon`.
    #[prop(optional, into)]
    class: String,
    children: Children,
) -> impl IntoView {
    let class_name = icon_classes(name, size, &class);
    view! {
        <svg
            class=class_name
            width="12"
            height="12"
            viewBox="0 0 12 12"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            aria-hidden="true"
            focusable="false"
            role="img"
            data-qa="dt-icon"
            attr:data-name=data_name
        >
            {children()}
        </svg>
    }
}

#[component]
pub fn DtIconPlus(
    #[prop(optional, default = "300")] size: &'static str,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <DtIcon name="plus" data_name="Plus" size=size class=class>
            <path
                fill="currentColor"
                d="M6 2a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 1 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 6 2Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconRefresh() -> impl IntoView {
    view! {
        <DtIcon name="refresh" data_name="Refresh">
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M10.5.5a.5.5 0 0 1 .5.5v3a.5.5 0 0 1-.5.5h-3a.5.5 0 0 1 0-1h1.697l-.529-.476a4 4 0 1 0 1.335 3.424.5.5 0 0 1 .995.105A5 5 0 1 1 9.33 2.275l.004.004.665.598V1a.5.5 0 0 1 .5-.5Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconSave() -> impl IntoView {
    view! {
        <DtIcon name="save" data_name="Save">
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M2.5 2a.5.5 0 0 0-.5.5v7a.5.5 0 0 0 .5.5H3V6.5a.5.5 0 0 1 .5-.5h5a.5.5 0 0 1 .5.5V10h.5a.5.5 0 0 0 .5-.5V4.207L7.793 2H4v1.5h3.5a.5.5 0 0 1 0 1h-4A.5.5 0 0 1 3 4V2h-.5Zm0-1A1.5 1.5 0 0 0 1 2.5v7A1.5 1.5 0 0 0 2.5 11h7A1.5 1.5 0 0 0 11 9.5V4a.5.5 0 0 0-.146-.354l-2.5-2.5A.5.5 0 0 0 8 1H2.5ZM8 10V7H4v3h4Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconMoreVertical(#[prop(optional, default = "300")] size: &'static str) -> impl IntoView {
    view! {
        <DtIcon name="more-vertical" data_name="More Vertical" size=size>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M5 2.5a1 1 0 1 1 2 0 1 1 0 0 1-2 0ZM5 6a1 1 0 1 1 2 0 1 1 0 0 1-2 0Zm0 3.5a1 1 0 1 1 2 0 1 1 0 0 1-2 0Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconClose(
    #[prop(optional, default = "300")] size: &'static str,
    /// Extra class on the svg. `DtImageViewer` passes `d-image-viewer__close-button`.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    view! {
        <DtIcon name="close" data_name="Close" size=size class=class>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M2.646 2.646a.5.5 0 0 1 .708 0L6 5.293l2.646-2.647a.5.5 0 1 1 .708.708L6.707 6l2.647 2.646a.5.5 0 1 1-.708.708L6 6.707 3.354 9.354a.5.5 0 1 1-.708-.708L5.293 6 2.646 3.354a.5.5 0 0 1 0-.708Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconXCircle(#[prop(optional, default = "300")] size: &'static str) -> impl IntoView {
    view! {
        <DtIcon name="x-circle" data_name="X Circle" size=size>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M6 1.5a4.5 4.5 0 1 0 0 9 4.5 4.5 0 0 0 0-9ZM.5 6a5.5 5.5 0 1 1 11 0 5.5 5.5 0 0 1-11 0Z"
            />
            <path
                fill="currentColor"
                d="M7.854 4.854a.5.5 0 0 0-.707-.707L6 5.293 4.854 4.147a.5.5 0 0 0-.707.707L5.293 6 4.147 7.147a.5.5 0 0 0 .707.707L6 6.707l1.147 1.147a.5.5 0 0 0 .707-.707L6.707 6l1.147-1.146Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconSatisfied(#[prop(optional, default = "300")] size: &'static str) -> impl IntoView {
    view! {
        <DtIcon name="satisfied" data_name="Satisfied" size=size>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M6 1.5a4.5 4.5 0 1 0 0 9 4.5 4.5 0 0 0 0-9ZM.5 6a5.5 5.5 0 1 1 11 0 5.5 5.5 0 0 1-11 0ZM4 4.5a.5.5 0 0 1 .5-.5h.005a.5.5 0 0 1 0 1H4.5a.5.5 0 0 1-.5-.5Zm3 0a.5.5 0 0 1 .5-.5h.005a.5.5 0 0 1 0 1H7.5a.5.5 0 0 1-.5-.5ZM3.7 6.6a.5.5 0 0 1 .698.098L4.4 6.7 4 7l.4-.3v-.001.002l.018.02a2.474 2.474 0 0 0 .408.374c.288.21.687.405 1.174.405.487 0 .886-.195 1.175-.404a2.474 2.474 0 0 0 .424-.395l.003-.003A.5.5 0 0 1 8.4 7.3L8 7l.4.3-.001.001-.001.002-.004.005-.01.014c-.01.01-.021.025-.036.042a3.474 3.474 0 0 1-.585.54c-.4.29-1 .596-1.763.596a2.994 2.994 0 0 1-1.763-.596 3.474 3.474 0 0 1-.62-.582l-.01-.014-.005-.005v-.002H3.6L4 7l-.4.3a.5.5 0 0 1 .1-.7Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconVerySatisfied(#[prop(optional, default = "300")] size: &'static str) -> impl IntoView {
    view! {
        <DtIcon name="very-satisfied" data_name="Very Satisfied" size=size>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M6 1.5a4.5 4.5 0 1 0 0 9 4.5 4.5 0 0 0 0-9ZM.5 6a5.5 5.5 0 1 1 11 0 5.5 5.5 0 0 1-11 0ZM4 4.5a.5.5 0 0 1 .5-.5h.005a.5.5 0 0 1 0 1H4.5a.5.5 0 0 1-.5-.5Zm3 0a.5.5 0 0 1 .5-.5h.005a.5.5 0 0 1 0 1H7.5a.5.5 0 0 1-.5-.5ZM2.618 6.177A.5.5 0 0 1 3 6h6a.5.5 0 0 1 .493.583A3.5 3.5 0 0 1 6 9.5a3.5 3.5 0 0 1-3.493-2.917.5.5 0 0 1 .111-.406ZM3.667 7a2.5 2.5 0 0 0 2.326 1.5h.014A2.5 2.5 0 0 0 8.333 7H3.667Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconCheck(#[prop(optional, default = "300")] size: &'static str) -> impl IntoView {
    view! {
        <DtIcon name="check" data_name="Check" size=size>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M10.354 2.646a.5.5 0 0 1 0 .708l-5.5 5.5a.5.5 0 0 1-.708 0l-2.5-2.5a.5.5 0 1 1 .708-.708L4.5 7.793l5.146-5.147a.5.5 0 0 1 .708 0Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconArrowLeftRight() -> impl IntoView {
    view! {
        <DtIcon name="arrow-left-right" data_name="Arrow Left Right">
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M8.146 1.146a.5.5 0 0 1 .708 0l2 2a.5.5 0 0 1 0 .708l-2 2a.5.5 0 1 1-.708-.708L9.293 4H4.5a.5.5 0 0 1 0-1h4.793L8.146 1.854a.5.5 0 0 1 0-.708Zm-4.292 5a.5.5 0 0 1 0 .708L2.707 8H7.5a.5.5 0 0 1 0 1H2.707l1.147 1.146a.5.5 0 0 1-.708.708l-2-2a.5.5 0 0 1 0-.708l2-2a.5.5 0 0 1 .708 0Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconConnectionApi() -> impl IntoView {
    view! {
        <DtIcon name="connection-api" data_name="Connection Api">
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M3.709 10.416a.5.5 0 0 1 0-.707L5.23 8.188 4.062 7.02l-1.52 1.52a.5.5 0 1 1-.707-.707l1.521-1.521-.584-.584a.5.5 0 0 1 0-.708l1.562-1.56a2.268 2.268 0 0 1 3.207 0l.271.271 1.21-1.209a.5.5 0 1 1 .707.708l-1.21 1.209.272.271a2.268 2.268 0 0 1 0 3.207L7.229 9.48a.5.5 0 0 1-.708 0l-.583-.584-1.522 1.521a.5.5 0 0 1-.707 0Zm3.125-6.25a1.268 1.268 0 0 0-1.793 0L3.832 5.375l3.043 3.043 1.209-1.209a1.268 1.268 0 0 0 0-1.793l-1.25-1.25Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconInfo(#[prop(optional, default = "300")] size: &'static str) -> impl IntoView {
    view! {
        <DtIcon name="info" data_name="Info" size=size>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M6 1.5a4.5 4.5 0 1 0 0 9 4.5 4.5 0 0 0 0-9ZM.5 6a5.5 5.5 0 1 1 11 0 5.5 5.5 0 0 1-11 0Zm5-2a.5.5 0 0 1 .5-.5h.005a.5.5 0 0 1 0 1H6a.5.5 0 0 1-.5-.5ZM6 5.5a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconBell(#[prop(optional, default = "300")] size: &'static str) -> impl IntoView {
    view! {
        <DtIcon name="bell" data_name="Bell" size=size>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M4.884 10.068a.5.5 0 0 1 .684.181.5.5 0 0 0 .865 0 .5.5 0 0 1 .865.502 1.5 1.5 0 0 1-2.595 0 .5.5 0 0 1 .181-.683ZM6 .5A3.5 3.5 0 0 0 2.5 4c0 1.676-.359 2.717-.689 3.323a2.968 2.968 0 0 1-.44.622 1.51 1.51 0 0 1-.153.142h-.001a.515.515 0 0 0-.16.18.498.498 0 0 0-.033.387A.5.5 0 0 0 1.5 9h9a.5.5 0 0 0 .283-.912l-.001-.001a1.507 1.507 0 0 1-.153-.142 2.97 2.97 0 0 1-.44-.622C9.859 6.717 9.5 5.676 9.5 4A3.5 3.5 0 0 0 6 .5Zm0 1A2.5 2.5 0 0 0 3.5 4c0 1.824-.391 3.033-.811 3.802-.038.07-.076.136-.114.198h6.85a4.705 4.705 0 0 1-.114-.198C8.891 7.032 8.5 5.824 8.5 4A2.5 2.5 0 0 0 6 1.5ZM1.057 8.268Zm9.886 0a.504.504 0 0 0-.161-.181h.001c.07.049.124.111.16.182Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconAlertCircle(#[prop(optional, default = "300")] size: &'static str) -> impl IntoView {
    view! {
        <DtIcon name="alert-circle" data_name="Alert Circle" size=size>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M6 1.5a4.5 4.5 0 1 0 0 9 4.5 4.5 0 0 0 0-9ZM.5 6a5.5 5.5 0 1 1 11 0 5.5 5.5 0 0 1-11 0ZM6 3.5a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-1 0V4a.5.5 0 0 1 .5-.5ZM5.5 8a.5.5 0 0 1 .5-.5h.005a.5.5 0 0 1 0 1H6a.5.5 0 0 1-.5-.5Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconAlertTriangle(#[prop(optional, default = "300")] size: &'static str) -> impl IntoView {
    view! {
        <DtIcon name="alert-triangle" data_name="Alert Triangle" size=size>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M5.24 1.197a1.5 1.5 0 0 1 2.06.556l3.998 6.997a1.504 1.504 0 0 1 0 1.5 1.5 1.5 0 0 1-1.297.75H2.002a1.5 1.5 0 0 1-1.31-2.249V8.75L4.69 1.753a1.5 1.5 0 0 1 .55-.556Zm.755.796a.5.5 0 0 0-.435.254v.001L1.557 9.25a.5.5 0 0 0 .438.75H10a.5.5 0 0 0 .432-.75l-.001-.002-4-7-.001-.001a.5.5 0 0 0-.435-.254Z"
            />
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M6 4a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-1 0v-2A.5.5 0 0 1 6 4Zm-.5 4.5A.5.5 0 0 1 6 8h.005a.5.5 0 0 1 0 1H6a.5.5 0 0 1-.5-.5Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconCheckCircle(#[prop(optional, default = "300")] size: &'static str) -> impl IntoView {
    view! {
        <DtIcon name="check-circle" data_name="Check Circle" size=size>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M.5 6a5.5 5.5 0 1 1 11 0 5.5 5.5 0 0 1-11 0ZM6 1.5a4.5 4.5 0 1 0 0 9 4.5 4.5 0 0 0 0-9Zm1.854 3.146a.5.5 0 0 1 0 .708l-2 2a.5.5 0 0 1-.708 0l-1-1a.5.5 0 1 1 .708-.708l.646.647 1.646-1.647a.5.5 0 0 1 .708 0Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconUser() -> impl IntoView {
    view! {
        <DtIcon name="user" data_name="User">
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M6 2a1.5 1.5 0 1 0 0 3 1.5 1.5 0 0 0 0-3ZM3.5 3.5a2.5 2.5 0 1 1 5 0 2.5 2.5 0 0 1-5 0Zm-.768 4.232A2.5 2.5 0 0 1 4.5 7h3A2.5 2.5 0 0 1 10 9.5v1a.5.5 0 0 1-1 0v-1A1.5 1.5 0 0 0 7.5 8h-3A1.5 1.5 0 0 0 3 9.5v1a.5.5 0 0 1-1 0v-1a2.5 2.5 0 0 1 .732-1.768Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconUsers() -> impl IntoView {
    view! {
        <DtIcon name="users" data_name="Users">
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M4.5 2a1.5 1.5 0 1 0 0 3 1.5 1.5 0 0 0 0-3ZM2 3.5a2.5 2.5 0 1 1 5 0 2.5 2.5 0 0 1-5 0Zm5.516-2.059a.5.5 0 0 1 .608-.36 2.5 2.5 0 0 1 0 4.843.5.5 0 0 1-.248-.968 1.5 1.5 0 0 0 0-2.907.5.5 0 0 1-.36-.608ZM1.232 7.732A2.5 2.5 0 0 1 3 7h3a2.5 2.5 0 0 1 2.5 2.5v1a.5.5 0 0 1-1 0v-1A1.5 1.5 0 0 0 6 8H3a1.5 1.5 0 0 0-1.5 1.5v1a.5.5 0 0 1-1 0v-1a2.5 2.5 0 0 1 .732-1.768Zm7.784-.292a.5.5 0 0 1 .609-.36A2.5 2.5 0 0 1 11.5 9.5v1a.5.5 0 0 1-1 0v-1a1.5 1.5 0 0 0-1.125-1.45.5.5 0 0 1-.36-.61Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconPhone(#[prop(optional, default = "300")] size: &'static str) -> impl IntoView {
    view! {
        <DtIcon name="phone" data_name="Phone" size=size>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M2.01 2.034a.036.036 0 0 0-.003.016 7.98 7.98 0 0 0 .554 2.508l1.128-.94a.125.125 0 0 0 .041-.127l-.349-1.396A.125.125 0 0 0 3.26 2H2.083a.092.092 0 0 0-.065.023.035.035 0 0 0-.009.011ZM3 5.493l1.33-1.108c.332-.276.476-.718.371-1.136l-.349-1.397A1.125 1.125 0 0 0 3.26 1H2.083c-.577 0-1.106.457-1.074 1.1a9.101 9.101 0 0 0 2.674 5.94c2.217 2.22 4.922 2.773 6.137 2.912.67.077 1.18-.463 1.18-1.077V8.738c0-.516-.352-.966-.853-1.091l-1.439-.359a1.125 1.125 0 0 0-1.068.296L6.394 8.832A8.804 8.804 0 0 1 4.39 7.334 8.226 8.226 0 0 1 3 5.493Zm4.35 3.796a9.892 9.892 0 0 0 2.583.67.05.05 0 0 0 .023-.002.046.046 0 0 0 .017-.01.096.096 0 0 0 .026-.072V8.738a.125.125 0 0 0-.095-.121l-1.439-.359a.125.125 0 0 0-.118.033l-.997.998Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconChevronDown(#[prop(optional, default = "300")] size: &'static str) -> impl IntoView {
    view! {
        <DtIcon name="chevron-down" data_name="Chevron Down" size=size>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M2.646 4.146a.5.5 0 0 1 .708 0L6 6.793l2.646-2.647a.5.5 0 1 1 .708.708l-3 3a.5.5 0 0 1-.708 0l-3-3a.5.5 0 0 1 0-.708Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconChevronLeft(#[prop(optional, default = "300")] size: &'static str) -> impl IntoView {
    view! {
        <DtIcon name="chevron-left" data_name="Chevron Left" size=size>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M7.854 2.646a.5.5 0 0 1 0 .708L5.207 6l2.647 2.646a.5.5 0 1 1-.708.708l-3-3a.5.5 0 0 1 0-.708l3-3a.5.5 0 0 1 .708 0Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconMoreHorizontal(
    #[prop(optional, default = "300")] size: &'static str,
) -> impl IntoView {
    view! {
        <DtIcon name="more-horizontal" data_name="More Horizontal" size=size>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M1.502 6a1 1 0 1 1 2 0 1 1 0 0 1-2 0Zm3.5 0a1 1 0 1 1 2 0 1 1 0 0 1-2 0Zm3.5 0a1 1 0 1 1 2 0 1 1 0 0 1-2 0Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconChevronRight(#[prop(optional, default = "300")] size: &'static str) -> impl IntoView {
    view! {
        <DtIcon name="chevron-right" data_name="Chevron Right" size=size>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M4.146 2.646a.5.5 0 0 1 .708 0l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 1 1-.708-.708L6.793 6 4.146 3.354a.5.5 0 0 1 0-.708Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconChevronsLeft(#[prop(optional, default = "300")] size: &'static str) -> impl IntoView {
    view! {
        <DtIcon name="chevrons-left" data_name="Chevrons Left" size=size>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M5.854 3.146a.5.5 0 0 1 0 .708L3.707 6l2.147 2.146a.5.5 0 1 1-.708.708l-2.5-2.5a.5.5 0 0 1 0-.708l2.5-2.5a.5.5 0 0 1 .708 0Z"
            />
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M9.354 3.146a.5.5 0 0 1 0 .708L7.207 6l2.147 2.146a.5.5 0 1 1-.708.708l-2.5-2.5a.5.5 0 0 1 0-.708l2.5-2.5a.5.5 0 0 1 .708 0Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconChevronsRight(#[prop(optional, default = "300")] size: &'static str) -> impl IntoView {
    view! {
        <DtIcon name="chevrons-right" data_name="Chevrons Right" size=size>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M6.146 3.146a.5.5 0 0 1 .708 0l2.5 2.5a.5.5 0 0 1 0 .708l-2.5 2.5a.5.5 0 1 1-.708-.708L8.293 6 6.146 3.854a.5.5 0 0 1 0-.708Z"
            />
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M2.646 3.146a.5.5 0 0 1 .708 0l2.5 2.5a.5.5 0 0 1 0 .708l-2.5 2.5a.5.5 0 1 1-.708-.708L4.793 6 2.646 3.854a.5.5 0 0 1 0-.708Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconSearch(#[prop(optional, default = "300")] size: &'static str) -> impl IntoView {
    view! {
        <DtIcon name="search" data_name="Search" size=size>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M5.5 1a4.5 4.5 0 1 0 2.81 8.016l1.837 1.838a.5.5 0 1 0 .707-.707L9.016 8.308A4.5 4.5 0 0 0 5.5 1Zm0 1a3.5 3.5 0 1 0 0 7 3.5 3.5 0 0 0 0-7Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconLaptop2(#[prop(optional, default = "300")] size: &'static str) -> impl IntoView {
    view! {
        <DtIcon name="laptop-2" data_name="Laptop 2" size=size>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M.5 2.5A1.5 1.5 0 0 1 2 1h8a1.5 1.5 0 0 1 1.5 1.5v5A1.5 1.5 0 0 1 10 9H2A1.5 1.5 0 0 1 .5 7.5v-5ZM2 2a.5.5 0 0 0-.5.5v5A.5.5 0 0 0 2 8h8a.5.5 0 0 0 .5-.5v-5A.5.5 0 0 0 10 2H2ZM.5 10.5A.5.5 0 0 1 1 10h10a.5.5 0 0 1 0 1H1a.5.5 0 0 1-.5-.5Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconSun(#[prop(optional, default = "300")] size: &'static str) -> impl IntoView {
    view! {
        <DtIcon name="sun" data_name="Sun" size=size>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M6 4.5a1.5 1.5 0 1 0 0 3 1.5 1.5 0 0 0 0-3ZM3.5 6a2.5 2.5 0 1 1 5 0 2.5 2.5 0 0 1-5 0ZM6 .5a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-1 0V1A.5.5 0 0 1 6 .5Zm0 9a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-1 0v-1a.5.5 0 0 1 .5-.5ZM2.111 2.111a.5.5 0 0 1 .708 0l.705.705a.5.5 0 1 1-.708.708l-.705-.705a.5.5 0 0 1 0-.708Zm6.365 6.365a.5.5 0 0 1 .708 0l.705.705a.5.5 0 0 1-.708.708l-.705-.705a.5.5 0 0 1 0-.708ZM.5 6a.5.5 0 0 1 .5-.5h1a.5.5 0 0 1 0 1H1A.5.5 0 0 1 .5 6Zm9 0a.5.5 0 0 1 .5-.5h1a.5.5 0 0 1 0 1h-1a.5.5 0 0 1-.5-.5ZM3.524 8.476a.5.5 0 0 1 0 .708l-.705.705a.5.5 0 0 1-.708-.708l.705-.705a.5.5 0 0 1 .708 0Zm6.365-6.365a.5.5 0 0 1 0 .708l-.705.705a.5.5 0 0 1-.708-.708l.705-.705a.5.5 0 0 1 .708 0Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconMoon(#[prop(optional, default = "300")] size: &'static str) -> impl IntoView {
    view! {
        <DtIcon name="moon" data_name="Moon" size=size>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M4.955 2.139a4 4 0 1 0 4.902 4.92 3.682 3.682 0 0 1-5.281-3.31 3.646 3.646 0 0 1 .38-1.61Zm1.392-.279A.5.5 0 0 0 6 1a5 5 0 1 0 5 5 .5.5 0 0 0-.852-.355 2.682 2.682 0 0 1-4.572-1.898 2.709 2.709 0 0 1 .391-1.38 2.97 2.97 0 0 1 .377-.504l.003-.003Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconSparkle(#[prop(optional, default = "300")] size: &'static str) -> impl IntoView {
    view! {
        <DtIcon name="sparkle" data_name="Sparkle" size=size>
            <path
                fill="currentColor"
                d="M9.264 1c.05 0 .093.034.107.082.23.745.81 1.335 1.545 1.572a.12.12 0 0 1 .084.113.12.12 0 0 1-.084.114 2.39 2.39 0 0 0-1.529 1.522.131.131 0 0 1-.123.092.131.131 0 0 1-.124-.092 2.392 2.392 0 0 0-1.509-1.516.127.127 0 0 1-.087-.12c0-.055.036-.103.087-.12a2.393 2.393 0 0 0 1.525-1.565A.114.114 0 0 1 9.264 1Zm0 6.505c.05 0 .093.034.108.081.23.746.81 1.335 1.544 1.572a.12.12 0 0 1 .084.114.12.12 0 0 1-.084.114 2.39 2.39 0 0 0-1.529 1.522.131.131 0 0 1-.123.092.131.131 0 0 1-.124-.092 2.393 2.393 0 0 0-1.509-1.516.127.127 0 0 1-.087-.12c0-.055.036-.103.087-.12a2.393 2.393 0 0 0 1.525-1.566.114.114 0 0 1 .108-.081ZM4.503 9.239v.27l-.082-.014a.267.267 0 0 1-.13-.097l-.038-.075c-.021-.064-.046-.127-.07-.19l.32.106Zm.25.084-.038.075a.262.262 0 0 1-.212.11v-.27l.318-.105a4.902 4.902 0 0 0-.068.19Zm-.251-4.926A5.882 5.882 0 0 1 2.918 5.99a5.882 5.882 0 0 1 1.584 1.595 5.885 5.885 0 0 1 1.585-1.596 5.883 5.883 0 0 1-1.585-1.592ZM7.7 6.279a4.87 4.87 0 0 0-2.879 2.854l-.318.106-.32-.106a4.875 4.875 0 0 0-2.738-2.802l-.267-.098A.258.258 0 0 1 1 5.99c0-.111.073-.209.178-.243a4.872 4.872 0 0 0 3.106-3.19.231.231 0 0 1 .219-.165c.1 0 .188.068.218.165A4.873 4.873 0 0 0 7.868 5.76c.1.032.169.124.17.23 0 .105-.07.198-.17.23l-.168.06Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconExternalLink(#[prop(optional, default = "300")] size: &'static str) -> impl IntoView {
    view! {
        <DtIcon name="external-link" data_name="External Link" size=size>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M2.5 3.5A.5.5 0 0 0 2 4v5.5a.5.5 0 0 0 .5.5H8a.5.5 0 0 0 .5-.5v-3a.5.5 0 0 1 1 0v3A1.5 1.5 0 0 1 8 11H2.5A1.5 1.5 0 0 1 1 9.5V4a1.5 1.5 0 0 1 1.5-1.5h3a.5.5 0 0 1 0 1h-3Z"
            />
            <path
                fill="currentColor"
                d="M7 1.5a.5.5 0 0 1 .5-.5h3a.5.5 0 0 1 .5.5v3a.5.5 0 0 1-1 0V2.707L5.354 7.354a.5.5 0 1 1-.708-.708L9.293 2H7.5a.5.5 0 0 1-.5-.5Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconLock(#[prop(optional, default = "300")] size: &'static str) -> impl IntoView {
    view! {
        <DtIcon name="lock" data_name="Lock" size=size>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M2.5 5A1.5 1.5 0 0 0 1 6.5V10a1.5 1.5 0 0 0 1.5 1.5h7A1.5 1.5 0 0 0 11 10V6.5A1.5 1.5 0 0 0 9.5 5H9V3.5a3 3 0 1 0-6 0V5h1V3.5a2 2 0 0 1 4 0V5H2.5Zm0 1a.5.5 0 0 0-.5.5V10a.5.5 0 0 0 .5.5h7a.5.5 0 0 0 .5-.5V6.5a.5.5 0 0 0-.5-.5h-7Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconShare(#[prop(optional, default = "300")] size: &'static str) -> impl IntoView {
    view! {
        <DtIcon name="share" data_name="Share" size=size>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M2 5.5a.5.5 0 0 1 .5.5v4a.5.5 0 0 0 .5.5h6a.5.5 0 0 0 .5-.5V6a.5.5 0 1 1 1 0v4A1.5 1.5 0 0 1 9 11.5H3A1.5 1.5 0 0 1 1.5 10V6a.5.5 0 0 1 .5-.5Z"
            />
            <path
                fill="currentColor"
                d="M6.5 7.5a.5.5 0 1 1-1 0V2.207L4.354 3.354a.5.5 0 0 1-.707-.707l2-2a.5.5 0 0 1 .707 0l2 2a.5.5 0 0 1-.707.707L6.5 2.207V7.5Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconShare2(#[prop(optional, default = "300")] size: &'static str) -> impl IntoView {
    view! {
        <DtIcon name="share-2" data_name="Share 2" size=size>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M7 2.5a2 2 0 1 1 .579 1.407l-2.654 1.55a2.001 2.001 0 0 1 0 1.087L7.581 8.09a2 2 0 1 1-.505.863L4.42 7.407a2 2 0 1 1 0-2.815l2.654-1.548A2.001 2.001 0 0 1 7 2.5Zm1 0a1 1 0 1 1 2 0 1 1 0 0 1-2 0ZM2 6a1 1 0 1 1 2 0 1 1 0 0 1-2 0Zm6 3.5a1 1 0 1 1 2 0 1 1 0 0 1-2 0Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconStar(#[prop(optional, default = "300")] size: &'static str) -> impl IntoView {
    view! {
        <DtIcon name="star" data_name="Star" size=size>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M6 .5a.5.5 0 0 1 .448.279l1.43 2.894 3.194.467a.5.5 0 0 1 .277.853L9.037 7.245l.546 3.18a.5.5 0 0 1-.726.527L6 9.45l-2.857 1.503a.5.5 0 0 1-.726-.527l.546-3.181L.65 4.993a.5.5 0 0 1 .277-.853l3.195-.467L5.552.78A.5.5 0 0 1 6 .5Zm0 1.63L4.903 4.35a.5.5 0 0 1-.376.274l-2.453.358 1.776 1.73a.5.5 0 0 1 .144.443l-.419 2.44 2.193-1.153a.5.5 0 0 1 .466 0l2.193 1.154-.419-2.441a.5.5 0 0 1 .144-.443l1.775-1.729-2.453-.358a.5.5 0 0 1-.376-.274L6 2.13Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconCommand(
    #[prop(optional, default = "300")] size: &'static str,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <DtIcon name="command" data_name="Command" size=size class=class>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M1.586 1.586A2 2 0 0 1 5 3v1h2V3a2 2 0 1 1 2 2H8v2h1a2 2 0 1 1-2 2V8H5v1a2 2 0 1 1-2-2h1V5H3a2 2 0 0 1-1.414-3.414ZM4 4V3a1 1 0 1 0-1 1h1Zm1 1v2h2V5H5ZM4 8H3a1 1 0 1 0 1 1V8Zm4 0v1a1 1 0 1 0 1-1H8Zm0-4h1a1 1 0 1 0-1-1v1Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconOption(
    #[prop(optional, default = "300")] size: &'static str,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <DtIcon name="option" data_name="Option" size=size class=class>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M1 1.5a.5.5 0 0 1 .5-.5h3a.5.5 0 0 1 .475.342L7.86 10h2.64a.5.5 0 1 1 0 1h-3a.5.5 0 0 1-.475-.342L4.14 2H1.5a.5.5 0 0 1-.5-.5Zm5.5 0A.5.5 0 0 1 7 1h3.5a.5.5 0 1 1 0 1H7a.5.5 0 0 1-.5-.5Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconLayoutGrid(
    #[prop(optional, default = "300")] size: &'static str,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <DtIcon name="layout-grid" data_name="Layout Grid" size=size class=class>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M1 1.5a.5.5 0 0 1 .5-.5H5a.5.5 0 0 1 .5.5V5a.5.5 0 0 1-.5.5H1.5A.5.5 0 0 1 1 5V1.5ZM2 2v2.5h2.5V2H2Zm4.5-.5A.5.5 0 0 1 7 1h3.5a.5.5 0 0 1 .5.5V5a.5.5 0 0 1-.5.5H7a.5.5 0 0 1-.5-.5V1.5Zm1 .5v2.5H10V2H7.5Zm-1 5a.5.5 0 0 1 .5-.5h3.5a.5.5 0 0 1 .5.5v3.5a.5.5 0 0 1-.5.5H7a.5.5 0 0 1-.5-.5V7Zm1 .5V10H10V7.5H7.5ZM1 7a.5.5 0 0 1 .5-.5H5a.5.5 0 0 1 .5.5v3.5a.5.5 0 0 1-.5.5H1.5a.5.5 0 0 1-.5-.5V7Zm1 .5V10h2.5V7.5H2Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconArrowUp(
    #[prop(optional, default = "300")] size: &'static str,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <DtIcon name="arrow-up" data_name="Arrow Up" size=size class=class>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M5.646 2.146a.5.5 0 0 1 .708 0l3.5 3.5a.5.5 0 0 1-.708.708L6.5 3.707V9.5a.5.5 0 0 1-1 0V3.707L2.854 6.354a.5.5 0 1 1-.708-.708l3.5-3.5Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconArrowDown(
    #[prop(optional, default = "300")] size: &'static str,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <DtIcon name="arrow-down" data_name="Arrow Down" size=size class=class>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M6 2a.5.5 0 0 1 .5.5v5.793l2.646-2.647a.5.5 0 1 1 .708.708l-3.5 3.5a.5.5 0 0 1-.708 0l-3.5-3.5a.5.5 0 1 1 .708-.708L5.5 8.293V2.5A.5.5 0 0 1 6 2Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconArrowLeft(
    #[prop(optional, default = "300")] size: &'static str,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <DtIcon name="arrow-left" data_name="Arrow Left" size=size class=class>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M6.354 2.146a.5.5 0 0 1 0 .708L3.707 5.5H9.5a.5.5 0 0 1 0 1H3.707l2.647 2.646a.5.5 0 1 1-.708.708l-3.5-3.5a.5.5 0 0 1 0-.708l3.5-3.5a.5.5 0 0 1 .708 0Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconArrowRight(
    #[prop(optional, default = "300")] size: &'static str,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <DtIcon name="arrow-right" data_name="Arrow Right" size=size class=class>
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M5.646 2.146a.5.5 0 0 1 .708 0l3.5 3.5a.5.5 0 0 1 0 .708l-3.5 3.5a.5.5 0 0 1-.708-.708L8.293 6.5H2.5a.5.5 0 0 1 0-1h5.793L5.646 2.854a.5.5 0 0 1 0-.708Z"
            />
        </DtIcon>
    }
}

#[component]
pub fn DtIconTrash() -> impl IntoView {
    view! {
        <DtIcon name="trash" data_name="Trash">
            <path
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M5 5a.5.5 0 0 1 .5.5v3a.5.5 0 1 1-1 0v-3A.5.5 0 0 1 5 5Zm2 0a.5.5 0 0 1 .5.5v3a.5.5 0 1 1-1 0v-3A.5.5 0 0 1 7 5Z M1.5 2.5a.5.5 0 0 0 0 1H2V10c0 .413.201.784.46 1.041.257.258.628.46 1.04.46h5c.413 0 .784-.202 1.041-.46.258-.258.46-.628.46-1.04V3.5h.5a.5.5 0 0 0 0-1h-2V2c0-.413-.202-.784-.46-1.042C7.784.701 7.413.5 7.001.5H5c-.413 0-.784.201-1.042.46-.258.257-.459.628-.459 1.04v.5h-2ZM3 10a.51.51 0 0 0 .166.334.51.51 0 0 0 .334.166h5a.51.51 0 0 0 .334-.166A.511.511 0 0 0 9 10V3.5H3V10Zm1.5-8a.51.51 0 0 1 .166-.334A.51.51 0 0 1 5 1.5h2a.51.51 0 0 1 .334.166A.51.51 0 0 1 7.5 2v.5h-3V2Z"
            />
        </DtIcon>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_and_extra_class_follow_the_vue_icon() {
        assert_eq!(
            icon_classes("plus", "300", ""),
            "d-icon d-icon--plus d-icon--size-300"
        );
        assert_eq!(
            icon_classes("plus", "100", "d-keyboard-shortcut__icon"),
            "d-icon d-icon--plus d-icon--size-100 d-keyboard-shortcut__icon"
        );
        assert_eq!(
            icon_classes(
                "command",
                "100",
                "d-keyboard-shortcut__icon d-keyboard-shortcut__icon--inverted"
            ),
            "d-icon d-icon--command d-icon--size-100 d-keyboard-shortcut__icon d-keyboard-shortcut__icon--inverted"
        );
    }
}
