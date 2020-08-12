use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::world::components::player::Player;

#[derive(Serialize, Deserialize)]
pub struct Instance {
    pub players: HashMap<usize, Player>
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            players: HashMap::new()
        }
    }
}