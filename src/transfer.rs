use serde::{Deserialize, Serialize};
use crate::world::components::player::Player;

#[derive(Serialize, Deserialize)]
pub struct Transfer {
    pub player: Player
}

impl Default for Transfer {
    fn default() -> Self {
        Self {
            player: Player::default()
        }
    }
}

#[derive(Serialize, Deserialize)]
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