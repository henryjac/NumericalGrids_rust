pub struct Point {
    x:f32,
    y:f32,
}


impl Point {
    pub fn default() -> Point {
        Point{x:0.0,y:0.0}
    }

    pub fn new(x:f32,y:f32) -> Point {
        Point{x,y}
    }

    pub fn print(&self) {
        println!("({x},{y})",x=self.x,y=self.y);
    }

    pub fn get_x(&self) -> f32 {
        return self.x
    }

    pub fn get_y(&self) -> f32 {
        return self.y
    }
}

