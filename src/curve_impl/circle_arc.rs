use crate::geometry::curves::Curves;
use crate::geometry::point::Point;
use crate::duals::DualNumber;

use std::f32::consts;

/// General circle arcs with
/// a radius, center, and start and end angles (in radians)
pub struct CircleArc<T> {
    r: T, // Radius
    c: Point<T>, // Center
    u: T, // Start angle
    v: T, // End angle
}

impl<T> CircleArc<T> {
    /// Circle arc defined with radius `r`, center `c` and start-end angles
    /// `u` and `v`.
    pub fn from(r: T, c: Point<T>, u: T, v: T) -> CircleArc<T> {
        CircleArc{r, c, u, v}
    }

    /// The unit circle
    pub fn unit() -> CircleArc<f32> {
        CircleArc{ r: 1_f32, c: Point::from(0.0,0.0), u: 0_f32, v: 2_f32*consts::PI }
    }

    /// The unit circle but scaled with radius `r`
    pub fn scaled_unit(r: f32) -> CircleArc<f32> {
        CircleArc::from(r, Point::from(0.0,0.0), 0_f32, 2_f32*consts::PI)
    }

    /// A quadrant from the unit circle where 0 is the 
    /// first quadrant
    pub fn unit_quadrant(quadrant: u8) -> CircleArc<f32> {
        let r = 1_f32;
        let c = Point::from(0.0,0.0);
        let u = (quadrant as f32) * std::f32::consts::PI / 2_f32;
        let v = ((quadrant + 1) as f32) * std::f32::consts::PI / 2_f32;
        CircleArc::<f32>::from(r, c, u, v)
    }

    /// Returns the center point of the circle arc
    pub fn center(&self) -> Point<T> where Point<T>: Copy {
        return self.c
    }
}

impl Curves<f32> for CircleArc<f32> {
    fn get_smin(&self) -> f32 {
        return self.u
    }
    fn get_smax(&self) -> f32 {
        return self.v
    }
    fn xs(&self, s: DualNumber<f32>) -> DualNumber<f32> {
        return  s.cos() * self.r + self.c.get_x()
    }
    fn ys(&self, s: DualNumber<f32>) -> DualNumber<f32> {
        return s.sin() * self.r + self.c.get_y()
    }
}

#[test]
fn test_circle_lengths() {
    let circle = CircleArc::<f32>::unit();
    let half_circle_large = CircleArc::from(10_f32, Point::from(3_f32,-3_f32), 0_f32, consts::PI);
    let delta = 1e-8;
    assert!((circle.integrate(circle.v) - 2_f32*consts::PI).abs() < delta);
    assert!((half_circle_large.integrate(half_circle_large.v) - 2_f32*consts::PI*10_f32) < delta);
}

#[test]
fn test_unit_quadrants() {
    let arcs = [CircleArc::<f32>::unit_quadrant(0), CircleArc::<f32>::unit_quadrant(1), CircleArc::<f32>::unit_quadrant(2), CircleArc::<f32>::unit_quadrant(3)];

    assert!(arcs[0].u == 0.0);
    assert!(arcs[0].v == std::f32::consts::PI / 2_f32);
    assert!(arcs[1].u == arcs[0].v);
    assert!(arcs[1].v == std::f32::consts::PI);
    assert!(arcs[2].u == arcs[1].v);
    assert!(arcs[2].v == 3_f32 * std::f32::consts::PI / 2_f32);
    assert!(arcs[3].u == arcs[2].v);
    assert!(arcs[3].v == 2_f32 * std::f32::consts::PI);

    assert_eq!(arcs[0].c.get_x(), 0_f32);
    assert_eq!(arcs[0].c.get_y(), 0_f32);

    let endpoints = [
        1_f32, 0_f32, 0_f32, 1_f32,
        0_f32, 1_f32, -1_f32, 0_f32,
        -1_f32, 0_f32, 0_f32, -1_f32,
        0_f32, -1_f32, 1_f32, 0_f32
    ] ;
    let delta = 1e-6;
    for (i,arc) in arcs.iter().enumerate() {
        assert!( (arc.xs(DualNumber::real(arc.get_smin())) - DualNumber::real(endpoints[i*4+0])).get_a().abs() < delta );
        assert!( (arc.ys(DualNumber::real(arc.get_smin())) - DualNumber::real(endpoints[i*4+1])).get_a().abs() < delta );
        assert!( (arc.xs(DualNumber::real(arc.get_smax())) - DualNumber::real(endpoints[i*4+2])).get_a().abs() < delta );
        assert!( (arc.ys(DualNumber::real(arc.get_smax())) - DualNumber::real(endpoints[i*4+3])).get_a().abs() < delta );
    }
}

#[test]
fn test_uncenterd_circles() {
    let c1 = CircleArc::from(2_f32, Point::from(1_f32, -1_f32), 0_f32, 2_f32 * consts::PI);

    assert!(c1.xy(0_f32).equal(&Point::from(3_f32, -1_f32)));
    assert!(c1.xy(0.5).equal(&Point::from(-1_f32, -1_f32)));
}
