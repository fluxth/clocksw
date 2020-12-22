// use crate::app::{ App };
use crate::views::view::{ View, SwitchView };
use crate::views::day_clock::{ DayClockView };
use crate::views::night_clock::{ NightClockView };

use rpi_led_matrix::{ LedCanvas };

pub struct Display {
    // views: Vec<Box<dyn View>>,
    current_view: Option<Box<dyn View>>
}

impl Display {

    pub const LED_WIDTH: i32 = 64;
    pub const LED_HEIGHT: i32 = 32;

    // pub fn switch_view(&'a mut self, index: usize) {
    //     self.current_view = Some(&mut self.views[index]);
    // }
    pub fn activate_display(&mut self) {
        self.current_view = Some(Box::new(DayClockView::new()));
    }

    pub fn switch_day_view(&mut self) {
        // if let Some(matrix) = App.matrix {
        //     matrix.led_matrix.
        // };

        self.current_view = Some(Box::new(DayClockView::new()));
    }

    pub fn switch_night_view(&mut self) {
        self.current_view = Some(Box::new(NightClockView::new()));
    }

    #[inline]
    pub fn draw(&mut self, canvas: &mut LedCanvas) -> bool {
        if let Some(view) = self.current_view.as_ref() {
            match view.should_switch_view() {
                SwitchView::DayClockView => {
                    self.switch_day_view()
                },
                SwitchView::NightClockView => {
                    self.switch_night_view()
                },
                SwitchView::Ignore => (),
            }
        }

        // TODO: Implement selective draw; make view sleep itself
        if let Some(view) = self.current_view.as_mut() {
            canvas.clear();
            return view.draw_next_frame(canvas)
        }
        
        false
    }

}

impl Display {

    pub fn new() -> Display {
        // let views: Vec<Box<dyn View>> = vec![Box::new(NightClockView::new())];

        Display {
            current_view: None,
        }
    }

    // pub fn set_brightness(brightness_percent: u8) {
        
    // }

}