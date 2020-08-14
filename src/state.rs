use serde::{Deserialize, Serialize};
use crate::world::components::player::Player;

#[derive(Serialize, Deserialize)]
pub struct State {
    pub player: Player,
    pub players: Vec<Player>
}

impl Default for State {
    fn default() -> Self {
        Self {
            player: Player::default(),
            players: Vec::default()
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Operation {
    pub id: isize,
    pub command: String,
}

impl Default for Operation {
    fn default() -> Self {
        Self {
            id: 0,
            command: String::from("Unknown"),
        }
    }
}