//! One lazy chunk per component. The shell never constructs a Dialtone
//! component. wasm-split puts each `#[lazy]` body in its own binary, and the
//! Trunk hook serves that binary the first time `load` runs.

use dialtone::{
    DtAvatar, DtBadge, DtBanner, DtBox, DtBreadcrumb, DtBreadcrumbs, DtButton, DtButtonGroup,
    DtCard, DtCheckbox, DtCheckboxGroup, DtChip, DtCodeblock, DtCollapsible, DtCombobox,
    DtComboboxMultiSelect, DtComboboxWithPopover, DtDatepicker, DtDescriptionList,
    DtDescriptionListItem, DtEmoji, DtEmojiPicker, DtEmojiTextWrapper, DtEmptyState, DtFilter,
    DtFilterPill, DtHovercard, DtIconBell, DtIconCheck, DtIconExternalLink, DtIconPlus,
    DtIllustration, DtInput, DtItemLayout, DtKeyboardShortcut, DtLazyShow, DtLink, DtListItem,
    DtListItemGroup, DtLoader, DtModeIsland, DtMotionText, DtNotice, DtPagination, DtPresence,
    DtProgressCircle, DtProse, DtRadio, DtRadioGroup, DtResizable, DtResizableHandle,
    DtResizablePanel, DtRichTextEditor, DtRootLayout, DtScroller, DtScrollerItem,
    DtSegmentedControl, DtSegmentedControlItem, DtSelectMenu, DtSelectOption, DtSkeleton,
    DtSkeletonParagraphOption, DtSlider, DtSliderValue, DtSplitButton, DtStack, DtTab, DtTabGroup,
    DtTabPanel, DtTable, DtText, DtTextList, DtTextListItem, DtToast, DtToggle, DtTooltip,
    DtValidationMessage, DtValidationMessages,
};
use leptos::html;
use leptos::prelude::*;
use leptos::task::spawn_local_scoped;
use std::ops::DerefMut;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

type ViewSlot = RwSignal<leptos::__reexports::send_wrapper::SendWrapper<Option<AnyView>>>;

const CATALOG: u8 = 0;
const DOCS: u8 = 1;

macro_rules! chunk {
    ($name:ident, catalog = $catalog:path, docs = $docs:path, thumb = $thumb:ident) => {
        #[lazy]
        fn $name(face: u8) -> AnyView {
            match face {
                0 => view! { <$catalog /> }.into_any(),
                1 => view! { <$docs /> }.into_any(),
                _ => view! { <$thumb /> }.into_any(),
            }
        }
    };
    ($name:ident, docs = $docs:path, thumb = $thumb:ident) => {
        #[lazy]
        fn $name(face: u8) -> AnyView {
            match face {
                1 => view! { <$docs /> }.into_any(),
                _ => view! { <$thumb /> }.into_any(),
            }
        }
    };
}

