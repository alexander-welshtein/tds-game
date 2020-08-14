use rand::prelude::ThreadRng;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::transfer::Operation;
use crate::utils::as_json_string;

#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    #[serde(with = "as_json_string")]
    id: usize,

    x: isize,
    y: isize,
    speed: isize,
    operation: Operation
}

impl Default for Player {
    fn default() -> Self {

        let mut rng = ThreadRng::default();

        Self {
            id: rng.gen::<usize>(),
            x: 0,
            y: 0,
            speed: 5,
            operation: Operation::default()
        }
    }
}

impl Player {
    pub fn apply_operation(&mut self, operation: Operation) {
        match operation.command.as_str() {
            "MoveLeft" => self.x -= self.speed,
            "MoveRight" => self.x += self.speed,
            "MoveUp" => self.y -= self.speed,
            "MoveDown" => self.y += self.speed,
            _ => {}
        };
        self.operation = operation;
    }
}