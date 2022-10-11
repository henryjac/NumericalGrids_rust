use crate::geometry::curves;

pub struct Domain {
    boundary: [Box<dyn curves::Curves>; 4],
    n: u16,
    m: u16,
    x: Vec<f32>,
    y: Vec<f32>,
}

impl Domain {
    /// Generates a domain defiend by four curves
    pub fn new(boundary: [Box<dyn curves::Curves>; 4], n: u16, m: u16) -> Domain {
        let x = vec![0_f32; (n*m).try_into().unwrap()];
        let y = vec![0_f32; (n*m).try_into().unwrap()];
        Domain{boundary, n, m, x, y}
    }
}
