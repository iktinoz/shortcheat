//! Font loading, text measuring and glyph drawing.
//!
//! Sizes follow Pillow: the size is in em pixels, and the text position is the
//! top of the ascender (not the baseline).

use std::fs;

use ab_glyph::{Font, FontVec, GlyphId, PxScale, ScaleFont};
use anyhow::{anyhow, Result};
use tiny_skia::Pixmap;

use crate::palette::Rgb;

pub const SANS: &[&str] = &[
    "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
    "/usr/share/fonts/TTF/DejaVuSans.ttf",
    "/usr/share/fonts/dejavu/DejaVuSans.ttf",
];
pub const BOLD: &[&str] = &[
    "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
    "/usr/share/fonts/TTF/DejaVuSans-Bold.ttf",
    "/usr/share/fonts/dejavu/DejaVuSans-Bold.ttf",
];
pub const MONO: &[&str] = &[
    "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
    "/usr/share/fonts/TTF/DejaVuSansMono.ttf",
    "/usr/share/fonts/dejavu/DejaVuSansMono.ttf",
];

pub struct Face {
    font: FontVec,
    scale: PxScale,
}

impl Face {
    /// Load the first existing font from `paths` at `size` em pixels.
    pub fn load(paths: &[&str], size: f32) -> Result<Face> {
        for path in paths {
            let Ok(bytes) = fs::read(path) else { continue };
            let font = FontVec::try_from_vec(bytes)
                .map_err(|e| anyhow!("invalid font file {path}: {e}"))?;
            // ab_glyph scales by (ascent - descent); Pillow scales by em size.
            let units_per_em = font.units_per_em().unwrap_or(1000.0);
            let scale = PxScale::from(size * font.height_unscaled() / units_per_em);
            return Ok(Face { font, scale });
        }
        Err(anyhow!(
            "no DejaVu font found, looked in:\n  {}\n\
             install it, for example: sudo apt install fonts-dejavu-core",
            paths.join("\n  ")
        ))
    }

    /// Text width in whole pixels: glyph advances plus kerning.
    pub fn width(&self, text: &str) -> i32 {
        let scaled = self.font.as_scaled(self.scale);
        let mut x = 0.0f32;
        let mut prev: Option<GlyphId> = None;
        for c in text.chars() {
            let id = scaled.glyph_id(c);
            if let Some(p) = prev {
                x += scaled.kern(p, id);
            }
            x += scaled.h_advance(id);
            prev = Some(id);
        }
        x as i32
    }

    /// Draw `text` with the top of the ascender at `y`, starting at `x`.
    pub fn draw(&self, pixmap: &mut Pixmap, x: i32, y: i32, text: &str, color: Rgb) {
        let scaled = self.font.as_scaled(self.scale);
        let baseline = y as f32 + scaled.ascent().ceil();
        let mut caret = x as f32;
        let mut prev: Option<GlyphId> = None;
        for c in text.chars() {
            let id = scaled.glyph_id(c);
            if let Some(p) = prev {
                caret += scaled.kern(p, id);
            }
            let glyph = id.with_scale_and_position(self.scale, ab_glyph::point(caret, baseline));
            if let Some(outlined) = self.font.outline_glyph(glyph) {
                let bounds = outlined.px_bounds();
                let (ox, oy) = (bounds.min.x as i32, bounds.min.y as i32);
                outlined.draw(|gx, gy, coverage| {
                    blend_pixel(pixmap, ox + gx as i32, oy + gy as i32, color, coverage);
                });
            }
            caret += scaled.h_advance(id);
            prev = Some(id);
        }
    }
}

/// Source-over blend of `color` with `coverage` as alpha.
///
/// The pixmap holds premultiplied RGBA.
fn blend_pixel(pixmap: &mut Pixmap, x: i32, y: i32, color: Rgb, coverage: f32) {
    let (w, h) = (pixmap.width() as i32, pixmap.height() as i32);
    if x < 0 || y < 0 || x >= w || y >= h || coverage <= 0.0 {
        return;
    }
    let a = coverage.min(1.0);
    let i = (y * w + x) as usize * 4;
    let px = &mut pixmap.data_mut()[i..i + 4];
    for (d, s) in px.iter_mut().zip([color.0, color.1, color.2]) {
        *d = (s as f32 * a + *d as f32 * (1.0 - a)).round() as u8;
    }
    px[3] = (255.0 * a + px[3] as f32 * (1.0 - a)).round() as u8;
}
