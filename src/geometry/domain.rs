use crate::geometry::curves;
use crate::geometry::point;

pub struct Domain {
    boundary: [Box<dyn curves::Curves>; 4],
    boundary_directions: [bool; 4],
    n: u16,
    m: u16,
    x: Vec<f32>,
    y: Vec<f32>,
}

impl Domain {
    /// Generates a domain defiend by four curves
    pub fn new(boundary: [Box<dyn curves::Curves>; 4], n: u16, m: u16) -> Domain {
        let (consistent, boundary_directions): (bool, [bool; 4]) = Self::consistency_check(&boundary);
        match consistent {
            true => (),
            false => println!("Curves are not consistent")
        }

        let mut x = vec![0_f32; (n*m).try_into().unwrap()];
        let mut y = vec![0_f32; (n*m).try_into().unwrap()];

        // gamma for curve
        let mut γ0 = vec![point::Point::default(); n.into()];
        let mut γ1 = vec![point::Point::default(); m.into()];
        let mut γ2 = vec![point::Point::default(); n.into()];
        let mut γ3 = vec![point::Point::default(); m.into()];

        let mut ξs = vec![0_f32; m.into()];
        let mut ηs = vec![0_f32; n.into()];

        let ψ0 = |s: f32| -> f32 { 1_f32 - s };
        let ψ1 = |s: f32| -> f32 { s };

        for i in 0..n.into() {
            ξs[i] = (i as f32) / ((n as f32) - 1_f32);
            γ0[i] = boundary[0].xy(ξs[i]);
            γ2[i] = boundary[2].xy(ξs[i]);
        }
        for j in 0..m.into() {
            ηs[j] = (j as f32) / ((m as f32) - 1_f32);
            γ1[j] = boundary[1].xy(ηs[j]);
            γ3[j] = boundary[3].xy(ηs[j]);
        }

        for i in 0..n.into() {
            for j in 0..m.into() {
                let edge_contr = 
                    ψ0(ξs[i])*γ3[j] +
                    ψ1(ξs[i])*γ1[j] +
                    ψ0(ηs[j])*γ0[i] +
                    ψ1(ηs[j])*γ2[i];
                let corner_contr = 
                    ψ0(ξs[i]) * ψ0(ηs[j]) * γ0[0] +
                    ψ0(ξs[i]) * ψ1(ηs[j]) * γ2[(n as usize) - 1] +
                    ψ1(ξs[i]) * ψ0(ηs[j]) * γ0[0] +
                    ψ1(ξs[i]) * ψ1(ηs[j]) * γ2[(n as usize) - 1];
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
    fn consistency_check(boundary: &[Box<dyn curves::Curves>; 4]) -> (bool, [bool; 4]) {
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
                return (false, boundary_directions)
            }
        }
        return (true, boundary_directions)
    }
}
