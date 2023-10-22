#[cfg(feature = "cheat_input_target")]
#[derive(Debug, bevy::prelude::Resource, PartialEq, Eq, Clone, Copy, Hash)]
pub enum InputTarget {
    Kart,
    Camera,
}
