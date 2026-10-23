# dialtone

Leptos 0.8 ports of [Dialtone](https://dialtone.dialpad.com/) Vue components. The crate emits Dialtone class names (`d-btn`, `d-card`, `d-item-layout`). It does not ship CSS. Your app loads the styles from `assets/`.

`view-sync` and `dialtone-demo` are the apps in this workspace that already depend on it. The component list, Vue prop names, and porting rules are in [`AGENTS.md`](AGENTS.md).

Vendored assets are Dialtone 10. See `assets/VERSION`:

- `@dialpad/dialtone` 10.7.0
- `@dialpad/dialtone-css` 9.1.1
- `@dialpad/dialtone-icons` 5.3.0
- `@dialpad/dialtone-tokens` 2.0.3

New ports follow tag `dialtone/v10.7.0`. Some older ports still emit Dialtone 9 class names. Both sets are in the vendored CSS.

## Prerequisites

- Rust, edition 2021, and Cargo. This crate is `publish = false`. Depend on it by path. It is not on crates.io.
- The `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`.
- Leptos 0.8 with the `csr` feature. This crate is client-side. It uses `web-sys` and is not an SSR or hydrate renderer.
- Trunk, if you build the same way `view-sync` and `dialtone-demo` do. Another wasm bundler is fine when it compiles that target and serves the CSS below.
- Tailwind CSS v4, if you follow the existing apps. Dialtone classes come from the vendored stylesheets. Tailwind still has to see `dialtone/src/**/*.rs`, or a utility class written in a component is purged.
- On Apple Silicon, this workspace links host build scripts with `scripts/zig-cc` (`.cargo/config.toml`). That avoids Xcode `cc`. Leave `CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER` unset. A crate outside this workspace needs its own working host linker for proc-macros and build scripts.

`html, body` in the vendored CSS set `font-size: 10px`, so `1rem` is 10px. `dialtone-demo/tailwind.css` rescales Tailwind so `p-4` stays 16px. Copy that `@theme` block if your utilities should keep their usual sizes.

## Add the crate

From a package inside this workspace, next to `dialtone/`:

```toml
[dependencies]
dialtone = { path = "../dialtone" }
leptos = { version = "0.8", features = ["csr"] }
```

`view-sync/Cargo.toml` and `dialtone-demo/Cargo.toml` use that path. Import the components. Do not add `mod dialtone` inside the app.

```rust
use dialtone::{DtButton, DtCard, DtStack};
use leptos::prelude::*;

#[component]
fn Example() -> impl IntoView {
    view! {
        <DtCard>
            <DtStack direction="row" gap="400" align="center">
                <DtButton>"Save"</DtButton>
            </DtStack>
        </DtCard>
    }
}
```

Check from `nicholas.romero/pagerduty-sync` (the experimental repo root has no `Cargo.toml`):

```
cargo check -p your-app --target wasm32-unknown-unknown
```

`trunk serve` from the app directory rebuilds a Trunk app. `view-sync` serves on port 8081. `dialtone-demo` serves on port 8082.

## Load the CSS and fonts

Put `data-dt-mode` on `<html>`. Dialtone color tokens read that attribute. `light` and `dark` are the values these apps set. `view-sync` and `dialtone-demo` also toggle `dialtone-theme-light`, `dialtone-theme-dark`, and `dt-theme-dark` for the app palette.

```html
<html lang="en" class="dialtone-theme-light" data-dt-mode="light">
```

Import the styles in `layer(base)`, in this order. Paths match a package that sits next to `dialtone/`:

```css
@import "tailwindcss";
@import "../dialtone/assets/css/no-layers/dialtone-default-theme.min.css" layer(base);
@import "../dialtone/assets/tokens/layered/tokens-base-colors.css" layer(base);
@import "../dialtone/assets/tokens/layered/tokens-dp-colors.css" layer(base);
@import "../dialtone/assets/css/no-layers/illustrations.css" layer(base);

@source "./src/**/*.rs";
@source "../dialtone/src/**/*.rs";
@source "./index.html";
```

`illustrations.css` is only needed for `DtIllustration`. The no-layers stylesheets point font files at `fonts/` beside the built CSS. Trunk copies them there. `view-sync/index.html` and `dialtone-demo/index.html` also copy the asset tree for anything that requests `/dialtone/...`:

```html
<link data-trunk rel="tailwind-css" href="tailwind.css" />
<link data-trunk rel="copy-dir" href="../dialtone/assets" data-target-path="dialtone" />
<link data-trunk rel="copy-dir" href="../dialtone/assets/fonts" data-target-path="fonts" />
```

Named icons are inline SVG. You do not fetch `assets/icons` at runtime. `DtEmoji` and `DtEmojiPicker` load JoyPixels PNGs from `static.dialpadcdn.com` unless you change the asset URL.

## Call-site rules

Vue prop names are `camelCase`. Rust props are `snake_case`. A doc comment on the component names the Vue prop when they differ (`contentClass`, `blockEndClass`).

- An optional `Option<T>` prop takes the inner value. Write `subtitle=slot` and `image_alt="Ada Lovelace".to_string()`. `Some(...)` does not type-check.
- A missing named slot is omitted. Pass `ViewFn` only when the slot has content. Call `slot.run()` is the component's job.
- A controlled value is usually `Signal<T>` with `#[prop(into)]`, so an `RwSignal<T>` passes directly. `DtToggle` `checked` is that shape. `DtFilterPill` `filters` is not: the caller writes `filters.into()`. Read `.get()` in markup and `.get_untracked()` in an event handler.
- `as` and `type` are keywords. The prop is `r#as` or `r#type`. The call site writes `r#as="section"` and `r#type="ordered"`.
- A `>` inside a `view!` attribute closes the tag. Write `on_change=move |next|` there. Put a typed closure in a string if you need one.
- An empty string omits an extra class, `id`, or `aria-label`. A non-empty `class` is appended on the root for components that take one.
- Unknown tokens warn in the browser console and fall back. They do not panic.

```rust
let checked = RwSignal::new(false);

view! {
    <DtToggle checked=checked on_change=move |next| checked.set(next)>
        "On call"
    </DtToggle>
    <DtItemLayout
        r#as="section"
        subtitle=ViewFn::from(|| view! { <span>"Primary"</span> })
    >
        "Ada Lovelace"
    </DtItemLayout>
}
```

## What you get

Public components are re-exported from `src/lib.rs`. The table in [`AGENTS.md`](AGENTS.md) is the full list. It includes avatar, badge, banner, box, breadcrumbs, button, card, checkbox, chip, code block, collapsible, combobox, datepicker, description list, dropdown, emoji, empty state, filter pill, hovercard, icon, illustration, image viewer, input, item layout, keyboard shortcut, lazy show, link, list item, loader, modal, mode island, motion text, notice, pagination, popover, presence, progress circle, prose, radio, resizable, rich text editor, root layout, scroller, segmented control, select menu, skeleton, split button, stack, tabs, text, toast, toggle, tooltip, and validation messages.

`DtSkeletonText`, `DtSkeletonParagraph`, `DtSkeletonShape`, and `DtSkeletonListItem` are options on `DtSkeleton`. `DtRecipeComboboxWithPopover` and `DtRecipeComboboxMultiSelect` are aliases. Placement for tooltip and popover is `dialtone::popper`. There is no Tippy.

Not in this crate:

- The rest of `@dialpad/dialtone-icons`. Use a named icon from the `AGENTS.md` table, or `DtIcon` with a `name` that already has a component.
- Vue directives: `tooltip`, `scrollbar`, `mode`, `focusgroup`, `focustrap`.
- Vue Router `to`. A link or button with `href` renders an `<a>`.
- `DtDropdownList` as its own component.

`dialtone-demo` renders every port at `#/components/{id}` when Trunk is serving that app.
