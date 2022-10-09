pub struct Point {
    x:f32,
    y:f32,
}

impl Point {
    pub fn default() -> Point {
        Point{x:0_f32,y:0_f32}
    }

    pub fn new(x:f32,y:f32) -> Point {
        Point{x,y}
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
