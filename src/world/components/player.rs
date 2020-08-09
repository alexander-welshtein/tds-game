use serde::{Deserialize, Serialize};
use crate::transfer::Operation;

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

