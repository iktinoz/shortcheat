#!/usr/bin/env python3
"""Generate a PNG keyboard shortcut cheat sheet from YAML files in ~/.config/shortcheat/.

Design: The Analog Naturalist / Everforest (see DESIGN.md).
"""

from __future__ import annotations

import glob
import os
from pathlib import Path

import yaml
from PIL import Image, ImageDraw, ImageFilter, ImageFont

# ── Configuration ──────────────────────────────────────────────────────────────

CONFIG_DIR = Path("~/.config/shortcheat").expanduser()
OUTPUT_FILE = Path("cheat_sheet.png")

IMAGE_WIDTH = 1920
COLUMNS = 4
COLUMN_GAP = 20
SIDEBAR_WIDTH = 60
EDGE_PADDING = 20
CARD_MARGIN = 16
CARD_PADDING = 28
CARD_RADIUS = 16
HEADER_HEIGHT = 72
FOOTER_HEIGHT = 44
SECTION_GAP = 20
APP_TITLE_HEIGHT = 30
SECTION_HEADER_HEIGHT = 24
SHORTCUT_ROW_HEIGHT = 22
APP_BOTTOM_PAD = 16

# ── Everforest Color Tokens ────────────────────────────────────────────────────


def _hex(h: str) -> tuple[int, int, int]:
    h = h.lstrip("#")
    return (int(h[0:2], 16), int(h[2:4], 16), int(h[4:6], 16))


C: dict[str, tuple[int, int, int]] = {
    "surface":                   _hex("#0c141a"),
    "surface_container_lowest":  _hex("#0a1117"),
    "surface_container_low":     _hex("#151d22"),
    "surface_container":         _hex("#1e272d"),
    "surface_container_highest": _hex("#2e363c"),
    "primary":                   _hex("#c2dc99"),
    "primary_container":         _hex("#a7c080"),
    "on_primary":                _hex("#1a2b12"),
    "secondary":                 _hex("#ffb595"),
    "on_surface":                _hex("#dbe3eb"),
    "on_surface_variant":        _hex("#9fa8b0"),
    "outline_variant":           _hex("#45483d"),
    "tertiary":                  _hex("#d2b478"),
    "shadow":                    _hex("#070f14"),
}

# ── Typography ─────────────────────────────────────────────────────────────────

_FONT_SANS = [
    "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
    "/usr/share/fonts/TTF/DejaVuSans.ttf",
    "/usr/share/fonts/dejavu/DejaVuSans.ttf",
]
_FONT_BOLD = [
    "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
    "/usr/share/fonts/TTF/DejaVuSans-Bold.ttf",
    "/usr/share/fonts/dejavu/DejaVuSans-Bold.ttf",
]
_FONT_MONO = [
    "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
    "/usr/share/fonts/TTF/DejaVuSansMono.ttf",
    "/usr/share/fonts/dejavu/DejaVuSansMono.ttf",
]


def _load_font(paths: list[str], size: int) -> ImageFont.FreeTypeFont | ImageFont.ImageFont:
    for path in paths:
        if os.path.exists(path):
            return ImageFont.truetype(path, size)
    return ImageFont.load_default()


def _build_fonts() -> dict:
    return {
        "display":  _load_font(_FONT_BOLD, 28),
        "title":    _load_font(_FONT_BOLD, 18),
        "section":  _load_font(_FONT_BOLD, 12),
        "body":     _load_font(_FONT_SANS, 13),
        "chip":     _load_font(_FONT_MONO, 11),
        "tab":      _load_font(_FONT_SANS, 13),
        "footer":   _load_font(_FONT_SANS, 10),
        "sidebar":  _load_font(_FONT_SANS, 16),
    }


# ── Key Chip Parsing ───────────────────────────────────────────────────────────

_KEY_ALIASES: dict[str, str] = {
    "super": "MOD", "meta": "MOD",
    "ctrl": "CTRL", "control": "CTRL",
    "shift": "SHFT",
    "alt": "ALT",
    "return": "RET", "enter": "RET",
    "backspace": "BKSP",
    "escape": "ESC",
    "tab": "TAB",
    "space": "SPC",
    "up": "↑", "down": "↓", "left": "←", "right": "→",
    "print": "PRT",
    "delete": "DEL",
    "home": "HOM", "end": "END",
    "pageup": "PG↑", "pagedown": "PG↓",
    "mouse scroll": "SCR",
}
for _i in range(1, 13):
    _KEY_ALIASES[f"f{_i}"] = f"F{_i}"

