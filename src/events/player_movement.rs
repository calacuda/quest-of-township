use bevy::prelude::*;

use crate::components::player_loc::PlayerLoc;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Message)]
pub struct PlayerMovement {
    pub from: PlayerLoc,
    pub to: PlayerLoc,
}
