use crate::drivers::Matrix;
use crate::display::Display;

use std::time;
use spin_sleep::{ SpinSleeper };

pub struct App {
    matrix: Matrix,
    display: Display,
}

const FPS: f32 = 20.;
const FPS_PERIOD_NS: u64 = (1. / FPS * 1e9) as u64;
const FPS_DURATION: time::Duration = time::Duration::from_nanos(FPS_PERIOD_NS);

// Methods
impl App {

    pub fn run(&mut self) {
        let mut canvas = self.matrix.offscreen_canvas();

        let sleeper = SpinSleeper::default();
        let mut last_draw: time::Instant;
        let mut elapsed: time::Duration;
        loop {
            last_draw = time::Instant::now();
            if self.display.draw(&mut canvas) {
                canvas = self.matrix.swap(canvas);
            }

            elapsed = time::Instant::now().duration_since(last_draw);
            if elapsed < FPS_DURATION {
                sleeper.sleep(FPS_DURATION - elapsed);
            }
            // TODO: else possible frame dropped?
        }
    }

}

// Related functions
impl App {

    pub fn new() -> App {
        // TODO: Handle unwraps
        App { 
            matrix: Matrix::new().unwrap(),
            display: Display::new(),
        }
    }

}
