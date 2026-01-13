use bevy::prelude::*;

use crate::{components::player_state::PlayerState, events::player_movement::PlayerMovement};

pub fn controls_player_move(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    // background: Single<&PlayerLoc, With<BackgroundMarker>>,
    player_state: Res<PlayerState>,
    mut move_msg: MessageWriter<PlayerMovement>,
) {
    // let mut event = None;
    // let from = background.clone();
    let from = player_state.loc;
    let mut to = player_state.loc;

    if keyboard_input.pressed(KeyCode::KeyW) {
        trace!("move up");
        to = to - (0, 1);
        // event = Some(PlayerMovement { from, to })
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        trace!("move down");
        to = to + (0, 1);
        // event = Some(PlayerMovement { from, to })
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        trace!("move left");
        to = to - (1, 0);
        // event = Some(PlayerMovement { from, to })
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        trace!("move right");
        to = to + (1, 0);
        // event = Some(PlayerMovement { from, to })
    }

    // *background.1 = tile_transform(background.0.0, background.0.1);
    if to != from {
        move_msg.write(PlayerMovement { from, to });
    }
    // else {
    //     trace!("no move");
    // }
}
