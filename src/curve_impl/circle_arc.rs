use std::f32::consts;

use crate::objects::curves::Curves;
use crate::objects::point::Point;

/// pub struct CircleArc
/// Struct for general circle arcs with
/// a radius, center, and start and end angles (in radians)
pub struct CircleArc {
    r: f32, // Radius
    c: Point, // Center
    u: f32, // Start angle
    v: f32, // End angle
}

impl CircleArc {
    pub fn unit() -> CircleArc {
        CircleArc{r: 1_f32,c: Point::new(0_f32,0_f32),u: 0_f32,v: 2_f32*consts::PI}
    }
}

impl Curves for CircleArc {
    fn get_tmin(&self) -> f32 {
        return self.u
    }
    fn get_tmax(&self) -> f32 {
        return self.v
    }
    fn xt(&self, t: f32) -> f32 {
        return self.c.get_x() + self.r*(self.u+t).cos()
    }
    fn yt(&self, t: f32) -> f32 {
        return self.c.get_y() + self.r*(self.u+t).sin()
    }
    fn dxt(&self, t: f32) -> f32 {
        return -self.r*t.sin()
    }
    fn dyt(&self, t: f32) -> f32 {
        return self.r*t.cos()
    }
}

#[test]
fn test_circle_lengths() {
    let circle = CircleArc::unit();
    let half_circle_large = CircleArc{r:10_f32, c:Point::new(3_f32,-3_f32), u:0_f32, v:consts::PI};
    let delta = 1e-8;
    assert!((circle.integrate(circle.v) - 2_f32*consts::PI).abs() < delta);
    assert!((half_circle_large.integrate(half_circle_large.v) - 2_f32*consts::PI*10_f32) < delta);
}