chunk!(
    breadcrumbs_chunk,
    catalog = crate::catalog::BreadcrumbsExample,
    docs = crate::docs::pages::BreadcrumbsPage,
    thumb = ThumbBreadcrumbs
);
chunk!(
    button_chunk,
    catalog = crate::catalog::ButtonExample,
    docs = crate::docs::pages::ButtonPage,
    thumb = ThumbButton
);
chunk!(
    button_group_chunk,
    catalog = crate::catalog::ButtonGroupExample,
    docs = crate::docs::pages::ButtonGroupPage,
    thumb = ThumbButtonGroup
);
chunk!(
    split_button_chunk,
    catalog = crate::catalog::SplitButtonExample,
    docs = crate::docs::pages::SplitButtonPage,
    thumb = ThumbSplit
);
chunk!(
    dropdown_chunk,
    catalog = crate::catalog::DropdownExample,
    docs = crate::docs::pages::DropdownPage,
    thumb = ThumbMenu
);
chunk!(
    segmented_chunk,
    catalog = crate::catalog::SegmentedControlExample,
    docs = crate::docs::pages::SegmentedControlPage,
    thumb = ThumbSegmented
);
chunk!(
    avatar_chunk,
    catalog = crate::catalog::AvatarExample,
    docs = crate::docs::pages::AvatarPage,
    thumb = ThumbAvatar
);
chunk!(
    presence_chunk,
    catalog = crate::catalog::PresenceExample,
    docs = crate::docs::pages::PresencePage,
    thumb = ThumbPresence
);
chunk!(
    badge_chunk,
    catalog = crate::catalog::BadgeExample,
    docs = crate::docs::pages::BadgePage,
    thumb = ThumbBadge
);
chunk!(
    card_chunk,
    catalog = crate::catalog::CardExample,
    docs = crate::docs::pages::CardPage,
    thumb = ThumbCard
);
chunk!(
    chip_chunk,
    catalog = crate::catalog::ChipExample,
    docs = crate::docs::pages::ChipPage,
    thumb = ThumbChip
);
chunk!(
    codeblock_chunk,
    catalog = crate::catalog::CodeblockExample,
    docs = crate::docs::pages::CodeblockPage,
    thumb = ThumbCodeblock
);
chunk!(
    description_list_chunk,
    catalog = crate::catalog::DescriptionListExample,
    docs = crate::docs::pages::DescriptionListPage,
    thumb = ThumbDescription
);
chunk!(
    item_layout_chunk,
    catalog = crate::catalog::ItemLayoutExample,
    docs = crate::docs::pages::ItemLayoutPage,
    thumb = ThumbItem
);
chunk!(
    keyboard_shortcut_chunk,
    catalog = crate::catalog::KeyboardShortcutExample,
    docs = crate::docs::pages::KeyboardShortcutPage,
    thumb = ThumbKeyboardShortcut
);
chunk!(
    lazy_show_chunk,
    catalog = crate::catalog::LazyShowExample,
    docs = crate::docs::pages::LazyShowPage,
    thumb = ThumbLazyShow
);
chunk!(
    link_chunk,
    catalog = crate::catalog::LinkExample,
    docs = crate::docs::pages::LinkPage,
    thumb = ThumbLink
);
chunk!(
    list_item_chunk,
    catalog = crate::catalog::ListItemExample,
    docs = crate::docs::pages::ListItemPage,
    thumb = ThumbListItem
);
chunk!(
    list_item_group_chunk,
    catalog = crate::catalog::ListItemGroupExample,
    docs = crate::docs::pages::ListItemGroupPage,
    thumb = ThumbListItemGroup
);
chunk!(
    skeleton_chunk,
    catalog = crate::catalog::SkeletonExample,
    docs = crate::docs::pages::SkeletonPage,
    thumb = ThumbSkeleton
);
chunk!(
    text_chunk,
    catalog = crate::catalog::TextExample,
    docs = crate::docs::pages::TextPage,
    thumb = ThumbText
);
chunk!(
    text_list_chunk,
    catalog = crate::catalog::TextListExample,
    docs = crate::docs::pages::TextListPage,
    thumb = ThumbTextList
);
chunk!(
    prose_chunk,
    catalog = crate::catalog::ProseExample,
    docs = crate::docs::pages::ProsePage,
    thumb = ThumbProse
);
chunk!(
    emoji_chunk,
    catalog = crate::catalog::EmojiExample,
    docs = crate::docs::pages::EmojiPage,
    thumb = ThumbEmoji
);
chunk!(
    emoji_picker_chunk,
    catalog = crate::catalog::EmojiPickerExample,
    docs = crate::docs::pages::EmojiPickerPage,
    thumb = ThumbEmojiPicker
);
chunk!(
    emoji_text_chunk,
    catalog = crate::catalog::EmojiTextWrapperExample,
    docs = crate::docs::pages::EmojiTextWrapperPage,
    thumb = ThumbEmojiText
);
chunk!(
    banner_chunk,
    catalog = crate::catalog::BannerExample,
    docs = crate::docs::pages::BannerPage,
    thumb = ThumbBanner
);
chunk!(
    empty_state_chunk,
    catalog = crate::catalog::EmptyStateExample,
    docs = crate::docs::pages::EmptyStatePage,
    thumb = ThumbEmpty
);
chunk!(
    loader_chunk,
    catalog = crate::catalog::LoaderExample,
    docs = crate::docs::pages::LoaderPage,
    thumb = ThumbLoader
);
chunk!(
    progress_circle_chunk,
    catalog = crate::catalog::ProgressCircleExample,
    docs = crate::docs::pages::ProgressCirclePage,
    thumb = ThumbProgressCircle
);
chunk!(
    modal_chunk,
    catalog = crate::catalog::ModalExample,
    docs = crate::docs::pages::ModalPage,
    thumb = ThumbOpen
);
chunk!(
    mode_island_chunk,
    catalog = crate::catalog::ModeIslandExample,
    docs = crate::docs::pages::ModeIslandPage,
    thumb = ThumbModeIsland
);
chunk!(
    motion_text_chunk,
    catalog = crate::catalog::MotionTextExample,
    docs = crate::docs::pages::MotionTextPage,
    thumb = ThumbMotion
);
chunk!(
    notice_chunk,
    catalog = crate::catalog::NoticeExample,
    docs = crate::docs::pages::NoticePage,
    thumb = ThumbNotice
);
chunk!(
    pagination_chunk,
    catalog = crate::catalog::PaginationExample,
    docs = crate::docs::pages::PaginationPage,
    thumb = ThumbPagination
);
chunk!(
    hovercard_chunk,
    catalog = crate::catalog::HovercardExample,
    docs = crate::docs::pages::HovercardPage,
    thumb = ThumbHovercard
);
chunk!(
    popover_chunk,
    catalog = crate::catalog::PopoverExample,
    docs = crate::docs::pages::PopoverPage,
    thumb = ThumbPopover
);
chunk!(
    toast_chunk,
    catalog = crate::catalog::ToastExample,
    docs = crate::docs::pages::ToastPage,
    thumb = ThumbToast
);
chunk!(
    tooltip_chunk,
    catalog = crate::catalog::TooltipExample,
    docs = crate::docs::pages::TooltipPage,
    thumb = ThumbTooltip
);
chunk!(
    validation_chunk,
    catalog = crate::catalog::ValidationMessagesExample,
    docs = crate::docs::pages::ValidationMessagesPage,
    thumb = ThumbValidation
);
chunk!(
    checkbox_chunk,
    catalog = crate::catalog::CheckboxExample,
    docs = crate::docs::pages::CheckboxPage,
    thumb = ThumbCheckbox
);
chunk!(
    checkbox_group_chunk,
    catalog = crate::catalog::CheckboxGroupExample,
    docs = crate::docs::pages::CheckboxGroupPage,
    thumb = ThumbChecks
);
chunk!(
    combobox_chunk,
    catalog = crate::catalog::ComboboxExample,
    docs = crate::docs::pages::ComboboxPage,
    thumb = ThumbCombo
);
chunk!(
    combobox_multi_chunk,
    catalog = crate::catalog::ComboboxMultiExample,
    docs = crate::docs::pages::MultiSelectPage,
    thumb = ThumbMulti
);
chunk!(
    combobox_popover_chunk,
    catalog = crate::catalog::ComboboxPopoverExample,
    docs = crate::docs::pages::ComboboxPopoverPage,
    thumb = ThumbComboPopover
);
chunk!(
    datepicker_chunk,
    catalog = crate::catalog::DatepickerExample,
    docs = crate::docs::pages::DatepickerPage,
    thumb = ThumbDatepicker
);
chunk!(
    input_chunk,
    catalog = crate::catalog::InputExample,
    docs = crate::docs::pages::InputPage,
    thumb = ThumbInput
);
chunk!(
    input_group_chunk,
    catalog = crate::catalog::InputGroupExample,
    docs = crate::docs::pages::InputGroupPage,
    thumb = ThumbInput
);
chunk!(
    radio_chunk,
    catalog = crate::catalog::RadioExample,
    docs = crate::docs::pages::RadioPage,
    thumb = ThumbRadio
);
chunk!(
    radio_group_chunk,
    catalog = crate::catalog::RadioGroupExample,
    docs = crate::docs::pages::RadioGroupPage,
    thumb = ThumbRadios
);
chunk!(
    select_chunk,
    catalog = crate::catalog::SelectMenuExample,
    docs = crate::docs::pages::SelectMenuPage,
    thumb = ThumbSelect
);
chunk!(
    toggle_chunk,
    catalog = crate::catalog::ToggleExample,
    docs = crate::docs::pages::TogglePage,
    thumb = ThumbSwitch
);
chunk!(
    filter_pill_chunk,
    catalog = crate::catalog::FilterPillExample,
    docs = crate::docs::pages::FilterPillPage,
    thumb = ThumbFilter
);
chunk!(
    box_chunk,
    catalog = crate::catalog::BoxExample,
    docs = crate::docs::pages::BoxPage,
    thumb = ThumbBox
);
chunk!(
    collapsible_chunk,
    catalog = crate::catalog::CollapsibleExample,
    docs = crate::docs::pages::CollapsiblePage,
    thumb = ThumbCollapsible
);
chunk!(
    resizable_chunk,
    catalog = crate::catalog::ResizableExample,
    docs = crate::docs::pages::ResizablePage,
    thumb = ThumbResizable
);
chunk!(
    rich_text_editor_chunk,
    catalog = crate::catalog::RichTextEditorExample,
    docs = crate::docs::pages::RichTextEditorPage,
    thumb = ThumbRichText
);
chunk!(
    root_layout_chunk,
    catalog = crate::catalog::RootLayoutExample,
    docs = crate::docs::pages::RootLayoutPage,
    thumb = ThumbLayout
);
chunk!(
    scroller_chunk,
    catalog = crate::catalog::ScrollerExample,
    docs = crate::docs::pages::ScrollerPage,
    thumb = ThumbScroller
);
chunk!(
    stack_chunk,
    catalog = crate::catalog::StackExample,
    docs = crate::docs::pages::StackPage,
    thumb = ThumbStack
);
chunk!(
    tab_chunk,
    catalog = crate::catalog::TabExample,
    docs = crate::docs::pages::TabPage,
    thumb = ThumbTab
);
chunk!(
    tab_group_chunk,
    catalog = crate::catalog::TabGroupExample,
    docs = crate::docs::pages::TabGroupPage,
    thumb = ThumbTabGroup
);
chunk!(
    icon_chunk,
    catalog = crate::catalog::IconExample,
    docs = crate::docs::pages::IconPage,
    thumb = ThumbIcon
);
chunk!(
    illustration_chunk,
    catalog = crate::catalog::IllustrationExample,
    docs = crate::docs::pages::IllustrationPage,
    thumb = ThumbIllustration
);
chunk!(
    image_viewer_chunk,
    catalog = crate::catalog::ImageViewerExample,
    docs = crate::docs::pages::ImageViewerPage,
    thumb = ThumbImageViewer
);
chunk!(
    slider_chunk,
    catalog = crate::catalog::SliderExample,
    docs = crate::docs::pages::SliderPage,
    thumb = ThumbSlider
);
chunk!(
    table_chunk,
    catalog = crate::catalog::TableExample,
    docs = crate::docs::pages::TablePage,
    thumb = ThumbTable
);

