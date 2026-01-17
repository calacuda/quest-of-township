use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Resource, Deserialize, Serialize,
)]
pub struct MapSize {
    pub w: i32,
    pub h: i32,
}
