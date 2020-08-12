use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::world::components::player::Player;

#[derive(Serialize, Deserialize)]
pub struct Instance {
    players: HashMap<usize, Player>
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            players: HashMap::new()
        }
    }
}

impl Instance {
    pub fn add_player(&mut self, session_id: usize) {
        self.players.insert(session_id, Player::default());
    }

    pub fn remove_player(&mut self, session_id: usize) {
        self.players.remove(&session_id);
    }

    pub fn get_player(&mut self, session_id: usize) -> Option<&mut Player> {
        self.players.get_mut(&session_id)
    }

    pub fn get_players(&self) -> &HashMap<usize, Player> {
        &self.players
    }
}