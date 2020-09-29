use crate::drivers::Matrix;
use crate::display::Display;

pub struct App {
    matrix: Matrix,
    display: Display,
}

// Methods
impl App {

    pub fn run(&mut self) {
        let mut canvas = self.matrix.offscreen_canvas();
        let duration = std::time::Duration::from_millis(1000);

        loop {
            if self.display.draw(&mut canvas) {
                    canvas = self.matrix.swap(canvas);
            }

            std::thread::sleep(duration);
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
