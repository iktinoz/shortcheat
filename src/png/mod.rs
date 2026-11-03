//! PNG cheat-sheet renderer.
//!
//! This is a port of the former Python generator. Function names, layout
//! constants, coordinates and draw order are kept. Like Pillow, rectangle
//! coordinates are inclusive: `[x0, y0, x1, y1]` covers `x1 - x0 + 1` pixels.

mod blur;
mod text;

use std::path::Path;

use anyhow::{anyhow, Result};
use tiny_skia::{
    Color, FillRule, Paint, Path as SkPath, PathBuilder, Pixmap, Rect, Stroke, Transform,
};

use crate::keys::{icon_for, parse_chips};
use crate::model::{App, Shortcut};
use crate::palette::{self, Rgb};
use text::Face;

// Layout constants.
const IMAGE_WIDTH: i32 = 1920;
const COLUMNS: usize = 4;
const COLUMN_GAP: i32 = 20;
const SIDEBAR_WIDTH: i32 = 60;
const EDGE_PADDING: i32 = 20;
const CARD_MARGIN: i32 = 16;
const CARD_PADDING: i32 = 28;
const CARD_RADIUS: i32 = 16;
const HEADER_HEIGHT: i32 = 72;
const SECTION_GAP: i32 = 20;
const APP_TITLE_HEIGHT: i32 = 30;
const SECTION_HEADER_HEIGHT: i32 = 24;
const SHORTCUT_ROW_HEIGHT: i32 = 22;
const APP_BOTTOM_PAD: i32 = 16;

// Key chip constants.
const CHIP_HEIGHT: i32 = 20;
const CHIP_RADIUS: i32 = 4;
const CHIP_H_PAD: i32 = 7;
const CHIP_GAP: i32 = 4;

// Shadow: offset, alpha and blur radius (standard deviation).
const SHADOW_DX: i32 = 6;
const SHADOW_DY: i32 = 10;
const SHADOW_ALPHA: f32 = 120.0;
const SHADOW_BLUR: f32 = 22.0;

struct Fonts {
    display: Face,
    title: Face,
    section: Face,
    body: Face,
    chip: Face,
    tab: Face,
    sidebar: Face,
}

impl Fonts {
    fn load() -> Result<Fonts> {
        Ok(Fonts {
            display: Face::load(text::BOLD, 28.0)?,
            title: Face::load(text::BOLD, 18.0)?,
            section: Face::load(text::BOLD, 12.0)?,
            body: Face::load(text::SANS, 13.0)?,
            chip: Face::load(text::MONO, 11.0)?,
            tab: Face::load(text::SANS, 13.0)?,
            sidebar: Face::load(text::SANS, 16.0)?,
        })
    }
}

// Colour and shape helpers.

/// Blend `over` at 15% opacity over `bg` (truncating, like the Python code).
fn ghost_color(over: Rgb, bg: Rgb) -> Rgb {
    let mix = |c: u8, b: u8| (c as f64 * 0.15 + b as f64 * 0.85) as u8;
    (mix(over.0, bg.0), mix(over.1, bg.1), mix(over.2, bg.2))
}

fn paint(color: Rgb, anti_alias: bool) -> Paint<'static> {
    let mut p = Paint::default();
    p.set_color(Color::from_rgba8(color.0, color.1, color.2, 255));
    p.anti_alias = anti_alias;
    p
}

/// Fill the inclusive pixel rectangle `[x0, y0, x1, y1]`.
fn fill_rect(pm: &mut Pixmap, x0: i32, y0: i32, x1: i32, y1: i32, color: Rgb) {
    let rect = Rect::from_xywh(
        x0 as f32,
        y0 as f32,
        (x1 - x0 + 1) as f32,
        (y1 - y0 + 1) as f32,
    );
    if let Some(rect) = rect {
        pm.fill_rect(rect, &paint(color, false), Transform::identity(), None);
    }
}

/// Rounded rectangle path with edges `l, t, r, b` (continuous coordinates).
fn rounded_rect_path(l: f32, t: f32, r: f32, b: f32, rad: f32) -> Option<SkPath> {
    let k = rad * 0.552_284_8;
    let mut pb = PathBuilder::new();
    pb.move_to(l + rad, t);
    pb.line_to(r - rad, t);
    pb.cubic_to(r - rad + k, t, r, t + rad - k, r, t + rad);
    pb.line_to(r, b - rad);
    pb.cubic_to(r, b - rad + k, r - rad + k, b, r - rad, b);
    pb.line_to(l + rad, b);
    pb.cubic_to(l + rad - k, b, l, b - rad + k, l, b - rad);
    pb.line_to(l, t + rad);
    pb.cubic_to(l, t + rad - k, l + rad - k, t, l + rad, t);
    pb.close();
    pb.finish()
}

