fn simpson_quad(f: &dyn Fn(f32) -> f32,a: f32,b: f32) -> f32 {
    return (b-a)/6.0 * (f(a) + 4.0*f((a+b)/2.0) + f(b))
}

// Find a way to input function here
fn asi_wtol(f: &dyn Fn(f32) -> f32, a: f32, b: f32, tol: f32) -> f32 {
    let c = (a+b)/2.0;
    let i1 = simpson_quad(f,a,b);
    let i2 = simpson_quad(f,a,c) + simpson_quad(f,c,b);
    let err = (i1 - i2).abs();
    match err < tol {
        true => return i2,
        false => return asi_wtol(f,a,(a+b)/2.0,tol/2.0) + asi_wtol(f,(a+b)/2.0,b,tol)
    }
}

pub fn asi(f: &dyn Fn(f32) -> f32, a: f32, b: f32) -> f32 {
    return asi_wtol(f,a,b,1e-8)
}

#[test]
fn test_asi_polynomials() {
    let x = |t: f32| -> f32 {return t};
    let x2 = |t: f32| -> f32 {return f32::powf(t,2_f32)};
    let delta: f32 = 1e-8; // Not very good approximation
    assert!((asi(&x,0_f32,1_f32) - 0.5).abs() < delta);
    assert!((asi(&x2,-1_f32,1_f32) - 2_f32/3_f32).abs() < delta);
}
