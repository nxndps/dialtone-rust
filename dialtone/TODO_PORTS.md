# Dialtone port TODO

Public Vue exports are `packages/dialtone-vue/index.js`. Confirm new work against tag `dialtone/v10.7.0`. Vendored CSS, icons, and tokens are Dialtone 10.11.0 (`dialtone/assets/VERSION`).

`@dialpad/dialtone-css` 9.4.0 is that package's own version. It ships with Dialtone 10.11.0.

Version below is the tag named in the component doc comment. `dialtone/AGENTS.md` matches the `dialtone/v10.7.0` doc comments for `DtBadge`, `DtEmptyState`, `DtSelectMenu`, and `DtToggle`.

## Left to port

The public export list at `dialtone/v10.11.0` adds only `DtSlider` over `dialtone/v10.7.0`. Its Vue sources and `slider.less` match the port (commit `4b56a42e5c5eeead57181a961060dd2ea8eaa053`, same bytes as staging `84923549`).

- [x] `DtSlider` — `packages/dialtone-vue/components/Slider`, confirmed against `dialtone/v10.11.0`

Not component ports. Leave them off the component table.

- The rest of `@dialpad/dialtone-icons`, including vendored `icons/globe.svg` in `dialtone/assets/icons/`. Those files are not loaded. `DtIcon` is the shell. A named icon copies its path into `icon.rs`.
- Vue directives: `tooltip`, `scrollbar`, `mode`, `focusgroup`, `focustrap`. They are not components. `DtTooltip`, `DtBox`, and `DtModeIsland` cover the component forms. `DtModal` and `DtImageViewer` trap Tab themselves.

`DtSkeletonText`, `DtSkeletonParagraph`, `DtSkeletonShape`, and `DtSkeletonListItem` are options on `DtSkeleton`. `KitchenSink`, `Plugins`, and `DropdownList.vue` are not public exports.

## Move to v10

Name tag `dialtone/v10.7.0` in the doc comment. `DtSlider` stays confirmed against `dialtone/v10.11.0` because it is not in `dialtone/v10.7.0`.

### Reference tag `dialtone/v10.7.0`

- [x] `DtBanner` — `banner.rs`. Follows `dialtone/v10.7.0`. Composes `DtNoticeIcon`, `DtNoticeContent`, and `DtNoticeAction`.
- [x] `DtButtonGroup` — `button_group.rs`. Follows `dialtone/v10.7.0`.
- [x] `DtCheckbox` — `checkbox.rs`. Follows `dialtone/v10.7.0`.
- [x] `DtCheckboxGroup` — `checkbox_group.rs`. Follows `dialtone/v10.7.0`.
- [x] `DtCodeblock` — `codeblock.rs`. Follows `dialtone/v10.7.0`.
- [x] `DtComboboxMultiSelect` — `combobox_multi_select.rs`. Follows `dialtone/v10.7.0`.
- [x] `DtDatepicker` — `datepicker.rs`. Follows `dialtone/v10.7.0`. Includes `DtDate`.
- [x] `DtEmoji` — `emoji.rs`. Follows `dialtone/v10.7.0`. Catalog is `@dialpad/dialtone-emojis` from `dialtone/v9.190.0`.
- [x] `DtEmojiPicker` — `emoji_picker.rs`. Follows `dialtone/v10.7.0`.
- [x] `DtEmojiTextWrapper` — `emoji_text_wrapper.rs`. Follows `dialtone/v10.7.0`.
- [x] `DtScroller` — `scroller.rs`. Follows `dialtone/v10.7.0`. Includes `DtScrollerItem`.
- [x] `DtStack` — `stack.rs`. Follows `dialtone/v10.7.0`.
- [x] `DtValidationMessages` — `validation.rs`. Follows `dialtone/v10.7.0`. Includes `DtValidationMessage`.

### Written from staging `a46ac063`, reference tag `dialtone/v10.7.0`

- [x] `DtFilterPill` — `filter_pill.rs`. Follows `dialtone/v10.7.0`.
- [x] `DtSegmentedControl` — `segmented_control.rs`. Follows `dialtone/v10.7.0`. Includes `DtSegmentedControlItem`.
- [x] `DtText` — `text.rs`. Follows `dialtone/v10.7.0`.
- [x] `DtTextList` — `text_list.rs`. Follows `dialtone/v10.7.0`. Includes `DtTextListItem`.

### No tag in the doc comment

Behavior may already match v10. Confirm against `dialtone/v10.7.0` and name the tag in the doc comment.

- [x] `DtButton` — `button.rs`. Follows `dialtone/v10.7.0`. `danger` warns and omits the kind class.
- [x] `DtCard` — `card.rs`. Follows `dialtone/v10.7.0`. `container_class` is a Rust extra.
- [x] `DtCollapsible` — `collapsible.rs`. Follows `dialtone/v10.7.0`. A set `open` does not toggle on click.
- [x] `DtComboboxWithPopover` — `combobox_with_popover.rs`. Follows `dialtone/v10.7.0`. `open_with_arrow_keys` defaults to false.
- [x] `DtDescriptionList` — `description_list.rs`. Follows `dialtone/v10.7.0`. Unknown direction and gap tokens are still emitted.
- [x] `DtInput` — `input.rs`. Follows `dialtone/v10.7.0`.
- [x] `DtInputGroup` — `input_group.rs`. Follows `dialtone/v10.7.0`.
- [x] `DtLoader` — `loader.rs`. Follows `dialtone/v10.7.0`. `size` defaults to `"500"`.
- [x] `DtModal` — `modal.rs`. Follows `dialtone/v10.7.0`. The mapping-editor widths are gone.
- [x] `DtPopover` — `popover.rs`. Follows `dialtone/v10.7.0`. `tether` defaults to false here. Vue's default is true. `initial_focus` stays `none`. Vue's default is `first`. `transition` stays empty. Vue's default is `fade`.
- [x] `DtRootLayout` — `root_layout.rs`. Follows `dialtone/v10.7.0`. `fixed` stays false. Vue's default is true.
- [x] `DtTooltip` — `tooltip.rs`. Follows `dialtone/v10.7.0`.

## Ahead of `dialtone/v10.7.0`

The vendored CSS is Dialtone 10.11.0. These stay on the confirmation tag.

- `DtLoader` still draws the v10.7.0 spinner on `div.d-loader`. Dialtone 10.11.0 defaults `kind` to `dots`. The vendored spin runs only with `d-loader--spinner`, so this port does not animate.
- `DtMotionText` does not emit `slide-in-gradient`. That class is in the 10.11.0 CSS and is not a v10.7.0 mode.
- `DtPopover` keeps `tether` false, `initial_focus` none, and `transition` empty. Vue's defaults are true, first, and fade.
- `DtRootLayout` keeps `fixed` false. Vue's default is true.
