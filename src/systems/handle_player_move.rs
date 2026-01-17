use bevy::prelude::*;
use bevy_spritefusion::prelude::*;

use crate::{
    components::{grid_loc::GridLoc, player_state::PlayerState, wall::LevelWalls},
    events::player_movement::PlayerMovement,
};

pub fn handle_player_move(
    mut move_msgs: MessageReader<PlayerMovement>,
    mut player_state: ResMut<PlayerState>,
    level_walls: Res<LevelWalls>,
    tile_attrs: Query<(&TilePos, &TileAttributes)>,
) {
    for move_msg in move_msgs.read() {
        if !player_state.is_in_motion() {
            let to = move_msg.to;

            // if move is legal
            if !level_walls.in_wall(&to)
                && !((vertical(&move_msg.from.into(), tile_attrs.into_iter())
                    || vertical(&move_msg.to.into(), tile_attrs.into_iter()))
                    && going_vert(&move_msg.to, &move_msg.from))
            {
                debug!("moving player to {to:?}");
                player_state.moving_to = Some(to);
            } else {
                debug!("ilegal move");
            }
        }
    }
}

fn going_vert(to: &GridLoc, from: &GridLoc) -> bool {
    trace!("going from {from:?} -> {to:?}");

    to.y != from.y
}

pub fn vertical(
    location: &TilePos,
    attributes: impl Iterator<Item = (&TilePos, &TileAttributes)>,
) -> bool {
    for (pos, attrs) in attributes {
        if pos == location {
            if let Some(walk_behind) = attrs.get_bool("vertical")
                && !walk_behind
            {
                return true;
            }
        }
    }

    false
}
