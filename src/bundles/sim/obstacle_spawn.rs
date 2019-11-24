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

    fn add_obstacle(updater: &LazyUpdate, entities: &Entities, x: f32, z: f32) {
        let obstacle = entities.create();
        updater.insert(obstacle, ObjectKind::Obstacle);
        updater.insert(obstacle, Extent::new(1.));
        updater.insert(
            obstacle,
            Transform3D::<D>::default().with_position([x, 0., z + 200.]),
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

    fn add_row(updater: &LazyUpdate, entities: &Entities, z: f32) {
        let mut rng = thread_rng();
        let row_lenght = rng.gen_range(1, 5);
        let rand_x = rng.gen_range(RIGHT_BOUND as i32, LEFT_BOUND as i32);
        let mut left_x = rand_x as f32;
        let mut right_x = rand_x as f32;

        let mut i = 0;
        println!("----- rand_x {}", rand_x);
        while i < row_lenght {
            let x = if i == 0 {
                rand_x as f32
            } else if i % 2 == 0 {
                    left_x -= 2.;
                    left_x
                } else {
                    right_x += 2.;
                    right_x
                };
            if left_x < LEFT_BOUND || right_x > RIGHT_BOUND {
                println!("----- x {}", x);
                Self::add_obstacle(&updater, &entities, x, z);
                i += 1;
            }
        }
    }
}

impl<'s> specs::System<'s> for ObstacleSpawnSystem {
    type SystemData = (
        ReadExpect<'s, Player>,
        ReadStorage<'s, Transform>,
        Write<'s, LastObstaclePlaced>,
        Entities<'s>,
        Read<'s, LazyUpdate>,
    );

    fn run(&mut self, (player, transforms, mut last_obstacle_placed, entities, updater): Self::SystemData) {
        if let Some(ptrans) = transforms.get(player.0) {
            let py_int = ptrans.position[2] as u32;
            if last_obstacle_placed.get_last_placed_z() != py_int {
                if py_int % DISTANCE == 0 {
                    Self::add_row(&updater, &entities, ptrans.position[2]);
                    last_obstacle_placed.set_last_placed_z(py_int);
                }
            }
        }
    }
}
