//! ratatui colours, built from the shared palette.

use ratatui::style::Color;

use crate::palette::{self, Rgb};

const fn rgb(c: Rgb) -> Color {
    Color::Rgb(c.0, c.1, c.2)
}

pub const SURFACE_CONTAINER_HIGHEST: Color = rgb(palette::SURFACE_CONTAINER_HIGHEST);
pub const PRIMARY: Color = rgb(palette::PRIMARY);
pub const SECONDARY: Color = rgb(palette::SECONDARY);
pub const ON_SURFACE: Color = rgb(palette::ON_SURFACE);
pub const ON_SURFACE_VARIANT: Color = rgb(palette::ON_SURFACE_VARIANT);
pub const TERTIARY: Color = rgb(palette::TERTIARY);
pub const OUTLINE_VARIANT: Color = rgb(palette::OUTLINE_VARIANT);
