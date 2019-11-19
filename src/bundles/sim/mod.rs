mod collide;
mod damage;
mod health;
mod movement;
mod score;
mod obstacle_spawn;

use super::CurrentInput;
use crate::{components::*, util::*};
use collide::CollisionSystem;
use damage::DamageSystem;
use health::HealthSystem;
use movement::MoveSystem;
use score::ScoreSystem;
use obstacle_spawn::ObstacleSpawnSystem;
use specs::prelude::*;
use specs_bundler::{Bundle, Bundler};
use specs_time::{TimeBundle, TimeSystem};
use specs_transform::TransformBundle;

pub use score::Score;

type TimePrecision = f32;
type Time = specs_time::Time<TimePrecision>;

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
        bundler.world.register::<Health>();
        bundler.world.insert(Score::default());

        bundler = bundler
            .bundle(TimeBundle::<TimePrecision>::default())
            .unwrap()
            .bundle(TransformBundle::<D>::default())
            .unwrap();
        bundler.dispatcher_builder = bundler
            .dispatcher_builder
            .with(
                MoveSystem,
                MoveSystem::name(),
                &[TimeSystem::<TimePrecision>::name()],
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
            .with(ObstacleSpawnSystem, ObstacleSpawnSystem::name(), &[MoveSystem::name()]);
        Ok(bundler)
    }
}
