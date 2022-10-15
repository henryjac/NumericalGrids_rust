use crate::geometry::curves::Curves;
use crate::duals::DualNumber;

/// Straight lines defined by \[a,b\]*t + \[c,d\]
/// with t ranging from p_min to p_max,
/// which can be gotten from traits `get_pmin()`
/// and `get_pmax()` of the Curves trait.
#[derive(Debug)]
pub struct StraightLine<T> {
    a: T, 
    b: T,
    c: T,
    d: T,

    s_min: T,
    s_max: T,
}

impl<T> StraightLine<T> {
    /// Line defined by the parameters with
    /// parametrization from 0 to 1.
    pub fn from(a: f32, b: f32, c: f32, d: f32, s_min: f32, s_max: f32) -> StraightLine<f32> {
        StraightLine{a,b,c,d,s_min,s_max}
    }

    /// Line of the unit square, with side being a value
    /// between 0 and 3, with 0 being y=0, 1 being x=1 and so on.
    pub fn unit(side: u8) -> StraightLine<f32> {
        let mut a=0_f32;
        let mut b=0_f32;
        let mut c=0_f32;
        let mut d=0_f32;
        match side {
            0 => a=1_f32,
            1 => {b=1_f32; c=1_f32;},
            2 => {a=-1_f32; c=1_f32; d=1_f32},
            3 => {b=-1_f32; d=1_f32},
            _ => {
                panic!("StraightLine::unit constructor expects a value between 0..3");
            }
        }

        let s_min=0_f32;
        let s_max=1_f32;

        StraightLine{a,b,c,d,s_min,s_max}
    }

    pub fn zero_centered(side: u8, length: f32) -> StraightLine<f32> {
        let mut a = 0_f32;
        let mut b = 0_f32;
        let c: f32;
        let d: f32;
        match side {
            0 => {
                a = length;
                c = -length/2_f32;
                d = -length/2_f32;
            }
            1 => {
                b = length;
                c = length/2_f32;
                d = -length/2_f32;
            }
            2 => {
                a = -length;
                c = length/2_f32;
                d = length/2_f32;
            }
            3 => {
                b = -length;
                c = -length/2_f32;
                d = length/2_f32;
            }
            _ => panic!("StraightLine::unit consturctor expects a value between 0..3"),
        }
        StraightLine::<f32>::from(a, b, c, d, 0_f32, 1_f32)
    }
}

impl Curves<f32> for StraightLine<f32> {
    /// Start of curve parametrization.
    fn get_smin(&self) -> f32 {
        return self.s_min
    }
    /// End of curve parametrization.
    fn get_smax(&self) -> f32 {
        return self.s_max
    }
    /// x-value of line at `s`.
    fn xs(&self, t: DualNumber<f32>) -> DualNumber<f32> {
        return t*self.a + self.c
    }
    /// y-value of line at `s`.
    fn ys(&self, t: DualNumber<f32>) -> DualNumber<f32> {
        return t*self.b + self.d
    }
}

#[test]
fn test_endpoint_unit() {
    let lines = [StraightLine::<f32>::unit(0), StraightLine::<f32>::unit(1), StraightLine::<f32>::unit(2), StraightLine::<f32>::unit(3)];
    let line_endpoints: [f32; 16] = [
        0_f32, 1_f32, 0_f32, 0_f32,
        1_f32, 1_f32, 0_f32, 1_f32,
        1_f32, 0_f32, 1_f32, 1_f32,
        0_f32, 0_f32, 1_f32, 0_f32,
    ];
    for (i, line) in lines.iter().enumerate() {
        assert_eq!(line.xs(DualNumber::real(line.s_min)).get_a(), DualNumber::real(line_endpoints[i*4+0]).get_a());
        assert_eq!(line.xs(DualNumber::real(line.s_max)).get_a(), DualNumber::real(line_endpoints[i*4+1]).get_a());
        assert_eq!(line.ys(DualNumber::real(line.s_min)).get_a(), DualNumber::real(line_endpoints[i*4+2]).get_a());
        assert_eq!(line.ys(DualNumber::real(line.s_max)).get_a(), DualNumber::real(line_endpoints[i*4+3]).get_a());
    }
}

#[test]
fn test_lengths_unit() {
    let x0 = StraightLine::<f32>::unit(0);
    let x1 = StraightLine::<f32>::unit(2);
    assert_eq!(x0.integrate(x0.s_max), 1_f32);
    assert_eq!(x1.integrate(x1.s_max), 1_f32);
}

#[test]
fn test_find_s() {
    let x0 = StraightLine::<f32>::unit(0);
    assert!((0.3 - x0.find_s(0.3).abs() < 1e-6));
    assert!((0.5 - x0.find_s(0.5).abs() < 1e-6));
    assert!((0.8 - x0.find_s(0.8).abs() < 1e-6));
}
