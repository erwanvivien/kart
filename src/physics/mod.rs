use std::sync::atomic::AtomicUsize;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{assets::AssetLoadingState, physics::collider::mesh_to_collider};

mod collider;

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct MeshCollider;

pub struct GltfColliderPlugin;

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone)]
#[derive(States)]
enum ColliderState {
    #[default]
    Waiting,
    Done,
}

const MAX_TICKS: usize = 200;
/// Only allow `MAX_TICKS` ticks to process colliders, after that we can assume
/// there was no colliders to process
static CONVERT_COLLIDERS_TICK_COUNT: AtomicUsize = AtomicUsize::new(0);

/// We don't want to try to process colliders for ever, so we bound this
/// to `MAX_TICKS` ticks and after AssetLoadingState::Done is reached
fn valid_state(
    asset_loading_state: Res<State<AssetLoadingState>>,
    processing_state: Res<State<ColliderState>>,
) -> bool {
    use std::sync::atomic::Ordering;
    let tick_count = CONVERT_COLLIDERS_TICK_COUNT.fetch_add(1, Ordering::Relaxed);

    tick_count < MAX_TICKS
        && in_state(AssetLoadingState::Done).run((), (asset_loading_state,))
        && !in_state(ColliderState::Done).run((), (processing_state,))
}

impl Plugin for GltfColliderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MeshCollider>();
        app.add_state::<ColliderState>();

        app.add_systems(Update, find_all_colliders.run_if(valid_state));
    }
}

fn find_all_colliders(
    mut meshes: ResMut<Assets<Mesh>>,
    mut cmds: Commands,
    mut query: Query<(Entity, &Transform, Option<&Children>), With<MeshCollider>>,
    child_mesh_query: Query<&Handle<Mesh>>,
    parent_query: Query<&Parent>,
    mut state: ResMut<NextState<ColliderState>>,
) {
    let mut found_collider = false;
    for (entity, entity_transform, children) in query.iter_mut() {
        let children = children.expect("MeshCollider component without children");

        found_collider = true;
        let mut found_mesh = false;
        for child in children.iter() {
            if let Ok(mesh_handle) = child_mesh_query.get(*child) {
                found_mesh = true;

                let mesh = meshes.remove(mesh_handle).expect("Mesh not found");
                let collider = mesh_to_collider(mesh).unwrap();

                // We find the top level entity to add the RigidBody component
                let mut top_level_entity = entity;
                while let Ok(parent) = parent_query.get(top_level_entity) {
                    top_level_entity = parent.get();
                }

                // Scale is not applied
                // let collider_transform = Transform::default();

                // Seems to apply rotation twice ?
                let collider_transform = *entity_transform;

                // Same as Transform::default() (???)
                // let collider_transform = Transform::from_scale(entity_transform.scale);
                dbg!(collider_transform);

                cmds.entity(entity).with_children(|parent| {
                    parent.spawn((collider, collider_transform));
                });
                // Only the top level entity has the RigidBody component
                cmds.entity(top_level_entity)
                    .insert((RigidBody::Dynamic, Ccd::enabled()));

                break;
            }
        }

        assert!(found_mesh, "MeshCollider component without mesh");
    }

    if found_collider {
        state.set(ColliderState::Done);
    }
}
