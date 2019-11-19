use crate::{components::*, util::*};
use specs::prelude::*;
use specs_transform::Transform3D;

pub struct ObstacleSpawnSystem;

const DISTANCE: u32 = 25;

impl ObstacleSpawnSystem {
    pub fn name() -> &'static str {
        "sim::obstacle_spawn_system"
    }
}

impl<'s> specs::System<'s> for ObstacleSpawnSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, ObjectKind>,
        Entities<'s>,
        Read<'s, LazyUpdate>,
    );

    fn run(&mut self, (transforms, kinds, entities, updater): Self::SystemData) {
        if let Some((_, ptrans)) = (&kinds, &transforms)
            .join()
            .find(|(obj, _)| obj.is_player())
        {
            let py_int = ptrans.position[2] as u32;
            if py_int % DISTANCE == 0 {
                let obstacle = entities.create();
                updater.insert(obstacle, ObjectKind::Obstacle);
                updater.insert(obstacle, Extent::new(1.));
                updater.insert(
                    obstacle,
                    Transform3D::<D>::default().with_position([3., 0., ptrans.position[2] + 200.]),
                );
                updater.insert(obstacle, Extent::new(1.));
                updater.insert(
                    obstacle,
                    Health {
                        current: 1,
                        full: 1,
                    },
                );
            }
        }
    }
}
