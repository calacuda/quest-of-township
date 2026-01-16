use bevy::{
    camera::ScalingMode,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_asset_loader::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use iyes_progress::{Progress, ProgressPlugin, ProgressReturningSystem, ProgressTracker};
use rustc_hash::FxHashSet;

use crate::{
    components::{
        background_marker::BackgroundMarker,
        grid_loc::GridLoc,
        player_state::PlayerState,
        wall::{LevelWalls, WallBundle},
    },
    events::player_movement::PlayerMovement,
    systems::{
        cache_wall_locations::cache_wall_locations, controls_player_move::controls_player_move,
        handle_player_move::handle_player_move, move_pc::move_pc,
        player_in_motion::player_in_motion,
    },
};

pub mod components;
pub mod events;
pub mod plugins;
pub mod systems;

pub type HashSet<T> = FxHashSet<T>;

pub const H_IN_TILES: usize = 19;
pub const W_IN_TILES: usize = 35;
pub const H_MAX: usize = H_IN_TILES - 1;
pub const W_MAX: usize = W_IN_TILES - 2;
pub const TILE_PIXLES: usize = 16;
pub const TILE_PIXLE_W: usize = 16;
pub const TILE_PIXLE_H: usize = 16;

#[derive(AssetCollection, Resource)]
struct SpriteTiles {
    // #[asset(path = "../assets/tile-sets/single-png/", collection(mapped, typed), image(sampler(filter = nearest)))]
    // floor: HashMap<AssetFileStem, Handle<Image>>,
    // #[asset(path = "../assets/sprites/single-png/", collection(mapped, typed), image(sampler(filter = nearest)))]
    // sprites: HashMap<AssetFileStem, Handle<Image>>,
    #[asset(texture_atlas_layout(
        tile_size_x = 16,
        tile_size_y = 16,
        padding_x = 1,
        padding_y = 1,
        rows = 12,
        columns = 54
    ))]
    pub sprite_sheet: Handle<TextureAtlasLayout>,
    #[asset(
        path = "sprites/Spritesheet/roguelikeChar_transparent.png",
        image(sampler(filter = nearest))
    )]
    pub sprites: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
struct WorldTiles {
    #[asset(texture_atlas_layout(
        tile_size_x = 16,
        tile_size_y = 16,
        padding_x = 1,
        padding_y = 1,
        rows = 31,
        columns = 57
    ))]
    pub sprite_sheet: Handle<TextureAtlasLayout>,
    #[asset(
        path = "../assets/tile-sets/Spritesheet/roguelikeSheet_transparent.png",
        image(sampler(filter = nearest))
    )]
    pub tiles: Handle<Image>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum AssetLoading {
    #[default]
    Loading,
    Loaded,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    // Set the default log level for everything
                    level: Level::INFO,
                    // Or use a filter string for fine-grained control
                    filter: format!("info,{}=trace", env!("CARGO_PKG_NAME").replace("-", "_")),
                    ..default()
                }),
            FrameTimeDiagnosticsPlugin::default(),
            ProgressPlugin::<AssetLoading>::new()
                .with_state_transition(AssetLoading::Loading, AssetLoading::Loaded),
        ))
        .add_plugins(TilemapPlugin)
        .init_state::<AssetLoading>()
        .add_loading_state(
            LoadingState::new(AssetLoading::Loading)
                .continue_to_state(AssetLoading::Loaded)
                .load_collection::<WorldTiles>()
                .load_collection::<SpriteTiles>(),
        )
        .add_message::<PlayerMovement>()
        .init_resource::<LevelWalls>()
        .add_systems(Startup, setup)
        .add_systems(
            OnEnter(AssetLoading::Loaded),
            || -> Progress { true.into() }.track_progress::<AssetLoading>(),
        )
        .add_systems(
            Update,
            (
                print_progress,
                track_fake_long_task.track_progress::<AssetLoading>(),
            )
                .chain()
                .run_if(in_state(AssetLoading::Loading))
                .after(LoadingStateSet(AssetLoading::Loading)),
        )
        .add_systems(
            Update,
            (
                (
                    controls_player_move.run_if(not(player_in_motion)),
                    handle_player_move.run_if(on_message::<PlayerMovement>),
                    move_pc,
                )
                    .chain(),
                cache_wall_locations,
            )
                .run_if(not(in_state(AssetLoading::Loading))),
        )
        .add_systems(
            OnEnter(AssetLoading::Loaded),
            (spawn_town, spawn_pc)
                .chain()
                .after(LoadingStateSet(AssetLoading::Loading)),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // let w = (W_IN_TILES * TILE_PIXLES) as u32;
    let h = (H_IN_TILES * TILE_PIXLES) as u32;

    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: h as f32,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    // TODO: spawn town
}

