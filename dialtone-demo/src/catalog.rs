use crate::docs::ShowCodeText;
use dialtone::{
    DtAvatar, DtBadge, DtBanner, DtBox, DtBreadcrumb, DtBreadcrumbs, DtButton, DtButtonGroup,
    DtCard, DtCheckbox, DtCheckboxGroup, DtChip, DtCodeblock, DtCollapsible, DtCombobox,
    DtComboboxMultiSelect, DtComboboxWithPopover, DtDatepicker, DtDescriptionList,
    DtDescriptionListItem, DtDropdown, DtDropdownSeparator, DtEmoji, DtEmojiPicker,
    DtEmojiSelection, DtEmojiTextWrapper, DtEmptyState, DtFilter, DtFilterPill, DtHovercard,
    DtIconAlertCircle, DtIconAlertTriangle, DtIconArrowLeftRight, DtIconBell, DtIconCheck,
    DtIconCheckCircle, DtIconChevronDown, DtIconChevronLeft, DtIconChevronRight,
    DtIconChevronsLeft, DtIconChevronsRight, DtIconClose, DtIconConnectionApi, DtIconExternalLink,
    DtIconInfo, DtIconLaptop2, DtIconMoon, DtIconMoreHorizontal, DtIconMoreVertical, DtIconPlus,
    DtIconRefresh, DtIconSatisfied, DtIconSave, DtIconSearch, DtIconSparkle, DtIconSun,
    DtIconTrash, DtIconUser, DtIconUsers, DtIconVerySatisfied, DtIconXCircle, DtIllustration,
    DtImageViewer, DtInput, DtInputGroup, DtItemLayout, DtKeyboardShortcut, DtLazyShow, DtLink,
    DtListItem, DtListItemGroup, DtLoader, DtModal, DtModeIsland, DtMotionText, DtNotice,
    DtPagination, DtPopover, DtPresence, DtProgressCircle, DtProse, DtRadio, DtRadioGroup,
    DtResizable, DtResizableHandle, DtResizablePanel, DtRichTextEditor, DtRootLayout, DtScroller,
    DtScrollerItem, DtSegmentedControl, DtSegmentedControlItem, DtSelectMenu, DtSelectOption,
    DtSkeleton, DtSkeletonListItemOption, DtSkeletonParagraphOption, DtSkeletonShapeKind,
    DtSkeletonShapeOption, DtSlider, DtSliderValue, DtSplitButton, DtStack, DtTab, DtTabGroup,
    DtTabGroupChange, DtTabPanel, DtTable, DtText, DtTextList, DtTextListItem, DtToast, DtToggle,
    DtTooltip, DtValidationMessage, DtValidationMessages,
};
use leptos::prelude::*;

#[component]
fn Example(
    id: &'static str,
    title: &'static str,
    summary: &'static str,
    code: &'static str,
    #[prop(optional)] stack: bool,
    children: ChildrenFn,
) -> impl IntoView {
    let preview = if stack {
        "demo-preview demo-preview--stack"
    } else {
        "demo-preview"
    };
    view! {
        <section class="scroll-mt-6" data-example=id>
            <h2>{title}</h2>
            <p class="demo-summary">{summary}</p>
            <div class=preview>{children()}</div>
            <ShowCodeText text=code />
        </section>
    }
}

#[component]
pub(crate) fn BreadcrumbsExample() -> impl IntoView {
    view! {
        <Example
            id="breadcrumbs"
            title="Breadcrumbs"
            summary="A trail of links for the current page. The last item is the current location."
            code=r##"<DtBreadcrumbs breadcrumbs=vec![
    DtBreadcrumb::new("#", "Root"),
    DtBreadcrumb::new("#", "Section"),
    DtBreadcrumb::new("#", "Current Page").selected(),
] />"##
        >
            <DtBreadcrumbs breadcrumbs=vec![
                DtBreadcrumb::new("#", "Root"),
                DtBreadcrumb::new("#", "Section"),
                DtBreadcrumb::new("#", "Current Page").selected(),
            ] />
        </Example>
    }
}

#[component]
pub(crate) fn ButtonExample() -> impl IntoView {
    let clicks = RwSignal::new(0u32);
    view! {
        <Example
            id="button"
            title="Button"
            summary="Primary, outlined, clear, link, icon, circle, and loading. The counter updates when the primary button is clicked."
            code=r##"<DtButton on_click=move || clicks.update(|n| *n += 1)>
    {move || format!("Clicked {}", clicks.get())}
</DtButton>
<DtButton importance="outlined" kind="critical">"Remove"</DtButton>
<DtButton
    kind="positive"
    icon=ViewFn::from(|| view! { <DtIconPlus /> })
>
    "Add"
</DtButton>
<DtButton link=true href="#button">"Text link"</DtButton>"##
        >
            <DtButton on_click=move || clicks.update(|count| *count += 1)>
                {move || format!("Clicked {}", clicks.get())}
            </DtButton>
            <DtButton importance="outlined">"Outlined"</DtButton>
            <DtButton importance="clear">"Clear"</DtButton>
            <DtButton importance="outlined" kind="critical">"Remove"</DtButton>
            <DtButton kind="positive" icon=ViewFn::from(|| view! { <DtIconPlus /> })>
                "Add"
            </DtButton>
            <DtButton
                circle=true
                aria_label="Add"
                icon=ViewFn::from(|| view! { <DtIconPlus /> })
            />
            <DtButton link=true href="#button">
                "Text link"
            </DtButton>
            <DtButton loading=true>"Saving"</DtButton>
            <DtButton disabled=true>"Disabled"</DtButton>
        </Example>
    }
}

#[component]
pub(crate) fn ButtonGroupExample() -> impl IntoView {
    let range = RwSignal::new("week");
    view! {
        <Example
            id="button-group"
            title="Button group"
            summary="Groups related buttons. Alignment here is start, and the selected button uses active."
            code=r##"<DtButtonGroup>
    <DtButton importance="outlined" active=true>"Week"</DtButton>
    <DtButton importance="outlined">"Month"</DtButton>
</DtButtonGroup>"##
        >
            {move || {
                let current = range.get();
                view! {
                    <DtButtonGroup>
                        <DtButton
                            importance="outlined"
                            active=current == "day"
                            on_click=move || range.set("day")
                        >
                            "Day"
                        </DtButton>
                        <DtButton
                            importance="outlined"
                            active=current == "week"
                            on_click=move || range.set("week")
                        >
                            "Week"
                        </DtButton>
                        <DtButton
                            importance="outlined"
                            active=current == "month"
                            on_click=move || range.set("month")
                        >
                            "Month"
                        </DtButton>
                    </DtButtonGroup>
                }
            }}
        </Example>
    }
}

#[component]
pub(crate) fn AvatarExample() -> impl IntoView {
    view! {
        <Example
            id="avatar"
            title="Avatar"
            summary="A photo, initials from a name, a presence dot, a group count, and an icon when there is no image."
            code=r##"<DtAvatar full_name="Ada Lovelace" size="sm" presence="active" image_src="/assets/images/person.png" image_alt="Ada Lovelace".to_string() />
<DtAvatar full_name="Grace Hopper" size="lg" presence="away" />
<DtAvatar icon=ViewFn::from(|| view! { <DtIconUser /> }) />"##
        >
            <DtAvatar
                full_name="Ada Lovelace"
                size="sm"
                presence="active"
                image_src="/assets/images/person.png"
                image_alt="Ada Lovelace".to_string()
            />
            <DtAvatar full_name="Grace Hopper" presence="away" />
            <DtAvatar full_name="Alan Turing" size="lg" presence="busy" />
            <DtAvatar full_name="Edsger Dijkstra" size="xl" presence="offline" />
            <DtAvatar full_name="On-call" group=3 />
            <DtAvatar icon=ViewFn::from(|| view! { <DtIconUser /> }) />
        </Example>
    }
}

#[component]
pub(crate) fn PresenceExample() -> impl IntoView {
    view! {
        <Example
            id="presence"
            title="Presence"
            summary="Status indicator for user availability."
            code=r##"<DtPresence presence="active" />
<DtPresence presence="away" />
<DtPresence presence="busy" />
<DtPresence presence="dnd" />
<DtPresence presence="offline" />"##
        >
            <DtPresence presence="active" />
            <DtPresence presence="away" />
            <DtPresence presence="busy" />
            <DtPresence presence="dnd" />
            <DtPresence presence="offline" />
        </Example>
    }
}

