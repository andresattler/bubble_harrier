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
        Entities<'s>,
        Read<'s, LazyUpdate>,
        Read<'s, Time>,
    );

    fn run(&mut self, (player, transforms, entities, updater, time): Self::SystemData) {
        self.cooldown = (self.cooldown - time.delta()).max(zero());
        if let Some(ptrans) = transforms.get(player.0) {
            if self.cooldown <= zero() {
                Self::add_row(&updater, &entities, ptrans.position[2]);
                let mut rng = thread_rng();
                self.cooldown = rng.gen_range(Self::COOLDOWN / 3., Self::COOLDOWN);
            }
        }
    }
}
