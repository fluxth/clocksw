pub fn get_cpu_temperature() -> Result<f32, &'static str> {
    let contents = std::fs::read_to_string("/sys/class/thermal/thermal_zone0/temp");
    match contents {
        Ok(v) => Ok(v.trim().parse::<f32>().unwrap_or(0.) / 1000.),
        Err(_) => Err("Cannot stat CPU temperature"),
    }
}