use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Message, Deserialize, Serialize)]
pub enum LevelEvent {
    Spawned { w: i32, h: i32 },
    Despawned,
}
