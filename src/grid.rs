use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    utils::HashMap,
};

use hexx::*;

use crate::colors::*;

/// World size of the hexagons (outer radius)
const HEX_SIZE: Vec2 = Vec2::splat(25.0);
const MAP_RADIUS: u32 = 10;

#[derive(Debug, Resource)]
struct HexGrid {
    pub entities: HashMap<Hex, Entity>,
    pub layout: HexLayout,
    pub hex_map: HexMap,
    pub default_mat: Handle<StandardMaterial>,
}

pub fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let layout = HexLayout {
        hex_size: HEX_SIZE,
        ..default()
    };
    let mesh = meshes.add(hexagonal_plane(&layout));
    let default_mat = materials.add(SURFACE0.into());
    let hex_map = HexMap::new(MAP_RADIUS);
    let entities = hex_map
        .all_coords()
        .map(|hex| {
            let pos = layout.hex_to_world_pos(hex);
            let entity = commands
                .spawn(MaterialMeshBundle {
                    mesh: mesh.clone(),
                    material: default_mat.clone(),
                    transform: Transform::from_xyz(pos.x, 0.0, pos.y),
                    ..default()
                })
                .id();
            (hex, entity)
        })
        .collect();

    commands.insert_resource(HexGrid {
        entities,
        layout,
        hex_map,
        default_mat,
    })
}

/// Compute a bevy mesh from the layout
fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = MeshInfo::hexagonal_plane(hex_layout, Hex::ZERO);
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices.to_vec());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals.to_vec());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs.to_vec());
    mesh.set_indices(Some(Indices::U16(mesh_info.indices)));
    mesh
}
