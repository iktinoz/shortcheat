//! Everforest colour tokens (see DESIGN.md), shared by the TUI and the PNG renderer.

pub type Rgb = (u8, u8, u8);

pub const SURFACE: Rgb = (0x0c, 0x14, 0x1a);
#[allow(dead_code)]
pub const SURFACE_CONTAINER_LOWEST: Rgb = (0x0a, 0x11, 0x17);
#[allow(dead_code)]
pub const SURFACE_CONTAINER_LOW: Rgb = (0x15, 0x1d, 0x22);
#[allow(dead_code)]
pub const SURFACE_CONTAINER: Rgb = (0x1e, 0x27, 0x2d);
pub const SURFACE_CONTAINER_HIGHEST: Rgb = (0x2e, 0x36, 0x3c);
pub const PRIMARY: Rgb = (0xc2, 0xdc, 0x99);
#[allow(dead_code)]
pub const PRIMARY_CONTAINER: Rgb = (0xa7, 0xc0, 0x80);
#[allow(dead_code)]
pub const ON_PRIMARY: Rgb = (0x1a, 0x2b, 0x12);
pub const SECONDARY: Rgb = (0xff, 0xb5, 0x95);
pub const ON_SURFACE: Rgb = (0xdb, 0xe3, 0xeb);
pub const ON_SURFACE_VARIANT: Rgb = (0x9f, 0xa8, 0xb0);
pub const OUTLINE_VARIANT: Rgb = (0x45, 0x48, 0x3d);
pub const TERTIARY: Rgb = (0xd2, 0xb4, 0x78);
pub const SHADOW: Rgb = (0x07, 0x0f, 0x14);
