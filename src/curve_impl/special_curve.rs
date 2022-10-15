use crate::geometry::curves::Curves;
use crate::duals::DualNumber;

/// A special curve
pub struct SpecialCurve;

impl SpecialCurve {
    pub fn new() -> SpecialCurve {
        SpecialCurve
    }
}

impl<T> Curves<T> for SpecialCurve {
    fn get_smin(&self) -> f32 {
        return -10_f32
    }
    fn get_smax(&self) -> f32 {
        return 5_f32
    }
    fn xs(&self, s: DualNumber<f32>) -> DualNumber<f32> {
        return s
    }
    fn ys(&self, s: DualNumber<f32>) -> DualNumber<f32> {
        match s.get_a() < -3_f32 {
            true => (((s+6_f32)*(-3_f32)).exp() + 1_f32).inv() * 1_f32 / 2_f32,
            false => ((s*3_f32).exp() + 1_f32).inv() * 1_f32 / 2_f32,
        }
    }
}
