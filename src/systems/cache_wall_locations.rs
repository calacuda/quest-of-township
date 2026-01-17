use bevy::prelude::*;

use crate::{
    HashSet,
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
        if let LevelEvent::Spawned { w, h } = *level_event {
            // let ldtk_project = ldtk_project_assets
            //     .get(ldtk_project_entities.single()?)
            //     .expect("LdtkProject should be loaded when level is spawned");
            // let level = ldtk_project
            //     .get_raw_level_by_iid(level_iid.get())
            //     .expect("spawned level should exist in project");

            let wall_locations: HashSet<_> = walls.iter().copied().collect();
            debug!("{} walls found", wall_locations.len());

            let new_level_walls = LevelWalls {
                wall_locations,
                level_width: w,
                level_height: h,
            };

            *level_walls = new_level_walls;
        }
    }
    Ok(())
}
