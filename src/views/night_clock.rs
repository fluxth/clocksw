use crate::views::view:: { View };
use crate::utils;

use rpi_led_matrix::{ LedCanvas };
use embedded_graphics::{
    pixelcolor::{Rgb888},
    prelude::*,
    // primitives::{Circle, Rectangle, Triangle},
    egtext, text_style,
};

use chrono::{prelude::*};
use profont;

const DAYS_OF_WEEK: [&'static str; 7] = ["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];
const WEEK_BRIGHTNESS: u8 = 150;
const COLORS_OF_WEEK: [Rgb888; 7] = [
    Rgb888::new(WEEK_BRIGHTNESS, 0, 0),
    Rgb888::new(WEEK_BRIGHTNESS, WEEK_BRIGHTNESS, 0),
    Rgb888::new(WEEK_BRIGHTNESS, 0, WEEK_BRIGHTNESS),
    Rgb888::new(0, WEEK_BRIGHTNESS, 0),
    Rgb888::new(WEEK_BRIGHTNESS, WEEK_BRIGHTNESS / 2, 0),
    Rgb888::new(0, WEEK_BRIGHTNESS, WEEK_BRIGHTNESS),
    Rgb888::new(WEEK_BRIGHTNESS / 2, 0, WEEK_BRIGHTNESS),
];

pub struct NightClockView {
    temp_cache: f32,
    temp_count: u8,
}

impl View for NightClockView {

    fn new() -> NightClockView {
        NightClockView {
            temp_cache: 0.,
            temp_count: 0,
        }
    }

    fn draw_next_frame(&mut self, canvas: &mut LedCanvas) -> bool {
        // let (width, height) = canvas.canvas_size();

        // let color = LedColor {
        //     red: 255,
        //     green: 255,
        //     blue: 255,
        // };

        let dt = Local::now();

        let time = dt.format("%H:%M:%S").to_string();
        let date = dt.format("%d-%m-%Y").to_string();
        egtext!(
            text = &time,
            top_left = (1, 0),
            style = text_style!(
                font = profont::ProFont12Point,
                text_color = Rgb888::new(255, 255, 255),
            )
        ).draw(canvas).unwrap();
        
        egtext!(
            text = &date,
            top_left = (2, 11),
            style = text_style!(
                font = profont::ProFont9Point,
                text_color = Rgb888::new(150, 150, 150),
            )
        ).draw(canvas).unwrap();

        let week_index = (dt.weekday().number_from_sunday() - 1) as usize;
        egtext!(
            text = DAYS_OF_WEEK[week_index],
            top_left = (2, 20),
            style = text_style!(
                font = profont::ProFont9Point,
                text_color = COLORS_OF_WEEK[week_index],
            )
        ).draw(canvas).unwrap();


        if self.temp_count == 0 {
            self.temp_count = 5;
            self.temp_cache = utils::get_cpu_temperature().unwrap()
        } else {
            self.temp_count -= 1;
        }

        egtext!(
            text = &format!("{:.1}c", self.temp_cache),
            top_left = (32, 20),
            style = text_style!(
                font = profont::ProFont9Point,
                text_color = Rgb888::new(150, 0, 0),
            )
        ).draw(canvas).unwrap();

        true
    }

}