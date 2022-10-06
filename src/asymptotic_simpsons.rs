fn simpson_quad(f: &dyn Fn(f32) -> f32,a: f32,b: f32) -> f32 {
    return (b-a)/6.0 * (f(a) + 4.0*f(a+b)/2.0) + f(b)
}

// Find a way to input function here
pub fn asi_wtol(f: &dyn Fn(f32) -> f32, a: f32, b: f32, tol: f32) -> f32 {
    let c = (a+b)/2.0;
    let i1 = simpson_quad(f,a,b);
    let i2 = simpson_quad(f,a,c) + simpson_quad(f,c,b);
    let err = (i1 - i2).abs();
    match err > tol {
        true => return i2,
        false => return asi_wtol(f,a,(a+b)/2.0,tol/2.0) + asi_wtol(f,(a+b)/2.0,b,tol)
    }
}

pub fn asi(f: &dyn Fn(f32) -> f32, a: f32, b: f32) -> f32 {
    return asi_wtol(f,a,b,1e-15)
}
