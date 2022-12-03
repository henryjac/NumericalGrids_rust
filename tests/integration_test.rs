use numerical_grids::geometry::domain::Domain;
use numerical_grids::geometry::curves::Curves;
use numerical_grids::curve_impl::special_curve::SpecialCurve;
use numerical_grids::curve_impl::straight_line::StraightLine;

#[test]
fn special_grid() {
    let special_now = std::time::Instant::now();

    let b1 = Box::new(SpecialCurve::new()) as Box<dyn Curves<f32>>;
    let b2 = Box::new(StraightLine::<f32>::from(0_f32,1_f32,5_f32,0_f32,0_f32,3_f32)) as Box<dyn Curves<f32>>;
    let b3 = Box::new(StraightLine::<f32>::from(1_f32,0_f32,0_f32,3_f32,-10_f32,5_f32)) as Box<dyn Curves<f32>>;
    let b4 = Box::new(StraightLine::<f32>::from(0_f32,1_f32,-10_f32,0_f32,0_f32,3_f32)) as Box<dyn Curves<f32>>;

    let boundary = [b1,b2,b3,b4];
    let _ = Domain::new(boundary, 6, 9);

    let special_elapsed = special_now.elapsed();
    println!("Special grid calculations took {}ms", special_elapsed.as_millis());
}