pub async fn load(id: &str, face: u8) -> AnyView {
    match id {
        "breadcrumbs" => breadcrumbs_chunk(face).await,
        "button" => button_chunk(face).await,
        "button-group" => button_group_chunk(face).await,
        "split-button" => split_button_chunk(face).await,
        "dropdown" => dropdown_chunk(face).await,
        "segmented-control" => segmented_chunk(face).await,
        "avatar" => avatar_chunk(face).await,
        "presence" => presence_chunk(face).await,
        "badge" => badge_chunk(face).await,
        "card" => card_chunk(face).await,
        "chip" => chip_chunk(face).await,
        "codeblock" => codeblock_chunk(face).await,
        "description-list" => description_list_chunk(face).await,
        "table" => table_chunk(face).await,
        "item-layout" => item_layout_chunk(face).await,
        "keyboard-shortcut" => keyboard_shortcut_chunk(face).await,
        "lazy-show" => lazy_show_chunk(face).await,
        "link" => link_chunk(face).await,
        "list-item" => list_item_chunk(face).await,
        "list-item-group" => list_item_group_chunk(face).await,
        "skeleton" => skeleton_chunk(face).await,
        "text" => text_chunk(face).await,
        "text-list" => text_list_chunk(face).await,
        "prose" => prose_chunk(face).await,
        "emoji" => emoji_chunk(face).await,
        "emoji-picker" => emoji_picker_chunk(face).await,
        "emoji-text-wrapper" => emoji_text_chunk(face).await,
        "banner" => banner_chunk(face).await,
        "empty-state" => empty_state_chunk(face).await,
        "loader" => loader_chunk(face).await,
        "progress-circle" => progress_circle_chunk(face).await,
        "modal" => modal_chunk(face).await,
        "mode-island" => mode_island_chunk(face).await,
        "motion-text" => motion_text_chunk(face).await,
        "notice" => notice_chunk(face).await,
        "pagination" => pagination_chunk(face).await,
        "hovercard" => hovercard_chunk(face).await,
        "popover" => popover_chunk(face).await,
        "toast" => toast_chunk(face).await,
        "tooltip" => tooltip_chunk(face).await,
        "validation-messages" => validation_chunk(face).await,
        "checkbox" => checkbox_chunk(face).await,
        "checkbox-group" => checkbox_group_chunk(face).await,
        "combobox" => combobox_chunk(face).await,
        "combobox-multi-select" => combobox_multi_chunk(face).await,
        "combobox-with-popover" => combobox_popover_chunk(face).await,
        "datepicker" => datepicker_chunk(face).await,
        "input" => input_chunk(face).await,
        "input-group" => input_group_chunk(face).await,
        "radio" => radio_chunk(face).await,
        "radio-group" => radio_group_chunk(face).await,
        "select-menu" => select_chunk(face).await,
        "slider" => slider_chunk(face).await,
        "toggle" => toggle_chunk(face).await,
        "filter-pill" => filter_pill_chunk(face).await,
        "box" => box_chunk(face).await,
        "collapsible" => collapsible_chunk(face).await,
        "resizable" => resizable_chunk(face).await,
        "rich-text-editor" => rich_text_editor_chunk(face).await,
        "root-layout" => root_layout_chunk(face).await,
        "scroller" => scroller_chunk(face).await,
        "stack" => stack_chunk(face).await,
        "tab" => tab_chunk(face).await,
        "tab-group" => tab_group_chunk(face).await,
        "icon" => icon_chunk(face).await,
        "illustration" => illustration_chunk(face).await,
        "image-viewer" => image_viewer_chunk(face).await,
        _ => view! { <p>"This component page is not available."</p> }.into_any(),
    }
}

