use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{HashSet, components::grid_loc::GridLoc};

#[derive(
    Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Component, Deserialize, Serialize,
)]
pub struct Wall;

#[derive(Default, Clone, Bundle)]
pub struct WallBundle {
    pub wall: Wall,
}

#[derive(Default, Clone, Resource)]
pub struct LevelWalls {
    pub wall_locations: HashSet<GridLoc>,
    pub level_width: i32,
    pub level_height: i32,
}

impl LevelWalls {
    pub fn in_wall(&self, grid_coords: &GridLoc) -> bool {
        grid_coords.x < 0
            || grid_coords.y < 0
            || grid_coords.x >= self.level_width
            || grid_coords.y >= self.level_height
            || self.wall_locations.contains(grid_coords)
    }
}
