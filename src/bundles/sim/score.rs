use crate::{components::*, resources::*, util::*};
use specs::prelude::*;

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