fn empty_slot() -> ViewSlot {
    RwSignal::new(leptos::__reexports::send_wrapper::SendWrapper::new(None))
}

fn start_load(owner: &Owner, id: &'static str, face: u8, slot: ViewSlot, ready: RwSignal<bool>) {
    owner.with(|| {
        spawn_local_scoped(async move {
            let view = load(id, face).await;
            slot.update(|holder| {
                *holder = leptos::__reexports::send_wrapper::SendWrapper::new(Some(view))
            });
            ready.set(true);
        });
    });
}

fn hash_targets(id: &str) -> bool {
    web_sys::window()
        .and_then(|window| window.location().hash().ok())
        .is_some_and(|hash| hash.trim_start_matches('#') == id)
}

fn mount_when_visible(
    id: &'static str,
    face: u8,
    node: NodeRef<html::Div>,
    slot: ViewSlot,
    ready: RwSignal<bool>,
    immediate: bool,
) {
    let Some(owner) = Owner::current() else {
        return;
    };
    let started = StoredValue::new(false);
    let observer_slot: StoredValue<Option<web_sys::IntersectionObserver>, LocalStorage> =
        StoredValue::new_local(None);
    on_cleanup(move || {
        if let Some(observer) = observer_slot.try_get_value().flatten() {
            observer.disconnect();
        }
    });
    Effect::new(move |_| {
        if started.get_value() {
            return;
        }
        if immediate || hash_targets(id) {
            started.set_value(true);
            start_load(&owner, id, face, slot, ready);
            return;
        }
        let Some(element) = node.get() else {
            return;
        };
        started.set_value(true);
        let callback_owner = owner.clone();
        let closure = Closure::wrap(Box::new(
            move |entries: js_sys::Array, observer: web_sys::IntersectionObserver| {
                let visible = entries.iter().any(|entry| {
                    let entry: web_sys::IntersectionObserverEntry = entry.unchecked_into();
                    entry.is_intersecting()
                });
                if !visible {
                    return;
                }
                observer.disconnect();
                start_load(&callback_owner, id, face, slot, ready);
            },
        )
            as Box<dyn FnMut(js_sys::Array, web_sys::IntersectionObserver)>);
        let options = web_sys::IntersectionObserverInit::new();
        options.set_root_margin("240px 0px");
        let observer = web_sys::IntersectionObserver::new_with_options(
            closure.as_ref().unchecked_ref(),
            &options,
        );
        let Ok(observer) = observer else {
            start_load(&owner, id, face, slot, ready);
            return;
        };
        observer.observe(&element);
        closure.forget();
        observer_slot.set_value(Some(observer));
    });
}

