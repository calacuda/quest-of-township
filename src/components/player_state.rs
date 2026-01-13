use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::player_loc::PlayerLoc;

#[derive(Clone, Copy, PartialEq, PartialOrd, Resource, Deserialize, Serialize)]
pub struct PlayerState {
    pub loc: PlayerLoc,
    pub distance_from_loc: f32,
    pub moving_to: Option<PlayerLoc>,
}

impl PlayerState {
    pub fn is_in_motion(&self) -> bool {
        self.moving_to.is_some()
    }
}
