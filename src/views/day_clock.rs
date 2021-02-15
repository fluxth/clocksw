// use profont;

// const DAYS_OF_WEEK: [&'static str; 7] = ["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];

use crate::display::Display;
use crate::views::view::View;
use crate::helpers::{
    cap_unit_float,
    time::CLocalTime
};
use crate::views::components::large_digital_clock;
use crate::utils::get_cpu_temperature;

use std::error::{ Error };
use std::cell::Cell;
use rpi_led_matrix::{ LedCanvas, LedColor };
use embedded_graphics::{
    prelude::*,
    style::PrimitiveStyle,
    primitives::Line,
    pixelcolor::Rgb888,
    egtext, text_style,
};

use crate::fonts::tinypixel::TinyPixel5;

pub struct DayClockView<'a> {
    display: &'a Display,
    current_time: libc::time_t,
    current_tm: Option<libc::tm>,
    initialized: bool,
    temperature_cache: f32,
}

impl<'a> DayClockView<'a> {

    const WEEK_BRIGHTNESS: u8 = 150;
    const COLORS_OF_WEEK: [Rgb888; 7] = [
        Rgb888::new(Self::WEEK_BRIGHTNESS, 0, 0),
        Rgb888::new(Self::WEEK_BRIGHTNESS, Self::WEEK_BRIGHTNESS, 0),
        Rgb888::new(Self::WEEK_BRIGHTNESS, 0, Self::WEEK_BRIGHTNESS),
        Rgb888::new(0, Self::WEEK_BRIGHTNESS, 0),
        Rgb888::new(Self::WEEK_BRIGHTNESS, Self::WEEK_BRIGHTNESS / 2, 0),
        Rgb888::new(0, Self::WEEK_BRIGHTNESS, Self::WEEK_BRIGHTNESS),
        Rgb888::new(Self::WEEK_BRIGHTNESS / 2, 0, Self::WEEK_BRIGHTNESS),
    ];

    #[inline]
    pub fn draw_border(&self, canvas: &mut LedCanvas) -> Result<(), Box<dyn Error>> {
        let tm = self.current_tm.ok_or("Time data not present")?;

        let color = Self::COLORS_OF_WEEK[tm.tm_wday as usize];
        let style = PrimitiveStyle::with_stroke(color, 1);

        // [50]--5--[00]--1--[10]
        //  |                 |
        //  4                 2
        //  |                 |
        // [40]------3-------[20]

        let sec = tm.tm_sec as f32;
        const LED_HALF_WIDTH: i32 = Display::LED_WIDTH / 2;

        // Segment 1
        if sec != 59. {
            Line::new(
                Point::new(LED_HALF_WIDTH, 0), 
                Point::new(
                    (LED_HALF_WIDTH) + (LED_HALF_WIDTH as f32 * cap_unit_float(sec / 10.)) as i32, 
                    0
                )
            )
            .into_styled(style)
            .draw(canvas)?;
        } else {
            Line::new(
                Point::new(0, 0), 
                Point::new(Display::LED_WIDTH - 1, 0)
            )
            .into_styled(style)
            .draw(canvas)?;
        }

        if sec < 10. {
            return Ok(())
        }

        // Segment 2
        Line::new(
            Point::new(Display::LED_WIDTH - 1, 0), 
            Point::new(
                Display::LED_WIDTH - 1,
                (Display::LED_HEIGHT as f32 * cap_unit_float((sec - 10.) / 10.)) as i32
            )
        )
        .into_styled(style)
        .draw(canvas)?;

        if sec < 20. {
            return Ok(())
        }

        // Segment 3
        Line::new(
            Point::new(Display::LED_WIDTH - 1, Display::LED_HEIGHT - 1), 
            Point::new(
                Display::LED_WIDTH - (Display::LED_WIDTH as f32 * cap_unit_float((sec - 20.) / 20.)) as i32,
                Display::LED_HEIGHT - 1
            )
        )
        .into_styled(style)
        .draw(canvas)?;

        if sec < 40. {
            return Ok(())
        }

        // Segment 4
        Line::new(
            Point::new(0, Display::LED_HEIGHT - 1), 
            Point::new(
                0,
                ((Display::LED_HEIGHT - 1) as f32 * (1. - cap_unit_float((sec - 40.) / 10.))) as i32
            )
        )
        .into_styled(style)
        .draw(canvas)?;

        if sec < 50. {
            return Ok(())
        }

        // Segment 5
        Line::new(
            Point::new(0, 0), 
            Point::new(
                (LED_HALF_WIDTH as f32 * cap_unit_float((sec - 50.) / 9.)) as i32,
                0
            )
        )
        .into_styled(style)
        .draw(canvas)?;

        Ok(())
    }

