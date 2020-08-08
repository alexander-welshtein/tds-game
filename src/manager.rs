use crate::client_state::{ClientState, Operation};

pub fn change_client_state(operation: Operation, state: &mut ClientState) {
    state.operation.id = operation.id;
    state.operation.command = operation.command;

    match state.operation.command.as_str() {
        "MoveLeft" => state.player.x -= state.player.speed,
        "MoveRight" => state.player.x += state.player.speed,
        "MoveUp" => state.player.y -= state.player.speed,
        "MoveDown" => state.player.y += state.player.speed,
        _ => {}
    };
}