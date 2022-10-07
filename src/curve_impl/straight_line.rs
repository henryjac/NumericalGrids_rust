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

    p_min:f32,
    p_max:f32,
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

        let p_min=0_f32;
        let p_max=1_f32;

        StraightLine{a,b,c,d,p_min,p_max}
    }

    /// Line defined by the parameters with
    /// parametrization from 0 to 1.
    pub fn default(a:f32,b:f32,c:f32,d:f32) -> StraightLine {
        let p_min = 0_f32;
        let p_max= 1_f32;
        StraightLine{a,b,c,d,p_min,p_max}
    }

    /// Line defined by `a`->`d` with parametrization from `p_min` to `p_max`.
    pub fn new(a:f32,b:f32,c:f32,d:f32,p_min:f32,p_max:f32) -> StraightLine {
        StraightLine{a,b,c,d,p_min,p_max}
    }
}

impl Curves for StraightLine {
    /// Start of curve parametrization.
    fn get_tmin(&self) -> f32 {
        return self.p_min
    }
    /// End of curve parametrization.
    fn get_tmax(&self) -> f32 {
        return self.p_max
    }
    /// X-value of line at `t`.
    fn xt(&self, t: f32) -> f32 {
        return self.a*t + self.c
    }
    fn yt(&self, t: f32) -> f32 {
        return self.b*t + self.d
    }
    fn dxt(&self, _t: f32) -> f32 {
        return self.a
    }
    fn dyt(&self, _t: f32) -> f32 {
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
