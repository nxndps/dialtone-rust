# DtCollapsible controlled anchor

Change `DtCollapsible` in `dialtone/src/collapsible.rs`.

The case page already passes a controlled `open` signal and an `on_open_change` callback for both the description and the attachments. The component never delivers the chevron click to that callback, so the parent cannot toggle attachments or abbreviate the description.

## 1. Run `on_open_change` from the default anchor when `open` is set

The handler around line 247 returns immediately in controlled mode:

```rust
on_click=Callback::new(move |_| {
    if controlled {
        return;
    }
    is_open.set(!is_open.get_untracked());
})
```

Call the callback with the requested state and leave local state alone. The parent may refuse the change. The description does that on a full body: it stays open and drops back to the five-line preview. Setting `is_open` here would close it anyway.

```rust
on_click=Callback::new(move |_| {
    let next = !is_open.get_untracked();
    if controlled {
        // The parent may keep the section open, so local
        // state stays put until `open` changes.
        if let Some(on_open_change) = on_open_change.as_ref() {
            on_open_change.run(next);
        }
        return;
    }
    is_open.set(next);
})
```

Uncontrolled collapsibles stay as they are: no `open` prop, and the anchor still toggles `is_open` itself. `finish` can keep calling `on_open_change` after the transition. Parent handlers have to accept that second call with the same value.

Update the doc comment that says a set `open` makes the default anchor do nothing.

## 2. Animate `block-size`, which is the property the theme transitions

`.d-collapsible__enter-active` and `.d-collapsible__leave-active` transition `block-size`. This file writes inline `height`, so `transitionend` does not fire and `finish` never clears the inline size. Every `height:` in this file should become `block-size:`.

- Closed, including the initial closed style and the style `finish` sets when closed: `block-size:0px;overflow:hidden;`
- Enter, first frame: `block-size:0px;overflow:hidden;`
- Enter, measured frame: `block-size:{scroll_height}px;overflow:hidden;`
- Leave, first frame: `block-size:{offset_height}px;overflow:hidden;`
- Leave, next frame: `block-size:0px;overflow:hidden;`

Leave the `max-height` suffix alone. An idle open section should still use an empty inline style so the body can grow after the transition.

## App side

No app change is required for this. `view-cases` already implements the description rule in `description_after_chevron`: a chevron on the full text returns to the preview and stays open, the next chevron hides the body, and opening shows the preview. Attachments already flip `attach_collapsed` from `on_open_change`. Those callbacks start running once the anchor calls `on_open_change`.
