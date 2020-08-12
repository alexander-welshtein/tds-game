use serde::{Deserialize, Serialize};
use crate::transfer::Operation;

#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    x: isize,
    y: isize,
    speed: isize,
    operation: Operation,
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