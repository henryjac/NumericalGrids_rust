use num::Zero;

use std::ops::{Add, Sub, Mul, Neg};
use std::fmt::Display;

pub enum Dimension {
    DimX, DimY, DimXY
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> 
    where T: Copy
{
    pub fn new() -> Point<T> where T: Zero {
        Point{x: T::zero(), y: T::zero()}
    }

    pub fn from(x: T, y: T) -> Point<T> {
        Point{x, y}
    }

    pub fn print(&self) where T: Display {
        println!("({},{})",self.x,self.y);
    }

    pub fn get_x(&self) -> T {
        return self.x
    }

    pub fn get_y(&self) -> T {
        return self.y
    }

    pub fn purge(self) {}
}

impl Point<f32> {
    pub fn rotate(&mut self, angle: f32) {
        self.x = self.x*angle.cos()  + self.y*angle.sin();
        self.y = -self.x*angle.sin() + self.y*angle.cos();
    }
    /// Are the two points approximately equal?
    pub fn equal(&self, other: &Point<f32>) -> bool {
        self.approx_equal_weps(other, 1e-5)
    }

    /// Are the two points approximately eps equal
    fn approx_equal_weps(&self, other: &Point<f32>, ε: f32) -> bool
    {
        let x_diff = (self.x - other.x).abs();
        let y_diff = (self.y - other.y).abs();
        return x_diff < ε && y_diff < ε
    }
}

impl Point<f64> {
    pub fn rotate(&mut self, angle: f64) {
        self.x = self.x*angle.cos()  + self.y*angle.sin();
        self.y = -self.x*angle.sin() + self.y*angle.cos();
    }
    /// Are the two points approximately equal?
    pub fn equal(&self, other: &Point<f64>) -> bool {
        self.approx_equal_weps(other, 1e-5)
    }

    /// Are the two points approximately eps equal
    fn approx_equal_weps(&self, other: &Point<f64>, ε: f64) -> bool
    {
        let x_diff = (self.x - other.x).abs();
        let y_diff = (self.y - other.y).abs();
        return x_diff < ε && y_diff < ε
    }
}

impl Point<i32> {
    pub fn equal(&self, other: &Point<i32>) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point<i64> {
    pub fn equal(&self, other: &Point<i64>) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T> Add for Point<T> 
    where T: Add<Output=T> + Copy
{
    type Output = Point<T>;
    fn add(self, other: Point<T>) -> Self::Output {
        Point::from(self.x + other.x, self.y + other.y)
    }
}

impl<T> Sub for Point<T> 
    where T: Sub<Output=T> + Copy
{
    type Output = Point<T>;
    fn sub(self, other: Point<T>) -> Self::Output {
        Point::from(self.x - other.x, self.y - other.y)
    }
}

impl<T> Neg for Point<T>
    where T: Neg<Output=T> + Copy
{
    type Output = Point<T>;
    fn neg(self) -> Self::Output {
        Point::from(-self.x, -self.y)
    }
}

impl<T> Mul<T> for Point<T> 
    where T: Mul<Output=T> + Copy
{
    type Output = Point<T>;
    fn mul(self, other: T) -> Self::Output {
        Point::from(self.x*other, self.y*other)
    }
}

impl<T> Display for Point<T> 
    where T: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[test]
fn test_new_point() {
    assert_eq!(Point::<f32>::new().x, 0_f32);
    assert_eq!(Point::<f32>::new().y, 0_f32);
}

#[test]
fn test_from_point() {
    let point = Point::from(1_f32,1_f32);
    assert_eq!(point.x, 1_f32);
    assert_eq!(point.y, 1_f32);
}

#[test]
fn test_get_funcs() {
    let point = Point{x: 1_f32, y: 0_f32};
    assert_eq!(point.get_x(), 1_f32);
    assert_eq!(point.get_y(), 0_f32);
}
