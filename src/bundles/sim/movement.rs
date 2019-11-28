use super::Time;
use crate::{components::*, resources::*, util::*};
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
        WriteStorage<'s, Vel>,
        ReadStorage<'s, ObjectKind>,
        Read<'s, Configuration>,
        Read<'s, Time>,
        Read<'s, CurrentInput>,
    );

    fn run(&mut self, (mut trans, mut vel, kinds, config, time, inp): Self::SystemData) {
        for (mut transform, mut velocity, kind) in (&mut trans, &mut vel, &kinds).join() {
            if let ObjectKind::Player = kind {
                let x_movement = if inp.keys.contains(&config.controls.left) {
                    config.player.speed_x
                } else if inp.keys.contains(&config.controls.right) {
                    -config.player.speed_x
                } else {
                    0.
                };

                if on_floor(transform) && inp.keys.contains(&config.controls.jump) {
                    debug!("Jumped!");
                    velocity.0[1] = 10.;
                }

                velocity.0[2] = if on_floor(transform) {
                    config.player.speed_z
                } else {
                    config.player.speed_z * 2.
                };

                velocity.0[0] = x_movement;
            }
            let new_pos =
                Vector::from(transform.position) + Vector::from(velocity.0).scale(time.delta());
            transform.position = in_bounds(new_pos);
        }
    }
}

fn on_floor(trans: &Transform) -> bool {
    trans.position[1] < 1.
}

/// Converts a vector to an array and makes sure the x position is in bounds.
fn in_bounds(ve: Vector) -> [D; 3] {
    let mut points: [D; 3] = ve.into();
    points[0] = points[0].min(LEFT_BOUND).max(RIGHT_BOUND);
    points[1] = points[1].max(0.);
    points
}
