pub mod arial;

use embedded_graphics::{ fonts::Font };

pub trait RawFont: Font {
    const FONT_TABLE: &'static [u8];

    fn char_offset_impl(c: char) -> u32 {
        0
    }

    fn char_width_impl(c: char) -> u32 {
        if Self::VARIABLE_WIDTH {
            // let mut x_max = 0;
            // for y in 0..Self::CHARACTER_SIZE.height {
            //     for x in (x_max..Self::CHARACTER_SIZE.width).rev() {
            //         if Self::character_pixel(c, x, y) {
            //             x_max = x;
            //             break;
            //         }
            //     }
            // }
            // x_max + 1
            Self::CHARACTER_SIZE.width
        } else {
            Self::CHARACTER_SIZE.width
        }
    }

    fn character_pixel_impl(c: char, x: u32, y: u32) -> bool {
        false
    }
}