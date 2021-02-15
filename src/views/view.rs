use rpi_led_matrix::LedCanvas; 

pub enum SwitchView {
    Ignore = 0,
    DayClockView,
    NightClockView,
}

pub trait View {

    // fn new() -> Self;

    fn view_activated(&mut self);
    fn update(&mut self) -> bool;
    fn draw_next_frame(&mut self, canvas: &mut LedCanvas) -> bool;

}
