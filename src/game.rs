use crate::bundles::{KissBundle, SimBundle};
use crate::{components::*, util::*};
use specs::prelude::*;
use specs_bundler::Bundler;
use specs_transform::Transform3D;

pub struct Game<'s> {
    world: World,
    dispatcher: Dispatcher<'s, 's>,
}

impl<'s> Game<'s> {
    pub fn new() -> Self {
        let mut world: World = World::new();

        let dispatcher = Bundler::new(&mut world, DispatcherBuilder::new())
            .bundle(SimBundle::default())
            .expect("Unable to bundle SimBundle")
            .bundle(KissBundle::default())
            .expect("Unable to bundle KissBundle")
            .build();
        add_entities(&mut world);

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

fn add_entities(world: &mut World) {
    world
        .create_entity()
        .with(ObjectKind::Obstacle)
        .with(Extent::new(1.))
        .with(Transform3D::<D>::default().with_position([3., 0., 40.]))
        .build();
    world
        .create_entity()
        .with(ObjectKind::Player)
        .with(Transform3D::<D>::default())
        .with(Vel::from([0., 0., 30.]))
        .with(Extent::new(1.))
        .build();
}
