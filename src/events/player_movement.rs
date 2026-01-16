use bevy::prelude::*;

use crate::components::grid_loc::GridLoc;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Message)]
pub struct PlayerMovement {
    pub from: GridLoc,
    pub to: GridLoc,
}
