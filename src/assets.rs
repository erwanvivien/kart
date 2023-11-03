use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AssetLoadingState {
    #[default]
    AssetLoading,
    Done,
}

#[derive(Resource, AssetCollection)]
pub struct KartAssets {
    #[asset(path = "karts/sedan.glb#Scene0")]
    pub sedan: Handle<Scene>,
    #[asset(path = "karts/sports.glb#Scene0")]
    pub sports: Handle<Scene>,
    #[asset(path = "karts/muscle.glb#Scene0")]
    pub muscle: Handle<Scene>,
    #[asset(path = "karts/monster_truck.glb#Scene0")]
    pub monster_truck: Handle<Scene>,
    #[asset(path = "karts/police.glb#Scene0")]
    pub police: Handle<Scene>,
}

#[derive(Resource, AssetCollection)]
pub struct TerrainAssets {
    #[asset(path = "terrains/map01.glb#Scene0")]
    pub map01: Handle<Scene>,
    #[asset(path = "terrains/map01.glb#Mesh0/Primitive0")]
    pub map01_mesh: Handle<Mesh>,
}
