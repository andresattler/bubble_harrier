mod movement;

use super::CurrentInput;
use crate::{components::*, util::*};
use movement::MoveSystem;
use specs::prelude::*;
use specs_bundler::{Bundle, Bundler};
use specs_time::{TimeBundle, TimeSystem};
use specs_transform::TransformBundle;

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
        bundler = bundler
            .bundle(TimeBundle::<TimePrecision>::default())
            .unwrap()
            .bundle(TransformBundle::<D>::default())
            .unwrap();
        bundler.dispatcher_builder = bundler.dispatcher_builder.with(
            MoveSystem,
            "movement_system",
            &[TimeSystem::<TimePrecision>::name()],
        );
        Ok(bundler)
    }
}
