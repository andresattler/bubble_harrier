use super::{CurrentInput, Time};
use crate::{components::*, util::*};
use kiss3d::event::Key;
use specs::prelude::*;

pub struct MoveSystem;

impl MoveSystem {
    pub fn name() -> &'static str {
        "sim::move_system"
    }
}

impl<'s> specs::System<'s> for MoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Vel>,
        ReadStorage<'s, ObjectKind>,
        Read<'s, Time>,
        Read<'s, CurrentInput>,
    );

    fn run(&mut self, (mut trans, vel, kinds, time, inp): Self::SystemData) {
        for (mut transform, velocity, kind) in (&mut trans, &vel, &kinds).join() {
            let mut points: [D; 3] = velocity.into();
            if let ObjectKind::Player = kind {
                let x_movement = if inp.keys.contains(&Key::A) {
                    10.
                } else if inp.keys.contains(&Key::D) {
                    -10.
                } else {
                    0.
                };
                points[0] = x_movement;
            }
            let new_pos: Vector =
                Vector::from(transform.position) + Vector::from(points).scale(time.delta());
            transform.position = new_pos.into();
        }
    }
}