#[component]
pub(crate) fn BadgeExample() -> impl IntoView {
    view! {
        <Example
            id="badge"
            title="Badge"
            summary="Label and count badges. Color is the type prop: info, positive, warning, critical, bulletin, or ai."
            code=r##"<DtBadge text="Info" type_="info" />
<DtBadge text="3" kind="count" type_="positive" />
<DtBadge text="AI" type_="ai" />"##
        >
            <DtBadge text="Default" />
            <DtBadge text="Info" type_="info" />
            <DtBadge text="Positive" type_="positive" />
            <DtBadge text="Warning" type_="warning" />
            <DtBadge text="Critical" type_="critical" />
            <DtBadge text="Bulletin" type_="bulletin" />
            <DtBadge text="AI" type_="ai" />
            <DtBadge text="3" kind="count" type_="info" />
            <DtBadge text="Outlined" type_="positive" outlined=true />
        </Example>
    }
}

#[component]
pub(crate) fn CardExample() -> impl IntoView {
    view! {
        <Example
            id="card"
            title="Card"
            summary="Header, content, and footer slots. The footer button is a real DtButton."
            stack=true
            code=r##"<DtCard
    header=ViewFn::from(|| view! { <span>"Schedule"</span> })
    footer=ViewFn::from(|| view! {
        <DtButton importance="outlined" size="sm">"Edit"</DtButton>
    })
>
    <p>"Primary on-call this week."</p>
</DtCard>"##
        >
            <DtCard
                container_class="w-full max-w-md"
                header=ViewFn::from(|| view! { <span>"Schedule"</span> })
                footer=ViewFn::from(|| {
                    view! { <DtButton importance="outlined" size="sm">"Edit"</DtButton> }
                })
            >
                <p class="m-0 text-sm text-fg-muted">"Primary on-call this week."</p>
            </DtCard>
        </Example>
    }
}

#[component]
pub(crate) fn ChipExample() -> impl IntoView {
    let chips = RwSignal::new(vec![
        "Voice".to_string(),
        "SMS".to_string(),
        "Meetings".to_string(),
    ]);
    view! {
        <Example
            id="chip"
            title="Chip"
            summary="Close removes a chip. The parent writes the list in on_close."
            code=r##"<DtChip on_close=move || chips.update(|list| list.retain(|item| item != "Voice"))>
    "Voice"
</DtChip>"##
        >
            {move || {
                chips
                    .get()
                    .into_iter()
                    .map(|label| {
                        let key = label.clone();
                        view! {
                            <DtChip on_close=move || {
                                let key = key.clone();
                                chips.update(|list| list.retain(|item| item != &key));
                            }>
                                {label}
                            </DtChip>
                        }
                    })
                    .collect_view()
            }}
            <Show when=move || chips.get().is_empty()>
                <DtButton
                    size="sm"
                    importance="outlined"
                    on_click=move || {
                        chips.set(vec![
                            "Voice".to_string(),
                            "SMS".to_string(),
                            "Meetings".to_string(),
                        ]);
                    }
                >
                    "Reset chips"
                </DtButton>
            </Show>
        </Example>
    }
}

#[component]
pub(crate) fn CodeblockExample() -> impl IntoView {
    view! {
        <Example
            id="codeblock"
            title="Code block"
            summary="Renders the text prop inside a Dialtone code block. The samples on this page use it."
            stack=true
            code=r##"<DtCodeblock text="<DtButton>\"Save\"</DtButton>" />"##
        >
            <DtCodeblock text="<DtButton>\"Save\"</DtButton>" />
        </Example>
    }
}

#[component]
pub(crate) fn ItemLayoutExample() -> impl IntoView {
    view! {
        <Example
            id="item-layout"
            title="Item layout"
            summary="Title children with left, subtitle, and right slots."
            stack=true
            code=r##"<DtItemLayout
    left=ViewFn::from(|| view! { <DtAvatar full_name="Ada Lovelace" size="sm" image_src="/assets/images/person.png" image_alt="Ada Lovelace".to_string() /> })
    subtitle=ViewFn::from(|| view! { <span>"Primary"</span> })
    right=ViewFn::from(|| view! { <DtBadge text="On call" type_="positive" /> })
>
    "Ada Lovelace"
</DtItemLayout>"##
        >
            <div class="w-full rounded-lg bg-surface">
                <DtItemLayout
                    left=ViewFn::from(|| view! {
                        <DtAvatar
                            full_name="Ada Lovelace"
                            size="sm"
                            image_src="/assets/images/person.png"
                            image_alt="Ada Lovelace".to_string()
                        />
                    })
                    subtitle=ViewFn::from(|| view! { <span>"Primary"</span> })
                    right=ViewFn::from(|| view! { <DtBadge text="On call" type_="positive" /> })
                >
                    "Ada Lovelace"
                </DtItemLayout>
                <DtItemLayout
                    left=ViewFn::from(|| view! { <DtAvatar full_name="Grace Hopper" size="sm" /> })
                    subtitle=ViewFn::from(|| view! { <span>"Secondary"</span> })
                    right=ViewFn::from(|| view! { <DtBadge text="Next" type_="info" /> })
                >
                    "Grace Hopper"
                </DtItemLayout>
            </div>
        </Example>
    }
}

#[component]
pub(crate) fn KeyboardShortcutExample() -> impl IntoView {
    view! {
        <Example
            id="keyboard-shortcut"
            title="Keyboard shortcut"
            summary="A key combination. Plus signs and {cmd} draw icons. The screen reader text is generated."
            code=r##"<DtKeyboardShortcut shortcut="{cmd}+Ctrl+X" />"##
        >
            <DtKeyboardShortcut shortcut="{cmd}+Ctrl+X" />
        </Example>
    }
}

#[component]
pub(crate) fn LazyShowExample() -> impl IntoView {
    let shown = RwSignal::new(false);
    view! {
        <Example
            id="lazy-show"
            title="Lazy show"
            summary="The child mounts the first time it is shown, then stays mounted."
            code=r##"<DtButton on_click=move || shown.set(!shown.get_untracked())>"Toggle"</DtButton>
<DtLazyShow show=shown transition="fade">
    "I'm Lazy!"
</DtLazyShow>"##
        >
            <DtButton on_click=move || shown.set(!shown.get_untracked())>"Toggle"</DtButton>
            <DtLazyShow show=shown transition="fade">
                "I'm Lazy!"
            </DtLazyShow>
        </Example>
    }
}

#[component]
pub(crate) fn LinkExample() -> impl IntoView {
    view! {
        <Example
            id="link"
            title="Link"
            summary="A text link. Tone changes the color. Unstyled drops the link styles."
            code=r##"<DtLink href="#link">"Base link"</DtLink>"##
        >
            <DtLink href="#link">"Base link"</DtLink>
        </Example>
    }
}

#[component]
pub(crate) fn ListItemExample() -> impl IntoView {
    view! {
        <Example
            id="list-item"
            title="List item"
            summary="Structured row for choices, navigation, or actions."
            stack=true
            code=r##"<DtListItem
    navigation_type="tab"
    start=ViewFn::from(|| view! { <DtIconCheck size="300" /> })
    subtitle=ViewFn::from(|| view! { <span>"Description"</span> })
    block_end=ViewFn::from(|| view! { <DtBadge text="Label" /> })
    end=ViewFn::from(|| view! { <DtIconExternalLink size="300" /> })
>
    "Default List Item"
</DtListItem>"##
        >
            <ul class="w-full">
                <DtListItem
                    navigation_type="tab"
                    start=ViewFn::from(|| view! { <DtIconCheck size="300" /> })
                    subtitle=ViewFn::from(|| view! { <span>"Description"</span> })
                    block_end=ViewFn::from(|| view! { <DtBadge text="Label" /> })
                    end=ViewFn::from(|| view! { <DtIconExternalLink size="300" /> })
                >
                    "Default List Item"
                </DtListItem>
            </ul>
        </Example>
    }
}

#[component]
pub(crate) fn ListItemGroupExample() -> impl IntoView {
    view! {
        <Example
            id="list-item-group"
            title="List item group"
            summary="Non-interactive heading that organizes related list items."
            stack=true
            code=r##"<DtListItemGroup heading="Example Heading">
    <DtListItem role="option" navigation_type="tab" class="d-pis-100">"Item 1"</DtListItem>
    <DtListItem role="option" navigation_type="tab" class="d-pis-100">"Item 2"</DtListItem>
    <DtListItem role="option" navigation_type="tab" class="d-pis-100">"Item 3"</DtListItem>
</DtListItemGroup>"##
        >
            <div class="w-full" role="listbox">
                <DtListItemGroup heading="Example Heading">
                    <DtListItem role="option" navigation_type="tab" class="d-pis-100">"Item 1"</DtListItem>
                    <DtListItem role="option" navigation_type="tab" class="d-pis-100">"Item 2"</DtListItem>
                    <DtListItem role="option" navigation_type="tab" class="d-pis-100">"Item 3"</DtListItem>
                </DtListItemGroup>
            </div>
        </Example>
    }
}

