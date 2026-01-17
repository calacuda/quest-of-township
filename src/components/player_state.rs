use bevy::prelude::*;

use crate::components::grid_loc::GridLoc;

#[derive(Clone, Copy, PartialEq, Resource)]
pub struct PlayerState {
    pub loc: GridLoc,
    pub last_loc: GridLoc,
    pub distance_from_loc: f32,
    pub moving_to: Option<GridLoc>,
}

impl PlayerState {
    pub fn is_in_motion(&self) -> bool {
        self.moving_to.is_some()
    }
}
