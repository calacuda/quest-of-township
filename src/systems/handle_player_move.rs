use bevy::prelude::*;

use crate::{components::player_state::PlayerState, events::player_movement::PlayerMovement};

pub fn handle_player_move(
    mut move_msgs: MessageReader<PlayerMovement>,
    mut player_state: ResMut<PlayerState>,
) {
    for move_msg in move_msgs.read() {
        // if is_legal_move(move_msg) {
        if !player_state.is_in_motion() {
            let to = move_msg.to;
            debug!("moving player to {to:?}");
            player_state.moving_to = Some(to);
        }
        // }
    }
}
