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
            let new_pos =
                Vector::from(transform.position) + Vector::from(points).scale(time.delta());
            transform.position = in_bounds(new_pos);
        }
    }
}

/// Converts a vector to an array and makes sure the x position is in bounds.
fn in_bounds(ve: Vector) -> [D; 3] {
    let mut points: [D; 3] = ve.into();
    points[0] = points[0].min(LEFT_BOUND).max(RIGHT_BOUND);
    points
}
