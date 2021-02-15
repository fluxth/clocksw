use crate::views::view::View;

use std::cell::Cell;
use rpi_led_matrix::LedCanvas;

pub struct Display {
    pub views: Vec<Box<dyn View>>,
    pub current_view_index: Cell<Option<usize>>
}

impl Display {
    pub const LED_WIDTH: i32 = 64;
    pub const LED_HEIGHT: i32 = 32;

    pub fn switch_view(&mut self, index: usize) -> Result<&mut dyn View, String> {
        if index < self.views.len() {
            self.current_view_index.set(Some(index));
            Ok(self.views[index].as_mut())
        } else {
            Err("Error".to_owned())
        }
    }

    pub fn get_current_view(&mut self) -> Option<&mut dyn View> {
        let index = self.current_view_index.get()?;
        Some(self.views[index].as_mut())
    }

    #[inline]
    pub fn draw(&mut self, canvas: &mut LedCanvas) -> bool {
        if let Some(view) = self.get_current_view() {
            if view.update() {
                // TODO: Implement selective draw; make view sleep itself
                canvas.clear();
                view.draw_next_frame(canvas)
            } else {
                false
            }

            /*let switch_to = view.should_switch_view();
            let view = match switch_to {
                SwitchView::Ignore => view,
                _ => {
                    let index = switch_to as usize;
                    // TODO: usize may underflow
                    self.switch_view(index - 1).unwrap()
                },
            };*/
        } else {
            false
        }
    }

}

impl Display {

    pub fn new() -> Self {
        let views: Vec<Box<dyn View>> = Vec::with_capacity(2);
        Self {
            views,
            current_view_index: Cell::new(None),
        }
    }

    // pub fn set_brightness(brightness_percent: u8) {
        
    // }

}
