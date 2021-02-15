use crate::views::view::View;
use crate::helpers::time::CLocalTime;
use crate::views::components::large_digital_clock;
use crate::display::Display;

use std::cell::Cell;
use rpi_led_matrix::LedCanvas;
use embedded_graphics::{
    pixelcolor::Rgb888,
};

pub struct NightClockView<'a> {
    display: &'a Display,
    current_time: libc::time_t,
    current_tm: Option<libc::tm>,
    initialized: bool,
    forced_render: bool,
}

impl<'a> NightClockView<'a> {

}

impl<'a> NightClockView<'a> {
    const TEXT_COLOR: Rgb888 = Rgb888::new(100, 100, 0);

    pub fn new(display: &'a Display) -> Self {
        Self {
            display,
            current_time: 0,
            current_tm: None,
            initialized: false,
            forced_render: false,
        }
    }
}

impl<'a> View for NightClockView<'a> {

    fn view_activated(&mut self) {
        // Display::set_brightness(8); 
    }

    fn update(&mut self) -> bool {
        if let Some(tm) = self.current_tm {
            if tm.tm_hour >= 7 {
                // set to day view
                // TODO: replace index with enum
                self.display.current_view_index.set(Some(0));
                return false
            }
        }

        if self.current_time == 0 || self.current_time % 30 == 0 {
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
        let render = self.forced_render || !self.initialized || self.current_time % 60 == 0;
        if render {
            match large_digital_clock::render_frame(canvas, &self.current_tm.unwrap(), &Self::TEXT_COLOR) {
                Ok(_) => if self.forced_render { self.forced_render = false },
                Err(_) => self.forced_render = true,
            };
        }

        if self.initialized {
            CLocalTime::sleep_until(self.current_time + 1);
        } else {
            self.current_time -= 1;
            self.initialized = true;
        }

        render
    }

}
