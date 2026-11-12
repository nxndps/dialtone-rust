---
title: Slider
description: A controlled control for one number or a range along a track.
status: ready
---

## Usage

`DtSlider` follows Dialtone 10.11.0. The caller owns `value`. `on_input` reports every change. Write that value back onto the signal or the thumb snaps back.

`readout` is tooltip, always, or never. `fill` `End` emits `d-slider--inverted`. That rule is empty in the vendored CSS, so the track does not invert.

A vertical slider needs a block size. The component does not measure one for you.

## Range

`DtSliderValue::Range` draws two thumbs. Pass `get_value_text` so each thumb has its own accessible description. `min_gap_steps` keeps the thumbs apart.

## Accessibility

Pass `label`, or hide it with `show_label` false and set `aria_label`. An empty name warns.
