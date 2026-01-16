use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::{
    TILE_PIXLE_H, TILE_PIXLE_W,
    components::{
        grid_loc::GridLoc,
        wall::{LevelWalls, Wall},
    },
    events::level_event::LevelEvent,
};

pub fn cache_wall_locations(
    mut level_walls: ResMut<LevelWalls>,
    mut level_messages: MessageReader<LevelEvent>,
    walls: Query<&GridLoc, With<Wall>>,
) -> Result {
    for level_event in level_messages.read() {
        if *level_event == LevelEvent::Spawned {
            // let ldtk_project = ldtk_project_assets
            //     .get(ldtk_project_entities.single()?)
            //     .expect("LdtkProject should be loaded when level is spawned");
            // let level = ldtk_project
            //     .get_raw_level_by_iid(level_iid.get())
            //     .expect("spawned level should exist in project");

            let wall_locations = walls.iter().copied().collect();

            let new_level_walls = LevelWalls {
                wall_locations,
                // level_width: level.px_wid / TILE_PIXLE_W as i32,
                // level_height: level.px_hei / TILE_PIXLE_H as i32,
            };

            *level_walls = new_level_walls;
        }
    }
    Ok(())
}
