use crate::bundles::{KissBundle, SimBundle};
use crate::{components::*, resources::*, util::*};
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
        self.world.maintain();
        self.dispatcher.dispatch(&self.world);
    }

    pub fn run(&mut self) {
        loop {
            self.run_frame()
        }
    }
}

fn add_entities(world: &mut World) {
    let player = world
        .create_entity()
        .with(ObjectKind::Player)
        .with(Transform3D::<D>::default())
        .with(Vel::from([0., 0., 30.]))
        .with(Extent::new(1.))
        .with(NodeBuilder::player())
        .with(Health {
            current: 5,
            full: 5,
        })
        .build();
    world.insert(Player(player));
    world.insert(Score::default());
}
