use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Transfer {
    pub player: Player
}

impl Transfer {
    pub fn new() -> Self {
        Self { player: Player::new() }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub x: isize,
    pub y: isize,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }
}