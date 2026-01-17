use bevy::prelude::*;

use crate::{components::map_size::MapSize, events::level_event::LevelEvent};

pub fn mk_size_res(mut level_messages: MessageReader<LevelEvent>, mut map_size: ResMut<MapSize>) {
    for msg in level_messages.read() {
        if let LevelEvent::Spawned { w, h } = msg {
            map_size.w = *w;
            map_size.h = *h;
        }
    }
}