#[component]
pub(crate) fn SkeletonExample() -> impl IntoView {
    view! {
        <Example
            id="skeleton"
            title="Skeleton"
            summary="Pass one option: paragraph, list item, or shape. The lines throb while content is loading."
            stack=true
            code=r##"<DtSkeleton
    aria_label="Loading schedule"
    paragraph_option=DtSkeletonParagraphOption { rows: 3, ..Default::default() }
/>
<DtSkeleton list_item_option=DtSkeletonListItemOption::default() />
<DtSkeleton
    shape_option=DtSkeletonShapeOption {
        shape: DtSkeletonShapeKind::Circle,
        size: "lg",
    }
/>"##
        >
            <div class="flex w-full max-w-xl flex-col gap-6">
                <DtSkeleton
                    aria_label="Loading schedule"
                    paragraph_option=DtSkeletonParagraphOption { rows: 3, ..Default::default() }
                />
                <DtSkeleton list_item_option=DtSkeletonListItemOption::default() />
                <div class="flex items-center gap-4">
                    <DtSkeleton
                        shape_option=DtSkeletonShapeOption {
                            shape: DtSkeletonShapeKind::Circle,
                            size: "lg",
                        }
                    />
                    <DtSkeleton
                        shape_option=DtSkeletonShapeOption {
                            shape: DtSkeletonShapeKind::Square,
                            size: "lg",
                        }
                    />
                </div>
            </div>
        </Example>
    }
}

#[component]
pub(crate) fn BannerExample() -> impl IntoView {
    let open = RwSignal::new(true);
    view! {
        <Example
            id="banner"
            title="Banner"
            summary="Kinds are base, info, positive, warning, and critical. A banner pins to the top of the viewport. This preview stacks them in the card. Close hides the positive banner until it is shown again."
            stack=true
            code=r##"<DtBanner
    header_text="Rotation updated"
    kind="positive"
    on_close=move || open.set(false)
>
    "The primary on-call changed."
</DtBanner>"##
        >
            <Show when=move || open.get()>
                <DtBanner
                    header_text="Rotation updated"
                    kind="positive"
                    on_close=move || open.set(false)
                >
                    "The primary on-call changed."
                </DtBanner>
            </Show>
            <Show when=move || !open.get()>
                <DtButton importance="outlined" size="sm" on_click=move || open.set(true)>
                    "Show banner"
                </DtButton>
            </Show>
            <DtBanner header_text="Heads up" kind="info" show_close=false>
                "Changes apply on the next sync."
            </DtBanner>
            <DtBanner header_text="Check the mapping" kind="warning" show_close=false>
                "Two schedules still need a Dialpad target."
            </DtBanner>
            <DtBanner header_text="Sync failed" kind="critical" show_close=false>
                "PagerDuty returned 503."
            </DtBanner>
        </Example>
    }
}

#[component]
pub(crate) fn LoaderExample() -> impl IntoView {
    view! {
        <Example
            id="loader"
            title="Loader"
            summary="Indeterminate loader. kind defaults to dots. spinner is the circular mark. An omitted size uses 500."
            code=r##"<DtLoader />
    <DtLoader size="200" />
    <DtLoader kind="spinner" aria_label="Loading schedules" />"##
        >
            <DtLoader />
            <DtLoader size="200" />
            <DtLoader kind="spinner" aria_label="Loading schedules" />
        </Example>
    }
}

#[component]
pub(crate) fn ProgressCircleExample() -> impl IntoView {
    view! {
        <Example
            id="progress-circle"
            title="Progress circle"
            summary="Circular indicator for upload or processing progress."
            code=r##"<DtProgressCircle aria_label="Upload progress" progress=25.0 />
<DtProgressCircle aria_label="Upload progress" progress=66.0 kind="brand" />
<DtProgressCircle aria_label="Upload progress" progress=80.0 kind="positive" />
<DtProgressCircle aria_label="Upload progress" progress=40.0 kind="ai" />"##
        >
            <DtProgressCircle aria_label="Upload progress" progress=25.0 />
            <DtProgressCircle aria_label="Upload progress" progress=66.0 kind="brand" />
            <DtProgressCircle aria_label="Upload progress" progress=80.0 kind="positive" />
            <DtProgressCircle aria_label="Upload progress" progress=40.0 kind="ai" />
        </Example>
    }
}

#[component]
pub(crate) fn MotionTextExample() -> impl IntoView {
    view! {
        <Example
            id="motion-text"
            title="Motion text"
            summary="Animated text. Shimmer keeps the words in place while the highlight moves."
            code=r##"<DtMotionText
    text="Welcome to Dialtone Motion Text"
    animation_mode="shimmer"
    r#loop=true
    class="d-headline--lg"
/>"##
        >
            <DtMotionText
                text="Welcome to Dialtone Motion Text".to_string()
                animation_mode="shimmer"
                r#loop=true
                class="d-headline--lg"
            />
        </Example>
    }
}

#[component]
pub(crate) fn NoticeExample() -> impl IntoView {
    view! {
        <Example
            id="notice"
            title="Notice"
            summary="Inline guidance that stays with the page. Close and the action sit beside the message."
            stack=true
            code=r##"<DtNotice header_text="Saved" kind="positive" show_close=false show_action=false>
    "The schedule was updated."
</DtNotice>
<DtNotice header_text="Check the mapping" kind="warning">
    "Two schedules still need a Dialpad target."
</DtNotice>"##
        >
            <DtNotice header_text="Saved" kind="positive" show_close=false show_action=false>
                "The schedule was updated."
            </DtNotice>
            <DtNotice header_text="Check the mapping" kind="warning">
                "Two schedules still need a Dialpad target."
            </DtNotice>
        </Example>
    }
}

#[component]
pub(crate) fn PaginationExample() -> impl IntoView {
    let page = RwSignal::new(1);
    view! {
        <Example
            id="pagination"
            title="Pagination"
            summary="Move through pages. The current page is the primary button."
            code=r##"<DtPagination
    aria_label="pagination"
    total_pages=5
    active_page=page
    on_change=move |next| page.set(next)
/>"##
        >
            <DtPagination
                aria_label="pagination".to_string()
                total_pages=5
                active_page=page
                on_change=move |next| page.set(next)
            />
        </Example>
    }
}

#[component]
pub(crate) fn ModalExample() -> impl IntoView {
    let show = RwSignal::new(false);
    view! {
        <Example
            id="modal"
            title="Modal"
            summary="A native dialog. The default calls show(), so it is not the top layer. A backdrop click closes when the press and the release are both on the backdrop. The width is the layout token."
            code=r##"<DtButton on_click=move || show.set(true)>"Open modal"</DtButton>
<DtModal
    open=show
    on_open=move |next| show.set(next)
    labelled_by_id="demo-modal-title"
    header_text="Edit mapping"
>
    <p>"Choose the Dialpad target for this schedule."</p>
</DtModal>"##
        >
            <DtButton on_click=move || show.set(true)>"Open modal"</DtButton>
            <DtModal
                open=show
                on_open=move |next| show.set(next)
                labelled_by_id="demo-modal-title"
                header_text="Edit mapping"
                footer=ViewFn::from(move || {
                    view! {
                        <DtButton importance="outlined" on_click=move || show.set(false)>
                            "Close"
                        </DtButton>
                    }
                })
            >
                <p class="m-0 text-sm">"Choose the Dialpad target for this schedule."</p>
            </DtModal>
        </Example>
    }
}

#[component]
pub(crate) fn ModeIslandExample() -> impl IntoView {
    view! {
        <Example
            id="mode-island"
            title="Mode"
            summary="Scopes light, dark, or inverted color mode on a primary surface."
            code=r##"<DtModeIsland class="d-p-200 d-bar-400">
    <DtText r#as="p">"Inverted"</DtText>
    <DtButton size="100">"Button"</DtButton>
</DtModeIsland>
<DtModeIsland mode="light" class="d-p-200 d-bar-400">
    <DtText r#as="p">"Light"</DtText>
    <DtButton size="100">"Button"</DtButton>
</DtModeIsland>
<DtModeIsland mode="dark" class="d-p-200 d-bar-400">
    <DtText r#as="p">"Dark"</DtText>
    <DtButton size="100">"Button"</DtButton>
</DtModeIsland>"##
        >
            <DtModeIsland class="d-p-200 d-bar-400">
                <DtText r#as="p">"Inverted"</DtText>
                <DtButton size="100">"Button"</DtButton>
            </DtModeIsland>
            <DtModeIsland mode="light" class="d-p-200 d-bar-400">
                <DtText r#as="p">"Light"</DtText>
                <DtButton size="100">"Button"</DtButton>
            </DtModeIsland>
            <DtModeIsland mode="dark" class="d-p-200 d-bar-400">
                <DtText r#as="p">"Dark"</DtText>
                <DtButton size="100">"Button"</DtButton>
            </DtModeIsland>
        </Example>
    }
}

