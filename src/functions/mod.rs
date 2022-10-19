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


/// Direction for which we take a derivative in 
/// reference coordinates
#[allow(non_camel_case_types)]
enum DiffDirection {
    ξ,
    η,
}

impl<'a> GridFunction<'a, f32> {
    /// Creates the 0-function on `domain`
    pub fn new(domain: &'a Domain<f32>) -> GridFunction<f32> {
        let n = domain.get_n();
        let m = domain.get_m();
        let h_ξ = 1_f32 / (n as f32);
        let h_η = 1_f32 / (n as f32);
        // Find if we have a 0 initializer
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
    
    /// Populates `values` with values from `fnc` on the domain
    pub fn generate_function_values(&mut self, fnc: &dyn Fn(f32,f32) -> f32) {
        let index_fnc = |j: usize, i: usize| -> f32 {
            let xy = self.domain.get_xy(i,j);
            fnc(xy.get_x(), xy.get_y())
        };
        self.values = DMatrix::from_fn(self.m.into(), self.n.into(), index_fnc);
    }

    /// Prints the `values` with the specific (x,y) coordinate corresponding
    pub fn print_values(&self) {
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

    fn get_value(&self, i: usize, j: usize) -> f32 {
        self.values[(i as u16 * self.m as u16 + j as u16) as usize]
    }

    pub fn pd_xy(self) -> [GridFunction<'a, f32>; 2] {
        let mut values_pdx = DMatrix::from_fn(self.n.into(), self.m.into(),|_,_| 0.0); 
        let mut values_pdy = DMatrix::from_fn(self.n.into(), self.m.into(),|_,_| 0.0); 
        for i in 0..self.n as usize {
            for j in 0..self.m as usize {
                let partial_return_vals = self.partial_derivative(i,j);
                values_pdx[i * self.m as usize + j] = partial_return_vals[0];
                values_pdy[i * self.m as usize + j] = partial_return_vals[1];
            }
        }
        [
            GridFunction::from(&self.domain, values_pdx),
            GridFunction::from(&self.domain, values_pdy)
        ]
    }

    /// Calculates pdx and pdy from a reference to &self
    pub fn pd_xy_ref(&'a self) -> [GridFunction<'a, f32>; 2] {
        let mut values_pdx = DMatrix::from_fn(self.n.into(), self.m.into(),|_,_| 0.0); 
        let mut values_pdy = DMatrix::from_fn(self.n.into(), self.m.into(),|_,_| 0.0); 
        for i in 0..self.n as usize {
            for j in 0..self.m as usize {
                let partial_return_vals = self.partial_derivative(i,j);
                values_pdx[i * self.m as usize + j] = partial_return_vals[0];
                values_pdy[i * self.m as usize + j] = partial_return_vals[1];
            }
        }
        [
            GridFunction::from(&self.domain, values_pdx),
            GridFunction::from(&self.domain, values_pdy)
        ]
    }

    pub fn pdx(self) -> GridFunction<'a, f32> {
        let mut values = DMatrix::from_fn(self.n.into(), self.m.into(),|_,_| 0.0); 
        for i in 0..self.n as usize {
            for j in 0..self.m as usize {
                values[i * self.m as usize + j] = self.partial_derivative(i, j)[0]
            }
        }
        GridFunction::from(&self.domain, values)
    }

    /// Calculates pdx from a reference to &self
    pub fn pdx_ref(&self) -> GridFunction<'a, f32> {
        let mut values = DMatrix::from_fn(self.n.into(), self.m.into(),|_,_| 0.0); 
        for i in 0..self.n as usize {
            for j in 0..self.m as usize {
                values[i * self.m as usize + j] = self.partial_derivative(i, j)[0]
            }
        }
        GridFunction::from(&self.domain, values)
    }