fn chunk_view(
    slot: ViewSlot,
    ready: RwSignal<bool>,
    pending: impl Fn() -> AnyView + Send + Sync + Clone + 'static,
) -> impl IntoView {
    view! {
        {move || {
            if !ready.get() {
                return pending();
            }
            slot.try_update(|holder| holder.deref_mut().take())
                .flatten()
                .unwrap_or_else(|| ().into_any())
        }}
    }
}

#[component]
pub(crate) fn LazySection(id: &'static str, title: &'static str) -> impl IntoView {
    let slot = empty_slot();
    let ready = RwSignal::new(false);
    let node = NodeRef::<html::Div>::new();
    mount_when_visible(id, CATALOG, node, slot, ready, false);
    let title = title.to_string();
    view! {
        <div id=id class="scroll-mt-6" node_ref=node>
            {chunk_view(slot, ready, move || {
                let title = title.clone();
                view! {
                    <section>
                        <h2>{title}</h2>
                        <p class="demo-pending">"Loading…"</p>
                    </section>
                }
                .into_any()
            })}
        </div>
    }
}

#[component]
pub(crate) fn LazyDocs(id: &'static str) -> impl IntoView {
    let slot = empty_slot();
    let ready = RwSignal::new(false);
    let node = NodeRef::<html::Div>::new();
    mount_when_visible(id, DOCS, node, slot, ready, true);
    view! {
        <div node_ref=node>
            {chunk_view(slot, ready, || {
                view! { <p id="playground" class="docs-note">"Loading component…"</p> }.into_any()
            })}
        </div>
    }
}

#[component]
pub(crate) fn LazyThumb(id: &'static str) -> impl IntoView {
    let slot = empty_slot();
    let ready = RwSignal::new(false);
    let node = NodeRef::<html::Div>::new();
    mount_when_visible(id, 2, node, slot, ready, false);
    view! {
        <div class="docs-thumb" node_ref=node>
            {chunk_view(slot, ready, || {
                view! { <span class="docs-note">"Loading…"</span> }.into_any()
            })}
        </div>
    }
}

#[component]
fn ThumbBreadcrumbs() -> impl IntoView {
    view! {
        <DtBreadcrumbs breadcrumbs=vec![
            DtBreadcrumb::new("#", "Root"),
            DtBreadcrumb::new("#", "Page").selected(),
        ] />
    }
}

#[component]
fn ThumbButton() -> impl IntoView {
    view! { <DtButton size="sm">"Save"</DtButton> }
}

#[component]
fn ThumbButtonGroup() -> impl IntoView {
    view! {
        <DtButtonGroup>
            <DtButton importance="outlined" size="sm" active=true>"Week"</DtButton>
            <DtButton importance="outlined" size="sm">"Month"</DtButton>
        </DtButtonGroup>
    }
}

#[component]
fn ThumbMenu() -> impl IntoView {
    view! { <DtButton importance="outlined" size="sm">"Menu"</DtButton> }
}

#[component]
fn ThumbOpen() -> impl IntoView {
    view! { <DtButton size="sm">"Open"</DtButton> }
}

#[component]
fn ThumbAvatar() -> impl IntoView {
    view! {
        <DtAvatar
            full_name="Ada Lovelace"
            presence="active"
            image_src="/assets/images/person.png"
            image_alt="Ada Lovelace".to_string()
        />
    }
}

