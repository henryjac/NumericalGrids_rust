use crate::asymptotic_simpsons::asi;
use crate::objects::point::Point;

/// trait Curves
/// Trait for general curves where a curve needs
/// an implementation of user parametrized x/y and 
/// dx/dy getters.
pub trait Curves {
    fn find_t(&self, s: f32) -> f32 {
        let f = |p: f32| -> f32 {
            return self.integrate(p) - s*self.integrate(self.get_tmax()) 
        };
        let df = |t: f32| -> f32 {
            return self.integrand(t)
        };
        return newton(&f, &df, 0_f32)
    }
    fn integrand(&self, t: f32) -> f32 {
        return (f32::powf(self.dxt(t),2_f32) + f32::powf(self.dyt(t),2_f32)).sqrt()
    }
    fn integrate(&self, t: f32) -> f32 {
        let f = |q: f32| -> f32 {
            self.integrand(q)
        };
        return asi(&f, self.get_tmin(), t)
    }
    fn xy(&self, s: f32) -> Point {
        let t = self.find_t(s);
        return Point::new(self.xt(t),self.yt(t))
    }

    // Needs an implmentation for structs who want this trait
    fn get_tmin(&self) -> f32;
    fn get_tmax(&self) -> f32;
    fn xt(&self, t: f32) -> f32;
    fn yt(&self, t: f32) -> f32;
    fn dxt(&self, t: f32) -> f32; // Use dual numbers to implement this, make xt and yt take in a template
    fn dyt(&self, t: f32) -> f32;
}

fn newton(f: &dyn Fn(f32) -> f32, df: &dyn Fn(f32) -> f32, x0: f32) -> f32 {
    return _newton(&f, &df, x0, 1e-10) 
}

// Maybe check for convergence as well in this method
fn _newton(f: &dyn Fn(f32) -> f32, df: &dyn Fn(f32) -> f32, mut x0: f32, tol: f32) -> f32 {
    let mut x1: f32 = x0;
    let mut err = 1_f32;
    while tol < err {
        x1 = x0 - f(x0) / df(x0);
        err = (x1 - x0).abs();
        x0 = x1;
    }
    return x1;
}

#[test]
fn test_newton() {
    let dottie = 0.73908514;
    let delta = 1e-8;
    let f = |x: f32| -> f32 {x - x.cos()};
    let df = |x: f32| -> f32 {1_f32 + x.sin()};
    assert!((newton(&f,&df,0.5_f32) - dottie).abs() < delta);
}
