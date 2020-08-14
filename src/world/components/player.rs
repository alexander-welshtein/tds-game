use rand::prelude::ThreadRng;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::state::Operation;
use crate::utils::as_json_string;

#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    #[serde(with = "as_json_string")]
    id: usize,

    x: isize,
    y: isize,
    speed: isize,

    #[serde(skip)]
    operations: Vec<Operation>
}

impl Default for Player {
    fn default() -> Self {

        let mut rng = ThreadRng::default();

        Self {
            id: rng.gen::<usize>(),
            x: 0,
            y: 0,
            speed: 5,
            operations: Vec::default()
        }
    }
}

impl Player {
    pub fn add_operation(&mut self, operation: Operation) {
        self.operations.push(operation)
    }

    pub fn handle_operations(&mut self) {
        while let Some(operation) = self.operations.pop().clone() {
            self.apply_operation(operation);
        }
    }

    fn apply_operation(&mut self, operation: Operation) {
        match operation.command.as_str() {
            "MoveLeft" => self.x -= self.speed,
            "MoveRight" => self.x += self.speed,
            "MoveUp" => self.y -= self.speed,
            "MoveDown" => self.y += self.speed,
            _ => {}
        };
    }
}