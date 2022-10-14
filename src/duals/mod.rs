use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;
use std::fmt::Display;

#[derive(Debug)]
pub struct DualNumber<T> {
    a: T,
    b: T,
}

impl<T> DualNumber<T> {
    pub fn from(a: T, b: T) -> DualNumber<T> {
        DualNumber{a, b}
    }
}

impl<T: Display> Display for DualNumber<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}+{}ε", self.a, self.b)
    }
}

impl<T> Add for DualNumber<T> 
    where T: Add<Output=T>
{
    type Output = Self;
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

impl<T: Sub<Output=T>> Sub for DualNumber<T> {
    type Output = Self;
    fn sub(self, other: DualNumber<T>) -> Self::Output {
        DualNumber {
            a: self.a - other.a,
            b: self.b - other.b,
        }
    }
}

impl<T: Mul<Output=T> + std::ops::Add<Output=T>> Mul<DualNumber<T>> for DualNumber<T>
    where T: Copy
{
    type Output = DualNumber<T>;
    fn mul(self, other: DualNumber<T>) -> Self::Output {
        DualNumber {
            a: self.a * other.a,
            b: self.a * other.b + self.b * other.a,
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

// impl<'a, 'b, T: Add<Output=T>> Add<T> for &'a DualNumber<T>
//     where &'a T: Add<T, Output=T>
// {
//     type Output = DualNumber<T>;
//     fn add(self, other: T) -> Self::Output {
//         DualNumber{
//             a: self.a + other,
//             b: self.b
//         }
//     }
// }
