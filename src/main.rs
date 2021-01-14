use clocksw::drivers::Matrix;
use clocksw::display::Display;

use std::time;
use spin_sleep::SpinSleeper;

const FPS: f32 = 20.;
const FPS_PERIOD_NS: u64 = (1. / FPS * 1e9) as u64;
const FPS_DURATION: time::Duration = time::Duration::from_nanos(FPS_PERIOD_NS);

fn main() {

    let matrix = Matrix::new().unwrap();
    let mut display = Display::new();

    // TODO: Handle panic outputs cleanly
    let mut canvas = matrix.offscreen_canvas();

    let sleeper = SpinSleeper::default();
    let mut last_draw: time::Instant;
    let mut elapsed: time::Duration;
    
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
