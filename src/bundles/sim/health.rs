use crate::{components::*, resources::*, util::*};
use specs::prelude::*;

pub struct HealthSystem;

impl<'s> specs::System<'s> for HealthSystem {
    type SystemData = (
        WriteStorage<'s, Health>,
        ReadStorage<'s, ObjectKind>,
        WriteStorage<'s, Transform>,
        WriteExpect<'s, Score>,
        Entities<'s>,
    );

    fn run(&mut self, (mut healths, kinds, mut transforms, mut score, entities): Self::SystemData) {
        for (mut health, kind, transform, entity) in
            (&mut healths, &kinds, &mut transforms, &entities).join()
        {
            if health.current <= 0 {
                match kind {
                    ObjectKind::Player => {
                        let new_pos: Vector = Vector::from([0., 0., 0.]);
                        transform.position = new_pos.into();
                        health.current = health.full;
                        score.reset();
                    }
                    _ => {
                        score.add_kills((health.full * 100) as f32);
                        entities.delete(entity).unwrap();
                    }
                }
            }
        }
    }
}
