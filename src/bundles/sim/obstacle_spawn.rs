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
                let row_lenght = rng.gen_range(1, 5);
                for _i in 1..=row_lenght {
                    let rand_x = rng.gen_range(RIGHT_BOUND, LEFT_BOUND);
                    updater
                        .create_entity(&entities)
                        .with(ObjectKind::Obstacle)
                        .with(Extent::new(1.))
                        .with(Transform::default().with_position([
                            rand_x,
                            0.,
                            ptrans.position[2] + 200.,
                        ]))
                        .with(NodeBuilder::obstacle())
                        .with(Extent::new(1.))
                        .with(Health::one())
                        .build();
                }
                self.cooldown = rng.gen_range(Self::COOLDOWN / 3., Self::COOLDOWN);
            }
        }
    }
}
