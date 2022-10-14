use std::f32::consts;

use crate::geometry::curves::Curves;
use crate::geometry::point::Point;
use crate::duals::DualNumber;

/// General circle arcs with
/// a radius, center, and start and end angles (in radians)
pub struct CircleArc {
    r: f32, // Radius
    c: Point, // Center
    u: f32, // Start angle
    v: f32, // End angle
}

impl CircleArc {
    /// The unit circle
    pub fn unit() -> CircleArc {
        CircleArc{ r: 1_f32, c: Point::from(0_f32, 0_f32), u: 0_f32, v: 2_f32*consts::PI }
    }

    /// The unit circle but scaled with radius `r`
    pub fn scaled_unit(r: f32) -> CircleArc {
        CircleArc{ r, c: Point::from(0_f32, 0_f32), u: 0_f32, v: 2_f32*consts::PI }
    }

    /// A quadrant from the unit circle where 0 is the 
    /// first quadrant
    pub fn unit_quadrant(quadrant: u8) -> CircleArc {
        let r = 1_f32;
        let c = Point::from(0_f32, 0_f32);
        let u = (quadrant as f32) * std::f32::consts::PI / 2_f32;
        let v = ((quadrant + 1) as f32) * std::f32::consts::PI / 2_f32;
        CircleArc{ r, c, u, v }
    }

    /// Circle arc defined with radius `r`, center `c` and start-end angles
    /// `u` and `v`.
    pub fn new(r: f32, c: Point, u: f32, v: f32) -> CircleArc {
        CircleArc{r, c, u, v}
    }

    /// Returns the center point of the circle arc
    pub fn center(&self) -> Point {
        return self.c
    }
}

impl Curves for CircleArc {
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
    let circle = CircleArc::unit();
    let half_circle_large = CircleArc{r:10_f32, c:Point::from(3_f32,-3_f32), u:0_f32, v:consts::PI};
    let delta = 1e-8;
    assert!((circle.integrate(circle.v) - 2_f32*consts::PI).abs() < delta);
    assert!((half_circle_large.integrate(half_circle_large.v) - 2_f32*consts::PI*10_f32) < delta);
}

#[test]
fn test_unit_quadrants() {
    let arcs = [CircleArc::unit_quadrant(0), CircleArc::unit_quadrant(1), CircleArc::unit_quadrant(2), CircleArc::unit_quadrant(3)];

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
        assert!( (arc.xs(arc.get_smin()) - endpoints[i*4+0]).abs() < delta );
        assert!( (arc.ys(arc.get_smin()) - endpoints[i*4+1]).abs() < delta );
        assert!( (arc.xs(arc.get_smax()) - endpoints[i*4+2]).abs() < delta );
        assert!( (arc.ys(arc.get_smax()) - endpoints[i*4+3]).abs() < delta );
    }
}

#[test]
fn test_uncenterd_circles() {
    let c1 = CircleArc::new(2_f32, Point::from(1_f32, -1_f32), 0_f32, 2_f32 * consts::PI);

    assert!(c1.xy(0_f32).approx_equal(&Point::from(3_f32, -1_f32)));
    assert!(c1.xy(0.5).approx_equal(&Point::from(-1_f32, -1_f32)));
}
