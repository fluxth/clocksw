pub mod time;

#[inline]
pub fn cap_unit_float(uf: f32) -> f32 {
    if uf < 0. {
        return 0.
    }

    if uf > 1. {
        return 1.
    }

    uf
}