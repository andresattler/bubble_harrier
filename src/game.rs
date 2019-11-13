use crate::bundles::KissBundle;
use crate::{components::*, util::*};
use specs::prelude::*;
use specs_bundler::Bundler;
use specs_time::TimeBundle;
use specs_transform::{Transform3D, TransformBundle};

pub struct Game<'s> {
    world: World,
    dispatcher: Dispatcher<'s, 's>,
}

impl<'s> Game<'s> {
    pub fn new() -> Self {
        let mut world: World = World::new();
        let trans: Transform3D<D> = Transform3D::default();

        world.register::<ObjectKind>();
        world.register::<Vel>();
        world.register::<Transform>();
        world
            .create_entity()
            .with(ObjectKind::Player)
            .with(trans)
            .with(Vel::from([0., 0., 30.]))
            .build();

        let dispatcher = Bundler::new(&mut world, DispatcherBuilder::new())
            .bundle(TimeBundle::<f64>::default())
            .unwrap()
            .bundle(TransformBundle::<f32>::default())
            .unwrap()
            .bundle(KissBundle::default())
            .expect("Unable to bundle KissBundle")
            .build();

        Self { world, dispatcher }
    }

    pub fn run_frame(&mut self) {
        self.dispatcher.dispatch(&self.world);
    }

    pub fn run(&mut self) {
        loop {
            self.run_frame()
        }
    }
}
