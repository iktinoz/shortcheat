# shortcheat-tui

Interactive terminal viewer for the same `~/.config/shortcheat/*.yaml` files that
[`scripts/generate_png.py`](../scripts/generate_png.py) renders to a PNG. Built
with [ratatui](https://ratatui.rs/) and styled to match the parent project's
**Everforest / "The Analog Naturalist"** palette.

## Run

```bash
mkdir -p ~/.config/shortcheat
cp ../examples/*.yaml ~/.config/shortcheat/   # once

cargo run --release
```

## Keys

| Key | Action |
|---|---|
| any printable | append to search |
| Backspace | delete last search char |
| ↑ / ↓ | scroll one row |
| PgUp / PgDn | scroll ten rows |
| Home | jump to top |
| Esc | clear search, or quit if already empty |
| Ctrl-C / Ctrl-D | quit |

Columns rebalance live as the filter narrows the list.
