extern crate num;

use std::ops::{Add, Sub, Mul, Div, Neg};
use std::fmt::Display;

use num::{One, Zero};

/// Dual numbers are an extension similar to the complex numbers,
/// where we have ε≠0 and ε²=0.
/// 
/// This gives rise to the ability to perform automatic differentiation
/// of analytical functions using the taylor expansion where terms
/// of order >= 2 will be 0 due to the nature of dual numbers.
#[derive(Debug,Copy,Clone)]
pub struct DualNumber<T> {
    a: T,
    b: T,
}

pub trait GetA {
    fn get_a(&self) -> f32;
}

impl GetA for f32 {
    fn get_a(&self) -> f32 {
        *self
    }
}

impl GetA for f64 {
    fn get_a(&self) -> f32 {
        *self as f32
    }
}

impl GetA for DualNumber<f32> {
    fn get_a(&self) -> f32 {
        self.a
    }
}

impl GetA for DualNumber<f64> {
    fn get_a(&self) -> f32 {
        self.a as f32
    }
}

impl<T> DualNumber<T> where T: Copy {
    /// Creates a dual number from generic `a` and `b`
    pub fn from(a: T, b: T) -> DualNumber<T> {
        DualNumber{a, b}
    }
    /// Creates a dual number from real value `a`
    pub fn real(a: T) -> DualNumber<T> where T: Zero {
        DualNumber {
            a, 
            b: T::zero(),
        }
    }
    /// Returns the real value of the dual number
    pub fn get_a(&self) -> T {
        self.a
    }
    /// Returns the dual value of the dual number
    pub fn get_b(&self) -> T {
        self.b
    }
    /// Inverts the dual number (as in doing 1/w)
    pub fn inv(&self) -> DualNumber<T> 
        where T: Div<Output=T> + Neg<Output=T> + Mul<Output=T> + One,
    {
        DualNumber {
            a: T::one() / self.a,
            b: -self.b / (self.a * self.a),
        }
    }
}

impl DualNumber<f32> {
    pub fn sin(&self) -> DualNumber<f32> {
        DualNumber {
            a: self.a.sin(),
            b: self.b * self.a.cos(),
        }
    }
    pub fn cos(&self) -> DualNumber<f32> {
        DualNumber {
            a: self.a.cos(),
            b: -self.b * self.a.sin(),
        }
    }
    pub fn exp(&self) -> DualNumber<f32> {
        DualNumber {
            a: self.a.exp(),
            b: self.b * self.a.exp(),
        }
    }
    pub fn pow(&self, a: u8) -> DualNumber<f32> {
        match a == 0 {
            true => return DualNumber::from(0.0, 0.0),
            false => (),
        }
        let mut dual = DualNumber::from(self.a, self.b);
        let factor = DualNumber::from(self.a, self.b);
        for _ in 0..a-1 {
            dual = dual * factor;
        }
        dual                                            
    }
}
impl DualNumber<f64> {
    pub fn sin(&self) -> DualNumber<f64> {
        DualNumber {
            a: self.a.sin(),
            b: self.b * self.a.cos(),
        }
    }
    pub fn cos(&self) -> DualNumber<f64> {
        DualNumber {
            a: self.a.cos(),
            b: -self.b * self.a.sin(),
        }
    }
    pub fn exp(&self) -> DualNumber<f64> {
        DualNumber {
            a: self.a.exp(),
            b: self.b * self.a.exp(),
        }
    }
    pub fn pow(&self, a: u8) -> DualNumber<f64> {
        match a == 0 {
            true => return DualNumber::from(0_f64, 0_f64),
            false => (),
        }
        let mut dual = DualNumber::from(self.a, self.b);
        let factor = DualNumber::from(self.a, self.b);
        for _ in 0..a-1 {
            dual = dual * factor;
        }
        dual                                            
    }
}

impl<T> Display for DualNumber<T> 
    where T: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}+{}ε", self.a, self.b)
    }
}

impl<T> Add for DualNumber<T> 
    where T: Add<Output=T>
{
    type Output = DualNumber<T>;
    fn add(self, other: DualNumber<T>) -> Self::Output { 
        DualNumber {
            a: self.a + other.a,
            b: self.b + other.b,
        }
    }
}

impl<T: Add<Output=T>> Add<T> for DualNumber<T> {
    type Output = Self;
    fn add(self, other: T) -> Self::Output {
        DualNumber {
            a: self.a + other,
            b: self.b,
        }
    }
}