#[component]
pub(crate) fn ToastExample() -> impl IntoView {
    let open = RwSignal::new(false);
    view! {
        <Example
            id="toast"
            title="Toast"
            summary="Positive toast with a 6000 ms duration. The wrapper that pins it sits outside DtToast."
            code=r##"let open = RwSignal::new(false);

<aside class="d-toast-wrapper">
    <DtToast
        open=open
        header_text="Saved"
        message="Schedule saved"
        kind="positive"
        duration=6000
        on_open=move |next| open.set(next)
        on_close=move || open.set(false)
    />
</aside>
<DtButton on_click=move || open.set(true)>"Show toast"</DtButton>"##
        >
            <div class="docs-toast-host">
                <aside class="d-toast-wrapper">
                    <DtToast
                        open=open
                        header_text="Saved"
                        message="Schedule saved"
                        kind="positive"
                        duration=6000
                        show_action=false
                        on_open=move |next| open.set(next)
                        on_close=move || open.set(false)
                    />
                </aside>
                <DtButton on_click=move || open.set(true)>"Show toast"</DtButton>
            </div>
        </Example>
    }
}

#[component]
pub(crate) fn TooltipExample() -> impl IntoView {
    view! {
        <Example
            id="tooltip"
            title="Tooltip"
            summary="Hover or focus the button. Placement flips when the tooltip does not fit."
            code=r##"<DtTooltip message="Copies the schedule id">
    <DtButton importance="outlined">"Hover"</DtButton>
</DtTooltip>"##
        >
            <DtTooltip message="Copies the schedule id">
                <DtButton importance="outlined">"Top"</DtButton>
            </DtTooltip>
            <DtTooltip message="Shown below the button" placement="bottom">
                <DtButton importance="outlined">"Bottom"</DtButton>
            </DtTooltip>
            <DtTooltip message="Shown to the right" placement="right">
                <DtButton importance="clear">"Right"</DtButton>
            </DtTooltip>
        </Example>
    }
}

#[component]
pub(crate) fn CheckboxExample() -> impl IntoView {
    let alerts = RwSignal::new(true);
    let required = RwSignal::new(false);
    view! {
        <Example
            id="checkbox"
            title="Checkbox"
            summary="Controlled checked signal. on_change writes it back. The second box shows a validation message."
            stack=true
            code=r##"<DtCheckbox
    checked=alerts
    on_change=move |next| alerts.set(next)
    label="Email alerts"
    description="Send a message when the rotation changes."
/>
<DtCheckbox
    checked=required
    on_change=move |next| required.set(next)
    label="Required acknowledgement"
    validation_state="critical"
    messages=vec![DtValidationMessage::critical("Select this to continue")]
/>"##
        >
            <DtCheckbox
                checked=Signal::derive(move || alerts.get())
                on_change=move |next| alerts.set(next)
                label="Email alerts"
                description="Send a message when the rotation changes."
            />
            <DtCheckbox
                checked=Signal::derive(move || required.get())
                on_change=move |next| required.set(next)
                label="Required acknowledgement"
                validation_state="critical"
                messages=vec![DtValidationMessage::critical("Select this to continue")]
            />
        </Example>
    }
}

#[component]
pub(crate) fn CheckboxGroupExample() -> impl IntoView {
    let selected = RwSignal::new(vec!["email".to_string()]);
    view! {
        <Example
            id="checkbox-group"
            title="Checkbox group"
            summary="Selected values live on the group. Each checkbox value is added or removed in on_change."
            stack=true
            code=r##"<DtCheckboxGroup
    name="channels"
    legend="Channels"
    selected_values=selected
    on_change=move |next| selected.set(next)
>
    <DtCheckbox value="email" label="Email" />
    <DtCheckbox value="sms" label="SMS" />
    <DtCheckbox value="push" label="Push" />
</DtCheckboxGroup>"##
        >
            <DtCheckboxGroup
                name="channels"
                legend="Channels"
                selected_values=selected
                on_change=move |next| selected.set(next)
            >
                <DtCheckbox value="email" label="Email" />
                <DtCheckbox value="sms" label="SMS" />
                <DtCheckbox value="push" label="Push" />
            </DtCheckboxGroup>
            <p class="m-0 text-sm text-fg-muted">
                {move || {
                    let values = selected.get();
                    if values.is_empty() {
                        "None selected".to_string()
                    } else {
                        format!("Selected: {}", values.join(", "))
                    }
                }}
            </p>
        </Example>
    }
}

#[component]
pub(crate) fn InputExample() -> impl IntoView {
    let name = RwSignal::new(String::new());
    view! {
        <Example
            id="input"
            title="Input"
            summary="The value signal is what the field shows. on_input writes each change."
            stack=true
            code=r##"<DtInput
    label="Name"
    placeholder="Ada Lovelace"
    value=name
    on_input=move |next| name.set(next)
/>"##
        >
            <div class="w-full max-w-md">
                <DtInput
                    label="Name"
                    placeholder="Ada Lovelace"
                    value=name
                    on_input=move |next| name.set(next)
                />
            </div>
        </Example>
    }
}

#[component]
pub(crate) fn InputGroupExample() -> impl IntoView {
    let first = RwSignal::new(String::new());
    let last = RwSignal::new(String::new());
    view! {
        <Example
            id="input-group"
            title="Input group"
            summary="A fieldset and legend around related inputs."
            stack=true
            code=r##"<DtInputGroup legend="Name">
    <DtInput label="First" value=first on_input=move |next| first.set(next) />
    <DtInput label="Last" value=last on_input=move |next| last.set(next) />
</DtInputGroup>"##
        >
            <DtInputGroup legend="Name" class="flex flex-col gap-3">
                <DtInput
                    label="First"
                    placeholder="Ada"
                    value=first
                    on_input=move |next| first.set(next)
                />
                <DtInput
                    label="Last"
                    placeholder="Lovelace"
                    value=last
                    on_input=move |next| last.set(next)
                />
            </DtInputGroup>
        </Example>
    }
}

#[component]
pub(crate) fn RadioExample() -> impl IntoView {
    let voicemail = RwSignal::new(true);
    view! {
        <Example
            id="radio"
            title="Radio"
            summary="A single option. on_change reports the option value. The checked signal is the boolean model."
            stack=true
            code=r##"<DtRadio
    value="voicemail"
    name="routing"
    label="To voicemail"
    description="So they can hear your voice"
    checked=Signal::derive(move || voicemail.get())
    on_change=move |_| voicemail.set(true)
/>"##
        >
            <DtRadio
                value="voicemail"
                name="routing"
                label="To voicemail"
                description="So they can hear your voice"
                checked=Signal::derive(move || voicemail.get())
                on_change=move |_| voicemail.set(true)
            />
        </Example>
    }
}

#[component]
pub(crate) fn RadioGroupExample() -> impl IntoView {
    let selected = RwSignal::new("apple".to_string());
    view! {
        <Example
            id="radio-group"
            title="Radio group"
            summary="One selected value for the group. on_change writes that value."
            stack=true
            code=r##"<DtRadioGroup
    name="fruits"
    legend="Fruits"
    selected=selected
    on_change=move |next| selected.set(next)
>
    <DtRadio value="apple" label="Apple" />
    <DtRadio value="banana" label="Banana" />
    <DtRadio value="other" label="Other" />
</DtRadioGroup>"##
        >
            <DtRadioGroup
                name="fruits"
                legend="Fruits"
                selected=selected
                on_change=move |next| selected.set(next)
            >
                <DtRadio value="apple" label="Apple" />
                <DtRadio value="banana" label="Banana" />
                <DtRadio value="other" label="Other" />
            </DtRadioGroup>
            <p class="m-0 text-sm text-fg-muted">
                {move || format!("Selected: {}", selected.get())}
            </p>
        </Example>
    }
}

#[component]
pub(crate) fn ToggleExample() -> impl IntoView {
    let enabled = RwSignal::new(true);
    let compact = RwSignal::new(false);
    view! {
        <Example
            id="toggle"
            title="Toggle"
            summary="Switch with a label. on_change receives the next checked value."
            code=r##"<DtToggle checked=enabled on_change=move |next| enabled.set(next)>
    "Sync enabled"
</DtToggle>"##
        >
            <DtToggle checked=enabled on_change=move |next| enabled.set(next)>
                "Sync enabled"
            </DtToggle>
            <DtToggle checked=compact size="sm" on_change=move |next| compact.set(next)>
                "Compact"
            </DtToggle>
        </Example>
    }
}

