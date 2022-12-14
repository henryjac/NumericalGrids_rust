use crate::numerical_methods::asymptotic_simpsons::asi;
use crate::numerical_methods::newton::newton;
use crate::geometry::point::Point;
use crate::duals;
use crate::duals::DualNumber;

use byteorder::WriteBytesExt;
use byteorder::LittleEndian;

use std::fs::File;
use std::io::Write;

/// General curves where a curve needs
/// an implementation of user parametrized x/y and 
/// dx/dy getters.
pub trait Curves<T> {
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
    /// The point in 2D space on the curve at `t`
    fn xy(&self, t: f32) -> Point<f32> {
        let s = self.find_s(t);
        return Point::from(
            self.xs(DualNumber::real(s)).get_a(),
            self.ys(DualNumber::real(s)).get_a(),
        )
    }
    /// Saves the curve to `location` with `precision` being how many
    /// gridpoints we have. The first byte is the precision and the remaining
    /// are pairs of (x,y) values
    fn save_curve(&self, location: &str, precision: u8) -> std::io::Result<()> {
        let mut file = File::create(location)?;
        file.write(&[precision])?;
        for j in 0..precision+1 {
            let s = (j as f32) / (precision as f32);
            let xy = self.xy(s);

            file.write_f32::<LittleEndian>(xy.get_x())?;
            file.write_f32::<LittleEndian>(xy.get_y())?;
        }
        Ok(())
    }
    /// Calculates the x-derivative of the curve at `s`, parametrized
    /// from `get_smin()` to `get_smax()`
    fn dxs(&self, s: f32) -> f32 {
        let xs = |s: DualNumber<f32>| -> DualNumber<f32> {
            self.xs(s)
        };
        duals::diff(&xs, s)
    }
    /// Calculates the y-derivative of the curve at `s`, parametrized
    /// from `get_smin()` to `get_smax()`
    fn dys(&self, s: f32) -> f32 {
        let ys = |s: DualNumber<f32>| -> DualNumber<f32> {
            self.ys(s)
        };
        duals::diff(&ys, s)
    }

    // Needs an implmentation for structs who want this trait
    /// Minimum value for curve parametrization
    fn get_smin(&self) -> f32;
    /// Maximum value for curve parametrization
    fn get_smax(&self) -> f32;
    /// Curve parametrization for x, `DualNumber` to be able
    /// to automatically take derivative
    fn xs(&self, s: DualNumber<f32>) -> DualNumber<f32>;
    /// Curve parametrization for y, `DualNumber` to be able
    /// to automatically take derivative
    fn ys(&self, s: DualNumber<f32>) -> DualNumber<f32>;
}
