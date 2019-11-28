use crate::{components::*, resources::*, util::*};
use na::zero;
use specs::prelude::*;

#[derive(Default)]
pub struct ShootingSystem {
    cooldown: TimePrecision,
}

impl ShootingSystem {
    const COOLDOWN: TimePrecision = 0.2;
    pub fn name() -> &'static str {
        "sim::shooting_system"
    }
}

impl<'s> specs::System<'s> for ShootingSystem {
    type SystemData = (
        ReadExpect<'s, Player>,
        Read<'s, LazyUpdate>,
        Read<'s, CurrentInput>,
        Read<'s, Time>,
        Read<'s, Configuration>,
        Entities<'s>,
        ReadStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (player, updater, input, timer, config, entities, transform): Self::SystemData,
    ) {
        self.cooldown = (self.cooldown - timer.delta()).max(zero());
        if self.cooldown <= zero() && input.keys.contains(&config.controls.shoot) {
            let ptrans = transform.get(**player).expect("No transform of player?!");
            let mut ntrans = Transform::default().with_position(ptrans.position.clone());
            ntrans.position[1] += 1.; // fire from up
            ntrans.position[2] += 1.; // Don't fire inside. Don't shoot yourself.
            let mut vel = Vel::from([0., 0., config.player.speed_z * 1.5]);
            vel.0[2] += 40.;
            updater
                .create_entity(&entities)
                .with(ObjectKind::Obstacle)
                .with(Extent::new(0.1))
                .with(Health::one())
                .with(NodeBuilder::projectile())
                .with(ntrans)
                .with(vel)
                .build();
            self.cooldown = Self::COOLDOWN;
        }
    }
}
