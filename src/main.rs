use bevy::diagnostic::DiagnosticsStore;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, platform::collections::HashMap};
use bevy_asset_loader::prelude::*;
use iyes_progress::{Progress, ProgressPlugin, ProgressReturningSystem, ProgressTracker};

const H_IN_TILES: usize = 19;
const W_IN_TILES: usize = 35;
const H_MAX: usize = H_IN_TILES - 1;
const W_MAX: usize = W_IN_TILES - 2;
const TILE_PIXLES: usize = 16;

#[derive(AssetCollection, Resource)]
struct OverWorldTiles {
    // if the sheet would have padding, you could set that with `padding_x` and `padding_y`.
    // if there would be space between the top left corner of the sheet and the first sprite, you could configure that with `offset_x` and `offset_y`
    // A texture atlas layout does not have a path as no asset file will be loaded for the layout
    // #[asset(texture_atlas_layout(
    //     tile_size_x = 16,
    //     tile_size_y = 16,
    //     padding_x = 1,
    //     padding_y = 1
    // ))]
    // #[asset(
    //     path = "../assets/tile-sets/Spritesheet/roguelikeSheet_transparent.png",
    //     collection(mapped, typed)
    // )]
    #[asset(path = "../assets/tile-sets/single-png/", collection(mapped, typed), image(sampler(filter = nearest)))]
    floor: HashMap<AssetFileStem, Handle<Image>>,
    // // female_adventurer_layout: Handle<TextureAtlasLayout>,
    // // you can configure the sampler for the sprite sheet image
    // #[asset(image(sampler(filter = nearest)))]
    // #[asset(path = "images/female_adventurer_sheet.png")]
    // female_adventurer: Handle<Image>,
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
            DefaultPlugins.set(ImagePlugin::default_nearest()), // .set(WindowPlugin {
            //     primary_window: Some(Window {
            //         resolution: WindowResolution::new(
            //             (W_IN_TILES * TILE_PIXLES) as f32,
            //             (H_IN_TILES * TILE_PIXLES) as f32,
            //         ),
            //         ..default()
            //     }),
            //     ..default()
            // })
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
        // .add_systems(Update, set_scale.run_if(on_event::<WindowResized>))
        .add_systems(OnEnter(MyStates::AssetLoading), render_description)
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
        // Camera {
        //     viewport: Some(Viewport {
        //         physical_position: UVec2::ZERO,
        //         physical_size: uvec2(w, h),
        //         depth: 0.0..1.,
        //     }),
        //     ..default()
        // },
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: h as f32,
                // viewport_width: w as f32,
            },
            // viewport_origin: Vec2::new(0.0, 1.0),
            // viewport_origin: Vec2::new(0.0, 0.0),
            ..OrthographicProjection::default_2d()
        }),
    ));
}

// fn set_scale(mut window: Single<&mut Window, With<PrimaryWindow>>) {
//     let target_w = (W_IN_TILES * TILE_PIXLES) as f32;
//     let target_h = (H_IN_TILES * TILE_PIXLES) as f32;
//
//     let current_h = window.height();
//     let scale = target_h / current_h;
//
//     window.resolution.set_scale_factor_override
// }

fn tile_transform(x: f32, y: f32) -> Transform {
    // (0, 0)
    // Transform::from_xyz(
    //         ((W_IN_TILES - 1) * TILE_PIXLES) as f32 * -0.5,
    //         ((H_IN_TILES - 1) * TILE_PIXLES) as f32 * 0.5,
    //         0.,
    //     ),
    let x_zero = (W_MAX * TILE_PIXLES) as f32 * -0.5;
    let y_zero = (H_MAX * TILE_PIXLES) as f32 * 0.5;

    Transform::from_xyz(
        x_zero + x * TILE_PIXLES as f32,
        y_zero - y * TILE_PIXLES as f32,
        0.,
    )
}

fn draw_atlas(mut commands: Commands, over_world: Res<OverWorldTiles>) {
    // info!("over_world floor tiles size: {}", over_world.floor.len());
    // info!("over_world key_one: {:?}", over_world.floor.keys().nth(0));
    // draw the original image (whole sprite sheet)
    commands.spawn((
        Sprite::from_image(
            over_world
                .floor
                .get("tile1438")
                .expect("Can access audio asset with file name")
                .to_owned(),
        ),
        // Transform::from_xyz(0., 0., 0.),
        // Transform::from_xyz(
        //     ((W_IN_TILES - 1) * TILE_PIXLES) as f32 * -0.5,
        //     ((H_IN_TILES - 1) * TILE_PIXLES) as f32 * 0.5,
        //     0.,
        // ),
        tile_transform(0., 0.),
    ));
    commands.spawn((
        Sprite::from_image(
            over_world
                .floor
                .get("tile000")
                .expect("Can access audio asset with file name")
                .to_owned(),
        ),
        // Transform::from_xyz(TILE_PIXLES as f32, TILE_PIXLES as f32 * -1., 0.),
        // Transform::from_xyz(
        //     ((W_IN_TILES - 1) * TILE_PIXLES) as f32 * 0.5,
        //     ((H_IN_TILES - 1) * TILE_PIXLES) as f32 * -0.5,
        //     0.,
        // ),
        tile_transform(W_MAX as f32, H_MAX as f32),
    ));

    commands.spawn((
        Sprite::from_image(
            over_world
                .floor
                .get("tile000")
                .expect("Can access audio asset with file name")
                .to_owned(),
        ),
        // Transform::from_xyz(TILE_PIXLES as f32, TILE_PIXLES as f32 * -1., 0.),
        // Transform::from_xyz(
        //     (((W_IN_TILES - 1) / 2) * TILE_PIXLES) as f32 * 0.5,
        //     (((H_IN_TILES - 1) / 2) * TILE_PIXLES) as f32 * -0.5,
        //     0.,
        // ),
        tile_transform((W_MAX / 2) as f32, (H_MAX / 2) as f32),
    ));

    commands.spawn((
        Sprite::from_image(
            over_world
                .sprites
                .get("tile000")
                .expect("Can access audio asset with file name")
                .to_owned(),
        ),
        // Transform::from_xyz(TILE_PIXLES as f32, TILE_PIXLES as f32 * -1., 0.),
        // Transform::from_xyz(
        //     (((W_IN_TILES - 1) / 2) * TILE_PIXLES) as f32 * 0.5,
        //     (((H_IN_TILES - 1) / 2) * TILE_PIXLES) as f32 * -0.5,
        //     0.,
        // ),
        tile_transform((W_MAX / 2) as f32, (H_MAX / 2) as f32),
    ));

    // // draw animated sprite using the texture atlas layout
    // commands.spawn((
    //     Sprite::from_atlas_image(
    //     over_world.female_adventurer.clone(),
    //         TextureAtlas::from(my_assets.female_adventurer_layout.clone()),
    //     ),
    //     Transform::from_xyz(0., 150., 0.),
    //     AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    // ));
}

fn render_description(mut commands: Commands) {
    // commands.spawn(Camera2d);
    commands.spawn(Text::new(
        r#"
    See the console for progress output
    
    This window will close when progress completes..."#,
    ));
}

fn track_fake_long_task() -> Progress {
    // if time.elapsed_secs_f64() > DURATION_LONG_TASK_IN_SECS {
    //     info!("Long fake task is completed");
    //     true.into()
    // } else {
    //     false.into()
    // }
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