#[component]
fn ThumbPresence() -> impl IntoView {
    view! {
        <DtPresence presence="active" />
        <DtPresence presence="away" />
        <DtPresence presence="busy" />
        <DtPresence presence="dnd" />
    }
}

#[component]
fn ThumbBadge() -> impl IntoView {
    view! { <DtBadge text="Info" type_="info" /> }
}

#[component]
fn ThumbCard() -> impl IntoView {
    view! {
        <DtCard container_class="w-full">
            <p class="m-0 text-sm">"On call this week"</p>
        </DtCard>
    }
}

#[component]
fn ThumbChip() -> impl IntoView {
    view! { <DtChip show_close=false>"Voice"</DtChip> }
}

#[component]
fn ThumbCodeblock() -> impl IntoView {
    view! { <DtCodeblock text="<DtButton />" /> }
}

#[component]
fn ThumbDescription() -> impl IntoView {
    view! { <DtDescriptionList items=vec![DtDescriptionListItem::new("Service", "Voice")] /> }
}

#[component]
fn ThumbSkeleton() -> impl IntoView {
    view! { <DtSkeleton paragraph_option=DtSkeletonParagraphOption { rows: 3, ..Default::default() } /> }
}

#[component]
fn ThumbBanner() -> impl IntoView {
    view! {
        <DtBanner header_text="Rotation updated" kind="positive" show_close=false>
            "The primary on-call changed."
        </DtBanner>
    }
}

#[component]
fn ThumbMotion() -> impl IntoView {
    view! {
        <DtMotionText text="Motion".to_string() animation_mode="shimmer" r#loop=true />
    }
}

#[component]
fn ThumbNotice() -> impl IntoView {
    view! {
        <DtNotice
            header_text="Saved"
            kind="positive"
            show_close=false
            show_action=false
        >
            "Schedule saved"
        </DtNotice>
    }
}

#[component]
fn ThumbHovercard() -> impl IntoView {
    view! {
        <DtHovercard
            anchor=ViewFn::from(|| view! {
                <DtButton importance="outlined" kind="muted" size="sm">"Hover"</DtButton>
            })
        >
            <p class="m-0 text-sm">"Preview"</p>
        </DtHovercard>
    }
}

#[component]
fn ThumbPagination() -> impl IntoView {
    view! {
        <DtPagination aria_label="pagination".to_string() total_pages=5 />
    }
}

#[component]
fn ThumbKeyboardShortcut() -> impl IntoView {
    view! { <DtKeyboardShortcut shortcut="{cmd}+K" /> }
}

#[component]
fn ThumbLazyShow() -> impl IntoView {
    let show = Signal::derive(|| true);
    view! { <DtLazyShow show=show>"Lazy"</DtLazyShow> }
}

#[component]
fn ThumbLink() -> impl IntoView {
    view! { <DtLink href="#link">"Link"</DtLink> }
}

#[component]
fn ThumbLoader() -> impl IntoView {
    view! { <DtLoader size="500" /> }
}

#[component]
fn ThumbProgressCircle() -> impl IntoView {
    view! { <DtProgressCircle aria_label="Upload progress" progress=66.0 /> }
}

#[component]
fn ThumbPopover() -> impl IntoView {
    view! { <DtButton importance="outlined" size="sm">"Details"</DtButton> }
}

#[component]
fn ThumbToast() -> impl IntoView {
    let open = RwSignal::new(true);
    view! {
        <DtToast
            open=open
            header_text="Saved"
            message="Schedule saved"
            kind="positive"
            show_close=false
            show_action=false
        />
    }
}

#[component]
fn ThumbTooltip() -> impl IntoView {
    view! {
        <DtTooltip message="Schedule id">
            <DtButton importance="outlined" size="sm">"Hover"</DtButton>
        </DtTooltip>
    }
}

#[component]
fn ThumbCheckbox() -> impl IntoView {
    view! { <DtCheckbox label="Email alerts" /> }
}

#[component]
fn ThumbRadio() -> impl IntoView {
    view! {
        <DtRadio
            label="To voicemail"
            value="voicemail"
            name="thumb-radio"
            checked=Signal::derive(|| true)
        />
    }
}

#[component]
fn ThumbRadios() -> impl IntoView {
    let selected = RwSignal::new("apple".to_string());
    view! {
        <DtRadioGroup
            name="thumb-fruits"
            legend="Fruits"
            selected=selected
            on_change=move |next| selected.set(next)
        >
            <DtRadio value="apple" label="Apple" />
            <DtRadio value="banana" label="Banana" />
        </DtRadioGroup>
    }
}

#[component]
fn ThumbChecks() -> impl IntoView {
    let selected = RwSignal::new(vec!["email".to_string()]);
    view! {
        <DtCheckboxGroup
            name="thumb-channels"
            legend="Channels"
            selected_values=selected
            on_change=move |next| selected.set(next)
        >
            <DtCheckbox value="email" label="Email" />
            <DtCheckbox value="sms" label="SMS" />
        </DtCheckboxGroup>
    }
}

