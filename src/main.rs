// Modules
mod asymptotic_simpsons;
mod point;
mod curves;
mod straight_line;
mod circle_arc;

// Traits
use crate::curves::Curves;

fn main() {
    let line = straight_line::StraightLine::default(1.0,2.0,3.0,4.0);
    let circle = circle_arc::CircleArc::unit();
    let xy = line.xy(0_f32);
    print!("Line at 0: ");
    xy.print();
    let xyc = circle.xy(0_f32);
    print!("Circle at 0: ");
    xyc.print();
}
