use crate::objects::curves::Curves;

/// pub struct StraightLine
///
/// Straight lines defined by \[a,b\]*t + \[c,d\]
/// with t ranging from p_min to p_max,
/// which can be gotten from traits `get_pmin()`
/// and `get_pmax()` of the Curves trait.
pub struct StraightLine {
    a:f32, 
    b:f32,
    c:f32,
    d:f32,

    s_min:f32,
    s_max:f32,
}

impl StraightLine {
    /// Line of the unit square, with side being a value
    /// between 0 and 3, with 0 being y=0, 1 being x=1 and so on.
    pub fn unit(side: i8) -> StraightLine {
        let mut a=0_f32;
        let mut b=0_f32;
        let mut c=0_f32;
        let mut d=0_f32;
        match side {
            0 => a=1_f32,
            1 => {b=1_f32; d=0_f32;},
            2 => {a=1_f32; c=1_f32;},
            3 => b=1_f32,
            _ => println!("StraightLine::unit constructor expects a value between 0->3")
        }

        let s_min=0_f32;
        let s_max=1_f32;

        StraightLine{a,b,c,d,s_min,s_max}
    }

    /// Line defined by the parameters with
    /// parametrization from 0 to 1.
    pub fn default(a:f32,b:f32,c:f32,d:f32) -> StraightLine {
        let s_min = 0_f32;
        let s_max= 1_f32;
        StraightLine{a,b,c,d,s_min,s_max}
    }

    /// Line defined by `a`->`d` with parametrization from `p_min` to `p_max`.
    pub fn new(a:f32,b:f32,c:f32,d:f32,s_min:f32,s_max:f32) -> StraightLine {
        StraightLine{a,b,c,d,s_min,s_max}
    }
}

impl Curves for StraightLine {
    /// Start of curve parametrization.
    fn get_smin(&self) -> f32 {
        return self.s_min
    }
    /// End of curve parametrization.
    fn get_smax(&self) -> f32 {
        return self.s_max
    }
    /// x-value of line at `s`.
    fn xs(&self, t: f32) -> f32 {
        return self.a*t + self.c
    }
    /// y-value of line at `s`.
    fn ys(&self, t: f32) -> f32 {
        return self.b*t + self.d
    }
    /// Derivative of x-value of line at `s`.
    fn dxs(&self, _t: f32) -> f32 {
        return self.a
    }
    /// Derivative of y-value of line at `s`.
    fn dys(&self, _t: f32) -> f32 {
        return self.b
    }
}

#[test]
fn test_endpoint_unit() {
    let line = StraightLine::unit(0);
    assert_eq!(line.xt(line.p_min), 0_f32);
    assert_eq!(line.xt(line.p_max), 1_f32);
    assert_eq!(line.yt(line.p_min), 0_f32);
    assert_eq!(line.yt(line.p_max), 0_f32);
}

#[test]
fn test_lengths_unit() {
    let line = StraightLine::unit(0);
    assert_eq!(line.integrate(line.p_max), 1_f32);
}