#[component]
fn ThumbCombo() -> impl IntoView {
    let value = RwSignal::new(String::new());
    let input = ViewFn::from(move || {
        view! {
            <DtInput
                label="Product"
                placeholder="Search"
                value=value
                on_input=move |next| value.set(next)
            />
        }
    });
    view! {
        <DtCombobox label="Product".to_string() input=input />
    }
}

#[component]
fn ThumbComboPopover() -> impl IntoView {
    let value = RwSignal::new(String::new());
    view! {
        <DtComboboxWithPopover
            label="Product".to_string()
            placeholder="Search".to_string()
            value=value
            on_input=move |next| value.set(next)
            append_to="parent"
        >
            <li class="d-list-item" role="option">"Voice"</li>
        </DtComboboxWithPopover>
    }
}

#[component]
fn ThumbMulti() -> impl IntoView {
    let selected = RwSignal::new(vec!["Voice".to_string()]);
    view! {
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
    }
}

#[component]
fn ThumbSwitch() -> impl IntoView {
    let on = RwSignal::new(true);
    view! { <DtToggle checked=on aria_label="Sync" on_change=move |next| on.set(next) /> }
}

#[component]
fn ThumbInput() -> impl IntoView {
    let value = RwSignal::new("Ada".to_string());
    view! {
        <div class="w-full">
            <DtInput label="Name" value=value on_input=move |next| value.set(next) />
        </div>
    }
}

#[component]
fn ThumbItem() -> impl IntoView {
    view! {
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
        >
            "Ada Lovelace"
        </DtItemLayout>
    }
}

#[component]
fn ThumbListItem() -> impl IntoView {
    view! {
        <DtListItem
            navigation_type="tab"
            start=ViewFn::from(|| view! { <DtIconCheck size="300" /> })
            end=ViewFn::from(|| view! { <DtIconExternalLink size="300" /> })
        >
            "Default List Item"
        </DtListItem>
    }
}

#[component]
fn ThumbListItemGroup() -> impl IntoView {
    view! {
        <DtListItemGroup heading="Example Heading">
            <DtListItem role="option" navigation_type="tab" class="d-pis-100">"Item 1"</DtListItem>
            <DtListItem role="option" navigation_type="tab" class="d-pis-100">"Item 2"</DtListItem>
        </DtListItemGroup>
    }
}

#[component]
fn ThumbBox() -> impl IntoView {
    view! {
        <DtBox padding="200" surface="secondary" border_width="100" border_radius="300">
            <p class="m-0 text-sm">"Box"</p>
        </DtBox>
    }
}

#[component]
fn ThumbCollapsible() -> impl IntoView {
    view! {
        <DtCollapsible anchor_text="Schedule">
            <p class="m-0 text-sm">"Weekly rotation"</p>
        </DtCollapsible>
    }
}

#[component]
fn ThumbResizable() -> impl IntoView {
    view! {
        <div class="w-full" style="block-size:96px">
            <DtResizable>
                <DtResizablePanel id="one">
                    <p class="m-0 text-xs">"One"</p>
                </DtResizablePanel>
                <DtResizableHandle before_panel_id="one" after_panel_id="two" disabled=true />
                <DtResizablePanel id="two">
                    <p class="m-0 text-xs">"Two"</p>
                </DtResizablePanel>
            </DtResizable>
        </div>
    }
}

#[component]
fn ThumbRichText() -> impl IntoView {
    let value = RwSignal::new("Hello".to_string());
    view! {
        <div class="w-full">
            <DtRichTextEditor
                value=value
                input_aria_label="Message"
                editable=false
                output_format="text"
            />
        </div>
    }
}

#[component]
fn ThumbLayout() -> impl IntoView {
    view! {
        <div class="docs-layout-frame docs-layout-frame--thumb">
            <DtRootLayout
                header=ViewFn::from(|| view! { <div class="docs-layout-bar">"Header"</div> })
                sidebar=ViewFn::from(|| view! { <div class="docs-layout-bar">"Side"</div> })
            >
                <p class="m-0 text-sm">"Content"</p>
            </DtRootLayout>
        </div>
    }
}

#[component]
fn ThumbIcon() -> impl IntoView {
    view! {
        <span class="docs-thumb-icons">
            <DtIconBell />
            <DtIconPlus />
        </span>
    }
}

#[component]
fn ThumbIllustration() -> impl IntoView {
    view! { <DtIllustration name="mind" /> }
}

#[component]
fn ThumbImageViewer() -> impl IntoView {
    view! {
        <img
            src="/assets/images/person.png"
            alt=""
            class="d-wmn-100 d-hmn-100 d-wmx-500 d-hmx-500"
        />
    }
}

#[component]
fn ThumbDatepicker() -> impl IntoView {
    view! { <DtDatepicker /> }
}

#[component]
fn ThumbSplit() -> impl IntoView {
    view! { <DtSplitButton omega_tooltip_text="More" size="sm">"Save"</DtSplitButton> }
}

