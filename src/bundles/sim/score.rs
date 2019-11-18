use crate::{components::*, util::*};
use specs::prelude::*;
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

#[derive(Clone, Copy, Debug, Default)]
pub struct ScoreSystem;

impl<'s> specs::System<'s> for ScoreSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, ObjectKind>,
        Write<'s, Score>,
    );

    fn run(&mut self, (trans, kinds, mut score): Self::SystemData) {
        for (transform, kind) in (&trans, &kinds).join() {
            if let ObjectKind::Player = kind {
                score.set_distance(transform.position[2]);
            }
        }
    }
}
