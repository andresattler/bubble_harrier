use crate::{components::*, resources::*, util::*};
use na::zero;
use rand::{thread_rng, Rng};
use specs::prelude::*;

#[derive(Default)]
pub struct ObstacleSpawnSystem {
    cooldown: TimePrecision,
}

impl ObstacleSpawnSystem {
    const COOLDOWN: TimePrecision = 1.;
    pub fn name() -> &'static str {
        "sim::obstacle_spawn_system"
    }

    fn add_obstacle(updater: &LazyUpdate, entities: &Entities, x: f32, z: f32) {
        updater
            .create_entity(&entities)
            .with(ObjectKind::Obstacle)
            .with(Extent::new(1.))
            .with(Transform::default().with_position([x, 0., z + 200.]))
            .with(NodeBuilder::obstacle())
            .with(Extent::new(1.))
            .with(Health::one())
            .build();
    }

    fn add_multiple(updater: &LazyUpdate, entities: &Entities, amount: u32, z: f32) {
        let mut rng = thread_rng();
        for _ in 1..=amount {
            let rand_x = rng.gen_range(RIGHT_BOUND, LEFT_BOUND);
            Self::add_obstacle(&updater, &entities, rand_x, z);
        }
    }
    fn add_row(updater: &LazyUpdate, entities: &Entities, row_length: u32, z: f32) {
        let mut rng = thread_rng();
        let rand_x = rng.gen_range(RIGHT_BOUND as i32, LEFT_BOUND as i32);
        let mut left_x = rand_x as f32;
        let mut right_x = rand_x as f32;

        Self::add_obstacle(&updater, &entities, rand_x as f32, z);
        for i in 1..=row_length {
            if i % 2 == 0 && left_x + 2.0 < LEFT_BOUND {
                left_x += 2.;
                Self::add_obstacle(&updater, &entities, left_x, z);
            } else if right_x - 2.0 > RIGHT_BOUND {
                right_x -= 2.;
                Self::add_obstacle(&updater, &entities, right_x, z);
            }
        }
    }
}

impl<'s> specs::System<'s> for ObstacleSpawnSystem {
    type SystemData = (
        ReadExpect<'s, Player>,
        ReadStorage<'s, Transform>,
        Entities<'s>,
        Read<'s, LazyUpdate>,
        Read<'s, Time>,
    );

    fn run(&mut self, (player, transforms, entities, updater, time): Self::SystemData) {
        self.cooldown = (self.cooldown - time.delta()).max(zero());
        if let Some(ptrans) = transforms.get(player.0) {
            if self.cooldown <= zero() {
                let mut rng = thread_rng();
                let amount = rng.gen_range(1, 5);
                let p = rng.gen_range(1, 3);
                match p {
                    1 => {
                        Self::add_row(&updater, &entities, amount, ptrans.position[2]);
                    },
                    _ => {
                        Self::add_multiple(&updater, &entities, amount, ptrans.position[2]);
                    }
                }
                self.cooldown = rng.gen_range(Self::COOLDOWN / 3., Self::COOLDOWN);
            }
        }
    }
}
