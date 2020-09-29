use rpi_led_matrix::{ LedCanvas };

pub trait View {

    fn draw_next_frame(&mut self, canvas: &mut LedCanvas) -> bool;

    fn new() -> Self;

}