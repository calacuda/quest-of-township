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
                && !(walk_behind(&move_msg.from.into(), tile_attrs.into_iter())
                    && going_down(&move_msg.to, &move_msg.from))
            {
                debug!("moving player to {to:?}");
                player_state.moving_to = Some(to);
            } else {
                debug!("ilegal move");
            }
        }
    }
}

fn going_down(to: &GridLoc, from: &GridLoc) -> bool {
    trace!("going from {from:?} -> {to:?}");

    to.y < from.y
}

pub fn walk_behind(
    location: &TilePos,
    attributes: impl Iterator<Item = (&TilePos, &TileAttributes)>,
) -> bool {
    for (pos, attrs) in attributes {
        if pos == location {
            if let Some(walk_behind) = attrs.get_bool("walk-behind")
                && walk_behind
            {
                return true;
            }
        }
    }

    false
}
