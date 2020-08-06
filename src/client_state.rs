use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ClientState {
    pub player: Player
}

impl ClientState {
    pub fn new() -> Self {
        Self { player: Player::new() }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub x: isize,
    pub y: isize,
    pub speed: isize
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            speed: 5
        }
    }
}