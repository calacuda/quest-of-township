use bevy::prelude::*;
use bevy_spritefusion::prelude::*;

use crate::events::level_event::LevelEvent;

pub fn level_loaded(mut evs: MessageWriter<LevelEvent>, size: Single<&SpriteFusionMapMarker>) {
    let map = &size.map;

    evs.write(LevelEvent::Spawned {
        w: map.map_width as i32,
        h: map.map_height as i32,
    });

    warn!("level-Loaded");
}
