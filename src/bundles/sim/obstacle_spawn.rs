use crate::{components::*, resources::*, util::*};
use rand::{thread_rng, Rng};
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
        ReadExpect<'s, Player>,
        ReadStorage<'s, Transform>,
        Entities<'s>,
        Read<'s, LazyUpdate>,
    );

    fn run(&mut self, (player, transforms, entities, updater): Self::SystemData) {
        if let Some(ptrans) = transforms.get(player.0) {
            let py_int = ptrans.position[2] as u32;
            if py_int % DISTANCE == 0 {
                let obstacle = entities.create();
                updater.insert(obstacle, ObjectKind::Obstacle);
                updater.insert(obstacle, Extent::new(1.));
                let mut rng = thread_rng();
                let row_lenght = rng.gen_range(1, 5);
                for _i in 1..=row_lenght {
                    let rand_x = rng.gen_range(RIGHT_BOUND, LEFT_BOUND);
                    updater.insert(
                        obstacle,
                        Transform3D::<D>::default().with_position([
                            rand_x,
                            0.,
                            ptrans.position[2] + 200.,
                        ]),
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
}
