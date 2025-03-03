use bevy::prelude::*;

use villagekit_engine::{Transform, *};

fn main() {
    App::new()
        .add_plugins(EnginePlugin)
        .add_systems(PostStartup, setup_model)
        .run();
}

#[derive(Clone)]
struct BundleOfSticks {}

impl Assembly for BundleOfSticks {
    fn products(&self) -> Vec<Product> {
        vec![
            (Stick {})
                .place()
                .translate(Length(num!(0)), Length(num!(0)), Length(num!(0))),
            (Stick {})
                .place()
                .translate(Length(num!(2)), Length(num!(2)), Length(num!(0))),
            (Stick {})
                .place()
                .translate(Length(num!(5)), Length(num!(5)), Length(num!(0))),
        ]
    }
}

#[derive(Clone)]
struct Stick {}

impl Stock for Stick {
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
    let test = BundleOfSticks {};
    spawn_product(sandbox.single(), test.place(), &mut commands);
}
