use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::input::Action;

#[allow(clippy::needless_pass_by_value)]
pub fn report_pressed_actions(action_state: Res<ActionState<Action>>) {
    for action in Action::variants() {
        #[cfg(feature = "cheat_input_target")]
        if matches!(action, Action::CameraMouse) {
            continue;
        }

        if action_state.just_pressed(action) {
            tracing::info!("{action:?} pressed");
        } else if action_state.just_released(action) {
            let held_for = action_state.previous_duration(action);
            tracing::info!("{action:?} released after {held_for:.2?}");
        }
    }
}