#[component]
pub(crate) fn ResizableExample() -> impl IntoView {
    view! {
        <Example
            id="resizable"
            title="Resizable"
            summary="Drag the handle between two panels."
            stack=true
            code=r##"<div class="w-full" style="block-size:240px">
    <DtResizable>
        <DtResizablePanel id="one">
            "One"
        </DtResizablePanel>
        <DtResizableHandle before_panel_id="one" after_panel_id="two" />
        <DtResizablePanel id="two">
            "Two"
        </DtResizablePanel>
    </DtResizable>
</div>"##
        >
            <div class="w-full" style="block-size:240px">
                <DtResizable>
                    <DtResizablePanel id="one">
                        <p class="m-0 p-4 text-sm">"One"</p>
                    </DtResizablePanel>
                    <DtResizableHandle before_panel_id="one" after_panel_id="two" />
                    <DtResizablePanel id="two">
                        <p class="m-0 p-4 text-sm">"Two"</p>
                    </DtResizablePanel>
                </DtResizable>
            </div>
        </Example>
    }
}

#[component]
pub(crate) fn RichTextEditorExample() -> impl IntoView {
    let value = RwSignal::new("Hello from Dialtone".to_string());
    view! {
        <Example
            id="rich-text-editor"
            title="Rich text editor"
            summary="Editable message text with formatting and links. Enter submits unless line breaks are on."
            stack=true
            code=r##"<DtRichTextEditor
    value=value
    on_change=move |next| value.set(next)
    input_aria_label="Message"
    placeholder="Type here..."
    link=true
/>"##
        >
            <div class="w-full">
                <DtRichTextEditor
                    value=value
                    on_change=move |next| value.set(next)
                    input_aria_label="Message"
                    placeholder="Type here..."
                    link=true
                />
            </div>
        </Example>
    }
}

#[component]
pub(crate) fn RootLayoutExample() -> impl IntoView {
    view! {
        <Example
            id="root-layout"
            title="Root layout"
            summary="Header, sidebar, content, and footer. Shown in a frame so the page itself keeps scrolling."
            stack=true
            code=r##"<DtRootLayout
    header=ViewFn::from(|| view! { <div class="p-3 text-sm">"Dialtone"</div> })
    sidebar=ViewFn::from(|| view! { <div class="p-3 text-sm">"Nav"</div> })
    footer=ViewFn::from(|| view! { <div class="p-3 text-xs">"Footer"</div> })
>
    <p class="p-3 text-sm">"Page content"</p>
</DtRootLayout>"##
        >
            <div class="demo-frame w-full">
                <DtRootLayout
                    header=ViewFn::from(|| view! { <div class="p-3 text-sm font-medium">"Dialtone"</div> })
                    sidebar=ViewFn::from(|| view! { <div class="p-3 text-sm text-fg-muted">"Nav"</div> })
                    footer=ViewFn::from(|| view! { <div class="p-3 text-xs text-fg-muted">"Footer"</div> })
                >
                    <p class="m-0 p-3 text-sm">"Page content sits in the main area."</p>
                </DtRootLayout>
            </div>
        </Example>
    }
}

#[component]
fn IconCell(name: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="demo-icon">
            {children()}
            <span>{name}</span>
        </div>
    }
}

#[component]
pub(crate) fn IconExample() -> impl IntoView {
    view! {
        <Example
            id="icon"
            title="Icon"
            summary="Named icons from the port. An omitted size is 500."
            code=r##"<DtIconInfo />
    <DtIconClose size="200" />"##
        >
            <div class="demo-icons">
                <IconCell name="Plus"><DtIconPlus /></IconCell>
                <IconCell name="Refresh"><DtIconRefresh /></IconCell>
                <IconCell name="Save"><DtIconSave /></IconCell>
                <IconCell name="More"><DtIconMoreVertical /></IconCell>
                <IconCell name="More horizontal"><DtIconMoreHorizontal /></IconCell>
                <IconCell name="Close"><DtIconClose /></IconCell>
                <IconCell name="X circle"><DtIconXCircle /></IconCell>
                <IconCell name="Satisfied"><DtIconSatisfied /></IconCell>
                <IconCell name="Very satisfied"><DtIconVerySatisfied /></IconCell>
                <IconCell name="Down"><DtIconChevronDown /></IconCell>
                <IconCell name="Left"><DtIconChevronLeft /></IconCell>
                <IconCell name="Right"><DtIconChevronRight /></IconCell>
                <IconCell name="Chevrons left"><DtIconChevronsLeft /></IconCell>
                <IconCell name="Chevrons right"><DtIconChevronsRight /></IconCell>
                <IconCell name="Check"><DtIconCheck /></IconCell>
                <IconCell name="Swap"><DtIconArrowLeftRight /></IconCell>
                <IconCell name="API"><DtIconConnectionApi /></IconCell>
                <IconCell name="Info"><DtIconInfo /></IconCell>
                <IconCell name="Bell"><DtIconBell /></IconCell>
                <IconCell name="Alert"><DtIconAlertCircle /></IconCell>
                <IconCell name="Warning"><DtIconAlertTriangle /></IconCell>
                <IconCell name="Success"><DtIconCheckCircle /></IconCell>
                <IconCell name="User"><DtIconUser /></IconCell>
                <IconCell name="Users"><DtIconUsers /></IconCell>
                <IconCell name="Trash"><DtIconTrash /></IconCell>
                <IconCell name="Search"><DtIconSearch /></IconCell>
                <IconCell name="Sparkle"><DtIconSparkle /></IconCell>
                <IconCell name="Laptop"><DtIconLaptop2 /></IconCell>
                <IconCell name="Sun"><DtIconSun /></IconCell>
                <IconCell name="Moon"><DtIconMoon /></IconCell>
                <IconCell name="Close 200"><DtIconClose size="200" /></IconCell>
            </div>
        </Example>
    }
}

#[component]
pub(crate) fn IllustrationExample() -> impl IntoView {
    view! {
        <Example
            id="illustration"
            title="Illustration"
            summary="Contextual artwork. The SVG is inlined, and ids are prefixed."
            code=r##"<DtIllustration name="mind" />"##
        >
            <DtIllustration name="mind" />
        </Example>
    }
}

#[component]
pub(crate) fn ImageViewerExample() -> impl IntoView {
    view! {
        <Example
            id="image-viewer"
            title="Image viewer"
            summary="A preview that opens the image full screen."
            code=r##"<DtImageViewer
    image_src="/assets/images/person.png"
    image_alt="Ada Lovelace"
    image_button_class="d-wmn-100 d-hmn-100 d-wmx-500 d-hmx-500"
    aria_label="Click to open image"
/>"##
        >
            <DtImageViewer
                image_src="/assets/images/person.png"
                image_alt="Ada Lovelace"
                image_button_class="d-wmn-100 d-hmn-100 d-wmx-500 d-hmx-500"
                aria_label="Click to open image"
            />
        </Example>
    }
}

fn scroller_rows() -> Vec<DtScrollerItem> {
    (1..=40)
        .map(|index| {
            DtScrollerItem::new(
                index.to_string(),
                ViewFn::from(move || {
                    view! {
                        <div class="px-3 text-sm" style="height: 32px; display: flex; align-items: center;">
                            {format!("Incident {index}")}
                        </div>
                    }
                }),
            )
        })
        .collect()
}

#[component]
pub(crate) fn SplitButtonExample() -> impl IntoView {
    view! {
        <Example
            id="split-button"
            title="Split button"
            summary="The start button runs the main action. The end button opens more actions."
            code=r##"<DtSplitButton omega_tooltip_text="More actions">
    "Save"
</DtSplitButton>
<DtSplitButton
    importance="outlined"
    omega_tooltip_text="More actions"
    dropdown_list=ViewFn::from(|| view! {
        <li class="d-list-item" role="menuitem" tabindex="-1">"Duplicate"</li>
        <li class="d-list-item" role="menuitem" tabindex="-1">"Archive"</li>
    })
>
    "Save"
</DtSplitButton>"##
        >
            <DtSplitButton omega_tooltip_text="More actions">"Save"</DtSplitButton>
            <DtSplitButton
                importance="outlined"
                omega_tooltip_text="More actions"
                dropdown_list=ViewFn::from(|| view! {
                    <li class="d-list-item" role="menuitem" tabindex="-1">"Duplicate"</li>
                    <li class="d-list-item" role="menuitem" tabindex="-1">"Archive"</li>
                })
            >
                "Save"
            </DtSplitButton>
        </Example>
    }
}

#[component]
pub(crate) fn DropdownExample() -> impl IntoView {
    view! {
        <Example
            id="dropdown"
            title="Dropdown"
            summary="A menu anchored to a button. Modal is off so the page stays usable."
            code=r##"<DtDropdown
    modal=false
    append_to="parent"
    anchor=ViewFn::from(|| view! { <DtButton importance="outlined">"Schedule"</DtButton> })
>
    <li class="d-list-item" role="menuitem" tabindex="-1">"Today"</li>
    <li class="d-list-item" role="menuitem" tabindex="-1">"This week"</li>
    <DtDropdownSeparator />
    <li class="d-list-item" role="menuitem" tabindex="-1">"Custom"</li>
</DtDropdown>"##
        >
            <DtDropdown
                modal=false
                append_to="parent"
                anchor=ViewFn::from(|| view! { <DtButton importance="outlined">"Schedule"</DtButton> })
            >
                <li class="d-list-item" role="menuitem" tabindex="-1">"Today"</li>
                <li class="d-list-item" role="menuitem" tabindex="-1">"This week"</li>
                <DtDropdownSeparator />
                <li class="d-list-item" role="menuitem" tabindex="-1">"Custom"</li>
            </DtDropdown>
        </Example>
    }
}

