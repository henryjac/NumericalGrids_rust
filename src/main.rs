mod asymptotic_simpsons;
mod objects;
mod curve_impl;

use crate::curve_impl::straight_line::StraightLine;
use crate::objects::curves::Curves;
use crate::objects::point::Point;

fn main() {
    let l0 = StraightLine::unit(0);
    let p = Point::new(3_f32,1_f32);

    println!("l0: {}, p: {}",l0.xt(1_f32), p.get_x())
}
