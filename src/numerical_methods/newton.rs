/// Solves the equation `f=0` using Newton's method with initial guess `x0`.
/// Requires derivative of `f` `df`.
/// 
/// TODO: Needs a good initial guess, implement check for if we have convergence or
/// even a solution, unsure right now if this exists.
pub fn newton(f: &dyn Fn(f32) -> f32, df: &dyn Fn(f32) -> f32, x0: f32) -> f32 {
    return _newton(&f, &df, x0, 1e-7, 1000) 
}

// Maybe check for convergence as well in this method
fn _newton(f: &dyn Fn(f32) -> f32, df: &dyn Fn(f32) -> f32, mut x0: f32, tol: f32, max_it: i32) -> f32 {
    let mut x1: f32 = x0;
    let mut err = 1_f32;
    let mut it = 0;
    while tol < err && it < max_it {
        x1 = x0 - f(x0) / df(x0);
        err = (x1 - x0).abs();
        x0 = x1;
        it += 1;
    }
    if it == max_it {
        println!("No convergence in newton's method");
    }
    return x1;
}

#[test]
fn test_newton() {
    let delta = 1e-8;

    let dottie = 0.73908514;
    let f1 = |x: f32| -> f32 {x - x.cos()};
    let df1 = |x: f32| -> f32 {1_f32 + x.sin()};
    let f2  = |x: f32| -> f32 {x*x - 16_f32};
    let df2 = |x: f32| -> f32 {2_f32*x};
    assert!((newton(&f1,&df1,0.5_f32) - dottie).abs() < delta);
    assert!((newton(&f2,&df2,1_f32) - 4_f32).abs() < delta);
}
