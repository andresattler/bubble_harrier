use super::Time;
use crate::{components::*, util::*};
use specs::prelude::*;

pub struct AccelerationSystem;

impl AccelerationSystem {
    const MAX_VELS: [D; 3] = [90., 30., 180.];

    pub fn name() -> &'static str {
        "sim::acceleration_system"
    }
}

impl<'s> specs::System<'s> for AccelerationSystem {
    type SystemData = (
        ReadStorage<'s, Force>,
        WriteStorage<'s, Vel>,
        Read<'s, Time>,
    );

    fn run(&mut self, (forces, mut vels, timer): Self::SystemData) {
        (&forces, &mut vels).join().for_each(|(force, vel)| {
            force
                .0
                .iter()
                .map(|v| v * timer.delta())
                .enumerate()
                .for_each(|(i, v)| vel.0[i] = (vel.0[i] + v).max(-Self::MAX_VELS[i]))
        });
    }
}
