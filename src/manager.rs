use crate::client_state::ClientState;

pub fn change_client_state(command: &str, state: &mut ClientState) {
    match command {
        "MoveLeft" => state.player.x -= state.player.speed,
        "MoveRight" => state.player.x += state.player.speed,
        "MoveUp" => state.player.y -= state.player.speed,
        "MoveDown" => state.player.y += state.player.speed,
        _ => {}
    };
}