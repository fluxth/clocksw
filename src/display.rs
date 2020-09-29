use crate::views::view::{ View };
use crate::views::night_clock::{ NightClockView };

use rpi_led_matrix::{ LedCanvas };

pub struct Display {
    current_view: NightClockView,
}

impl Display {

    #[inline]
    pub fn draw(&mut self, canvas: &mut LedCanvas) -> bool {
        canvas.clear();

        // TODO: Implement constant fps
        self.current_view.draw_next_frame(canvas)
    }

}

impl Display {

    pub fn new() -> Display {
        let view: NightClockView = View::new();
        Display {
            current_view: view
        }
    }

}