use std::collections::HashMap;

use bevy::{prelude::*, reflect::TypePath};
use leafwing_input_manager::prelude::*;

mod camera;

#[cfg(feature = "cheat_input_target")]
pub use camera::InputTarget;

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[derive(Actionlike, TypePath)]
pub enum Action {
    Forward,
    Backward,
    Left,
    Right,
    Jump,

    #[cfg(feature = "cheat")]
    ChangeInputTarget,
    #[cfg(feature = "cheat")]
    CameraUp,
    #[cfg(feature = "cheat")]
    CameraDown,
    #[cfg(feature = "cheat")]
    CameraMouse,

}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(super) struct Manager(pub HashMap<Vec<KeyCode>, Action>);

impl Manager {
    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let file = std::fs::read_to_string(path)?;

        let manager: Self = ron::from_str(&file).expect("Failed to parse input file");
        Ok(manager)
    }
}

impl From<Manager> for InputMap<Action> {
    fn from(manager: Manager) -> Self {
        let mut map = InputMap::default();

        for (keys, action) in manager.0 {
            map.insert_chord(keys, action);
        }

        #[cfg(feature = "cheat_input_target")]
        map.insert(DualAxis::mouse_motion(), Action::CameraMouse);

        map
    }
}

/// Return (forward, right) velocity, both ranges from -1 to 1
pub fn get_axis_input(action_state: &Res<ActionState<Action>>) -> (f32, f32) {
    let forward_pressed = action_state.pressed(Action::Forward);
    let backward_pressed = action_state.pressed(Action::Backward);

    // Front is +Z
    let mut velocity = 0f32;
    if forward_pressed {
        velocity += 1f32;
    }
    if backward_pressed {
        velocity -= 1f32;
    }

    let left_pressed = action_state.pressed(Action::Left);
    let right_pressed = action_state.pressed(Action::Right);

    // Left is +X
    let mut steering = 0f32;
    if left_pressed {
        steering += 1f32;
    }
    if right_pressed {
        steering -= 1f32;
    }

    (velocity, steering)
}

#[cfg(feature = "cheat_input_target")]
pub fn change_input_target(
    mut input_target: ResMut<camera::InputTarget>,
    action_state: Res<ActionState<Action>>,
) {
    if action_state.just_pressed(Action::ChangeInputTarget) {
        *input_target = match *input_target {
            InputTarget::Kart => InputTarget::Camera,
            InputTarget::Camera => InputTarget::Kart,
        };

        tracing::info!("Changing input target to {:?}", *input_target);
    }
}
