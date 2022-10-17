use crate::geometry::domain::Domain;

use byteorder::WriteBytesExt;
use byteorder::LittleEndian;

use nalgebra::DMatrix;

use std::ops::Add;
use std::ops::Mul;

use std::fs::File;
use std::io::Write;

/// Functions defined on specific domain with values on the 
/// discrete points on the domain.
pub struct GridFunction<'a, T> {
    domain: &'a Domain<T>,
    values: DMatrix<f32>,
    n: u8,
    m: u8,
    h_ξ: f32,
    h_η: f32,
}

impl<'a> GridFunction<'a, f32> {
    /// Creates the 0-function on `domain`
    pub fn new(domain: &'a Domain<f32>) -> GridFunction<f32> {
        let n = domain.get_n();
        let m = domain.get_m();
        let h_ξ = 1_f32 / (n as f32);
        let h_η = 1_f32 / (n as f32);
        let values = DMatrix::from_fn(n.into(),m.into(),|_,_| 0.0);
        GridFunction{domain, values, n, m, h_ξ, h_η}
    }

    /// Creates a function from `fnc` on `domain`
    pub fn from_fnc(domain: &'a Domain<f32>, fnc: &'a dyn Fn(f32,f32) -> f32,) -> GridFunction<f32> {
        let index_fnc = |j: usize, i: usize| -> f32 {
            let xy = domain.get_xy(i,j);
            fnc(xy.get_x(), xy.get_y())
        };
        let n = domain.get_n();
        let m = domain.get_m();
        let h_ξ = 1_f32 / (n as f32);
        let h_η = 1_f32 / (n as f32);
        let values = DMatrix::from_fn(m.into(), n.into(), index_fnc);
        GridFunction{domain, values, n, m, h_ξ, h_η}
    }

    /// Creates a function from `domain` and matrix `values`. 
    ///
    /// Sizes needs to be compatible
    pub fn from(domain: &'a Domain<f32>, values: DMatrix<f32>,) -> GridFunction<f32> {
        let n = domain.get_n();
        let m = domain.get_m();
        let h_ξ = 1_f32 / (n as f32);
        let h_η = 1_f32 / (n as f32);
        GridFunction{domain, values, n, m, h_ξ, h_η}
    }

    pub fn generate_function_values(&mut self, fnc: &dyn Fn(f32,f32) -> f32) {
        let index_fnc = |j: usize, i: usize| -> f32 {
            let xy = self.domain.get_xy(i,j);
            fnc(xy.get_x(), xy.get_y())
        };
        self.values = DMatrix::from_fn(self.m.into(), self.n.into(), index_fnc);
    }

    pub fn print(&self) {
        println!("{}",self.values);
    }

    pub fn save_function(&self, location: &str) -> std::io::Result<()> {
        let mut file = File::create(location)?;
        file.write(&[self.n])?;
        file.write(&[self.m])?;
        for i in 0..self.n {
            for j in 0..self.m {
                file.write_f32::<LittleEndian>(
                    self.values[(i as u16 * self.m as u16 + j as u16) as usize]
                )?;
            }
        };
        Ok(())
    }

    fn partial_derivative(
        &self, f: &dyn Fn(usize, usize) -> f32, dir: DiffDirection, i: usize, j: usize) -> f32 
    {
        let last_index: usize;
        let current_index: usize;
        let is: [usize; 4];
        let js: [usize; 4];
        let h: f32;
        match dir {
            DiffDirection::Xi => {
                current_index = i;
                last_index = (self.n-1) as usize;
                h = self.h_ξ;
                is = set_indices(i, true);
                js = set_indices(j, false);
            },
            DiffDirection::Eta => {
                current_index = j;
                last_index = (self.m-1) as usize;
                h = self.h_η;
                is = set_indices(i, false);
                js = set_indices(j, true);
            }
        }
        match current_index {
            0 => {
                one_sided_diff(-1, f(i,j), f(is[0],js[0]), f(is[2],js[2]), h)
            },
            last_index => {
                one_sided_diff(1, f(i,j), f(is[1],js[1]), f(is[3],js[3]), h)
            },
            _ => {
                central_diff(f(is[0],js[0]), f(is[1],js[1]), h)
            }
        }
    }
}

impl<'a> Add for GridFunction<'a, f32> {
    type Output = GridFunction<'a, f32>;
    fn add(self, other: GridFunction<'a, f32>) -> Self::Output {
        // let values = match &self.domain as *const _ == &other.domain as *const _ {
        let values = match std::ptr::eq(self.domain, other.domain) {
            true => self.values + other.values,
            false => panic!("Can't add functions defined on different domains."),
        };
        GridFunction::from(self.domain, values)
    }
}

impl<'a> Mul for GridFunction<'a, f32> {
    type Output = GridFunction<'a, f32>;
    fn mul(self, other: GridFunction<'a, f32>) -> Self::Output {
        let values = match std::ptr::eq(self.domain, other.domain) {
            true => self.values.component_mul(&other.values),
            false => panic!("Can't multiply functions defined on different domains."),
        };
        GridFunction::from(self.domain, values)
    }
}

impl<'a, 'b> Add<&'b GridFunction<'b, f32>> for &'a GridFunction<'a, f32> {
    type Output = GridFunction<'a, f32>;
    fn add(self, other: &'b GridFunction<f32>) -> Self::Output {
        let values = match std::ptr::eq(self.domain, other.domain) {
            true => &self.values + &other.values,
            false => panic!("Can't add functions defined on different domains."),
        };
        GridFunction::from(self.domain, values)
    }
}

enum DiffDirection {
    Xi,
    Eta,
}

fn one_sided_diff(side: i8, s0: f32, s1: f32, s2: f32, h: f32) -> f32 {
    return (side as f32)*(3_f32*s0 - 4_f32*s1 + s2) / (2_f32*h)
}

fn central_diff(sp1: f32, sm1: f32, h: f32) -> f32 {
    return (sp1 - sm1) / (2_f32*h)
}

fn set_indices(i0: usize, change: bool) -> [usize; 4] {
    match change {
        true => {
            return [i0+1, i0-1, i0+2, i0-2] 
        },
        false => {
            return [i0, i0, i0, i0]
        }
    }
}
