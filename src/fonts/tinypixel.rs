use embedded_graphics::{
    geometry::Size,
    fonts::Font,
};

use crate::fonts::{ RawFont };

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct TinyPixel5;

impl RawFont for TinyPixel5 {
    const FONT_TABLE: &'static [u8] = include_bytes!("../../data/fonts/TinyPixel-5-Reduced.rftable");
    const IS_REDUCED: bool = true;
    const PAD_ENABLE: bool = false;
}

impl Font for TinyPixel5 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../data/fonts/TinyPixel-5-Reduced.rawfont");

    const CHARACTER_SIZE: Size = Size::new(4, 6);
    const FONT_IMAGE_WIDTH: u32 = 8;
    const VARIABLE_WIDTH: bool = false;

    fn char_offset(c: char) -> u32 {
        Self::char_offset_impl(c)
    }

    fn char_width(c: char) -> u32 {
        Self::char_width_impl(c)
    }

    fn character_pixel(c: char, x: u32, y: u32) -> bool {
        Self::character_pixel_impl(c, x, y)
    }
}