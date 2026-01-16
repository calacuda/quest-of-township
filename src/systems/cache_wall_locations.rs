use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    TILE_PIXLE_H, TILE_PIXLE_W,
    components::wall::{LevelWalls, Wall},
};

pub fn cache_wall_locations(
    mut level_walls: ResMut<LevelWalls>,
    mut level_messages: MessageReader<LevelEvent>,
    walls: Query<&GridCoords, With<Wall>>,
    ldtk_project_entities: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) -> Result {
    for level_event in level_messages.read() {
        if let LevelEvent::Spawned(level_iid) = level_event {
            let ldtk_project = ldtk_project_assets
                .get(ldtk_project_entities.single()?)
                .expect("LdtkProject should be loaded when level is spawned");
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("spawned level should exist in project");

            let wall_locations = walls.iter().copied().collect();

            let new_level_walls = LevelWalls {
                wall_locations,
                level_width: level.px_wid / TILE_PIXLE_W as i32,
                level_height: level.px_hei / TILE_PIXLE_H as i32,
            };

            *level_walls = new_level_walls;
        }
    }
    Ok(())
}
