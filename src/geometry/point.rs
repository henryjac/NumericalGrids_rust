pub enum Dimension {
    DimX, DimY, DimXY
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn default() -> Point {
        Point{x: 0_f32,y: 0_f32}
    }

    pub fn new(x:f32,y:f32) -> Point {
        Point{x, y}
    }

    pub fn print(&self) {
        println!("({},{})",self.x,self.y);
    }

    pub fn get_x(&self) -> f32 {
        return self.x
    }

    pub fn get_y(&self) -> f32 {
        return self.y
    }

    pub fn purge(self) {}

    // Following 3 methods should really be an implementation of Vector struct
    pub fn invert(&mut self) {
        // TODO: Implement error for if x or y is 0
        self.x = 1_f32 / self.x;
        self.y = 1_f32 / self.y;
    }

    pub fn flip(&mut self, dim: Dimension) {
        match dim {
            Dimension::DimX => self.y = -self.y,
            Dimension::DimY => self.x = -self.x,
            Dimension::DimXY => {
                self.x = -self.x;
                self.y = -self.y;
            }
        }
    }

    pub fn rotate(&mut self, angle: f32) {
        self.x = self.x*angle.cos()  + self.y*angle.sin();
        self.y = -self.x*angle.sin() + self.y*angle.cos();
    }

    /// Are the two points approximately equal?
    pub fn approx_equal(&self, other: &Point) -> bool {
        self.approx_equal_weps(other, 1e-6)
    }

    fn approx_equal_weps(&self, other: &Point, eps: f32) -> bool {
        let x_diff = (self.x - other.x).abs();
        let y_diff = (self.y - other.y).abs();
        if x_diff < eps && y_diff < eps {
            return true
        } else {
            return false
        }
    }
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, other: Point) -> Self::Output {
        Point::new(self.x+other.x, self.y+other.y)
    }
}

impl std::ops::Mul<f32> for Point {
    type Output = Self;
    fn mul(self, other: f32) -> Self::Output {
        Point::new(self.x*other, self.y*other)
    }
}

impl std::ops::Mul<Point> for f32 {
    type Output = Point;
    fn mul(self, other: Point) -> Self::Output {
        Point::new(self*other.x, self*other.y)
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[test]
fn test_default_point() {
    assert_eq!(Point::default().x, 0_f32);
    assert_eq!(Point::default().y, 0_f32);
}

#[test]
fn test_new_point() {
    let point = Point::new(1_f32,1_f32);
    assert_eq!(point.x, 1_f32);
    assert_eq!(point.y, 1_f32);
}

#[test]
fn test_get_funcs() {
    let point = Point{x: 1_f32, y: 0_f32};
    assert_eq!(point.get_x(), 1_f32);
    assert_eq!(point.get_y(), 0_f32);
}
