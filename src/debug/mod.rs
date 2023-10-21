#[cfg(feature = "debug_screen")]
#[cfg(any(
    feature = "debug_screen_fps",
    feature = "debug_screen_position",
    feature = "debug_screen_camera",
))]
pub mod screen;

#[cfg(feature = "debug_screen")]
#[cfg(not(any(
    feature = "debug_screen_fps",
    feature = "debug_screen_position",
    feature = "debug_screen_camera",
)))]
pub mod screen {
    use bevy::app::{App, Plugin};

    pub struct ScreenDebugPlugin;

    impl Plugin for ScreenDebugPlugin {
        fn build(&self, _app: &mut App) {}
    }
}

#[cfg(feature = "debug_input")]
pub mod input;
