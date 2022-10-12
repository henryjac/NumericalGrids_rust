use crate::geometry::curves::Curves;

pub struct SpecialCurve;

impl SpecialCurve {
    pub fn new() -> SpecialCurve {
        SpecialCurve
    }
}

impl Curves for SpecialCurve {
    fn get_smin(&self) -> f32 {
        return -10_f32
    }
    fn get_smax(&self) -> f32 {
        return 5_f32
    }
    fn xs(&self, s: f32) -> f32 {
        return s
    }
    fn ys(&self, s: f32) -> f32 {
        match s < -3_f32 {
            true => 1_f32/2_f32 * 1_f32/(1_f32 + (-3_f32*(s+6_f32)).exp()),
            false => 1_f32/2_f32 * 1_f32/(1_f32 + (3_f32*s).exp()),
        }
    }
    fn dxs(&self, _s: f32) -> f32 {
        return 1_f32
    }
    fn dys(&self, s: f32) -> f32 {
        match s < -3_f32 {
            true => 3_f32*(-3_f32*(s+6_f32)).exp() / 
                (2_f32*f32::powf((-3_f32*(s+6_f32)).exp()+1_f32, 2_f32)),
            false => -3_f32*(3_f32*s).exp() / 
                (2_f32*f32::powf((3_f32*s).exp()+1_f32,2_f32)),
        }
    }
}
