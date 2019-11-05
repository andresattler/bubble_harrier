use super::player::Player;
use crate::util::*;

#[derive(Debug, Default)]
pub struct Game {
    player: Player,
}

impl Game {
    pub const LEFT_BOUND: D = -500.;
    pub const RIGHT_BOUND: D = 500.;
    pub const LOWER_BOUND: D = 0.;
    pub const UPPER_BOUND: D = 700.;
}
