use std::collections::BTreeMap;

use bevy::prelude::*;

use villagekit_engine::*;

fn main() {
    App::new()
        .add_plugins(EnginePlugin)
        .add_systems(Startup, setup_model)
        .run();
}

fn setup_model(mut commands: Commands) {
    let meshes = BTreeMap::from([(
        "cube".into(),
        RenderableMesh::Cuboid {
            x_length: 1.0.into(),
            y_length: 1.0.into(),
            z_length: 10.0.into(),
        },
    )]);
    let materials = BTreeMap::from([(
        "white".into(),
        RenderableMaterial::Color(RenderableColor::Hsla {
            hue: 0.0.into(),
            saturation: 1.0.into(),
            lightness: 0.5.into(),
            alpha: 0.5.into(),
        }),
    )]);
    let instances = vec![RenderableInstance {
        mesh: Some("cube".into()),
        material: Some("white".into()),
        transform: Some(Transform::from_xyz(0.0, 0.5, 0.0)),
        children: None,
    }];
    let renderable = Renderable {
        meshes,
        materials,
        instances,
    };

    spawn_renderable(renderable, commands.reborrow());
}
