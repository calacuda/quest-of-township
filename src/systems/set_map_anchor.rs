use bevy::prelude::*;
use bevy_ecs_tilemap::anchor::TilemapAnchor;

use crate::components::background_marker::BackgroundMarker;

pub fn set_map_anchor(mut anchor: Single<&mut TilemapAnchor, With<BackgroundMarker>>) {
    **anchor = TilemapAnchor::TopLeft;
    debug!("set map anchor to {:?}", **anchor);
}
