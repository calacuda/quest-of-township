use bevy::prelude::*;

use crate::{
    TILE_PIXLES,
    components::{background_marker::BackgroundMarker, player_state::PlayerState},
    tile_transform,
};

pub fn move_pc(
    mut background: Single<&mut Transform, With<BackgroundMarker>>,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
) {
    if let Some(move_to) = player_state.moving_to {
        let from = player_state.loc;
        let speed = (TILE_PIXLES as f32 * 0.5) / Vec2::from(from).distance(move_to.into());
        let step = speed * time.delta_secs();
        player_state.distance_from_loc += step;
        let set_to = Vec2::from(from).lerp(move_to.into(), player_state.distance_from_loc);
        let set_to = tile_transform(set_to[0], set_to[1]);
        background.translation = set_to.translation;

        if player_state.distance_from_loc >= 1.0 {
            **background = tile_transform(move_to.0 as f32, move_to.1 as f32);
            player_state.moving_to = None;
            player_state.loc = move_to;
            player_state.distance_from_loc = 0.0;
        }
    }
}
