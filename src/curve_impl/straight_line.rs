use crate::objects::curves::Curves;

pub struct StraightLine {
    a:f32, 
    b:f32,
    c:f32,
    d:f32,

    p_min:f32,
    p_max:f32,
}

/// pub struct StraightLine
/// Straight lines defined by
/// | a |         | c |
/// |   | * t  +  |   |
/// | b |         | d |
/// with t ranging from p_min to p_max
impl StraightLine {
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

    pub fn default(a:f32,b:f32,c:f32,d:f32) -> StraightLine {
        let p_min = 0_f32;
        let p_max:f32 = 1.0;
        StraightLine{a,b,c,d,p_min,p_max}
    }

    pub fn new(a:f32,b:f32,c:f32,d:f32,p_min:f32,p_max:f32) -> StraightLine {
        StraightLine{a,b,c,d,p_min,p_max}
    }
}

impl Curves for StraightLine {
    fn get_pmin(&self) -> f32 {
        return self.p_min
    }
    fn get_pmax(&self) -> f32 {
        return self.p_max
    }
    fn xp(&self, t: f32) -> f32 {
        return self.a*t + self.c
    }
    fn yp(&self, t: f32) -> f32 {
        return self.b*t + self.d
    }
    fn dxp(&self, _t: f32) -> f32 {
        return self.a
    }
    fn dyp(&self, _t: f32) -> f32 {
        return self.b
    }
}

#[test]
fn test_endpoint_unit() {
    let line = StraightLine::unit(0);
    assert_eq!(line.xp(line.p_min), 0_f32);
    assert_eq!(line.xp(line.p_max), 1_f32);
    assert_eq!(line.yp(line.p_min), 0_f32);
    assert_eq!(line.yp(line.p_max), 0_f32);
}

#[test]
fn test_lengths_unit() {
    let line = StraightLine::unit(0);
    assert_eq!(line.integrate(line.p_max), 1_f32);
}
