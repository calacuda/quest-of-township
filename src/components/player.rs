use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Deserialize, Serialize, Component,
)]
pub struct Player;
