use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod input;

use crate::input::Action;

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

    let input_config =
        input::Manager::from_file("input.manager").expect("Failed to load input config");
    let input_map: InputMap<Action> = input_config.into();

    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugins(InputManagerPlugin::<Action>::default());

    app.init_resource::<ActionState<Action>>();
    app.insert_resource(input_map);

    app.add_systems(Startup, setup);

    #[cfg(feature = "debug_input")]
    app.add_systems(Update, input::debug::report_pressed_actions);

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
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
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
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
