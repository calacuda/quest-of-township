use bevy::{
    camera::ScalingMode,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    platform::collections::HashMap,
    prelude::*,
};
use bevy_asset_loader::prelude::*;
use iyes_progress::{Progress, ProgressPlugin, ProgressReturningSystem, ProgressTracker};

const H_IN_TILES: usize = 19;
const W_IN_TILES: usize = 35;
const H_MAX: usize = H_IN_TILES - 1;
const W_MAX: usize = W_IN_TILES - 2;
const TILE_PIXLES: usize = 16;
const TILE_PIXLE_W: usize = 16;
const TILE_PIXLE_H: usize = 16;

#[derive(AssetCollection, Resource)]
struct OverWorldTiles {
    #[asset(path = "../assets/tile-sets/single-png/", collection(mapped, typed), image(sampler(filter = nearest)))]
    floor: HashMap<AssetFileStem, Handle<Image>>,
    #[asset(path = "../assets/sprites/single-png/", collection(mapped, typed), image(sampler(filter = nearest)))]
    sprites: HashMap<AssetFileStem, Handle<Image>>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum MyStates {
    #[default]
    AssetLoading,
    Next,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            ProgressPlugin::<MyStates>::new()
                .with_state_transition(MyStates::AssetLoading, MyStates::Next),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .init_state::<MyStates>()
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
                .continue_to_state(MyStates::Next)
                .load_collection::<OverWorldTiles>(),
        )
        .add_systems(Startup, setup)
        // .add_systems(OnEnter(MyStates::AssetLoading), render_description)
        .add_systems(
            OnEnter(MyStates::Next),
            || -> Progress { true.into() }.track_progress::<MyStates>(),
        )
        .add_systems(
            Update,
            (
                print_progress,
                track_fake_long_task.track_progress::<MyStates>(),
            )
                .chain()
                .run_if(in_state(MyStates::AssetLoading))
                .after(LoadingStateSet(MyStates::AssetLoading)),
        )
        .add_systems(
            OnEnter(MyStates::Next),
            draw_atlas.after(LoadingStateSet(MyStates::AssetLoading)),
        )
        .run();
}

fn setup(mut commands: Commands) {
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
}

pub fn tile_transform(x: f32, y: f32) -> Transform {
    let x_zero = (W_MAX * TILE_PIXLES) as f32 * -0.5;
    let y_zero = (H_MAX * TILE_PIXLES) as f32 * 0.5;

    Transform::from_xyz(
        x_zero + x * TILE_PIXLES as f32,
        y_zero - y * TILE_PIXLES as f32,
        0.,
    )
}

fn draw_atlas(mut commands: Commands, over_world: Res<OverWorldTiles>) {
    // draw the original image (whole sprite sheet)
    // top left tile
    commands.spawn((
        Sprite::from_image(
            over_world
                .floor
                .get("tile1438")
                .expect("Can access tile asset with file name")
                .to_owned(),
        ),
        tile_transform(0., 0.),
    ));

    // bottom right tile
    commands.spawn((
        Sprite::from_image(
            over_world
                .floor
                .get("tile000")
                .expect("Can access tile asset with file name")
                .to_owned(),
        ),
        tile_transform(W_MAX as f32, H_MAX as f32),
    ));

    // middle tiles
    commands.spawn((
        Sprite::from_image(
            over_world
                .floor
                .get("tile000")
                .expect("Can access tile asset with file name")
                .to_owned(),
        ),
        tile_transform((W_MAX / 2) as f32, (H_MAX / 2) as f32),
    ));

    // draw character sprite
    commands.spawn((
        Sprite::from_image(
            over_world
                .sprites
                .get("tile000")
                .expect("Can access tile asset with file name")
                .to_owned(),
        ),
        tile_transform((W_MAX / 2) as f32, (H_MAX / 2) as f32),
    ));
}

fn track_fake_long_task() -> Progress {
    false.into()
}

fn print_progress(
    progress: Res<ProgressTracker<MyStates>>,
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
