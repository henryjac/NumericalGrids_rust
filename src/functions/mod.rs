use crate::geometry::domain::Domain;

use byteorder::WriteBytesExt;
use byteorder::LittleEndian;

use std::fs::File;
use std::io::Write;

use nalgebra as na;

pub struct GridFunction<'a> {
    values: na::DMatrix<f32>,
    domain: &'a Domain,
    n: u8,
    m: u8,
}

impl<'a> GridFunction<'a> {
    pub fn new(domain: &'a Domain) -> GridFunction {
        let n = domain.get_n();
        let m = domain.get_m();
        let values = na::DMatrix::from_fn(n.into(),m.into(),|_,_| 0.0);
        GridFunction{values, domain, n, m }
    }

    pub fn from_fnc(fnc: &'a dyn Fn(f32,f32) -> f32, domain: &'a Domain) -> GridFunction {
        let index_fnc = |j: usize, i: usize| -> f32 {
            let xy = domain.get_xy(i,j);
            fnc(xy.get_x(), xy.get_y())
        };
        let n = domain.get_n();
        let m = domain.get_m();
        let values = na::DMatrix::from_fn(m.into(), n.into(), index_fnc);
        GridFunction{values, domain, n, m}
    }

    pub fn from(values: na::DMatrix<f32>, domain: &'a Domain) -> GridFunction {
        let n = domain.get_n();
        let m = domain.get_m();
        GridFunction{values, domain, n, m}
    }

    pub fn generate_function_values(&mut self, fnc: &dyn Fn(f32,f32) -> f32) {
        let index_fnc = |j: usize, i: usize| -> f32 {
            let xy = self.domain.get_xy(i,j);
            fnc(xy.get_x(), xy.get_y())
        };
        self.values = na::DMatrix::from_fn(self.m.into(), self.n.into(), index_fnc);
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
}

impl<'a> std::ops::Add for GridFunction<'a> {
    type Output = Self;
    fn add(self, other: GridFunction) -> Self::Output {
        // let values = match &self.domain as *const _ == &other.domain as *const _ {
        let values = match std::ptr::eq(self.domain, other.domain) {
            true => self.values + other.values,
            false => {
                println!("Can't add functions defined on different domains.");
                std::process::exit(1);
            }
        };
        GridFunction::from(values, self.domain)
    }
}

// impl<'a, 'b> std::ops::Add<&'a GridFunction> for &'b GridFunction {
//     type Output = Self;
//     fn add(self, other: &'a GridFunction) -> Self::Output {
//         let values = match std::ptr::eq(self.domain, other.domain) {
//             true => &self.values + &other.values,
//             false => {
//                 println!("Can't add functions defined on different domains.");
//                 std::process::exit(1);
//             }
//         };
//         GridFunction { values, domain: self.domain, n: self.domain.get_n(), m: self.domain.get_m()}
//         // GridFunction::from(values, self.domain)
//     }
// }
