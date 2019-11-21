use std::fmt::{Display, Formatter, Result as DRes};

#[derive(Clone, Copy, Debug, Default)]
pub struct Score {
    distance: f32,
}

impl Score {
    pub fn set_distance(&mut self, distance: f32) {
        self.distance = distance;
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut Formatter) -> DRes {
        f.write_str(&format!("{}", (self.distance as u32)))
    }
}
