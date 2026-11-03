# ShortCheat

Persönliches Keyboard-Shortcut Cheat-Sheet, generiert aus YAML-Dateien.
Ein Programm (`shortcheat`, Rust) bietet zwei Ausgaben:

- `shortcheat` startet einen interaktiven Viewer im Terminal (TUI).
- `shortcheat png` schreibt das Cheat-Sheet als PNG.

Design: **The Analog Naturalist / Everforest** (siehe `DESIGN.md`).

![cheat_sheet](cheat_sheet.png)

## Struktur

```
examples/           Beispiel-YAML-Dateien (KDE, Hyprland, Firefox, VS Code)
src/
  main.rs           Kommandozeile (clap): TUI oder Unterbefehl `png`
  config.rs         YAML-Dateien aus ~/.config/shortcheat/ laden
  model.rs, filter.rs, keys.rs, palette.rs   gemeinsame Daten, Suche, Tasten-Chips, Farben
  tui.rs, ui.rs, theme.rs                    TUI (ratatui)
  png/              PNG-Renderer (tiny-skia, ab_glyph)
Cargo.toml          Rust-Projekt (Binary `shortcheat`)
cheat_sheet.png     Generiertes Cheat-Sheet
DESIGN.md           Design-System-Dokumentation
```

Eigene YAML-Dateien liegen unter `~/.config/shortcheat/`.

## YAML-Format

```yaml
app: "Anwendungsname"
description: "Kurzbeschreibung"
sections:
  - name: "Sektionsname"
    shortcuts:
      - keys: "Ctrl + S"
        description: "Speichern"
```

Modifier-Tasten werden automatisch normalisiert:
`Super`/`Meta` → `MOD`, `Ctrl`/`Control` → `CTRL`, `Shift` → `SHFT`, `Alt` → `ALT`, `Return` → `RET`, usw.

## Abhängigkeiten

- Rust (stable) mit Cargo
- Schriftarten für das PNG: DejaVu Sans / DejaVu Sans Bold / DejaVu Sans Mono
  (System-Fonts, Debian/Ubuntu: `sudo apt install fonts-dejavu-core`)

Rust-Bibliotheken (werden von Cargo geladen): `ratatui`, `crossterm`, `serde`,
`serde_yaml`, `dirs`, `anyhow`, `clap`, `tiny-skia`, `ab_glyph`.

```bash
# Installation nach ~/.cargo/bin
cargo install --path .
```

## Verwendung

```bash
# YAML-Dateien in Config-Verzeichnis ablegen
mkdir -p ~/.config/shortcheat
cp examples/*.yaml ~/.config/shortcheat/

# Terminal-Viewer starten
shortcheat

# PNG generieren (Standard: cheat_sheet.png im aktuellen Verzeichnis)
shortcheat png
shortcheat png -o /pfad/zu/cheat_sheet.png
```

Ohne Installation: `cargo run --release` (TUI) oder `cargo run --release -- png`.

Das PNG ist 1920 px breit, die Höhe ergibt sich aus dem Inhalt.

### Tasten im Terminal-Viewer

| Taste | Aktion |
|---|---|
| beliebiges Zeichen | an die Suche anhängen |
| Backspace | letztes Suchzeichen löschen |
| ↑ / ↓ | eine Zeile scrollen |
| PgUp / PgDn | zehn Zeilen scrollen |
| Home | an den Anfang springen |
| Esc | Suche leeren, oder beenden wenn sie leer ist |
| Ctrl-C / Ctrl-D | beenden |

Die Spalten werden beim Filtern neu verteilt.

## Cheat-Sheet anzeigen

### Hyprland – Hold-to-show via Special Workspace

Das Bild wird **einmalig beim Start geladen** und in einem unsichtbaren
Scratchpad-Workspace versteckt. `SUPER + F1` blendet es sofort ein,
Loslassen blendet es sofort wieder aus – keine Prozessstarts, keine
Race-Conditions.

Füge folgendes in `~/.config/hypr/hyprland.conf` ein:

```hyprlang
# Bild einmalig beim Start laden (läuft unsichtbar im Hintergrund)
exec-once = imv -b 000000 /home/patrick/repos/shortcheat/cheat_sheet.png

# Fensterregeln: Bild in Scratchpad-Workspace verstecken
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

# Hold-to-show: Einblenden beim Drücken, Ausblenden beim Loslassen
bind  = SUPER, F1, togglespecialworkspace, cheat
bindr = SUPER, F1, togglespecialworkspace, cheat
```

**Ersteinrichtung** (einmalig nach dem Speichern, da `exec-once` erst
beim nächsten Hyprland-Start greift):

```bash
imv -b 000000 /home/patrick/repos/shortcheat/cheat_sheet.png &
```

> **Hinweis nach Hyprland-Reload:** Da `exec-once` nur beim Booten
> ausgeführt wird, imv nach einem `hyprland reload` ggf. manuell neu
> starten, damit sich der Workspace-Zustand zurücksetzt.

### KDE – Globalen Shortcut anlegen

Systemeinstellungen → Shortcuts → Globale Shortcuts, Befehl:

```bash
pqiv -c -i -t --hide-cursor-at-bins /path/to/cheat_sheet.png
```

## Design

Das Programm implementiert das **Everforest / "The Analog Naturalist"**-Design aus `DESIGN.md`:

- Farbpalette: `surface` `#0c141a` → `primary` `#c2dc99` → `secondary` `#ffb595`
- 4-Spalten-Grid mit automatischer Höhenbalancierung
- Key-Chips als gerundete Rechteck-Badges (Modifier normalisiert)
- Kategorie-Icons per Zeichen-Mapping
- Weicher Schatten hinter der Card (Gaussscher Weichzeichner)
- Ghost-Borders (15 % Opacity) statt harter Trennlinien
