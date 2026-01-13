use bevy::prelude::*;

use crate::components::player_state::PlayerState;

pub fn player_in_motion(player_state: Res<PlayerState>) -> bool {
    player_state.is_in_motion()
}
