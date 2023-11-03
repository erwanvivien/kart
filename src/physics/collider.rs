use bevy::{
    prelude::*,
    render::mesh::{Indices, VertexAttributeValues},
};
use bevy_rapier3d::prelude::Collider;

#[derive(Debug, Component)]
pub(super) enum ColliderFromMeshError {
    MissingPositions,
    MissingIndices,
    InvalidPositionsType(&'static str),
}

// From https://github.com/Defernus/bevy_gltf_collider/blob/a8ce443/src/mesh_collider.rs#L24-L60
pub(super) fn mesh_to_collider(mesh: Mesh) -> Result<Collider, ColliderFromMeshError> {
    let positions = mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .map_or(Err(ColliderFromMeshError::MissingPositions), Ok)?;

    let indices = mesh
        .indices()
        .map_or(Err(ColliderFromMeshError::MissingIndices), Ok)?;

    let positions = match positions {
        VertexAttributeValues::Float32x3(positions) => Ok(positions),
        v => Err(ColliderFromMeshError::InvalidPositionsType(
            v.enum_variant_name(),
        )),
    }?;

    let indices: Vec<u32> = match indices {
        Indices::U32(indices) => indices.clone(),
        Indices::U16(indices) => indices.iter().map(|&i| i as u32).collect(),
    };

    debug_assert!(indices.len() % 3 == 0);

    let triple_indices: Vec<[u32; 3]> = indices.chunks(3).map(|v| [v[0], v[1], v[2]]).collect();
    let vertices: Vec<Vec3> = positions.iter().map(|v| Vec3::from_array(*v)).collect();

    Ok(Collider::convex_decomposition(&vertices, &triple_indices))
}