    pub fn pdy(self) -> GridFunction<'a, f32> {
        let mut values = DMatrix::from_fn(self.n.into(), self.m.into(),|_,_| 0.0); 
        for i in 0..self.n as usize {
            for j in 0..self.m as usize {
                values[i * self.m as usize + j] = self.partial_derivative(i, j)[1]
            }
        }
        GridFunction::from(&self.domain, values)
    }

    /// Calculates pdy from a reference to &self
    pub fn pdy_ref(&self) -> GridFunction<'a, f32> {
        let mut values = DMatrix::from_fn(self.n.into(), self.m.into(),|_,_| 0.0); 
        for i in 0..self.n as usize {
            for j in 0..self.m as usize {
                values[i * self.m as usize + j] = self.partial_derivative(i, j)[1]
            }
        }
        GridFunction::from(&self.domain, values)
    }

    pub fn laplace(&self) -> GridFunction<f32> {
        let [dx,dy] = self.pd_xy_ref();
        dx.pdx() + dy.pdy()
    }

    /// Calculates the partial derivatives in both x- and y-directionsa at index i,j,
    /// returns a list of the values
    #[allow(non_snake_case)]
    fn partial_derivative(&self, i: usize, j: usize) -> [f32; 2] {
        let x = |i: usize, j: usize| -> f32 {self.domain.get_xy(i,j).get_x()};
        let y = |i: usize, j: usize| -> f32 {self.domain.get_xy(i,j).get_y()};
        let f = |i: usize, j: usize| -> f32 {self.get_value(i,j)};

        let x_ξ = self.partial_derivative_of_fnc(&x, DiffDirection::ξ, i, j);
        let x_η = self.partial_derivative_of_fnc(&x, DiffDirection::η, i, j);
        let y_ξ = self.partial_derivative_of_fnc(&y, DiffDirection::ξ, i, j);
        let y_η = self.partial_derivative_of_fnc(&y, DiffDirection::η, i, j);

        let u_ξ = self.partial_derivative_of_fnc(&f, DiffDirection::ξ, i, j);
        let u_η = self.partial_derivative_of_fnc(&f, DiffDirection::η, i, j);

        let det_J = x_ξ * y_η - x_η * y_ξ;

        let u_x = (u_ξ * y_η - u_η * y_ξ) / det_J;
        let u_y = (u_η * x_ξ - u_ξ * y_η) / det_J;

        return [u_x, u_y]
    }

    fn partial_derivative_of_fnc(
        &self, f: &dyn Fn(usize, usize) -> f32, dir: DiffDirection, i: usize, j: usize) -> f32 
    {
        let last_index: usize;
        let current_index: usize;
        let is: [usize; 4];
        let js: [usize; 4];
        let h: f32;
        match dir {
            DiffDirection::ξ => {
                current_index = i;
                last_index = (self.n-1) as usize;
                h = self.h_ξ;
                is = set_indices(i, true);
                js = set_indices(j, false);
            },
            DiffDirection::η => {
                current_index = j;
                last_index = (self.m-1) as usize;
                h = self.h_η;
                is = set_indices(i, false);
                js = set_indices(j, true);
            }
        }
        if current_index == 0 {
            one_sided_diff(-1, f(i,j), f(is[0],js[0]), f(is[2],js[2]), h)
        } else if current_index == last_index {
            one_sided_diff(1, f(i,j), f(is[1],js[1]), f(is[3],js[3]), h)
        } else {
            central_diff(f(is[0],js[0]), f(is[1],js[1]), h)
        }
    }
}

impl<'a> Add for GridFunction<'a, f32> {
    type Output = GridFunction<'a, f32>;
    fn add(self, other: GridFunction<'a, f32>) -> Self::Output {
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

fn one_sided_diff(side: i8, s0: f32, s1: f32, s2: f32, h: f32) -> f32 {
    return (side as f32)*(3_f32*s0 - 4_f32*s1 + s2) / (2_f32*h)
}

fn central_diff(sp1: f32, sm1: f32, h: f32) -> f32 {
    return (sp1 - sm1) / (2_f32*h)
}

fn set_indices(i0: usize, change: bool) -> [usize; 4] {
    match change {
        true => {
            match i0 {
                // If i0 is 0 or 1 we don't use the negative values
                // and so we dont do the subtractions since we have usize values
                0 => [i0+1, i0, i0+2, i0],
                1 => [i0+1, i0-1, i0+2, i0],
                _ => [i0+1, i0-1, i0+2, i0-2],
            }
        },
        false => {
            return [i0, i0, i0, i0]
        }
    }
}
