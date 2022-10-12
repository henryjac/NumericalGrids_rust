use crate::geometry::domain::Domain;

use nalgebra as na;

pub struct GridFunction<'a> {
    values: na::DMatrix<f32>,
    domain: &'a Domain,
    n: u8,
    m: u8,
}

impl<'a> GridFunction<'a> {
    pub fn new(domain: &Domain) -> GridFunction {
        let n = domain.get_n();
        let m = domain.get_m();
        let values = na::DMatrix::from_fn(n.into(),m.into(),|_,_| 0.0);
        GridFunction{ values, domain, n, m }
    }

    fn from(values: na::DMatrix<f32>, domain: &Domain) -> GridFunction {
        let n = domain.get_n();
        let m = domain.get_m();
        GridFunction{ values, domain, n, m}
    }

    pub fn generate_function_values(&mut self, fnc: &dyn Fn(f32,f32) -> f32) {
        let index_fnc = |i: usize, j: usize| -> f32 {
            let xy = self.domain.get_xy(i,j);
            fnc(xy.get_x(), xy.get_y())
        };
        self.values = na::DMatrix::from_fn(self.n.into(), self.m.into(), index_fnc);
    }
}

impl<'a> std::ops::Add for GridFunction<'a> {
    type Output = Self;
    fn add(self, other: GridFunction) -> Self::Output {
        let values = match std::ptr::eq(&self.domain, &other.domain) {
            true => self.values + other.values,
            false => {
                println!("Can't add functions defined on different domains.");
                std::process::exit(1);
            }
        };
        GridFunction::from(values, self.domain)
    }
}
