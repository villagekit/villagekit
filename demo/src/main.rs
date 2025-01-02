use std::collections::BTreeMap;

use bevy::prelude::*;

use uuid::Uuid;
use villagekit_engine::{spawn_part, EnginePlugin, PartInstance, PartMaterial, PartMesh, PartSpec};

fn main() {
    App::new()
        .add_plugins(EnginePlugin)
        .add_systems(Startup, setup_model)
        .run();
}

fn setup_model(mut commands: Commands) {
    let cube_id = Uuid::new_v4();
    let part_meshes = BTreeMap::from([(
        cube_id,
        PartMesh::Cuboid {
            x_length: 1.0,
            y_length: 1.0,
            z_length: 10.0,
        },
    )]);
    let white_id = Uuid::new_v4();
    let part_materials = BTreeMap::from([(
        white_id,
        PartMaterial::Color {
            color: Color::WHITE,
        },
    )]);
    let part_instances = vec![PartInstance {
        mesh: Some(cube_id),
        material: Some(white_id),
        transform: Some(Transform::from_xyz(0.0, 0.5, 0.0)),
        children: None,
    }];
    let part_spec = PartSpec {
        meshes: part_meshes,
        materials: part_materials,
        instances: part_instances,
    };

    spawn_part(part_spec, commands.reborrow());
}