#[component]
pub(crate) fn SegmentedControlExample() -> impl IntoView {
    let range = RwSignal::new("week".to_string());
    view! {
        <Example
            id="segmented-control"
            title="Segmented control"
            summary="One selected value. Arrow keys move the selection. The indicator does not animate until segmented CSS is vendored."
            code=r##"<DtSegmentedControl
    aria_label="Range"
    value=range
    on_change=move |next| range.set(next)
>
    <DtSegmentedControlItem value="day">"Day"</DtSegmentedControlItem>
    <DtSegmentedControlItem value="week">"Week"</DtSegmentedControlItem>
    <DtSegmentedControlItem value="month">"Month"</DtSegmentedControlItem>
</DtSegmentedControl>"##
        >
            <DtSegmentedControl
                aria_label="Range"
                value=range
                on_change=move |next| range.set(next)
            >
                <DtSegmentedControlItem value="day">"Day"</DtSegmentedControlItem>
                <DtSegmentedControlItem value="week">"Week"</DtSegmentedControlItem>
                <DtSegmentedControlItem value="month">"Month"</DtSegmentedControlItem>
            </DtSegmentedControl>
        </Example>
    }
}

#[component]
pub(crate) fn DescriptionListExample() -> impl IntoView {
    view! {
        <Example
            id="description-list"
            title="Description list"
            summary="Term and description pairs."
            stack=true
            code=r##"<DtDescriptionList items=vec![
    DtDescriptionListItem::new("Service", "Voice"),
    DtDescriptionListItem::new("Owner", "Ada Lovelace"),
] />"##
        >
            <div class="w-full">
                <DtDescriptionList items=vec![
                    DtDescriptionListItem::new("Service", "Voice"),
                    DtDescriptionListItem::new("Owner", "Ada Lovelace"),
                ] />
            </div>
        </Example>
    }
}

#[component]
pub(crate) fn TextExample() -> impl IntoView {
    view! {
        <Example
            id="text"
            title="Text"
            summary="Typography compositions. Headline text uses a heading element."
            stack=true
            code=r##"<DtText variant="headline-lg" r#as="h2">"On call"</DtText>
<DtText variant="body-md">"The primary rotation changed this morning."</DtText>
<DtText variant="label-sm" tone="muted">"Updated today"</DtText>"##
        >
            <DtText variant="headline-lg" r#as="h2">"On call"</DtText>
            <DtText variant="body-md">"The primary rotation changed this morning."</DtText>
            <DtText variant="label-sm" tone="muted">"Updated today"</DtText>
        </Example>
    }
}

#[component]
pub(crate) fn TextListExample() -> impl IntoView {
    view! {
        <Example
            id="text-list"
            title="Text list"
            summary="Unordered and ordered lists."
            code=r##"<DtTextList>
    <DtTextListItem>"Voice"</DtTextListItem>
    <DtTextListItem>"Meetings"</DtTextListItem>
</DtTextList>
<DtTextList r#type="ordered">
    <DtTextListItem>"Page the primary"</DtTextListItem>
    <DtTextListItem>"Ack the incident"</DtTextListItem>
</DtTextList>"##
        >
            <DtTextList>
                <DtTextListItem>"Voice"</DtTextListItem>
                <DtTextListItem>"Meetings"</DtTextListItem>
            </DtTextList>
            <DtTextList r#type="ordered">
                <DtTextListItem>"Page the primary"</DtTextListItem>
                <DtTextListItem>"Ack the incident"</DtTextListItem>
            </DtTextList>
        </Example>
    }
}

#[component]
pub(crate) fn HovercardExample() -> impl IntoView {
    view! {
        <Example
            id="hovercard"
            title="Hovercard"
            summary="A preview that opens from hover or from focus inside the card. Click does not toggle."
            code=r##"<DtHovercard
    anchor=ViewFn::from(|| view! {
        <DtButton importance="outlined" kind="muted">"Hover over me"</DtButton>
    })
>
    <p class="m-0 max-w-xs text-sm">"Primary on-call this week."</p>
</DtHovercard>"##
        >
            <DtHovercard
                anchor=ViewFn::from(|| view! {
                    <DtButton importance="outlined" kind="muted">"Hover over me"</DtButton>
                })
            >
                <p class="m-0 max-w-xs text-sm">"Primary on-call this week."</p>
            </DtHovercard>
        </Example>
    }
}

#[component]
pub(crate) fn PopoverExample() -> impl IntoView {
    view! {
        <Example
            id="popover"
            title="Popover"
            summary="A dialog anchored to a button. Modal is off so the page stays usable."
            code=r##"<DtPopover
    modal=false
    append_to="parent"
    padding="small"
    anchor=ViewFn::from(|| view! { <DtButton importance="outlined">"Details"</DtButton> })
>
    <p class="m-0 max-w-xs text-sm">"The rotation repeats every week."</p>
</DtPopover>"##
        >
            <DtPopover
                modal=false
                append_to="parent"
                padding="small"
                anchor=ViewFn::from(|| view! { <DtButton importance="outlined">"Details"</DtButton> })
            >
                <p class="m-0 max-w-xs text-sm">"The rotation repeats every week."</p>
            </DtPopover>
        </Example>
    }
}

#[component]
pub(crate) fn ValidationMessagesExample() -> impl IntoView {
    view! {
        <Example
            id="validation-messages"
            title="Validation messages"
            summary="Critical wins over warning, positive, and info. Each block below is its own list, so each kind is visible."
            stack=true
            code=r##"<DtValidationMessages validation_messages=vec![
    DtValidationMessage::critical("Choose a product"),
] />
<DtValidationMessages validation_messages=vec![
    DtValidationMessage::warning("This channel is quiet overnight"),
] />
<DtValidationMessages validation_messages=vec![
    DtValidationMessage::positive("Saved"),
] />"##
        >
            <DtValidationMessages validation_messages=vec![
                DtValidationMessage::critical("Choose a product"),
            ] />
            <DtValidationMessages validation_messages=vec![
                DtValidationMessage::warning("This channel is quiet overnight"),
            ] />
            <DtValidationMessages validation_messages=vec![
                DtValidationMessage::positive("Saved"),
            ] />
        </Example>
    }
}

#[component]
pub(crate) fn ComboboxExample() -> impl IntoView {
    let value = RwSignal::new(String::new());
    let show_list = Signal::derive(move || !value.get().is_empty());
    view! {
        <Example
            id="combobox"
            title="Combobox"
            summary="Type to show the list. The input and the list are slots."
            stack=true
            code=r##"let show_list = Signal::derive(move || !value.get().is_empty());
let input = ViewFn::from(move || view! {
    <DtInput
        label="Label Text"
        placeholder="Type to show the items"
        value=value
        on_input=move |next| value.set(next)
    />
});
<DtCombobox
    label="Label Text".to_string()
    show_list=show_list
    on_escape=move |_| value.set(String::new())
    input=input
    list=list
/>"##
        >
            <div class="w-full">
                <DtCombobox
                    label="Label Text".to_string()
                    show_list=show_list
                    on_escape=move |_| value.set(String::new())
                    on_select=move |index: usize| {
                        if let Some(name) = ["Voice", "Meetings"].get(index) {
                            value.set(name.to_string());
                        }
                    }
                    input=ViewFn::from(move || {
                        view! {
                            <DtInput
                                label="Label Text"
                                placeholder="Type to show the items"
                                value=value
                                on_input=move |next| value.set(next)
                            />
                        }
                    })
                    list=ViewFn::from(move || {
                        view! {
                            <ol class="d-p-0 d-mbs-100 d-hmx-500 d-of-y-auto">
                                <li class="d-list-item" role="option" on:click=move |_| value.set("Voice".to_string())>"Voice"</li>
                                <li class="d-list-item" role="option" on:click=move |_| value.set("Meetings".to_string())>"Meetings"</li>
                            </ol>
                        }
                    })
                />
            </div>
        </Example>
    }
}

