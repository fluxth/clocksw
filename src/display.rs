use crate::views::view::{ View, SwitchView };
use crate::views::day_clock::DayClockView;
use crate::views::night_clock::NightClockView;

use rpi_led_matrix::LedCanvas;

pub struct Display {
    views: Vec<Box<dyn View>>,
    current_view_index: Option<usize>,
}

impl Display {

    pub const LED_WIDTH: i32 = 64;
    pub const LED_HEIGHT: i32 = 32;

    pub fn switch_view(&mut self, index: usize) -> Result<&mut dyn View, String> {
        if index < self.views.len() {
            self.current_view_index = Some(index);
            Ok(self.views[index].as_mut())
        } else {
            Err("Error".to_owned())
        }
    }

    pub fn get_current_view(&mut self) -> Option<&mut dyn View> {
        let index = self.current_view_index?;
        Some(self.views[index].as_mut())
    }

    #[inline]
    pub fn draw(&mut self, canvas: &mut LedCanvas) -> bool {
        if let Some(view) = self.get_current_view() {
            let view = match view.should_switch_view() {
                SwitchView::DayClockView => self.switch_view(0).unwrap(),
                SwitchView::NightClockView => self.switch_view(1).unwrap(),
                SwitchView::Ignore => self.get_current_view().unwrap(),
            };

            // TODO: Implement selective draw; make view sleep itself
            canvas.clear();
            view.draw_next_frame(canvas)
        } else {
            false
        }
    }

}

impl Display {

    pub fn new() -> Display {
        let views: Vec<Box<dyn View>> = vec![
            Box::new(DayClockView::new()),
            Box::new(NightClockView::new()),
        ];

        Display {
            views,
            current_view_index: Some(0),
        }
    }

    // pub fn set_brightness(brightness_percent: u8) {
        
    // }

}