fn spawn_town(mut commands: Commands, asset_server: Res<AssetServer>, tile_sheet: Res<WorldTiles>) {
    let map_size = TilemapSize { x: 32, y: 32 };

    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn_empty().id();

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage = TileStorage::empty(map_size);

    // Spawn the elements of the tilemap.
    // Alternatively, you can use helpers::filling::fill_tilemap.
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        // texture: TilemapTexture::Single(texture_handle),
        texture: TilemapTexture::Single(tile_sheet.tiles.clone()),
        tile_size,
        anchor: TilemapAnchor::TopLeft,
        spacing: TilemapSpacing { x: 0., y: 0. },
        ..Default::default()
    });
}

pub fn tile_transform(x: f32, y: f32) -> Transform {
    let x_zero = (W_MAX * TILE_PIXLES) as f32 * -0.5 - TILE_PIXLES as f32 * 0.5;
    let y_zero = (H_MAX * TILE_PIXLES) as f32 * 0.5 + TILE_PIXLES as f32 * 0.5;

    Transform::from_xyz(
        x_zero + (-x + (W_MAX / 2) as f32) * TILE_PIXLES as f32,
        y_zero + (y - (H_IN_TILES / 2) as f32) * TILE_PIXLES as f32,
        0.,
    )
}

pub fn character_tile_transform(x: f32, y: f32) -> Transform {
    let x_zero = (W_MAX * TILE_PIXLES) as f32 * -0.5;
    let y_zero = (H_MAX * TILE_PIXLES) as f32 * 0.5;

    Transform::from_xyz(
        x_zero + x * TILE_PIXLES as f32,
        y_zero - y * TILE_PIXLES as f32,
        0.,
    )
}

fn spawn_pc(mut commands: Commands, sprite_sheet: Res<SpriteTiles>) {
    // draw character sprite
    // debug!("spawning player character");
    warn!("spawning player character not impl'ed yet");

    for index in [1, 65, 338, 220, 362, 424, 347, 193] {
        let texture_handle = sprite_sheet.sprites.clone();
        let layout_handle = sprite_sheet.sprite_sheet.clone();
        let mut atlas = TextureAtlas::from(layout_handle);
        atlas.index = index;

        commands.spawn((
            Sprite::from_atlas_image(texture_handle, atlas),
            character_tile_transform((W_MAX / 2) as f32, (H_MAX / 2) as f32),
        ));
    }
}

fn track_fake_long_task() -> Progress {
    false.into()
}

fn print_progress(
    progress: Res<ProgressTracker<AssetLoading>>,
    diagnostics: Res<DiagnosticsStore>,
    mut last_done: Local<u32>,
) {
    let progress = progress.get_global_progress();
    if progress.done > *last_done {
        *last_done = progress.done;
        info!(
            "[Frame {}] Changed progress: {:?}",
            diagnostics
                .get(&FrameTimeDiagnosticsPlugin::FRAME_COUNT)
                .map(|diagnostic| diagnostic.value().unwrap_or(0.))
                .unwrap_or(0.),
            progress
        );
    }
}
