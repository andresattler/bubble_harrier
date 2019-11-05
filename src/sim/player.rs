use super::game::Game;
use super::*;
use crate::util::*;

#[derive(Debug)]
pub struct Player {
    transform: Transform,
}

impl Player {
    const STD_VEL: [D; 3] = [10., 10., 30.];
}

impl Default for Player {
    fn default() -> Self {
        let transform = Transform {
            pos: [0., 0., 0.].into(),
            vel: [0., 0., Self::STD_VEL[2]].into(),
            scale: 1.,
        };
        Player { transform }
    }
}

impl Commandable for Player {
    fn handle(&mut self, c: Command) -> SimResult {
        match c {
            Command::Move(dir) => {
                let scale: Vector = [dir[0], dir[1], 1.].into();
                let vel = self.transform.vel;
                self.transform.vel = [
                    (vel[0] + scale[0] * Self::STD_VEL[0]),
                    (vel[1] + scale[1] * Self::STD_VEL[1]),
                    vel[2],
                ]
                .into()
            }
        }
        Ok(())
    }
}

impl Updateable for Player {
    fn update(&mut self, d: Duration) -> SimResult {
        self.transform.update(d)
    }
}
