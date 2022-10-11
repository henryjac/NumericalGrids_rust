use crate::numerical_methods::asymptotic_simpsons::asi;
use crate::numerical_methods::newton::newton;
use crate::geometry::point::Point;

/// General curves where a curve needs
/// an implementation of user parametrized x/y and 
/// dx/dy getters.
pub trait Curves {
    /// Returns the `t` that corresponds to `s` where
    /// x(t) = X(s), where x(t) is the curve parametrized from
    /// 0->1 and X(s) is the curve in user coordinates.
    fn find_s(&self, s: f32) -> f32 {
        let f = |p: f32| -> f32 {
            return self.integrate(p) - s*self.integrate(self.get_smax()) 
        };
        let df = |p: f32| -> f32 {
            return self.integrand(p)
        };
        return newton(&f, &df, 0_f32)
    }
    /// Returns the value of the integrand for the curve length at `s`.
    fn integrand(&self, s: f32) -> f32 {
        return (f32::powf(self.dxs(s),2_f32) + f32::powf(self.dys(s),2_f32)).sqrt()
    }
    /// Returns the length of the curve from `get_smin()` to `s`.
    fn integrate(&self, s: f32) -> f32 {
        let f = |p: f32| -> f32 {
            self.integrand(p)
        };
        return asi(&f, self.get_smin(), s)
    }
    fn xy(&self, t: f32) -> Point {
        let s = self.find_s(t);
        return Point::new(self.xs(s),self.ys(s))
    }

    // Needs an implmentation for structs who want this trait
    fn get_smin(&self) -> f32;
    fn get_smax(&self) -> f32;
    fn xs(&self, s: f32) -> f32;
    fn ys(&self, s: f32) -> f32;
    fn dxs(&self, s: f32) -> f32; // Use dual numbers to implement this, make xs and ys take in a template
    fn dys(&self, s: f32) -> f32;
}