#[component]
pub(crate) fn ComboboxMultiExample() -> impl IntoView {
    let selected = RwSignal::new(vec!["Voice".to_string()]);
    view! {
        <Example
            id="combobox-multi-select"
            title="Combobox multi-select"
            summary="Selected products stay as chips. Removing a chip updates the list."
            stack=true
            code=r##"<DtComboboxMultiSelect
    label="Products".to_string()
    selected_items=selected
    on_select=move |_| {}
    on_remove=move |item| {
        selected.update(|items| items.retain(|current| current != &item));
    }
>
    <li class="d-list-item" role="option">"Voice"</li>
    <li class="d-list-item" role="option">"Meetings"</li>
</DtComboboxMultiSelect>"##
        >
            <div class="w-full">
                <DtComboboxMultiSelect
                    label="Products".to_string()
                    selected_items=selected
                    on_select=move |_| {}
                    on_remove=move |item| {
                        selected.update(|items| items.retain(|current| current != &item));
                    }
                >
                    <li class="d-list-item" role="option">"Voice"</li>
                    <li class="d-list-item" role="option">"Meetings"</li>
                </DtComboboxMultiSelect>
            </div>
        </Example>
    }
}

#[component]
pub(crate) fn ComboboxPopoverExample() -> impl IntoView {
    let value = RwSignal::new(String::new());
    view! {
        <Example
            id="combobox-with-popover"
            title="Combobox with popover"
            summary="The same field, with the list in a popover anchored to the input."
            stack=true
            code=r##"<DtComboboxWithPopover
    label="Product".to_string()
    placeholder="Search"
    value=value
    on_input=move |next| value.set(next)
    append_to="parent"
>
    <li class="d-list-item" role="option">"Voice"</li>
    <li class="d-list-item" role="option">"Meetings"</li>
</DtComboboxWithPopover>"##
        >
            <div class="w-full">
                <DtComboboxWithPopover
                    label="Product".to_string()
                    placeholder="Search"
                    value=value
                    on_input=move |next| value.set(next)
                    append_to="parent"
                >
                    <li class="d-list-item" role="option">"Voice"</li>
                    <li class="d-list-item" role="option">"Meetings"</li>
                </DtComboboxWithPopover>
            </div>
        </Example>
    }
}

#[component]
pub(crate) fn DatepickerExample() -> impl IntoView {
    view! {
        <Example
            id="datepicker"
            title="Datepicker"
            summary="Pick a date from the calendar."
            code=r##"<DtDatepicker />"##
        >
            <DtDatepicker />
        </Example>
    }
}

#[component]
pub(crate) fn SelectMenuExample() -> impl IntoView {
    let value = RwSignal::new("voice".to_string());
    view! {
        <Example
            id="select-menu"
            title="Select menu"
            summary="A native select. The label and the selected value stay in sync."
            stack=true
            code=r##"<DtSelectMenu
    label="Product"
    value=value
    on_change=move |next| value.set(next)
    options=vec![
        DtSelectOption::new("voice", "Voice"),
        DtSelectOption::new("meetings", "Meetings"),
        DtSelectOption::new("sms", "SMS"),
    ]
/>"##
        >
            <div class="w-full">
                <DtSelectMenu
                    label="Product"
                    value=value
                    on_change=move |next| value.set(next)
                    options=vec![
                        DtSelectOption::new("voice", "Voice"),
                        DtSelectOption::new("meetings", "Meetings"),
                        DtSelectOption::new("sms", "SMS"),
                    ]
                />
            </div>
        </Example>
    }
}

#[component]
pub(crate) fn BoxExample() -> impl IntoView {
    view! {
        <Example
            id="box"
            title="Box"
            summary="A surface with padding, a border, and a card shadow."
            code=r##"<DtBox
    padding="300"
    surface="primary"
    border_width="100"
    border_radius="400"
    shadow="card"
>
    <p class="m-0 text-sm">"Card body content with supporting text."</p>
</DtBox>"##
        >
            <DtBox
                padding="300"
                surface="primary"
                border_width="100"
                border_radius="400"
                shadow="card"
            >
                <p class="m-0 text-sm">"Card body content with supporting text."</p>
            </DtBox>
        </Example>
    }
}

#[component]
pub(crate) fn CollapsibleExample() -> impl IntoView {
    view! {
        <Example
            id="collapsible"
            title="Collapsible"
            summary="The anchor shows and hides the content."
            stack=true
            code=r##"<DtCollapsible anchor_text="Schedule">
    <p class="m-0 text-sm">"Weekly rotation"</p>
</DtCollapsible>"##
        >
            <div class="w-full">
                <DtCollapsible anchor_text="Schedule">
                    <p class="m-0 text-sm">"Weekly rotation"</p>
                </DtCollapsible>
            </div>
        </Example>
    }
}

#[component]
pub(crate) fn ScrollerExample() -> impl IntoView {
    let items = RwSignal::new(scroller_rows());
    view! {
        <Example
            id="scroller"
            title="Scroller"
            summary="Forty rows, each 32px. Only the rows near the viewport are mounted."
            stack=true
            code=r##"<DtScroller
    items=items
    item_size=32.0
    scroller_height=160
/>"##
        >
            <div class="w-full overflow-hidden rounded-lg border border-black/10">
                <DtScroller items=items item_size=32.0 scroller_height=160 />
            </div>
        </Example>
    }
}

#[component]
pub(crate) fn StackExample() -> impl IntoView {
    view! {
        <Example
            id="stack"
            title="Stack"
            summary="Row and column stacks. Column is the default and adds no direction class."
            stack=true
            code=r##"<DtStack direction="row" gap="400" align="center">
    <DtBadge text="Voice" type_="info" />
    <DtBadge text="On call" type_="positive" />
    <DtButton size="sm" importance="outlined">"Edit"</DtButton>
</DtStack>
<DtStack gap="200">
    <span>"Primary"</span>
    <span>"Secondary"</span>
</DtStack>"##
        >
            <DtStack direction="row" gap="400" align="center">
                <DtBadge text="Voice" type_="info" />
                <DtBadge text="On call" type_="positive" />
                <DtButton size="sm" importance="outlined">"Edit"</DtButton>
            </DtStack>
            <DtStack gap="200">
                <span>"Primary"</span>
                <span>"Secondary"</span>
            </DtStack>
        </Example>
    }
}

#[component]
pub(crate) fn TabExample() -> impl IntoView {
    let selected = RwSignal::new("overview");
    view! {
        <Example
            id="tab"
            title="Tab"
            summary="Each tab owns its selected state. Grouped tabs are the Tab group example."
            stack=true
            code=r##"<DtTab
    id="overview"
    panel_id="overview"
    selected=true
    on_click=move || selected.set("overview")
>
    "Overview"
</DtTab>"##
        >
            <div class="flex gap-2" role="tablist">
                {move || {
                    let current = selected.get();
                    view! {
                        <DtTab
                            id="overview"
                            panel_id="overview"
                            selected={current == "overview"}
                            on_click=move || selected.set("overview")
                        >
                            "Overview"
                        </DtTab>
                        <DtTab
                            id="people"
                            panel_id="people"
                            selected={current == "people"}
                            on_click=move || selected.set("people")
                        >
                            "People"
                        </DtTab>
                    }
                }}
            </div>
            <p class="m-0 text-sm">
                {move || if selected.get() == "overview" { "Overview panel" } else { "People panel" }}
            </p>
        </Example>
    }
}

#[component]
pub(crate) fn TabGroupExample() -> impl IntoView {
    let selected = RwSignal::new("overview");
    let selected_id = Signal::derive(move || selected.get().to_string());
    view! {
        <Example
            id="tab-group"
            title="Tab group"
            summary="Tabs share one selected panel id. The indicator does not animate."
            code=r##"<DtTabGroup
    label="Sections"
    selected=selected
    on_change=move |next: DtTabGroupChange| {
        selected.set(if next.selected == "people" { "people" } else { "overview" });
    }
    tabs=ViewFn::from(|| view! {
        <DtTab id="overview" panel_id="overview">"Overview"</DtTab>
        <DtTab id="people" panel_id="people">"People"</DtTab>
    })
>
    <DtTabPanel id="overview" tab_id="overview">"Overview"</DtTabPanel>
    <DtTabPanel id="people" tab_id="people">"People"</DtTabPanel>
</DtTabGroup>"##
        >
            <DtTabGroup
                label="Sections"
                selected=selected_id
                tabs=ViewFn::from(|| {
                    view! {
                        <DtTab id="overview" panel_id="overview">"Overview"</DtTab>
                        <DtTab id="people" panel_id="people">"People"</DtTab>
                    }
                })
                on_change=move |next: DtTabGroupChange| {
                    selected.set(if next.selected == "people" {
                        "people"
                    } else {
                        "overview"
                    });
                }
            >
                <DtTabPanel id="overview" tab_id="overview">
                    <p class="m-0 text-sm">"Overview"</p>
                </DtTabPanel>
                <DtTabPanel id="people" tab_id="people">
                    <p class="m-0 text-sm">"People"</p>
                </DtTabPanel>
            </DtTabGroup>
        </Example>
    }
}

