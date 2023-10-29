use bevy::prelude::*;

fn default_text_style() -> bevy::text::TextStyle {
    static mut DEFAULT_TEXT_STYLE: Option<bevy::text::TextStyle> = None;

    if unsafe { DEFAULT_TEXT_STYLE.as_ref() }.is_none() {
        tracing::info!("Creating default text style");

        let text_style = bevy::text::TextStyle {
            font_size: 32f32,
            ..Default::default()
        };
        unsafe { DEFAULT_TEXT_STYLE = Some(text_style) };
    }

    unsafe { DEFAULT_TEXT_STYLE.clone() }.unwrap()
}

#[cfg(feature = "debug_screen_fps")]
#[derive(Component)]
pub struct FpsDebug;

#[cfg(feature = "debug_screen_position")]
#[derive(Component)]
pub struct PositionDebug;

#[cfg(feature = "debug_screen_camera")]
#[derive(Component)]
pub struct CameraDebug;

#[cfg(feature = "debug_screen_speed")]
#[derive(Component)]
pub struct SpeedDebug;

pub struct ScreenDebugPlugin;

fn layout_text(mut commands: Commands) {
    let mut text_offset = 0f32;

    let mut get_offset_style = || {
        let offset = text_offset;
        text_offset += default_text_style().font_size;

        Style {
            position_type: PositionType::Absolute,
            top: Val::Px(offset),
            left: Val::Px(10f32),
            ..Default::default()
        }
    };

    #[cfg(feature = "debug_screen_fps")]
    {
        let text_bundle = TextBundle::from_sections([TextSection::new("", default_text_style())])
            .with_style(get_offset_style());

        commands.spawn((FpsDebug, text_bundle));
    }

    #[cfg(feature = "debug_screen_position")]
    {
        let text_bundle = TextBundle::from_sections([TextSection::new("", default_text_style())])
            .with_style(get_offset_style());

        commands.spawn((PositionDebug, text_bundle));
    }

    #[cfg(feature = "debug_screen_camera")]
    {
        let text_bundle = TextBundle::from_sections([TextSection::new("", default_text_style())])
            .with_style(get_offset_style());

        commands.spawn((CameraDebug, text_bundle));
    }

    #[cfg(feature = "debug_screen_speed")]
    {
        let text_bundle = TextBundle::from_sections([TextSection::new("", default_text_style())])
            .with_style(get_offset_style());

        commands.spawn((SpeedDebug, text_bundle));
    }
}

impl Plugin for ScreenDebugPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "debug_screen_fps")]
        app.add_plugins(FrameTimeDiagnosticsPlugin);

        app.add_systems(Startup, layout_text);

        app.add_systems(
            Update,
            (
                #[cfg(feature = "debug_screen_fps")]
                fps_debug,
                #[cfg(feature = "debug_screen_position")]
                position_debug,
                #[cfg(feature = "debug_screen_camera")]
                camera_debug,
                #[cfg(feature = "debug_screen_speed")]
                speed_debug,
            )
                .run_if(in_state(crate::AssetLoadingState::Done)),
        );
    }
}

#[cfg(feature = "debug_screen_fps")]
use bevy::diagnostic::{Diagnostic, DiagnosticsStore, FrameTimeDiagnosticsPlugin};

#[cfg(feature = "debug_screen_fps")]
pub fn fps_debug(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsDebug>>) {
    let mut text_query = query.single_mut();
    let text = text_query.as_mut();

    if let Some(Some(value)) = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .map(Diagnostic::smoothed)
    {
        text.sections[0].value = format!("FPS: {value:.2}");
    }
}

#[cfg(feature = "debug_screen_position")]
pub fn position_debug(
    mut query: Query<&mut Text, With<PositionDebug>>,
    player: Query<(&Transform, &crate::kart::Kart), Without<PositionDebug>>,
) {
    let (player_transform, _) = player.single();

    let mut text_query = query.single_mut();
    let text = text_query.as_mut();

    let position = player_transform.translation.to_array();
    let rotation = player_transform.rotation.to_euler(EulerRot::YXZ);
    text.sections[0].value = format!("Player: Position {position:.2?} | Rotation {rotation:.2?}");
}

#[cfg(feature = "debug_screen_camera")]
pub fn camera_debug(
    mut query: Query<&mut Text, With<CameraDebug>>,
    camera: Query<(&Transform, &crate::camera::MainCamera), Without<CameraDebug>>,
) {
    let (camera_transform, _) = camera.single();

    let mut text_query = query.single_mut();
    let text = text_query.as_mut();

    let position = camera_transform.translation.to_array();
    let rotation = camera_transform.rotation.to_euler(EulerRot::YXZ);
    text.sections[0].value = format!("Camera: Position {position:.2?} | Rotation {rotation:.2?}");
}

#[cfg(feature = "debug_screen_speed")]
pub fn speed_debug(
    mut query: Query<&mut Text, With<SpeedDebug>>,
    player: Query<(&crate::kart::Speed, &crate::kart::Kart), Without<SpeedDebug>>,
) {
    let (player_speed, _) = player.single();

    let mut text_query = query.single_mut();
    let text = text_query.as_mut();

    let current_speed = player_speed.forward_speed;
    let acceleration = player_speed.acceleration;
    text.sections[0].value =
        format!("Kart Speed: {current_speed:.2}m/s | Acceleration {acceleration:.2}m/s2");
}
