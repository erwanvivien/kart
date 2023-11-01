use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    assets::KartAssets,
    input::{get_axis_input, Action},
};

#[derive(Debug, Component)]
pub struct Speed {
    /// Acceleration in meters per second squared
    pub acceleration: f32,
    /// Current speed in meters per second
    pub forward_speed: f32,
}

impl Default for Speed {
    fn default() -> Self {
        Self {
            acceleration: 0f32,
            forward_speed: 0f32,
        }
    }
}

#[allow(unused)]
#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum KartVariants {
    #[default]
    Sedan,
    Sports,
    Muscle,
    MonsterTruck,
    Police,
}

impl KartVariants {
    pub fn get_handle(&self, assets: &KartAssets) -> Handle<Scene> {
        match *self {
            KartVariants::Sedan => assets.sedan.clone(),
            KartVariants::Sports => assets.sports.clone(),
            KartVariants::Muscle => assets.muscle.clone(),
            KartVariants::MonsterTruck => assets.monster_truck.clone(),
            KartVariants::Police => assets.police.clone(),
        }
    }
}

#[cfg(feature = "cheat_input_target")]
impl KartVariants {
    pub fn next(&self) -> Self {
        match *self {
            KartVariants::Sedan => KartVariants::Sports,
            KartVariants::Sports => KartVariants::Muscle,
            KartVariants::Muscle => KartVariants::MonsterTruck,
            KartVariants::MonsterTruck => KartVariants::Police,
            KartVariants::Police => KartVariants::Sedan,
        }
    }
}

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct FrontWheels;

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct BackWheels;

#[derive(Debug, Component)]
pub struct Kart {
    /// Maximum speed in meters per second
    pub max_speed: f32,
    /// Minimum speed in meters per second
    pub min_speed: f32,
    /// Max steering angle in radians
    pub max_steering_angle: f32,
    /// Distance between the front and back wheels
    pub wheel_distance: f32,
}

impl Default for Kart {
    fn default() -> Self {
        Self {
            max_speed: 10f32,
            min_speed: -5f32,
            max_steering_angle: 30f32.to_radians(),
            wheel_distance: 2f32,
        }
    }
}

pub fn update_kart_position(
    #[cfg(feature = "cheat_input_target")] input_target: Res<crate::input::InputTarget>,
    time: Res<Time>,
    action_state: Res<ActionState<Action>>,
    mut query: Query<(&mut Transform, &mut Speed, &Kart)>,
) {
    #[cfg(feature = "cheat_input_target")]
    if *input_target != crate::input::InputTarget::Kart {
        return;
    }

    // Basic algorithm from http://engineeringdotnet.blogspot.com/2010/04/simple-2d-car-physics-in-games.html
    let (mut transform, mut speed, kart) = query.single_mut();

    let (input_velocity, input_steering) = get_axis_input(&action_state);
    if input_velocity == 0f32 && input_steering == 0f32 {
        return;
    }

    let velocity = input_velocity * kart.max_speed;
    let steering_angle = input_steering * kart.max_steering_angle;

    let forward_offset = Vec3::new(0f32, 0f32, velocity * time.delta_seconds());
    let wheel_distance = Vec3::new(0f32, 0f32, kart.wheel_distance / 2f32);

    // Compute the current position of the front and rear wheels
    let rear_wheel = transform.translation - transform.rotation * wheel_distance;
    let front_wheel = transform.translation + transform.rotation * wheel_distance;

    // Compute the new position of the front and rear wheels
    // front_wheet needs to be rotated by the steering angle
    let rear_wheel = rear_wheel + transform.rotation * forward_offset;
    let front_wheel =
        front_wheel + (transform.rotation * Quat::from_rotation_y(steering_angle)) * forward_offset;

    let new_position = (rear_wheel + front_wheel) / 2f32;

    let new_direction = (front_wheel - rear_wheel).normalize();
    let new_rotation = Quat::from_rotation_arc(Vec3::Z, new_direction);

    transform.translation = new_position;
    transform.rotation = new_rotation;
}

pub fn update_front_wheels(
    #[cfg(feature = "cheat_input_target")] input_target: Res<crate::input::InputTarget>,
    action_state: Res<ActionState<Action>>,
    mut query: Query<&mut Transform, With<FrontWheels>>,
    kart_query: Query<&Kart>,
) {
    #[cfg(feature = "cheat_input_target")]
    if *input_target != crate::input::InputTarget::Kart {
        return;
    }

    let (_, input_steering) = get_axis_input(&action_state);

    let kart = kart_query.single();
    let steering_angle = input_steering * kart.max_steering_angle;

    for mut transform in query.iter_mut() {
        transform.rotation = Quat::from_rotation_y(steering_angle);
    }
}
