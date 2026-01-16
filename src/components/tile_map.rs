use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::grid_loc::GridLoc;

pub type TileLayer = Vec<(GridLoc, Tile)>;
pub type MapID = usize;

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Component, Deserialize, Serialize)]
pub struct TileMap {
    pub layers: Vec<TileLayer>,
    pub palayer_layer: usize,
}

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum Tile {
    Display {
        tile_set: Option<usize>,
        tile: usize,
    },
    Wall {
        from: GridLoc,
        to: GridLoc,
    },
    Warp {
        to: MapID,
    },
    // TODO: add enviormental
}
