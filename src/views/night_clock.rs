// use crate::display::{ Display };
use crate::views::view::{ View, SwitchView };
use crate::helpers::time::{ LocalTime };
use crate::views::components::large_digital_clock;

use rpi_led_matrix::{ LedCanvas };
use embedded_graphics::{
    pixelcolor::{Rgb888},
};

pub struct NightClockView {
    current_time: libc::time_t,
    current_tm: Option<libc::tm>,
    initialized: bool,
    forced_render: bool,
}

impl NightClockView {

}

impl NightClockView {
    const TEXT_COLOR: Rgb888 = Rgb888::new(100, 100, 0);

    pub fn new() -> NightClockView {
        NightClockView {
            current_time: 0,
            current_tm: None,
            initialized: false,
            forced_render: false,
        }
    }
}

impl View for NightClockView {

    fn view_activated(&mut self) {
        // Display::set_brightness(8); 
    }

    #[inline]
    fn draw_next_frame(&mut self, canvas: &mut LedCanvas) -> bool {
        if self.current_time == 0 || self.current_time % 30 == 0 {
            self.current_time = LocalTime::now();
        } else {
            self.current_time += 1;
        }

        match self.current_tm {
            Some(tm) => {
                self.current_tm = Some(
                    LocalTime::tm_modify(tm, self.current_time)
                );
            },
            None => {
                self.current_tm = Some(LocalTime::tm_new(self.current_time));
            }
        };

        let render = self.forced_render || !self.initialized || self.current_time % 60 == 0;
        if render {
            match large_digital_clock::render_frame(canvas, &self.current_tm.unwrap(), &Self::TEXT_COLOR) {
                Ok(_) => if self.forced_render { self.forced_render = false },
                Err(_) => self.forced_render = true,
            };
        }

        if self.initialized {
            LocalTime::sleep_until(self.current_time + 1);
        } else {
            self.current_time -= 1;
            self.initialized = true;
        }

        render
    }

    fn should_switch_view(&self) -> SwitchView {
        if let Some(tm) = self.current_tm {
            if tm.tm_hour >= 7 {
                return SwitchView::DayClockView
            }
        }

        SwitchView::Ignore
    }

}