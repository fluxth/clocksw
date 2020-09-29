use rpi_led_matrix::{
    LedMatrix,
    LedCanvas,
    LedMatrixOptions, 
    LedRuntimeOptions,
};

pub struct Matrix {
    pub led_matrix: LedMatrix,
}

impl Matrix {

    pub fn swap(&self, canvas: LedCanvas) -> LedCanvas {
        self.led_matrix.swap(canvas)
    }

    pub fn canvas(&self) -> LedCanvas {
        self.led_matrix.canvas()
    }

    pub fn offscreen_canvas(&self) -> LedCanvas {
        self.led_matrix.offscreen_canvas()
    }

}

impl Matrix {
    pub fn new() -> Result<Matrix, &'static str> {
        let (opts, run_opts) = Matrix::configure()?;
        let led_matrix = LedMatrix::new(Some(opts), Some(run_opts))?;

        Ok(Matrix {
            led_matrix
        })
    }

    fn configure() -> Result<(LedMatrixOptions, LedRuntimeOptions), &'static str> {
        let mut opts = LedMatrixOptions::new();
        let mut run_opts = LedRuntimeOptions::new();

        opts.set_rows(32);
        opts.set_cols(64);

        opts.set_hardware_mapping("adafruit-hat-pwm");
        opts.set_hardware_pulsing(true);

        opts.set_refresh_rate(false);
        opts.set_brightness(50)?;
        opts.set_pwm_bits(8)?;
        opts.set_pwm_lsb_nanoseconds(500);
        opts.set_limit_refresh(80);

        // TODO: make this work
        // if std::env::var("DAEMON").unwrap_or_default().as_str() == "1" {
            run_opts.set_daemon(true);
        // }

        Ok((opts, run_opts))
    }
}