# ShortCheat

A personal keyboard-shortcut cheat sheet, generated from YAML files.
One program (`shortcheat`, written in Rust) has two outputs:

- `shortcheat` starts an interactive viewer in the terminal (TUI).
- `shortcheat png` writes the cheat sheet as a PNG file.

Design: **The Analog Naturalist / Everforest** (see `DESIGN.md`).

![cheat_sheet](cheat_sheet.png)

## Structure

```
examples/           Example YAML files (KDE, Hyprland, Firefox, VS Code)
src/
  main.rs           Command line (clap): TUI or subcommand `png`
  config.rs         Loads YAML files from ~/.config/shortcheat/
  model.rs, filter.rs, keys.rs, palette.rs   shared data, search, key chips, colors
  tui.rs, ui.rs, theme.rs                    TUI (ratatui)
  png/              PNG renderer (tiny-skia, ab_glyph)
Cargo.toml          Rust project (binary `shortcheat`)
cheat_sheet.png     Generated cheat sheet
DESIGN.md           Design system documentation
```

Put your own YAML files in `~/.config/shortcheat/`.

## YAML format

```yaml
app: "Application name"
description: "Short description"
sections:
  - name: "Section name"
    shortcuts:
      - keys: "Ctrl + S"
        description: "Save"
```

Modifier keys are normalized automatically:
`Super`/`Meta` → `MOD`, `Ctrl`/`Control` → `CTRL`, `Shift` → `SHFT`, `Alt` → `ALT`, `Return` → `RET`, and so on.

## Dependencies

- Rust (stable) with Cargo
- Fonts for the PNG: DejaVu Sans / DejaVu Sans Bold / DejaVu Sans Mono
  (system fonts; on Debian/Ubuntu: `sudo apt install fonts-dejavu-core`)

Rust libraries (downloaded by Cargo): `ratatui`, `crossterm`, `serde`,
`serde_yaml`, `dirs`, `anyhow`, `clap`, `tiny-skia`, `ab_glyph`.

```bash
# Install to ~/.cargo/bin
cargo install --path .
```

## Usage

```bash
# Copy the YAML files to the config directory
mkdir -p ~/.config/shortcheat
cp examples/*.yaml ~/.config/shortcheat/

# Start the terminal viewer
shortcheat

# Generate a PNG (default: cheat_sheet.png in the current directory)
shortcheat png
shortcheat png -o /path/to/cheat_sheet.png
```

Without installing: `cargo run --release` (TUI) or `cargo run --release -- png`.

The PNG is 1920 px wide. Its height depends on the content.

### Keys in the terminal viewer

| Key | Action |
|---|---|
| any character | append to the search |
| Backspace | delete the last search character |
| ↑ / ↓ | scroll one line |
| PgUp / PgDn | scroll ten lines |
| Home | jump to the start |
| Esc | clear the search, or quit if it is empty |
| Ctrl-C / Ctrl-D | quit |

The columns are redistributed when you filter.

## Show the cheat sheet

### Hyprland – hold to show via special workspace

The image is **loaded once at startup** and hidden in an invisible
Scratchpad workspace. `SUPER + F1` shows it immediately.
Releasing the key hides it again. No processes are started on each press,
and there are no race conditions.

Add the following to `~/.config/hypr/hyprland.conf`:

```hyprlang
# Load the image once at startup (runs invisibly in the background)
exec-once = imv -b 000000 /path/to/shortcheat/cheat_sheet.png

# Window rules: hide the image in the Scratchpad workspace
windowrule {
  match:title  = cheat_sheet.png
  workspace    = special:cheat silent
  float        = on
  center       = on
  border_size  = 0
  no_shadow    = on
  no_anim      = on
  opacity      = 0.85 0.85
}

# Hold to show: show on key press, hide on key release
bind  = SUPER, F1, togglespecialworkspace, cheat
bindr = SUPER, F1, togglespecialworkspace, cheat
```

**First-time setup** (run once after saving the config, because `exec-once`
only runs at the next Hyprland start):

```bash
imv -b 000000 /path/to/shortcheat/cheat_sheet.png &
```

> **Note after a Hyprland reload:** `exec-once` only runs at boot.
> After a `hyprland reload`, you may have to restart imv manually
> so that the workspace state is reset.

### KDE – create a global shortcut

System Settings → Shortcuts → Global Shortcuts, command:

```bash
pqiv -c -i -t --hide-cursor-at-bins /path/to/cheat_sheet.png
```

## Design

The program implements the **Everforest / "The Analog Naturalist"** design from `DESIGN.md`:

- Color palette: `surface` `#0c141a` → `primary` `#c2dc99` → `secondary` `#ffb595`
- 4-column grid with automatic height balancing
- Key chips as rounded-rectangle badges (modifiers normalized)
- Category icons by character mapping
- Soft shadow behind the card (Gaussian blur)
- Ghost borders (15 % opacity) instead of hard divider lines

## License

MIT, see [`LICENSE`](LICENSE).
