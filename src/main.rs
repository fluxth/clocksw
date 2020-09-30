mod app;
mod drivers;
mod display;
mod views;
mod utils;
mod fonts;

use app::App;

// Entry point: parse args then initialize app in lib
fn main() {
    let mut a = App::new();
    a.run()
}
