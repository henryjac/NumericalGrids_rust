use crate::geometry::domain::Domain;

use byteorder::WriteBytesExt;
use byteorder::LittleEndian;

use std::ops::Add;
use std::ops::Mul;

use std::fs::File;
use std::io::Write;

use nalgebra as na;

/// Functions defined on specific domain with values on the 
/// discrete points on the domain.
pub struct GridFunction<'a> {
    domain: &'a Domain,
    values: na::DMatrix<f32>,
    n: u8,
    m: u8,
}

impl<'a> GridFunction<'a> {
    /// Creates the 0-function on `domain`
    pub fn new(domain: &'a Domain) -> GridFunction {
        let n = domain.get_n();
        let m = domain.get_m();
        let values = na::DMatrix::from_fn(n.into(),m.into(),|_,_| 0.0);
        GridFunction{domain, values, n, m }
    }

    /// Creates a function from `fnc` on `domain`
    pub fn from_fnc(domain: &'a Domain, fnc: &'a dyn Fn(f32,f32) -> f32,) -> GridFunction {
        let index_fnc = |j: usize, i: usize| -> f32 {
            let xy = domain.get_xy(i,j);
            fnc(xy.get_x(), xy.get_y())
        };
        let n = domain.get_n();
        let m = domain.get_m();
        let values = na::DMatrix::from_fn(m.into(), n.into(), index_fnc);
        GridFunction{domain, values, n, m}
    }

    /// Creates a function from `domain` and matrix `values`. 
    ///
    /// Sizes needs to be compatible
    pub fn from(domain: &'a Domain, values: na::DMatrix<f32>,) -> GridFunction {
        let n = domain.get_n();
        let m = domain.get_m();
        GridFunction{domain, values, n, m}
    }
    
    /// Populates `values` with values from `fnc` on the domain
    pub fn generate_function_values(&mut self, fnc: &dyn Fn(f32,f32) -> f32) {
        let index_fnc = |j: usize, i: usize| -> f32 {
            let xy = self.domain.get_xy(i,j);
            fnc(xy.get_x(), xy.get_y())
        };
        self.values = na::DMatrix::from_fn(self.m.into(), self.n.into(), index_fnc);
    }

    /// Prints the `values` with the specific (x,y) coordinate corresponding
    pub fn print(&self) {
        println!("{}",self.values);
        // TODO: Print (x,y) for each value in values
    }

    /// Saves the values of the function to `location`.
    ///
    /// The first two bytes are the size of the domain and the remaining are the
    /// z-values of the function
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
}

impl<'a> Add for GridFunction<'a> {
    type Output = GridFunction<'a>;
    fn add(self, other: GridFunction<'a>) -> Self::Output {
        // let values = match &self.domain as *const _ == &other.domain as *const _ {
        let values = match std::ptr::eq(self.domain, other.domain) {
            true => self.values + other.values,
            false => panic!("Can't add functions defined on different domains."),
        };
        GridFunction::from(self.domain, values)
    }
}

impl<'a> Mul for GridFunction<'a> {
    type Output = GridFunction<'a>;
    fn mul(self, other: GridFunction<'a>) -> Self::Output {
        let values = match std::ptr::eq(self.domain, other.domain) {
            true => self.values.component_mul(&other.values),
            false => panic!("Can't multiply functions defined on different domains."),
        };
        GridFunction::from(self.domain, values)
    }
}

impl<'a, 'b> Add<&'b GridFunction<'b>> for &'a GridFunction<'a> {
    type Output = GridFunction<'a>;
    fn add(self, other: &'b GridFunction) -> Self::Output {
        let values = match std::ptr::eq(self.domain, other.domain) {
            true => &self.values + &other.values,
            false => panic!("Can't add functions defined on different domains."),
        };
        GridFunction::from(self.domain, values)
    }
}
