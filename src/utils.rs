pub fn get_cpu_temperature() -> Result<f32, &'static str> {
    if let Ok(result) = std::fs::read_to_string("/sys/class/thermal/thermal_zone0/temp") {
        let value = result.trim().parse::<f32>();
        Ok(value.unwrap_or(0.) / 1000.)
    } else {
        Err("Cannot stat CPU temperature")
    }
}