use bevy::prelude::*;

use crate::kart;

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

pub fn sync_camera_to_player(
    time: Res<Time>,
    player: Query<(&Transform, &kart::Speed), With<kart::Kart>>,
    mut camera: Query<(&mut Transform, &MainCamera), Without<kart::Kart>>,
) {
    let (player_transform, _player_speed) = player.single();
    let (mut camera_transform, camera) = camera.single_mut();
    let camera_transform = camera_transform.as_mut();

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
