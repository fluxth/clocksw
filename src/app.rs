use crate::drivers::Matrix;
use crate::display::Display;

use std::time;
use spin_sleep::{ SpinSleeper };

pub struct App {
    matrix: Option<Matrix>,
    display: Option<Display>,
}

const FPS: f32 = 20.;
const FPS_PERIOD_NS: u64 = (1. / FPS * 1e9) as u64;
const FPS_DURATION: time::Duration = time::Duration::from_nanos(FPS_PERIOD_NS);

// Methods
impl App {

    pub fn run(&mut self) {

        // TODO: Handle unwraps
        self.matrix = Some(Matrix::new().unwrap());
        self.display = Some(Display::new());
        
        // TODO: Handle panic outputs cleanly
        let matrix = match self.matrix.as_ref() {
            Some(r) => r,
            None => panic!("LED matrix failed to initialize!"),
        };

        let display = match self.display.as_mut() {
            Some(r) => r,
            None => panic!("Renderer failed to initialize!"),
        };

        let mut canvas = matrix.offscreen_canvas();

        let sleeper = SpinSleeper::default();
        let mut last_draw: time::Instant;
        let mut elapsed: time::Duration;

        display.activate_display();
        
        loop {
            last_draw = time::Instant::now();
            if display.draw(&mut canvas) {
                canvas = matrix.swap(canvas);
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

    pub const fn new() -> App {
        App { 
            matrix: None, 
            display: None
        }
    }
}
