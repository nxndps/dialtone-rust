//! Leptos ports of Dialtone Vue components.
//!
//! Import like `@dialpad/dialtone-vue`:
//!
//! ```ignore
//! use dialtone::{DtAvatar, DtBadge, DtBanner, DtButton, DtButtonGroup, DtCard};
//! use dialtone::{DtCheckbox, DtCheckboxGroup, DtChip, DtCodeblock, DtCollapsible};
//! use dialtone::{DtCombobox, DtComboboxMultiSelect, DtComboboxWithPopover, DtDatepicker};
//! use dialtone::{DtDescriptionList, DtDropdown, DtEmoji, DtEmojiPicker, DtEmojiTextWrapper};
//! use dialtone::{DtEmptyState, DtFilterPill, DtInput, DtPopover, DtResizable};
//! use dialtone::{DtResizablePanel, DtResizableHandle, DtRichTextEditor, DtScroller};
//! use dialtone::{DtTab, DtTabGroup, DtTabPanel};
//! use dialtone::{DtSegmentedControl, DtSelectMenu, DtSplitButton, DtStack, DtText};
//! use dialtone::{DtTextList, DtTextListItem, DtTooltip, DtValidationMessages};
//! ```
//!
//! The crate renders Dialtone class names. The app supplies Dialtone CSS.

use std::sync::atomic::{AtomicU32, Ordering};

pub mod avatar;
pub mod badge;
pub mod banner;
#[path = "box.rs"]
pub mod r#box;
pub mod breadcrumbs;
pub mod button;
pub mod button_group;
pub mod card;
pub mod checkbox;
pub mod checkbox_group;
pub mod chip;
pub mod codeblock;
pub mod collapsible;
pub mod combobox;
pub mod combobox_multi_select;
pub mod combobox_with_popover;
pub mod datepicker;
pub mod description_list;
pub mod dropdown;
pub mod emoji;
pub mod emoji_picker;
pub mod emoji_text_wrapper;
pub mod empty_state;
pub mod filter_pill;
pub mod hovercard;
pub mod icon;
pub mod illustration;
pub mod image_viewer;
pub mod input;
pub mod input_group;
pub mod item_layout;
pub mod keyboard_shortcut;
pub mod lazy_show;
pub mod link;
pub mod list_item;
pub mod list_item_group;
mod list_nav;
pub mod loader;
pub mod modal;
pub mod mode_island;
pub mod motion_text;
pub mod notice;
pub mod notice_action;
pub mod notice_content;
pub mod notice_icon;
pub mod pagination;
pub mod popover;
pub mod popper;
pub mod presence;
pub mod progress_circle;
pub mod prose;
pub mod radio;
pub mod radio_group;
pub mod resizable;
pub mod rich_text_editor;
pub mod root_layout;
pub mod scroller;
pub mod segmented_control;
pub mod select_menu;
pub mod skeleton;
pub mod split_button;
pub mod stack;
pub mod tab;
pub mod text;
pub mod text_list;
pub mod toast;
pub mod toggle;
pub mod tooltip;
mod validation;

pub use avatar::*;
pub use badge::*;
pub use banner::*;
pub use breadcrumbs::*;
pub use button::*;
pub use button_group::*;
pub use card::*;
pub use checkbox::*;
pub use checkbox_group::*;
pub use chip::*;
pub use codeblock::*;
pub use collapsible::*;
pub use combobox::*;
pub use combobox_multi_select::*;
pub use combobox_with_popover::*;
pub use datepicker::*;
pub use description_list::*;
pub use dropdown::*;
pub use emoji::*;
pub use emoji_picker::*;
pub use emoji_text_wrapper::*;
pub use empty_state::*;
pub use filter_pill::*;
pub use hovercard::*;
pub use icon::*;
pub use illustration::*;
pub use image_viewer::*;
pub use input::*;
pub use input_group::*;
pub use item_layout::*;
pub use keyboard_shortcut::*;
pub use lazy_show::*;
pub use link::*;
pub use list_item::*;
pub use list_item_group::*;
pub use loader::*;
pub use modal::*;
pub use mode_island::*;
pub use motion_text::*;
pub use notice::*;
pub use notice_action::*;
pub use notice_content::*;
pub use notice_icon::*;
pub use pagination::*;
pub use popover::*;
pub use presence::*;
pub use progress_circle::*;
pub use prose::*;
pub use r#box::*;
pub use radio::*;
pub use radio_group::*;
pub use resizable::*;
pub use rich_text_editor::*;
pub use root_layout::*;
pub use scroller::*;
pub use segmented_control::*;
pub use select_menu::*;
pub use skeleton::*;
pub use split_button::*;
pub use stack::*;
pub use tab::*;
pub use text::*;
pub use text_list::*;
pub use toast::*;
pub use toggle::*;
pub use tooltip::*;
pub use validation::{DtValidationMessage, DtValidationMessages};

pub(crate) fn unique_id(prefix: &'static str) -> String {
    static NEXT: AtomicU32 = AtomicU32::new(1);
    format!("{prefix}-{}", NEXT.fetch_add(1, Ordering::Relaxed))
}
