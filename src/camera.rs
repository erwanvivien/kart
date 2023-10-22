use bevy::prelude::*;

use crate::kart;

#[cfg(feature = "cheat_input_target")]
use crate::Action;
#[cfg(feature = "cheat_input_target")]
use leafwing_input_manager::prelude::*;

#[derive(Component)]
pub struct MainCamera {
    /// The rigidity of the camera's movement
    ///
    /// 0 means the camera will follow exactly the player's position
    /// 1 means it'll take some time to catch up, it can be more
    pub smoothness: f32,
    /// The point the camera will look at, relative to the player's position
    ///
    /// `[0, 0, 0]` means it'll look at the player's position
    pub look_at: Vec3,
}

const CAMERA_ARM: Vec3 = Vec3::new(0f32, 1f32, -4f32);

impl Default for MainCamera {
    fn default() -> Self {
        Self {
            smoothness: 0.1,
            look_at: Vec3::Y * 1.5f32,
        }
    }
}

#[cfg(feature = "cheat_input_target")]
fn move_camera(
    time: Res<Time>,
    action_state: Res<ActionState<Action>>,
    camera_transform: &mut Transform,
) {
    use crate::input::get_axis_input;
    const VELOCITY_COEFFICIENT: f32 = 5f32;

    let (velocity, steering) = get_axis_input(&action_state);
    let up_down = {
        let up_pressed = action_state.pressed(Action::CameraUp);
        let down_pressed = action_state.pressed(Action::CameraDown);

        let mut up_down = 0f32;
        if up_pressed {
            up_down += 1f32;
        }
        if down_pressed {
            up_down -= 1f32;
        }

        up_down
    };

    let velocity = velocity * VELOCITY_COEFFICIENT * time.delta_seconds();
    let steering = steering * VELOCITY_COEFFICIENT * time.delta_seconds();
    let up_down = up_down * VELOCITY_COEFFICIENT * time.delta_seconds();

    camera_transform.translation += camera_transform.forward() * velocity
        + camera_transform.up() * up_down
        + camera_transform.left() * steering;

    if let Some(camera_pan_vector) = action_state.axis_pair(Action::CameraMouse) {
        const MOUSE_SENSITIVITY: f32 = 0.01f32;

        let (yaw, pitch, _roll) = camera_transform.rotation.to_euler(EulerRot::YXZ);
        let (delta_x, delta_y) = (camera_pan_vector.x(), camera_pan_vector.y());

        let yaw = -delta_x * MOUSE_SENSITIVITY + yaw;
        let pitch = -delta_y * MOUSE_SENSITIVITY + pitch;

        use std::f32::{consts::FRAC_PI_2, EPSILON};
        const MAX_PITCH: f32 = FRAC_PI_2 - EPSILON;
        let pitch = pitch.clamp(-MAX_PITCH, MAX_PITCH);

        camera_transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0f32);
    }
}

pub fn sync_camera_to_player(
    #[cfg(feature = "cheat_input_target")] input_target: Res<crate::input::InputTarget>,
    #[cfg(feature = "cheat_input_target")] action_state: Res<ActionState<Action>>,
    time: Res<Time>,
    player: Query<(&Transform, &kart::Speed), With<kart::Kart>>,
    mut camera: Query<(&mut Transform, &MainCamera), Without<kart::Kart>>,
) {
    let (player_transform, _player_speed) = player.single();
    let (mut camera_transform, camera) = camera.single_mut();
    let camera_transform = camera_transform.as_mut();

    #[cfg(feature = "cheat_input_target")]
    if *input_target == crate::input::InputTarget::Camera {
        return move_camera(time, action_state, camera_transform);
    }

    // From https://github.com/h3r2tic/dolly/blob/73501b8cc047065637290d8ccd0f5ede705abcb4/src/util.rs#L37
    // An ad-hoc multiplier to make default smoothness parameters
    // produce good-looking results.
    const SMOOTHNESS_MULT: f32 = 8.0;

    // Calculate the exponential blending based on frame time
    let smoothness = camera.smoothness.max(1e-5);
    let interpolation_time = 1.0 - (-SMOOTHNESS_MULT * time.delta_seconds() / smoothness).exp();

    // Update the camera position to be behind the player

    let target = player_transform.translation + player_transform.rotation * CAMERA_ARM;

    camera_transform.translation = camera_transform
        .translation
        .lerp(target, interpolation_time);

    // Update the camera rotation and add some rotation based on the kart's speed

    camera_transform.look_at(player_transform.translation + camera.look_at, Vec3::Y);

    // TODO: Add rotation based on kart acceleration
    // let pitch_rotation = Quat::from_rotation_x(player_speed.acceleration * 1f32.to_radians());
    // camera_transform.rotation *= pitch_rotation;
}