/// Fill the inclusive rounded rectangle `[x0, y0, x1, y1]`.
fn fill_rounded_rect(pm: &mut Pixmap, x0: i32, y0: i32, x1: i32, y1: i32, radius: i32, color: Rgb) {
    let path = rounded_rect_path(
        x0 as f32,
        y0 as f32,
        (x1 + 1) as f32,
        (y1 + 1) as f32,
        radius as f32,
    );
    if let Some(path) = path {
        pm.fill_path(
            &path,
            &paint(color, true),
            FillRule::Winding,
            Transform::identity(),
            None,
        );
    }
}

/// Draw a one pixel outline just inside the inclusive rounded rectangle.
fn outline_rounded_rect(
    pm: &mut Pixmap,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    radius: i32,
    color: Rgb,
) {
    let path = rounded_rect_path(
        x0 as f32 + 0.5,
        y0 as f32 + 0.5,
        (x1 + 1) as f32 - 0.5,
        (y1 + 1) as f32 - 0.5,
        radius as f32 - 0.5,
    );
    if let Some(path) = path {
        let stroke = Stroke {
            width: 1.0,
            ..Stroke::default()
        };
        pm.stroke_path(
            &path,
            &paint(color, true),
            &stroke,
            Transform::identity(),
            None,
        );
    }
}

/// One pixel high horizontal line from `x0` to `x1` (inclusive).
fn hline(pm: &mut Pixmap, x0: i32, x1: i32, y: i32, color: Rgb) {
    fill_rect(pm, x0, y, x1, y, color);
}

/// Icon followed by two spaces, or an empty string.
fn icon_prefix(name: &str) -> String {
    let icon = icon_for(name);
    if icon.is_empty() {
        String::new()
    } else {
        format!("{icon}  ")
    }
}

// Layout.

fn measure_app_block(app: &App) -> i32 {
    let mut height = APP_TITLE_HEIGHT;
    for section in &app.sections {
        height += SECTION_HEADER_HEIGHT;
        height += section.shortcuts.len() as i32 * SHORTCUT_ROW_HEIGHT;
    }
    height + APP_BOTTOM_PAD
}

/// Distribute apps across `COLUMNS` columns, always filling the shortest one.
fn build_grid(apps: &[App]) -> Vec<Vec<&App>> {
    let mut col_heights = [0i32; COLUMNS];
    let mut columns: Vec<Vec<&App>> = vec![Vec::new(); COLUMNS];
    for app in apps {
        let h = measure_app_block(app);
        // `min_by_key` returns the first minimum, like Python's `index(min(..))`.
        let idx = (0..COLUMNS).min_by_key(|&i| col_heights[i]).unwrap_or(0);
        columns[idx].push(app);
        col_heights[idx] += h + SECTION_GAP;
    }
    columns
}

fn content_height(columns: &[Vec<&App>]) -> i32 {
    columns
        .iter()
        .map(|col| {
            col.iter()
                .map(|a| measure_app_block(a) + SECTION_GAP)
                .sum::<i32>()
        })
        .max()
        .unwrap_or(200)
}

fn image_height(content_height: i32) -> i32 {
    EDGE_PADDING + HEADER_HEIGHT + CARD_PADDING + content_height + EDGE_PADDING
}

// Key chips.

fn chip_width(label: &str, font: &Face) -> i32 {
    font.width(label) + CHIP_H_PAD * 2
}

/// Draw key chips right-aligned, ending at `x_right`.
fn render_key_chips(pm: &mut Pixmap, chips: &[String], x_right: i32, y: i32, font: &Face) {
    let mut x = x_right;
    for chip in chips.iter().rev() {
        let w = chip_width(chip, font);
        x -= w;
        let (x1, y1) = (x + w, y + CHIP_HEIGHT);
        fill_rounded_rect(
            pm,
            x,
            y,
            x1,
            y1,
            CHIP_RADIUS,
            palette::SURFACE_CONTAINER_HIGHEST,
        );
        outline_rounded_rect(pm, x, y, x1, y1, CHIP_RADIUS, palette::OUTLINE_VARIANT);
        let tw = font.width(chip);
        font.draw(
            pm,
            x + (w - tw) / 2,
            y + (CHIP_HEIGHT - 11) / 2,
            chip,
            palette::ON_SURFACE,
        );
        x -= CHIP_GAP;
    }
}

// Component renderers.

fn render_sidebar(pm: &mut Pixmap, x0: i32, y0: i32, x1: i32, y1: i32, fonts: &Fonts) {
    fill_rect(pm, x0, y0, x1, y1, palette::SURFACE_CONTAINER_LOWEST);
    let icons = ["⊞", "◎", "▶", "≡", "⊙"];
    let icon_h = 32;
    let n = icons.len() as i32;
    let spacing = 8.max((y1 - y0 - n * icon_h).div_euclid(n + 1));
    let cx = (x0 + x1).div_euclid(2);
    for (i, icon) in icons.iter().enumerate() {
        let iy = y0 + spacing + i as i32 * (icon_h + spacing);
        let tw = fonts.sidebar.width(icon);
        fonts
            .sidebar
            .draw(pm, cx - tw / 2, iy, icon, palette::ON_SURFACE_VARIANT);
    }
}

