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
    should_move_left: bool,
    #[serde(skip)]
    should_move_right: bool,
    #[serde(skip)]
    should_move_up: bool,
    #[serde(skip)]
    should_move_down: bool,

    #[serde(skip)]
    operations: Vec<Operation>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            id: ThreadRng::default().gen::<usize>(),

            x: 0,
            y: 0,
            speed: 20,

            should_move_left: false,
            should_move_right: false,
            should_move_up: false,
            should_move_down: false,

            operations: Vec::default(),
        }
    }
}

impl Player {
    pub fn add_operation(&mut self, operation: Operation) {
        self.operations.push(operation)
    }

    pub fn update(&mut self) {
        while let Some(operation) = self.operations.pop() {
            match operation.command.as_str() {
                "KeyLeftDown" => self.should_move_left = true,
                "KeyLeftUp" => self.should_move_left = false,
                "KeyRightDown" => self.should_move_right = true,
                "KeyRightUp" => self.should_move_right = false,
                "KeyUpDown" => self.should_move_up = true,
                "KeyUpUp" => self.should_move_up = false,
                "KeyDownDown" => self.should_move_down = true,
                "KeyDownUp" => self.should_move_down = false,
                _ => {}
            };
        }

        if self.should_move_left {
            self.x -= self.speed
        }

        if self.should_move_right {
            self.x += self.speed
        }

        if self.should_move_up {
            self.y -= self.speed
        }

        if self.should_move_down {
            self.y += self.speed
        }
    }
}