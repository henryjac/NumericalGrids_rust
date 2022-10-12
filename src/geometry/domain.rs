use crate::geometry::curves::Curves;
use crate::geometry::point::Point;

use byteorder::WriteBytesExt;
use byteorder::LittleEndian;

use std::fs::File;
use std::io::Write;

pub struct Domain {
    boundary: [Box<dyn Curves>; 4],
    pub boundary_directions: [bool; 4],
    n: u8,
    m: u8,
    x: Vec<f32>,
    y: Vec<f32>,
}

impl Domain {
    /// Generates a domain defiend by four curves
    pub fn new(boundary: [Box<dyn Curves>; 4], n: u8, m: u8) -> Domain {
        let (consistent, boundary_directions): (bool, [bool; 4]) = Self::consistency_check(&boundary);
        match consistent {
            true => (),
            false => { 
                println!("Curves are not consistent");
                // Maybe do proper error handling some other way
            }
        }

        let mut x = vec![0_f32; (n*m).try_into().unwrap()];
        let mut y = vec![0_f32; (n*m).try_into().unwrap()];

        // gamma for curve
        let mut γ0 = vec![Point::new(); n.into()];
        let mut γ1 = vec![Point::new(); m.into()];
        let mut γ2 = vec![Point::new(); n.into()];
        let mut γ3 = vec![Point::new(); m.into()];

        let mut ξs = vec![0_f32; n.into()];
        let mut ηs = vec![0_f32; m.into()];

        let ψ0 = |s: f32| -> f32 { 1_f32 - s };
        let ψ1 = |s: f32| -> f32 { s };

        let ξη = |s: f32, dir: bool, dir_opposite: bool| -> f32 {
            match dir == dir_opposite {
                true => 1_f32 - s,
                false => s,
            }
        };

        for i in 0..n.into() {
            ξs[i] = (i as f32) / ((n as f32) - 1_f32);
            γ0[i] = boundary[0].xy(ξs[i]);
            γ2[i] = boundary[2].xy(ξη(ξs[i], boundary_directions[2], boundary_directions[0]));
        }
        for j in 0..m.into() {
            ηs[j] = (j as f32) / ((m - 1) as f32);
            γ1[j] = boundary[1].xy(ηs[j]);
            γ3[j] = boundary[3].xy(ξη(ηs[j], boundary_directions[1], boundary_directions[3]));
        }

        for i in 0..n.into() {
            for j in 0..m.into() {
                let edge_contr = 
                    ψ0(ξs[i])*γ3[j] +
                    ψ1(ξs[i])*γ1[j] +
                    ψ0(ηs[j])*γ0[i] +
                    ψ1(ηs[j])*γ2[i];
                let corner_contr = 
                    -ψ0(ξs[i]) * ψ0(ηs[j]) * γ0[0]
                    -ψ0(ξs[i]) * ψ1(ηs[j]) * γ2[0]
                    -ψ1(ξs[i]) * ψ0(ηs[j]) * γ0[(n as usize) - 1]
                    -ψ1(ξs[i]) * ψ1(ηs[j]) * γ2[(n as usize) - 1];
                let xy_value = edge_contr + corner_contr;
                x[i*(m as usize)+j] = xy_value.get_x();
                y[i*(m as usize)+j] = xy_value.get_y();
            }
        }
        Domain{boundary, boundary_directions, n, m, x, y}
    }

    /// Checks if the curves making up the boundary ends where other curves start
    ///
    /// Returns consistency and the direction of the curves in a tuple
    fn consistency_check(boundary: &[Box<dyn Curves>; 4]) -> (bool, [bool; 4]) {
        let mut boundary_directions: [bool; 4] = [true; 4];
        for i in 0..4 {
            let mut checks = [false; 4];
            for j in 0..2 {
                for k in 0..2 {
                    checks[j*2+k] = boundary[i].xy(j as f32)
                        .approx_equal(&boundary[(i+1)%4].xy(k as f32));
                }
            }
            if checks[0] || checks[1] {
                boundary_directions[i] = false;
            } else if checks[2] || checks[3] {
                boundary_directions[i] = true;
            } else {
                println!("Consistency check failed at curve {}",i);
                return (false, boundary_directions)
            }
        }
        return (true, boundary_directions)
    }

    pub fn get_n(&self) -> u8 {
        return self.n
    }

    pub fn get_m(&self) -> u8 {
        return self.m
    }

    pub fn get_xy(&self, i: usize, j: usize) -> Point {
        return Point::from(self.x[i*(self.m as usize) + j], self.y[i*(self.m as usize) + j])
    }

    pub fn save_grid(&self, location: &str) -> std::io::Result<()> {
        let mut file = File::create(location)?;
        file.write(&[self.n])?;
        file.write(&[self.m])?;
        for i in 0..(self.n*self.m).into() {
            file.write_f32::<LittleEndian>(self.x[i])?;
            file.write_f32::<LittleEndian>(self.y[i])?;
        }
        Ok(())
    }

    pub fn save_boundary(&self, location: &str, precision: u8) -> std::io::Result<()> {
        let mut file = File::create(location)?;
        file.write(&[precision])?;
        for i in 0..4 {
            for j in 0..precision+1 {
                let s = match self.boundary_directions[i] {
                    true => (j as f32) / (precision as f32),
                    false => 1_f32 - (j as f32) / (precision as f32),
                };
                let xy = self.boundary[i].xy(s);

                file.write_f32::<LittleEndian>(xy.get_x())?;
                file.write_f32::<LittleEndian>(xy.get_y())?;
            }
        }
        Ok(())
    }
}
