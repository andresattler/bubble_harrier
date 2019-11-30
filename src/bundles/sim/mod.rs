mod acceleration;
mod collide;
mod damage;
mod despawn;
mod health;
mod movement;
mod obstacle_spawn;
mod score;
mod shooting;

use crate::{components::*, util::*};
use acceleration::AccelerationSystem;
use collide::CollisionSystem;
use damage::DamageSystem;
use despawn::DespawnSystem;
use health::HealthSystem;
use movement::MoveSystem;
use obstacle_spawn::ObstacleSpawnSystem;
use score::ScoreSystem;
use shooting::ShootingSystem;
use specs::prelude::*;
use specs_bundler::{Bundle, Bundler};
use specs_time::{TimeBundle, TimeSystem};
use specs_transform::TransformBundle;

#[derive(Debug, Default)]
pub struct SimBundle<'deps> {
    deps: Vec<&'deps str>,
}

impl<'deps, 'world, 'a, 'b> Bundle<'world, 'a, 'b> for SimBundle<'deps> {
    type Error = ();

    #[inline]
    fn bundle(
        self,
        mut bundler: Bundler<'world, 'a, 'b>,
    ) -> Result<Bundler<'world, 'a, 'b>, Self::Error> {
        bundler.world.register::<ObjectKind>();
        bundler.world.register::<Vel>();
        bundler.world.register::<Transform>();
        bundler.world.register::<Extent>();
        bundler.world.register::<Collision>();
        bundler.world.register::<Force>();
        bundler.world.register::<Health>();

        bundler = bundler
            .bundle(TimeBundle::<TimePrecision>::default())
            .unwrap()
            .bundle(TransformBundle::<D>::default())
            .unwrap();
        bundler.dispatcher_builder = bundler
            .dispatcher_builder
            .with(
                AccelerationSystem,
                AccelerationSystem::name(),
                &[TimeSystem::<TimePrecision>::name()],
            )
            .with(
                MoveSystem,
                MoveSystem::name(),
                &[
                    TimeSystem::<TimePrecision>::name(),
                    AccelerationSystem::name(),
                ],
            )
            .with(
                ShootingSystem::default(),
                ShootingSystem::name(),
                &[MoveSystem::name()],
            )
            .with(
                CollisionSystem,
                CollisionSystem::name(),
                &[MoveSystem::name()],
            )
            .with(ScoreSystem, "score_system", &[MoveSystem::name()])
            .with(
                DamageSystem,
                DamageSystem::name(),
                &[CollisionSystem::name()],
            )
            .with(HealthSystem, "health_system", &[DamageSystem::name()])
            .with(
                ObstacleSpawnSystem::default(),
                ObstacleSpawnSystem::name(),
                &[MoveSystem::name()],
            )
            .with(DespawnSystem, DespawnSystem::name(), &[MoveSystem::name()]);
        Ok(bundler)
    }
}
