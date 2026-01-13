use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::player_loc::PlayerLoc;

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Message, Deserialize, Serialize)]
pub struct PlayerMovement {
    pub from: PlayerLoc,
    pub to: PlayerLoc,
}
