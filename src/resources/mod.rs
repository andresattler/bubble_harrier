mod config;

pub use config::Configuration;
use kiss3d::event::Key;
use specs::Entity;
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter, Result as DRes};
use std::ops::Deref;

#[derive(Clone, Copy, Debug)]
pub struct Player(pub Entity);

impl Deref for Player {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

#[derive(Clone, Default, Debug)]
pub struct CurrentInput {
    pub keys: BTreeSet<Key>,
}
