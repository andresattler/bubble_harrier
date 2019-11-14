use crate::{components::*, util::*};
use specs::prelude::*;

pub struct HealthSystem;

impl<'s> specs::System<'s> for HealthSystem {
    type SystemData = (
        ReadStorage<'s, Health>,
        ReadStorage<'s, ObjectKind>,
        WriteStorage<'s, Transform>,
        Entities<'s>,
    );

    fn run(&mut self, (healths, kinds, mut transforms, entities): Self::SystemData) {
        for (health, kind, transform, entity) in
            (&healths, &kinds, &mut transforms, &entities).join()
        {
            if health.current == 0 {
                match kind {
                    ObjectKind::Player => {
                        let new_pos: Vector = Vector::from([0., 0., 0.]);
                        transform.position = new_pos.into();
                    }
                    ObjectKind::Obstacle => {
                        entities.delete(entity).unwrap();
                    }
                }
            }
        }
    }
}