fn render_header(pm: &mut Pixmap, card_x: i32, card_y: i32, card_width: i32, fonts: &Fonts) {
    let hx = card_x + CARD_PADDING;
    let hy = card_y + 16;

    fonts
        .display
        .draw(pm, hx, hy, "EVERFOREST COMMAND", palette::PRIMARY);

    let tab_y = hy + 28 + 6;
    fonts.tab.draw(pm, hx, tab_y, "Shortcuts", palette::PRIMARY);
    let tw = fonts.tab.width("Shortcuts");
    fill_rect(pm, hx, tab_y + 17, hx + tw, tab_y + 19, palette::PRIMARY);

    let corner_icons = ["⚙", "?", "○"];
    let mut rx = card_x + card_width - CARD_PADDING;
    for icon in corner_icons.iter().rev() {
        let iw = fonts.tab.width(icon);
        rx -= iw + 4;
        fonts
            .tab
            .draw(pm, rx, tab_y, icon, palette::ON_SURFACE_VARIANT);
        rx -= 16;
    }

    let sep_y = card_y + HEADER_HEIGHT - 1;
    let ghost = ghost_color(palette::OUTLINE_VARIANT, palette::SURFACE_CONTAINER_LOW);
    hline(
        pm,
        card_x + CARD_PADDING,
        card_x + card_width - CARD_PADDING,
        sep_y,
        ghost,
    );
}

fn render_section_header(pm: &mut Pixmap, name: &str, x: i32, y: i32, fonts: &Fonts) -> i32 {
    let label = format!("{}{}", icon_prefix(name), name.to_uppercase());
    fonts.section.draw(pm, x, y, &label, palette::SECONDARY);
    y + SECTION_HEADER_HEIGHT
}

fn render_shortcut_row(
    pm: &mut Pixmap,
    shortcut: &Shortcut,
    x: i32,
    y: i32,
    col_right: i32,
    fonts: &Fonts,
) -> i32 {
    let chips = parse_chips(&shortcut.keys);
    let text_y = y + (CHIP_HEIGHT - 13) / 2;
    fonts
        .body
        .draw(pm, x, text_y, &shortcut.description, palette::ON_SURFACE);
    render_key_chips(pm, &chips, col_right, y, &fonts.chip);
    y + SHORTCUT_ROW_HEIGHT
}

fn render_app_block(
    pm: &mut Pixmap,
    app: &App,
    x: i32,
    mut y: i32,
    col_right: i32,
    fonts: &Fonts,
) -> i32 {
    let title = format!("{}{}", icon_prefix(&app.app), app.app);
    fonts.title.draw(pm, x, y, &title, palette::PRIMARY);
    y += APP_TITLE_HEIGHT;

    for section in &app.sections {
        y = render_section_header(pm, &section.name, x, y, fonts);
        for shortcut in &section.shortcuts {
            y = render_shortcut_row(pm, shortcut, x, y, col_right, fonts);
        }
    }

    y + APP_BOTTOM_PAD
}

/// Draw the blurred ambient shadow behind the card onto the opaque `pm`.
///
/// Like Pillow, the colour channels are blurred together with the alpha
/// channel, so the colour fades with the blur factor `f`.
fn render_shadow(pm: &mut Pixmap, x0: i32, y0: i32, x1: i32, y1: i32) -> Result<()> {
    let (w, h) = (pm.width(), pm.height());
    let mut mask = Pixmap::new(w, h).ok_or_else(|| anyhow!("image too large"))?;
    fill_rounded_rect(&mut mask, x0, y0, x1, y1, CARD_RADIUS, palette::SHADOW);

    let mut plane: Vec<f32> = mask
        .data()
        .chunks_exact(4)
        .map(|p| p[3] as f32 / 255.0)
        .collect();
    blur::gaussian_blur(&mut plane, w as usize, h as usize, SHADOW_BLUR);

    let c = [palette::SHADOW.0, palette::SHADOW.1, palette::SHADOW.2];
    for (px, f) in pm.data_mut().chunks_exact_mut(4).zip(plane) {
        let alpha = SHADOW_ALPHA / 255.0 * f;
        for (d, s) in px.iter_mut().zip(c) {
            let src = s as f32 * f;
            *d = (src * alpha + *d as f32 * (1.0 - alpha)).round() as u8;
        }
    }
    Ok(())
}

