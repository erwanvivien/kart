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

    app.init_resource::<ActionState<Action>>();
    app.insert_resource(input_map);

    #[cfg(feature = "cheat_input_target")]
    app.insert_resource(input::InputTarget::Kart);

    app.add_systems(Startup, setup);

    #[cfg(feature = "debug_input")]
    app.add_systems(Update, debug::input::report_pressed_actions);
    #[cfg(feature = "cheat_input_target")]
    app.add_systems(Update, input::change_input_target);
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
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                min_x: -0.5,
                max_x: 0.5,
                min_y: -0.5,
                max_y: 0f32,
                min_z: -1f32,
                max_z: 0.2f32,
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        kart::Speed::default(),
        kart::Kart::default(),
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
