use embedded_graphics::{
    geometry::Size,
    fonts::Font,
};

use crate::fonts::{ RawFont };

// const EMPTY_BUF: [u8; 0] = [];

// #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
// pub struct ArialRegular24;

// impl RawFont for ArialRegular24 {
//     const FONT_TABLE: &'static [u8] = include_bytes!("../../data/fonts/ArialMT-24.rftable");
//     const IS_REDUCED: bool = false;
// }

// impl Font for ArialRegular24 {
//     const FONT_IMAGE: &'static [u8] = include_bytes!("../../data/fonts/ArialMT-24.rawfont");

//     const CHARACTER_SIZE: Size = Size::new(15, 23);
//     const FONT_IMAGE_WIDTH: u32 = 16;
//     const VARIABLE_WIDTH: bool = false;

//     fn char_offset(c: char) -> u32 {
//         Self::char_offset_impl(c)
//     }

//     fn char_width(c: char) -> u32 {
//         Self::char_width_impl(c)
//     }

//     fn character_pixel(c: char, x: u32, y: u32) -> bool {
//         Self::character_pixel_impl(c, x, y)
//     }
// }

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ArialRegularReduced24;

impl RawFont for ArialRegularReduced24 {
    const FONT_TABLE: &'static [u8] = include_bytes!("../../data/fonts/ArialMT-24-Reduced.rftable");
    const IS_REDUCED: bool = true;
}

impl Font for ArialRegularReduced24 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../data/fonts/ArialMT-24-Reduced.rawfont");

    const CHARACTER_SIZE: Size = Size::new(13, 23);
    const FONT_IMAGE_WIDTH: u32 = 16;
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