/// Render the cheat sheet for `apps` and write it to `output`.
pub fn generate(apps: &[App], output: &Path) -> Result<()> {
    let fonts = Fonts::load()?;
    let columns = build_grid(apps);

    let content_height = content_height(&columns);

    let card_x = EDGE_PADDING + SIDEBAR_WIDTH + CARD_MARGIN;
    let card_width = IMAGE_WIDTH - card_x - EDGE_PADDING;
    let col_width =
        (card_width - 2 * CARD_PADDING - (COLUMNS as i32 - 1) * COLUMN_GAP) / COLUMNS as i32;

    let image_height = image_height(content_height);
    let card_y = EDGE_PADDING;
    let card_height = image_height - 2 * EDGE_PADDING;

    // Base background.
    let mut pm = Pixmap::new(IMAGE_WIDTH as u32, image_height as u32)
        .ok_or_else(|| anyhow!("cannot allocate {IMAGE_WIDTH}x{image_height} image"))?;
    let (r, g, b) = palette::SURFACE;
    pm.fill(Color::from_rgba8(r, g, b, 255));

    // Ambient shadow behind card.
    render_shadow(
        &mut pm,
        card_x + SHADOW_DX,
        card_y + SHADOW_DY,
        card_x + card_width + SHADOW_DX,
        card_y + card_height + SHADOW_DY,
    )?;

    // Main card.
    fill_rounded_rect(
        &mut pm,
        card_x,
        card_y,
        card_x + card_width,
        card_y + card_height,
        CARD_RADIUS,
        palette::SURFACE_CONTAINER_LOW,
    );

    // Sidebar (drawn after the card so it sits on the surface layer).
    render_sidebar(
        &mut pm,
        EDGE_PADDING,
        EDGE_PADDING,
        EDGE_PADDING + SIDEBAR_WIDTH,
        EDGE_PADDING + card_height,
        &fonts,
    );

    // Header.
    render_header(&mut pm, card_x, card_y, card_width, &fonts);

    // Content grid.
    let content_top = card_y + HEADER_HEIGHT + CARD_PADDING;
    for (col_idx, col_apps) in columns.iter().enumerate() {
        let cx = card_x + CARD_PADDING + col_idx as i32 * (col_width + COLUMN_GAP);
        let col_right = cx + col_width;
        let mut cy = content_top;
        for app in col_apps {
            cy = render_app_block(&mut pm, app, cx, cy, col_right, &fonts);
        }
    }

    pm.save_png(output)
        .map_err(|e| anyhow!("writing {}: {e}", output.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Section;

    fn app(name: &str, shortcuts: usize) -> App {
        App {
            app: name.into(),
            description: String::new(),
            sections: vec![Section {
                name: "S".into(),
                shortcuts: (0..shortcuts)
                    .map(|i| Shortcut {
                        keys: "Ctrl + A".into(),
                        description: format!("d{i}"),
                    })
                    .collect(),
            }],
        }
    }

    #[test]
    fn app_block_height() {
        // title 30 + section header 24 + 3 rows * 22 + bottom pad 16
        assert_eq!(measure_app_block(&app("a", 3)), 30 + 24 + 66 + 16);
    }

    #[test]
    fn grid_fills_shortest_column_first() {
        // Heights: 10 rows -> 290, 1 row -> 92, 1 row -> 92, 1 row -> 92, 2 rows -> 114.
        let apps = vec![
            app("big", 10),
            app("b", 1),
            app("c", 1),
            app("d", 1),
            app("e", 2),
        ];
        let grid = build_grid(&apps);
        let names: Vec<Vec<&str>> = grid
            .iter()
            .map(|c| c.iter().map(|a| a.app.as_str()).collect())
            .collect();
        // Columns 1..3 take b, c, d; then the first minimum (column 1, 112) takes e.
        assert_eq!(names, [vec!["big"], vec!["b", "e"], vec!["c"], vec!["d"]]);
    }

    #[test]
    fn ties_go_to_the_first_column() {
        let apps = vec![app("a", 1), app("b", 1)];
        let grid = build_grid(&apps);
        assert_eq!(grid[0].len(), 1);
        assert_eq!(grid[1].len(), 1);
    }

    #[test]
    fn image_height_for_fixture() {
        let apps = vec![app("a", 1), app("b", 4), app("c", 2)];
        let grid = build_grid(&apps);
        // Tallest column holds "b": 30 + 24 + 4 * 22 + 16 + 20 = 178.
        let content = content_height(&grid);
        assert_eq!(content, 178);
        // 20 + 72 + 28 + 178 + 20
        assert_eq!(image_height(content), 318);
    }

    #[test]
    fn ghost_color_truncates() {
        let g = ghost_color((0x45, 0x48, 0x3d), (0x15, 0x1d, 0x22));
        assert_eq!(g, (28, 35, 38));
    }
}
