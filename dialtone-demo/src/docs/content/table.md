---
title: Table
description: A data table with an optional caption and striped rows.
status: ready
---

## Usage

There is no Vue `DtTable`. This is the `d-table` CSS component from Dialtone 10.7.0. `striped` adds `d-table--striped`. A non-empty `caption` renders `caption.d-table__caption`. Put `thead` and `tbody` in the default slot.

`d-table--inverted` is deprecated. Wrap the table in [Mode](mode-island.md) to invert it.

## Accessibility

Give the table a caption when the heading outside the table does not name it. Use `th` for column headers.