CHIP_HEIGHT = 20
CHIP_RADIUS = 4
CHIP_H_PAD = 7
CHIP_GAP = 4


def _parse_keys(keys_str: str) -> list[str]:
    """Split 'Super + Shift + Q' into normalized chip labels."""
    tokens = []
    for part in keys_str.split("+"):
        part = part.strip()
        if not part:
            continue
        lower = part.lower()
        tokens.append(_KEY_ALIASES.get(lower, part.upper() if len(part) <= 5 else part))
    return tokens or [keys_str[:6].upper()]


def _text_width(text: str, font: ImageFont.FreeTypeFont | ImageFont.ImageFont) -> int:
    try:
        return int(font.getlength(text))
    except AttributeError:
        return font.getbbox(text)[2]


def _chip_width(label: str, font: ImageFont.FreeTypeFont | ImageFont.ImageFont) -> int:
    return _text_width(label, font) + CHIP_H_PAD * 2


def _render_key_chips(
    draw: ImageDraw.ImageDraw,
    chips: list[str],
    x_right: int,
    y: int,
    font: ImageFont.FreeTypeFont | ImageFont.ImageFont,
) -> None:
    """Draw key chips right-aligned, ending at x_right."""
    x = x_right
    for chip in reversed(chips):
        w = _chip_width(chip, font)
        x -= w
        draw.rounded_rectangle(
            [x, y, x + w, y + CHIP_HEIGHT],
            radius=CHIP_RADIUS,
            fill=C["surface_container_highest"],
            outline=C["outline_variant"],
        )
        tw = _text_width(chip, font)
        draw.text(
            (x + (w - tw) // 2, y + (CHIP_HEIGHT - 11) // 2),
            chip,
            fill=C["on_surface"],
            font=font,
        )
        x -= CHIP_GAP


# ── Category Icons ─────────────────────────────────────────────────────────────

_ICONS: dict[str, str] = {
    "hyprland": "◈",
    "kde": "⬡",
    "firefox": "◎",
    "vs code": "⌥",
    "vscode": "⌥",
    "window": "⬜",
    "workspace": "◱",
    "virtual desktop": "◱",
    "navigation": "⇥",
    "system": "⚙",
    "application": "▦",
    "terminal": "▶",
    "shell": "▶",
    "developer": "⎇",
    "editing": "✎",
    "page": "☰",
    "git": "⎇",
    "session": "⏻",
}


def _icon(name: str) -> str:
    lower = name.lower()
    for key, icon in _ICONS.items():
        if key in lower:
            return icon + "  "
    return ""


# ── Ghost border helper ────────────────────────────────────────────────────────


def _ghost_color(over: tuple, bg: tuple) -> tuple:
    """Blend color at 15% opacity over bg."""
    return tuple(int(c * 0.15 + b * 0.85) for c, b in zip(over, bg))


# ── YAML Loading ───────────────────────────────────────────────────────────────


def load_shortcuts(config_dir: Path) -> list[dict]:
    """Load all YAML shortcut files from config_dir, sorted by filename."""
    files = sorted(glob.glob(str(config_dir / "*.yaml")))
    apps = []
    for f in files:
        with open(f) as fh:  # noqa: PTH123
            data = yaml.safe_load(fh)
            if data:
                apps.append(data)
    return apps


# ── Layout ─────────────────────────────────────────────────────────────────────


def _measure_app_block(app_data: dict) -> int:
    height = APP_TITLE_HEIGHT
    for section in app_data.get("sections", []):
        height += SECTION_HEADER_HEIGHT
        height += len(section.get("shortcuts", [])) * SHORTCUT_ROW_HEIGHT
    return height + APP_BOTTOM_PAD


def _build_grid(apps: list[dict]) -> list[list[dict]]:
    """Distribute apps across COLUMNS columns, balancing total height."""
    col_heights = [0] * COLUMNS
    columns: list[list[dict]] = [[] for _ in range(COLUMNS)]
    for app in apps:
        h = _measure_app_block(app)
        idx = col_heights.index(min(col_heights))
        columns[idx].append(app)
        col_heights[idx] += h + SECTION_GAP
    return columns


# ── Component Renderers ────────────────────────────────────────────────────────


def _render_sidebar(
    image: Image.Image,
    x0: int, y0: int, x1: int, y1: int,
    fonts: dict,
) -> None:
    draw = ImageDraw.Draw(image)
    draw.rectangle([x0, y0, x1, y1], fill=C["surface_container_lowest"])
    icons = ["⊞", "◎", "▶", "≡", "⊙"]
    icon_h = 32
    spacing = max(8, (y1 - y0 - len(icons) * icon_h) // (len(icons) + 1))
    cx = (x0 + x1) // 2
    for i, icon in enumerate(icons):
        iy = y0 + spacing + i * (icon_h + spacing)
        tw = _text_width(icon, fonts["sidebar"])
        draw.text((cx - tw // 2, iy), icon, fill=C["on_surface_variant"], font=fonts["sidebar"])


def _render_header(
    draw: ImageDraw.ImageDraw,
    card_x: int, card_y: int, card_width: int,
    fonts: dict,
) -> None:
    hx = card_x + CARD_PADDING
    hy = card_y + 16

    draw.text((hx, hy), "EVERFOREST COMMAND", fill=C["primary"], font=fonts["display"])

    tabs = ["Shortcuts", "Workflows", "Extensions"]
    tab_y = hy + 28 + 6
    tab_x = hx
    for i, tab in enumerate(tabs):
        active = i == 0
        color = C["primary"] if active else C["on_surface_variant"]
        draw.text((tab_x, tab_y), tab, fill=color, font=fonts["tab"])
        tw = _text_width(tab, fonts["tab"])
        if active:
            draw.rectangle([tab_x, tab_y + 17, tab_x + tw, tab_y + 19], fill=C["primary"])
        tab_x += tw + 24

    corner_icons = ["⚙", "?", "○"]
    rx = card_x + card_width - CARD_PADDING
    for icon in reversed(corner_icons):
        iw = _text_width(icon, fonts["tab"])
        rx -= iw + 4
        draw.text((rx, tab_y), icon, fill=C["on_surface_variant"], font=fonts["tab"])
        rx -= 16

    sep_y = card_y + HEADER_HEIGHT - 1
    ghost = _ghost_color(C["outline_variant"], C["surface_container_low"])
    draw.line(
        [(card_x + CARD_PADDING, sep_y), (card_x + card_width - CARD_PADDING, sep_y)],
        fill=ghost,
        width=1,
    )


def _render_section_header(
    draw: ImageDraw.ImageDraw,
    name: str,
    x: int, y: int,
    fonts: dict,
) -> int:
    label = f"{_icon(name)}{name.upper()}"
    draw.text((x, y), label, fill=C["secondary"], font=fonts["section"])
    return y + SECTION_HEADER_HEIGHT


def _render_shortcut_row(
    draw: ImageDraw.ImageDraw,
    shortcut: dict,
    x: int, y: int, col_right: int,
    fonts: dict,
) -> int:
    desc = shortcut.get("description", "")
    chips = _parse_keys(shortcut.get("keys", ""))
    text_y = y + (CHIP_HEIGHT - 13) // 2
    draw.text((x, text_y), desc, fill=C["on_surface"], font=fonts["body"])
    _render_key_chips(draw, chips, col_right, y, fonts["chip"])
    return y + SHORTCUT_ROW_HEIGHT


def _render_app_block(
    draw: ImageDraw.ImageDraw,
    app_data: dict,
    x: int, y: int, col_right: int,
    fonts: dict,
) -> int:
    app_name = app_data.get("app", "")
    draw.text((x, y), f"{_icon(app_name)}{app_name}", fill=C["primary"], font=fonts["title"])
    y += APP_TITLE_HEIGHT

    for section in app_data.get("sections", []):
        y = _render_section_header(draw, section.get("name", ""), x, y, fonts)
        for shortcut in section.get("shortcuts", []):
            y = _render_shortcut_row(draw, shortcut, x, y, col_right, fonts)

    return y + APP_BOTTOM_PAD


def _render_footer(
    draw: ImageDraw.ImageDraw,
    card_x: int, card_bottom: int, card_width: int,
    fonts: dict,
) -> None:
    fy = card_bottom - FOOTER_HEIGHT + (FOOTER_HEIGHT - 10) // 2

    draw.text(
        (card_x + CARD_PADDING, fy),
        "© 2024 THE ANALOG NATURALIST",
        fill=C["on_surface_variant"],
        font=fonts["footer"],
    )

    links = ["DOCUMENTATION", "CHANGELOG", "PRIVACY"]
    rx = card_x + card_width - CARD_PADDING
    for link in reversed(links):
        lw = _text_width(link, fonts["footer"])
        rx -= lw
        draw.text((rx, fy), link, fill=C["on_surface_variant"], font=fonts["footer"])
        rx -= 24

    sep_y = card_bottom - FOOTER_HEIGHT
    ghost = _ghost_color(C["outline_variant"], C["surface_container_low"])
    draw.line(
        [(card_x + CARD_PADDING, sep_y), (card_x + card_width - CARD_PADDING, sep_y)],
        fill=ghost,
        width=1,
    )


# ── Main Generator ─────────────────────────────────────────────────────────────


def generate_image(apps: list[dict], output: Path) -> None:
    fonts = _build_fonts()
    columns = _build_grid(apps)

    col_heights = [sum(_measure_app_block(a) + SECTION_GAP for a in col) for col in columns]
    content_height = max(col_heights) if col_heights else 200

    card_x = EDGE_PADDING + SIDEBAR_WIDTH + CARD_MARGIN
    card_width = IMAGE_WIDTH - card_x - EDGE_PADDING
    col_width = (card_width - 2 * CARD_PADDING - (COLUMNS - 1) * COLUMN_GAP) // COLUMNS

    image_height = (
        EDGE_PADDING
        + HEADER_HEIGHT
        + CARD_PADDING
        + content_height
        + FOOTER_HEIGHT
        + EDGE_PADDING
    )
    card_y = EDGE_PADDING
    card_height = image_height - 2 * EDGE_PADDING

    # Base background
    image = Image.new("RGB", (IMAGE_WIDTH, image_height), C["surface"])

    # Ambient shadow behind card
    shadow = Image.new("RGBA", (IMAGE_WIDTH, image_height), (0, 0, 0, 0))
    ImageDraw.Draw(shadow).rounded_rectangle(
        [card_x + 6, card_y + 10, card_x + card_width + 6, card_y + card_height + 10],
        radius=CARD_RADIUS,
        fill=(*C["shadow"], 120),
    )
    shadow = shadow.filter(ImageFilter.GaussianBlur(radius=22))
    image = Image.alpha_composite(image.convert("RGBA"), shadow).convert("RGB")

    draw = ImageDraw.Draw(image)

    # Main card
    draw.rounded_rectangle(
        [card_x, card_y, card_x + card_width, card_y + card_height],
        radius=CARD_RADIUS,
        fill=C["surface_container_low"],
    )

    # Sidebar (drawn after card so it sits on the surface layer)
    _render_sidebar(
        image,
        EDGE_PADDING, EDGE_PADDING,
        EDGE_PADDING + SIDEBAR_WIDTH, EDGE_PADDING + card_height,
        fonts,
    )
    draw = ImageDraw.Draw(image)

    # Header
    _render_header(draw, card_x, card_y, card_width, fonts)

    # Content grid
    content_top = card_y + HEADER_HEIGHT + CARD_PADDING
    for col_idx, col_apps in enumerate(columns):
        cx = card_x + CARD_PADDING + col_idx * (col_width + COLUMN_GAP)
        col_right = cx + col_width
        cy = content_top
        for app_data in col_apps:
            cy = _render_app_block(draw, app_data, cx, cy, col_right, fonts)

    # Footer
    _render_footer(draw, card_x, card_y + card_height, card_width, fonts)

    image.save(output)
    print(f"Saved cheat sheet to {output}")  # noqa: T201


def main() -> None:
    if not CONFIG_DIR.exists():
        print(f"Config directory not found: {CONFIG_DIR}")  # noqa: T201
        print("Create it and add YAML files. See examples/ in the repo.")  # noqa: T201
        raise SystemExit(1)

    apps = load_shortcuts(CONFIG_DIR)
    if not apps:
        print(f"No YAML files found in {CONFIG_DIR}")  # noqa: T201
        raise SystemExit(1)

    generate_image(apps, OUTPUT_FILE)


if __name__ == "__main__":
    main()
