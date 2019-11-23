use crate::util::*;
use kiss3d::event::Key;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Controls {
    pub jump: Key,
    pub left: Key,
    pub right: Key,
    pub shoot: Key,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            jump: Key::W,
            left: Key::A,
            right: Key::D,
            shoot: Key::Space,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Player {
    pub speed_z: D,
    pub speed_x: D,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub controls: Controls,
    pub player: Player,
}
