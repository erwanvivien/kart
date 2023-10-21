use std::collections::HashMap;

use bevy::{prelude::*, reflect::TypePath};
use leafwing_input_manager::prelude::*;

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[derive(Actionlike, TypePath)]
pub enum Action {
    Forward,
    Backward,
    Left,
    Right,
    Jump,
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

        map
    }
}
