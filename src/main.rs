mod point;
mod straight_line;

fn main() {
    let line = straight_line::StraightLine::default(1.0,2.0,3.0,4.0);
    let a = line.get_point(1_f32);
    a.print();
    let mut x = a.get_x();
    println!("{}",x);
    x = 3_f32;
    println!("{}",x);
}

// impl std::fmt::Display for Point {
//     fn Display(&self) -> char {
//         write!("({x},{y})",self.x,self.y);
//     }
// }
