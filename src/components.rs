use crate::util::*;
use specs::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum ObjectKind {
    Player,
    Obstacle,
}

impl Component for ObjectKind {
    type Storage = DenseVecStorage<Self>;
}

// Velocity conversion to easily reset the velocity of a thing to something standardized
impl Into<Vel> for ObjectKind {
    fn into(self) -> Vel {
        match self {
            ObjectKind::Player => Vel([0., 0., 20.]),
            ObjectKind::Obstacle => Vel([0., 0., 0.]),
        }
    }
}

pub struct Vel([D; 3]);

impl From<[D; 3]> for Vel {
    fn from(i: [D; 3]) -> Self {
        Vel(i)
    }
}

impl Component for Vel {
    type Storage = DenseVecStorage<Self>;
}
