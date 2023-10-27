use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod camera;
mod debug;
mod input;
mod kart;

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
    #[cfg(feature = "debug_axis")]
    app.add_plugins(bevy_debug_grid::DebugGridPlugin::with_floor_grid());

    app.init_resource::<ActionState<Action>>();
    app.insert_resource(input_map);

    #[cfg(feature = "cheat_input_target")]
    app.insert_resource(input::InputTarget::Kart);

    app.add_systems(Startup, setup);

    #[cfg(feature = "debug_input")]
    app.add_systems(Update, debug::input::report_pressed_actions);
    #[cfg(feature = "cheat_input_target")]
    app.add_systems(Update, input::change_input_target);
    #[cfg(feature = "cheat_kart_change")]
    app.add_systems(Update, input::change_kart);
    app.add_systems(Update, kart::update_kart_position);
    app.add_systems(
        Update,
        camera::sync_camera_to_player.after(kart::update_kart_position),
    );

    // Change InputMap clash strategy
    app.insert_resource(ClashStrategy::PrioritizeLongest);

    app.run();
}

/// set up a simple 3D scene
fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(500.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // player
    let kart_variant = kart::KartVariants::default();
    commands.spawn((
        SceneBundle {
            scene: asset_server.load(&kart_variant.asset_path()),
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
