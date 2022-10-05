mod point;

pub struct StraightLine {
    a:f32, 
    b:f32,
    c:f32,
    d:f32,

    p_min:f32,
    p_max:f32,
}

impl StraightLine {
    pub fn def() -> StraightLine {
        let a=1_f32;
        let b=0_f32;
        let c=0_f32;
        let d=0_f32;

        let p_min=0_f32;
        let p_max=1_f32;

        StraightLine{a,b,c,d,p_min,p_max}
    }

    pub fn default(a:f32,b:f32,c:f32,d:f32) -> StraightLine {
        let p_min = 0_f32;
        let p_max:f32 = 1.0;
        StraightLine{a,b,c,d,p_min,p_max}
    }

    pub fn new(a:f32,b:f32,c:f32,d:f32,p_min:f32,p_max:f32) -> StraightLine {
        StraightLine{a,b,c,d,p_min,p_max}
    }

    // Should return Point or None, or raise an exception of invalid value, if
    // we have similar exception handling in Rust as other languages
    pub fn get_point(&self,t:f32) -> point::Point {
        match t<self.p_min || t>self.p_max {
            true => {
                println!("Line not defined for values outside ({},{})",self.p_min,self.p_max);
                return point::Point::default();
            },
            false => {
                return point::Point::new(self.a*t+self.c, self.b*t+self.d)
            }
        }
    }
}