    #[inline]
    pub fn draw_temperature(&mut self, canvas: &mut LedCanvas) -> Result<(), Box<dyn Error>> {
        if self.temperature_cache < 0. || self.current_time % 5 == 0 {
            self.temperature_cache = get_cpu_temperature().unwrap_or(0.);
        }

        const CPU_COLOR: Rgb888 = Rgb888::new(200, 0, 0);

        let temp_whole = self.temperature_cache.floor();
        let temp_decimal = ((self.temperature_cache - temp_whole) * 10.) as u8;

        egtext!(
            text = &format!("{:02}", temp_whole),
            top_left = (1, 2),
            style = text_style!(
                font = TinyPixel5,
                text_color = CPU_COLOR,
            )
        ).draw(canvas)?;

        canvas.set(10, 6, &LedColor::from(CPU_COLOR));

        egtext!(
            text = &format!("{}", temp_decimal),
            top_left = (11, 2),
            style = text_style!(
                font = TinyPixel5,
                text_color = CPU_COLOR,
            )
        ).draw(canvas)?;

        Ok(())
    }

}

impl<'a> DayClockView<'a> {
    const TEXT_COLOR: Rgb888 = Rgb888::new(255, 255, 255);

    pub fn new(display: &'a Display) -> DayClockView {
        DayClockView {
            display,
            current_time: 0,
            current_tm: None,
            initialized: false,
            temperature_cache: -1.,
        }
    }
}

impl<'a> View for DayClockView<'a> {

    fn view_activated(&mut self) {
        // Display::set_brightness(8);
    }

    #[inline]
    fn update(&mut self) -> bool {
        if let Some(tm) = self.current_tm {
            if tm.tm_hour < 7 {
               // switch night 
               // TODO: Replace index with enum
               self.display.current_view_index.set(Some(1));
               return false
            }
        }

        if self.current_time == 0 || self.current_time % 10 == 0 {
            self.current_time = CLocalTime::now();
        } else {
            self.current_time += 1;
        }

        match self.current_tm {
            Some(tm) => {
                self.current_tm = Some(
                    CLocalTime::tm_modify(tm, self.current_time)
                );
            },
            None => {
                self.current_tm = Some(CLocalTime::tm_new(self.current_time));
            }
        };

        true
    }

    #[inline]
    fn draw_next_frame(&mut self, canvas: &mut LedCanvas) -> bool {

        // Draw temperature
        let _ = self.draw_temperature(canvas);

        // Draw seconds
        let _ = egtext!(
            text = &format!("{:02}", self.current_tm.unwrap().tm_sec),
            top_left = (27, 2),
            style = text_style!(
                font = TinyPixel5,
                text_color = Rgb888::new(100, 100, 100),
            )
        ).draw(canvas);

        // Draw border
        let _ = self.draw_border(canvas);
        let _ = large_digital_clock::render_frame(
            canvas, &self.current_tm.unwrap(), &Self::TEXT_COLOR
        );

        if self.initialized {
            CLocalTime::sleep_until(self.current_time + 1);
        } else {
            self.current_time -= 1;
            self.initialized = true;
        }

        true
    }

}

// let (width, height) = canvas.canvas_size();

// let color = LedColor {
//     red: 255,
//     green: 255,
//     blue: 255,
// };

// let dt = Local::now();

// let time = dt.format("%H:%M:%S").to_string();
// let date = dt.format("%d-%m-%Y").to_string();
// egtext!(
//     text = &time,
//     top_left = (1, 0),
//     style = text_style!(
//         font = profont::ProFont12Point,
//         text_color = Rgb888::new(255, 255, 255),
//     )
// ).draw(canvas).unwrap();

// egtext!(
//     text = &date,
//     top_left = (2, 11),
//     style = text_style!(
//         font = profont::ProFont9Point,
//         text_color = Rgb888::new(150, 150, 150),
//     )
// ).draw(canvas).unwrap();

// let week_index = (dt.weekday().number_from_sunday() - 1) as usize;
// egtext!(
//     text = DAYS_OF_WEEK[week_index],
//     top_left = (2, 20),
//     style = text_style!(
//         font = profont::ProFont9Point,
//         text_color = COLORS_OF_WEEK[week_index],
//     )
// ).draw(canvas).unwrap();


// if self.temp_count == 0 {
//     self.temp_count = 100;
//     self.temp_cache = utils::get_cpu_temperature().unwrap()
// } else {
//     self.temp_count -= 1;
// }

// egtext!(
//     text = &format!("{:.1}c", self.temp_cache),
//     top_left = (32, 20),
//     style = text_style!(
//         font = profont::ProFont9Point,
//         text_color = Rgb888::new(150, 0, 0),
//     )
// ).draw(canvas).unwrap();

// true
