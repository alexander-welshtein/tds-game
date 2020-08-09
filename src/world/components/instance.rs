use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::world::components::player::Player;

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