extern crate num;

use std::ops::{Add, Sub, Mul, Div, Neg};
use std::fmt::Display;

use num::{One, Zero};

#[derive(Debug,Copy,Clone)]
pub struct DualNumber<T> {
    a: T,
    b: T,
}

impl<T> DualNumber<T> where T: Copy {
    pub fn from(a: T, b: T) -> DualNumber<T> {
        DualNumber{a, b}
    }
    pub fn real(a: T) -> DualNumber<T> where T: Zero {
        DualNumber {
            a, 
            b: T::zero(),
        }
    }
    pub fn get_a(&self) -> T {
        self.a
    }
    pub fn get_b(&self) -> T {
        self.b
    }
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
