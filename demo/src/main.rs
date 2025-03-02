use bevy::prelude::*;

use serde_json::{from_str, from_value};
use villagekit_engine::{Transform, *};

fn main() {
    App::new()
        .add_plugins(EnginePlugin)
        .add_systems(PostStartup, setup_model)
        .run();
}

struct Test {}

impl Stock for Test {
    fn render(&self) -> Renderable {
        Renderable::default()
            .insert_mesh(
                "cube".into(),
                RenderableMesh::Cuboid {
                    x_length: Meter(num!(1)).into(),
                    y_length: Meter(num!(1)).into(),
                    z_length: Meter(num!(10)).into(),
                },
            )
            .insert_material(
                "red".into(),
                RenderableMaterial::Color {
                    color: RenderableColor::Hsla {
                        hue: num!(0),
                        saturation: num!(1),
                        lightness: num!(0.5),
                        alpha: num!(1),
                    },
                },
            )
            .insert_instance(RenderableInstance {
                mesh: Some("cube".into()),
                material: Some("red".into()),
                transform: Some(Transform::default()),
                children: Some(vec![]),
            })
    }
}

fn setup_model(mut commands: Commands, sandbox: Query<Entity, With<Sandbox>>) {
    let test = Test {};
    let renderable = test.render();
    spawn_renderable(sandbox.single(), renderable, commands.reborrow());
}
