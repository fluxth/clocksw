use crate::fonts::arial;

use std::error::{ Error };
use rpi_led_matrix::{ LedCanvas };
use embedded_graphics::{
    prelude::*,
    egtext, text_style,
    pixelcolor::{Rgb888},
};

const Y_POS: i32 = 8;

#[inline]
pub fn render_frame(
        canvas: &mut LedCanvas, 
        tm: &libc::tm,
        text_color: &Rgb888) -> Result<(), Box<dyn Error>> {

    let hr = format!("{:02}", tm.tm_hour);
    let min = format!("{:02}", tm.tm_min);

    let mut pad_x = 0;
    if hr.chars().next().unwrap() == '1' {
        pad_x = -2;
    }

    egtext!(
        text = &hr,
        top_left = (2 + pad_x, Y_POS),
        style = text_style!(
            font = arial::ArialRegularReduced24,
            text_color = *text_color,
        )
    ).draw(canvas)?;

    egtext!(
        text = &min,
        top_left = (34 + pad_x, Y_POS),
        style = text_style!(
            font = arial::ArialRegularReduced24,
            text_color = *text_color,
        )
    ).draw(canvas)?;

    egtext!(
        text = ":",
        top_left = (27 + pad_x, Y_POS + 2),
        style = text_style!(
            font = arial::ArialRegularReduced24,
            text_color = *text_color,
        )
    ).draw(canvas)?;

    Ok(())
}