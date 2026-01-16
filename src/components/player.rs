use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Deserialize, Serialize, Component,
)]
pub struct Player;

#[derive(Default, Clone, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
}
