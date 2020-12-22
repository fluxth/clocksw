use rpi_led_matrix::{ LedCanvas };

pub enum SwitchView {
    Ignore,
    DayClockView,
    NightClockView,
}

pub trait View {

    // fn new() -> Self;

    fn view_activated(&mut self);
    fn draw_next_frame(&mut self, canvas: &mut LedCanvas) -> bool;
    fn should_switch_view(&self) -> SwitchView;

}