#[component]
pub(crate) fn ProseExample() -> impl IntoView {
    view! {
        <Example
            id="prose"
            title="Prose"
            summary="Typography for native HTML such as Markdown, CMS content, and assistant output."
            stack=true
            code=r##"<DtProse>
    <h2>"Release notes"</h2>
    <p>
        <a href="https://dialtone.dialpad.com">"Dialtone"</a>
        " covers buttons and lists."
    </p>
    <ul>
        <li>"Voice"</li>
        <li>"Meetings"</li>
    </ul>
</DtProse>"##
        >
            <DtProse>
                <h2>"Release notes"</h2>
                <p>
                    <a href="https://dialtone.dialpad.com">"Dialtone"</a>
                    " covers buttons and lists."
                </p>
                <ul>
                    <li>"Voice"</li>
                    <li>"Meetings"</li>
                </ul>
            </DtProse>
        </Example>
    }
}

#[component]
pub(crate) fn EmojiExample() -> impl IntoView {
    view! {
        <Example
            id="emoji"
            title="Emoji"
            summary="An emoji image from a shortcode. :smile:, :grinning:, and :smiley: are in the vendored set."
            code=r##"<DtEmoji code=":smile:" />
    <DtEmoji code=":grinning:" size="300" />"##
        >
            <DtEmoji code=":grinning:" size="300" />
            <DtEmoji code=":smile:" />
            <DtEmoji code=":smiley:" size="700" />
        </Example>
    }
}

#[component]
pub(crate) fn EmojiPickerExample() -> impl IntoView {
    let selected = RwSignal::new(String::new());
    view! {
        <Example
            id="emoji-picker"
            title="Emoji picker"
            summary="Search the grid and pick an emoji. The name under the grid is the selection."
            stack=true
            code=r##"<DtEmojiPicker
    on_selected_emoji=move |selection: DtEmojiSelection| {
        selected.set(selection.emoji.name);
    }
/>"##
        >
            <DtEmojiPicker
                on_selected_emoji=move |selection: DtEmojiSelection| {
                    selected.set(selection.emoji.name);
                }
            />
            <p class="demo-summary">{move || {
                let name = selected.get();
                if name.is_empty() {
                    "Select an emoji.".to_string()
                } else {
                    format!("Selected {name}.")
                }
            }}</p>
        </Example>
    }
}

#[component]
pub(crate) fn EmojiTextWrapperExample() -> impl IntoView {
    view! {
        <Example
            id="emoji-text-wrapper"
            title="Emoji text wrapper"
            summary="Shortcodes and unicode emoji in the string become emoji images."
            stack=true
            code=r##"<DtEmojiTextWrapper text="Nice work :smile: today" />"##
        >
            <DtEmojiTextWrapper text="Nice work :smile: today" />
        </Example>
    }
}

#[component]
pub(crate) fn EmptyStateExample() -> impl IntoView {
    view! {
        <Example
            id="empty-state"
            title="Empty state"
            summary="Headline, body, and an icon for a screen with nothing to show. The bell is size 800."
            stack=true
            code=r##"<DtEmptyState
    header_text="No incidents"
    body_text="Alerts land here when a service pages you."
    icon=ViewFn::from(|| view! { <DtIconBell size="800" /> })
/>"##
        >
            <DtEmptyState
                header_text="No incidents"
                body_text="Alerts land here when a service pages you."
                icon=ViewFn::from(|| view! { <DtIconBell size="800" /> })
            />
        </Example>
    }
}

#[component]
pub(crate) fn FilterPillExample() -> impl IntoView {
    let filters = RwSignal::new(vec![
        DtFilter {
            name: "Voice".to_string(),
            active: true,
        },
        DtFilter {
            name: "SMS".to_string(),
            active: false,
        },
        DtFilter {
            name: "Meetings".to_string(),
            active: false,
        },
    ]);
    view! {
        <Example
            id="filter-pill"
            title="Filter pill"
            summary="Checkboxes filter the list. on_change writes the signal, and clear turns every filter off."
            code=r##"<DtFilterPill
    filters=filters.into()
    label="Channel"
    on_change=move |next: Vec<DtFilter>| filters.set(next)
/>"##
        >
            <DtFilterPill
                filters=filters.into()
                label="Channel"
                on_change=move |next| filters.set(next)
            />
        </Example>
    }
}

#[component]
pub(crate) fn SliderExample() -> impl IntoView {
    let volume = RwSignal::new(DtSliderValue::Single(40.0));
    view! {
        <Example
            id="slider"
            title="Slider"
            summary="A controlled number along a track. on_input writes the next value."
            stack=true
            code=r##"let volume = RwSignal::new(DtSliderValue::Single(40.0));
<DtSlider
    label="Volume"
    value=volume
    on_input=Callback::new(move |next| volume.set(next))
/>"##
        >
            <DtSlider
                label="Volume"
                value=volume
                on_input=Callback::new(move |next| volume.set(next))
                style="inline-size: 16rem"
            />
        </Example>
    }
}

#[component]
pub(crate) fn TableExample() -> impl IntoView {
    view! {
        <Example
            id="table"
            title="Table"
            summary="A captioned data table. Striped rows use d-table--striped."
            stack=true
            code=r##"<DtTable caption="On call" striped=true>
    <thead>
        <tr>
            <th>"Name"</th>
            <th>"Role"</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td>"Ada Lovelace"</td>
            <td>"Primary"</td>
        </tr>
        <tr>
            <td>"Grace Hopper"</td>
            <td>"Secondary"</td>
        </tr>
    </tbody>
</DtTable>"##
        >
            <DtTable caption="On call" striped=true>
                <thead>
                    <tr>
                        <th>"Name"</th>
                        <th>"Role"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"Ada Lovelace"</td>
                        <td>"Primary"</td>
                    </tr>
                    <tr>
                        <td>"Grace Hopper"</td>
                        <td>"Secondary"</td>
                    </tr>
                </tbody>
            </DtTable>
        </Example>
    }
}

#[component]
pub fn Catalog() -> impl IntoView {
    let sections = [
        ("breadcrumbs", "Breadcrumbs"),
        ("button", "Button"),
        ("button-group", "Button group"),
        ("split-button", "Split button"),
        ("dropdown", "Dropdown"),
        ("segmented-control", "Segmented control"),
        ("avatar", "Avatar"),
        ("presence", "Presence"),
        ("badge", "Badge"),
        ("card", "Card"),
        ("chip", "Chip"),
        ("codeblock", "Code block"),
        ("description-list", "Description list"),
        ("table", "Table"),
        ("item-layout", "Item layout"),
        ("keyboard-shortcut", "Keyboard shortcut"),
        ("lazy-show", "Lazy show"),
        ("link", "Link"),
        ("list-item", "List item"),
        ("list-item-group", "List item group"),
        ("skeleton", "Skeleton"),
        ("text", "Text"),
        ("text-list", "Text list"),
        ("prose", "Prose"),
        ("emoji", "Emoji"),
        ("emoji-picker", "Emoji picker"),
        ("emoji-text-wrapper", "Emoji text wrapper"),
        ("banner", "Banner"),
        ("empty-state", "Empty state"),
        ("loader", "Loader"),
        ("progress-circle", "Progress circle"),
        ("modal", "Modal"),
        ("mode-island", "Mode"),
        ("motion-text", "Motion text"),
        ("notice", "Notice"),
        ("pagination", "Pagination"),
        ("hovercard", "Hovercard"),
        ("popover", "Popover"),
        ("toast", "Toast"),
        ("tooltip", "Tooltip"),
        ("validation-messages", "Validation messages"),
        ("checkbox", "Checkbox"),
        ("checkbox-group", "Checkbox group"),
        ("combobox", "Combobox"),
        ("combobox-multi-select", "Combobox multi-select"),
        ("combobox-with-popover", "Combobox with popover"),
        ("datepicker", "Datepicker"),
        ("input", "Input"),
        ("input-group", "Input group"),
        ("radio", "Radio"),
        ("radio-group", "Radio group"),
        ("select-menu", "Select menu"),
        ("slider", "Slider"),
        ("toggle", "Toggle"),
        ("filter-pill", "Filter pill"),
        ("box", "Box"),
        ("collapsible", "Collapsible"),
        ("resizable", "Resizable"),
        ("rich-text-editor", "Rich text editor"),
        ("root-layout", "Root layout"),
        ("scroller", "Scroller"),
        ("stack", "Stack"),
        ("tab", "Tab"),
        ("tab-group", "Tab group"),
        ("icon", "Icon"),
        ("illustration", "Illustration"),
        ("image-viewer", "Image viewer"),
    ];
    view! {
        <div>
            {sections.into_iter().map(|(id, title)| {
                view! { <crate::chunks::LazySection id=id title=title /> }
            }).collect_view()}
        </div>
    }
}