#[component]
fn ThumbSegmented() -> impl IntoView {
    let value = RwSignal::new("week".to_string());
    view! {
        <DtSegmentedControl aria_label="Range" value=value on_change=move |next| value.set(next)>
            <DtSegmentedControlItem value="day">"Day"</DtSegmentedControlItem>
            <DtSegmentedControlItem value="week">"Week"</DtSegmentedControlItem>
        </DtSegmentedControl>
    }
}

#[component]
fn ThumbText() -> impl IntoView {
    view! { <DtText variant="body-md">"On call"</DtText> }
}

#[component]
fn ThumbTextList() -> impl IntoView {
    view! {
        <DtTextList>
            <DtTextListItem>"Voice"</DtTextListItem>
            <DtTextListItem>"Meetings"</DtTextListItem>
        </DtTextList>
    }
}

#[component]
fn ThumbProse() -> impl IntoView {
    view! {
        <DtProse>
            <p>"Body text"</p>
        </DtProse>
    }
}

#[component]
fn ThumbModeIsland() -> impl IntoView {
    view! {
        <DtModeIsland class="d-px-200 d-py-100 d-bar-300">
            "Aa"
        </DtModeIsland>
    }
}

#[component]
fn ThumbValidation() -> impl IntoView {
    view! {
        <DtValidationMessages validation_messages=vec![DtValidationMessage::critical("Choose a product")] />
    }
}

#[component]
fn ThumbSelect() -> impl IntoView {
    let value = RwSignal::new("voice".to_string());
    view! {
        <DtSelectMenu
            label="Product"
            value=value
            on_change=move |next| value.set(next)
            options=vec![DtSelectOption::new("voice", "Voice"), DtSelectOption::new("sms", "SMS")]
        />
    }
}

#[component]
fn ThumbScroller() -> impl IntoView {
    let items = RwSignal::new(
        (1..=12)
            .map(|index| {
                DtScrollerItem::new(
                    index.to_string(),
                    ViewFn::from(
                        move || view! { <div class="px-2 text-sm">{format!("Row {index}")}</div> },
                    ),
                )
            })
            .collect::<Vec<_>>(),
    );
    view! { <DtScroller items=items item_size=28.0 scroller_height=96 /> }
}

#[component]
fn ThumbStack() -> impl IntoView {
    view! {
        <DtStack direction="row" gap="200" align="center">
            <DtBadge text="Voice" />
            <DtBadge text="SMS" />
        </DtStack>
    }
}

#[component]
fn ThumbTab() -> impl IntoView {
    view! {
        <DtTab id="overview" panel_id="overview" selected=true>"Overview"</DtTab>
    }
}

#[component]
fn ThumbTabGroup() -> impl IntoView {
    let selected = Signal::derive(|| "overview".to_string());
    view! {
        <DtTabGroup
            label="Sections"
            selected=selected
            tabs=ViewFn::from(|| {
                view! {
                    <DtTab id="overview" panel_id="overview">"Overview"</DtTab>
                    <DtTab id="people" panel_id="people">"People"</DtTab>
                }
            })
        >
            <DtTabPanel id="overview" tab_id="overview">"Overview"</DtTabPanel>
            <DtTabPanel id="people" tab_id="people">"People"</DtTabPanel>
        </DtTabGroup>
    }
}

#[component]
fn ThumbEmoji() -> impl IntoView {
    view! {
        <span class="docs-thumb-icons">
            <DtEmoji code=":smile:" />
            <DtEmoji code=":grinning:" />
        </span>
    }
}

#[component]
fn ThumbEmojiPicker() -> impl IntoView {
    view! { <DtEmojiPicker show_search=false /> }
}

#[component]
fn ThumbEmojiText() -> impl IntoView {
    view! { <DtEmojiTextWrapper text="Nice work :smile:" /> }
}

#[component]
fn ThumbEmpty() -> impl IntoView {
    view! {
        <DtEmptyState
            size="sm"
            header_text="No incidents"
            body_text="Alerts land here."
            icon=ViewFn::from(|| view! { <DtIconBell size="800" /> })
        />
    }
}

#[component]
fn ThumbFilter() -> impl IntoView {
    let filters = RwSignal::new(vec![
        DtFilter {
            name: "Voice".to_string(),
            active: true,
        },
        DtFilter {
            name: "SMS".to_string(),
            active: false,
        },
    ]);
    view! {
        <DtFilterPill
            filters=filters.into()
            label="Channel"
            on_change=move |next| filters.set(next)
        />
    }
}

#[component]
fn ThumbSlider() -> impl IntoView {
    let volume = RwSignal::new(DtSliderValue::Single(40.0));
    view! {
        <DtSlider
            label="Volume"
            value=volume
            on_input=Callback::new(move |next| volume.set(next))
            style="inline-size: 8rem"
        />
    }
}

#[component]
fn ThumbTable() -> impl IntoView {
    view! {
        <DtTable caption="On call" striped=true>
            <thead>
                <tr>
                    <th>"Name"</th>
                    <th>"Role"</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>"Ada"</td>
                    <td>"Primary"</td>
                </tr>
            </tbody>
        </DtTable>
    }
}
