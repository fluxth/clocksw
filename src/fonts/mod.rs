// Export fonts
pub mod arial;
pub mod tinypixel;

use embedded_graphics::{ fonts::Font };
use byteorder::{ByteOrder, LittleEndian};

#[allow(dead_code)]
pub struct RawFontMetadata {
    bitsize: u8,
    x: i8,
    y: i8,
    width: u8,
    height: u8,
    bitmap_index: usize,
}

pub trait RawFont: Font {
    const FONT_TABLE: &'static [u8];

    // Hard-coded: the decoding of u16 as little endian
    const TABLE_SIZE: u16 = ((Self::FONT_TABLE[1] as u16) << 8) 
                            | Self::FONT_TABLE[0] as u16;
    const IS_REDUCED: bool;
    const PAD_ENABLE: bool = true;

    const TABLE_ITEM_SIZE_BYTES: u32 = 7;

    #[inline]
    fn char_offset_reduced_impl(c: char) -> u32 {
        if c >= '+' && c <= ':' {
            return c as u32 - '+' as u32
        }

        // the ? character
        let q_pos = ':' as u32 - '+' as u32 + 1;

        if c >= 'A' && c <= 'F' {
            return c as u32 - 'A' as u32 + q_pos + 1
        }

        q_pos
    }

    #[inline]
    fn char_offset_impl(c: char) -> u32 {        
        if Self::IS_REDUCED {
            return Self::char_offset_reduced_impl(c)   
        }

        let fallback = '?' as u32 - ' ' as u32;
        if c < ' ' {
            return fallback
        }

        if c <= '~' {
            return c as u32 - ' ' as u32
        }

        fallback
    }

    fn get_metadata(c: char) -> Option<RawFontMetadata> {
        let char_index = Self::char_offset_impl(c);
        if char_index >= Self::TABLE_SIZE as u32 {
            return None
        }

        // Get index from table: Padded 2 bytes for length header
        let table_index = 2 + (char_index * Self::TABLE_ITEM_SIZE_BYTES) as usize;

        Some(RawFontMetadata {
            bitsize: Self::FONT_TABLE[table_index + 2],
            x: Self::FONT_TABLE[table_index + 3] as i8, // u8 to i8 not including negatives?
            y: Self::FONT_TABLE[table_index + 4] as i8,
            width: Self::FONT_TABLE[table_index + 5],
            height: Self::FONT_TABLE[table_index + 6],
            bitmap_index: LittleEndian::read_u16(
                &Self::FONT_TABLE[table_index..=table_index+1]
            ) as usize,
        })
    }

    #[inline]
    fn char_width_impl(c: char) -> u32 {
        if Self::VARIABLE_WIDTH {
            if let Some(meta) = Self::get_metadata(c) {
                return meta.bitsize as u32 + meta.x as u32
            }
        }
            
        Self::CHARACTER_SIZE.width
    }

    #[inline]
    fn character_pixel_impl(c: char, x: u32, y: u32) -> bool {
        if let Some(meta) = Self::get_metadata(c) {
            let max_x = meta.bitsize as i32 + meta.x as i32;
            let max_y = meta.height as i32;

            // TODO: Add yPad and font baseline support
            if x >= max_x as u32 || y >= max_y as u32 {
                return false
            }

            let bytesize: usize = match meta.bitsize {
                1..=8 => 1,
                9..=16 => 2,
                17..=32 => 4,
                33..=64 => 8,
                _ => return false
            };

            let row_index_bytes = meta.bitmap_index + (y as usize * bytesize);
            let payload = &Self::FONT_IMAGE[
                row_index_bytes ..= row_index_bytes+bytesize
            ];

            let row_value: u64 = match bytesize {
                1 => Self::FONT_IMAGE[row_index_bytes] as u64,
                2 => LittleEndian::read_u16(payload) as u64,
                4 => LittleEndian::read_u32(payload) as u64,
                8 => LittleEndian::read_u64(payload) as u64,
                _ => 0
            };

            let mut read_x = x as i32;
            if Self::PAD_ENABLE {
                read_x -= meta.x as i32;
            }

            if read_x < 0 {
                read_x = 0;
            }
            
            return (row_value & (1 << read_x as u32)) > 0
        }
        
        false
    }
}