impl<T> Sub<DualNumber<T>> for DualNumber<T> 
    where T: Sub<Output=T>
{
    type Output = DualNumber<T>;
    fn sub(self, other: DualNumber<T>) -> Self::Output {
        DualNumber {
            a: self.a - other.a,
            b: self.b - other.b,
        }
    }
}

impl<T> Sub<T> for DualNumber<T>
    where T: Sub<Output=T>
{
    type Output = DualNumber<T>;
    fn sub(self, other: T) -> Self::Output {
        DualNumber {
            a: self.a - other,
            b: self.b
        }
    }
}

impl<T> Mul<DualNumber<T>> for DualNumber<T>
    where T: Copy + Mul<Output=T> + Add<Output=T>
{
    type Output = DualNumber<T>;
    fn mul(self, other: DualNumber<T>) -> Self::Output {
        DualNumber {
            a: self.a * other.a,
            b: self.a * other.b + self.b * other.a,
        }
    }
}

impl<T> Mul<T> for DualNumber<T> 
    where T: Mul<Output=T> + Copy
{
    type Output = DualNumber<T>;
    fn mul(self, other: T) -> DualNumber<T> {
        DualNumber{
            a: self.a * other,
            b: self.b * other,
        }
    }
}

impl<T> Div<DualNumber<T>> for DualNumber<T>
    where T: Copy + Add<Output=T> + Div<Output=T> + Mul<Output=T> + Neg<Output=T>
{
    type Output = DualNumber<T>;
    fn div(self, other: DualNumber<T>) -> Self::Output {
        DualNumber {
            a: self.a / other.a,
            b: -self.a * other.b / (self.a*self.a) + self.b / other.a,
        }
    }
}

impl<T> Div<T> for DualNumber<T> 
    where T: Div<Output=T> + Copy
{
    type Output = DualNumber<T>;
    fn div(self, other: T) -> DualNumber<T> {
        DualNumber{
            a: self.a / other,
            b: self.b / other,
        }
    }
}

impl<'a, 'b, T> Add<&'b DualNumber<T>> for &'a DualNumber<T>
    where &'a T: Add<&'b T, Output=T>
{
    type Output = DualNumber<T>;
    fn add(self, other: &'b DualNumber<T>) -> Self::Output {
        DualNumber {
            a: &self.a + &other.a, 
            b: &self.b + &other.b,
        }
    }
}

impl<'a, 'b, T> Sub<&'b DualNumber<T>> for &'a DualNumber<T>
    where &'a T: Sub<&'b T, Output=T>
{
    type Output = DualNumber<T>;
    fn sub(self, other: &'b DualNumber<T>) -> Self::Output {
        DualNumber {
            a: &self.a - &other.a,
            b: &self.b - &other.b,
        }
    }
}

impl<'a, 'b, T> Mul<&'b DualNumber<T>> for &'a DualNumber<T>
    where 
        &'a T: Mul<&'b T, Output=T>, 
        T: Add<Output=T>
{
    type Output = DualNumber<T>;
    fn mul(self, other: &'b DualNumber<T>) -> Self::Output {
        DualNumber {
            a: &self.a * &other.a,
            b: &self.a * &other.b + &self.b * &other.a,
        }
    }
}

impl<'a, T> Div<&'a DualNumber<T>> for &'a DualNumber<T>
    where
        &'a T: Div<&'a T, Output=T> + Mul<&'a T, Output=T>,
        T: Add<Output=T> + Div<Output=T> + Neg<Output=T>
{
    type Output = DualNumber<T>;
    fn div(self, other: &'a DualNumber<T>) -> Self::Output {
        DualNumber {
            a: &self.a / &other.a,
            b: -(&self.a * &other.b) / (&self.a * &self.a) + &self.b / &other.a,
        }
    }
}

pub fn diff<'a, T>(f: &'a dyn Fn(DualNumber<T>) -> DualNumber<T>, x: T) -> T 
    where T: One + Copy
{
    f(DualNumber::from(x,T::one())).b
}

#[test]
fn test_differentiation_powers() {
    let f1 = |x: DualNumber<f32>| -> DualNumber<f32> {
        x.pow(2)
    };
    let f2 = |x: DualNumber<f32>| -> DualNumber<f32> {
        x.pow(5) + x.pow(2)
    };
    assert_eq!(diff(&f1, 5.0), 10.0);
    assert_eq!(diff(&f2, 2.0), 84.0);
}
