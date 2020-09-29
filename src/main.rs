mod app;
mod drivers;
mod display;
mod views;
mod utils;

// Entry point: parse args then initialize app in lib
fn main() {
    let mut a = app::App::new();
    a.run()
}
