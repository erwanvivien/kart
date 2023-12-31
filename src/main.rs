use assets::{AssetLoadingState, KartAssets, TerrainAssets};
use bevy::prelude::*;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_gltf_components::ComponentsFromGltfPlugin;
use bevy_rapier3d::prelude::*;
use kart::{BackWheels, FrontWheels};
use leafwing_input_manager::prelude::*;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[cfg(all(not(debug_assertions), feature = "dynamic_linking"))]
compile_error!("Dynamic linking should not supported for release builds");

mod assets;
mod camera;
mod debug;
mod input;
mod kart;
mod physics;

use crate::input::Action;

#[cfg(feature = "cheat")]
const INPUT_FILE: &str = "input_cheat.manager";
#[cfg(not(feature = "cheat"))]
const INPUT_FILE: &str = "input.manager";

fn main() {
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        .with_env_filter("kart")
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let input_config = input::Manager::from_file(INPUT_FILE).expect("Failed to load input config");
    let input_map: InputMap<Action> = input_config.into();

    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    #[cfg(feature = "debug_screen")]
    app.add_plugins(debug::screen::ScreenDebugPlugin);
    app.add_plugins(InputManagerPlugin::<Action>::default());
    app.add_plugins(ComponentsFromGltfPlugin);
    #[cfg(feature = "debug_axis")]
    app.add_plugins(bevy_debug_grid::DebugGridPlugin::with_floor_grid());
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
    app.add_plugins(physics::GltfColliderPlugin);
    #[cfg(feature = "debug_rapier")]
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.add_state::<AssetLoadingState>();
    app.add_loading_state(
        LoadingState::new(AssetLoadingState::AssetLoading)
            .continue_to_state(AssetLoadingState::Done),
    );
    app.add_collection_to_loading_state::<_, KartAssets>(AssetLoadingState::AssetLoading);
    app.add_collection_to_loading_state::<_, TerrainAssets>(AssetLoadingState::AssetLoading);

    app.add_systems(OnEnter(AssetLoadingState::Done), || {
        tracing::info!("Assets loaded!");
    });

    app.init_resource::<ActionState<Action>>();
    app.insert_resource(input_map);

    #[cfg(feature = "cheat_input_target")]
    app.insert_resource(input::InputTarget::Kart);

    // Needed for the `ComponentsFromGltfPlugin`
    app.register_type::<FrontWheels>();
    app.register_type::<BackWheels>();

    app.add_systems(OnEnter(AssetLoadingState::Done), setup);

    app.add_systems(
        Update,
        (
            #[cfg(feature = "cheat_kart_change")]
            input::change_kart,
            #[cfg(feature = "cheat_input_target")]
            input::change_input_target,
            #[cfg(feature = "debug_input")]
            debug::input::report_pressed_actions,
            // Normal systems
            kart::update_kart_position,
            kart::update_front_wheels,
            camera::sync_camera_to_player.after(kart::update_kart_position),
        )
            .run_if(in_state(AssetLoadingState::Done)),
    );

    // Change InputMap clash strategy
    app.insert_resource(ClashStrategy::PrioritizeLongest);

    app.run();
}

/// set up a simple 3D scene
fn setup(
    terrain_assets: Res<assets::TerrainAssets>,
    kart_assets: Res<assets::KartAssets>,
    mut commands: Commands,
) {
    // plane

    commands.spawn((
        SceneBundle {
            scene: terrain_assets.map01.clone(),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(10f32, 0.5f32, 10f32),
        Transform::from_xyz(0.0, -0.5, 0.0),
    ));

    // player
    let kart_variant = kart::KartVariants::default();
    commands.spawn((
        SceneBundle {
            scene: kart_variant.get_handle(&kart_assets),
            transform: Transform::from_xyz(0.0, 0.3f32, 0.0),
            ..default()
        },
        kart::Speed::default(),
        kart::Kart::default(),
        kart_variant,
    ));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn((Camera3dBundle::default(), camera::MainCamera::default()));
}
