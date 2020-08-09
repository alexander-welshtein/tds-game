use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub x: isize,
    pub y: isize,
    pub speed: isize,
    pub operation: Operation,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            speed: 5,
            operation: Operation::default(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Instance {
    pub players: HashMap<usize, Player>
}

impl Instance {
    pub fn new() -> Self {
        Self {
            players: HashMap::new()
        }
    }
}