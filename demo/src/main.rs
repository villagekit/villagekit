use std::marker::PhantomData;

use bevy::prelude::*;

use villagekit_engine::{traits::*, Transform, *};

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
            Beam::x(
                (Length(num!(0)), Length(num!(10))),
                Length(num!(0)),
                Length(num!(0)),
            ),
            Beam::y(
                Length(num!(0)),
                (Length(num!(0)), Length(num!(10))),
                Length(num!(0)),
            )
            .translate(Length(num!(1)), Length(num!(0)), Length(num!(1))),
            Beam::z(
                Length(num!(0)),
                Length(num!(0)),
                (Length(num!(0)), Length(num!(10))),
            )
            .translate(Length(num!(1)), Length(num!(1)), Length(num!(0))),
        ]
    }
}

// TODO Add support for variants with custom grid units.
#[derive(Clone)]
struct Beam {
    length: Length,
}

impl Beam {
    fn x(x: (Length, Length), y: Length, z: Length) -> Product {
        let length = (x.0 - x.1).abs();

        let mut beam = Self { length }.place();

        if x.0 > x.1 {
            beam = beam.mirror_x()
        }

        beam.translate(x.0, y, z)
    }

    fn y(x: Length, y: (Length, Length), z: Length) -> Product {
        let length = (y.0 - y.1).abs();

        let mut beam = Self { length }.place();

        beam = beam.change_basis(Matrix3 {
            x_axis: Y_AXIS,
            y_axis: X_AXIS,
            z_axis: Z_AXIS,
        });

        if y.0 > y.1 {
            beam = beam.mirror_y()
        }

        beam.translate(x, y.0, z)
    }

    fn z(x: Length, y: Length, z: (Length, Length)) -> Product {
        let length = (z.0 - z.1).abs();

        let mut beam = Self { length }.place();

        beam = beam.change_basis(Matrix3 {
            x_axis: Z_AXIS,
            y_axis: Y_AXIS,
            z_axis: X_AXIS,
        });

        if z.0 > z.1 {
            beam = beam.mirror_z()
        }

        beam.translate(x, y, z.0)
    }
}

impl Stock for Beam {
    fn render(&self) -> Renderable {
        let grid_unit: Length = Meter(num!(1)).into();

        Renderable::default()
            .insert_mesh(
                "cube".into(),
                RenderableMesh::Cuboid {
                    x_length: self.length,
                    y_length: grid_unit,
                    z_length: grid_unit,
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
                transform: Some(Transform::default().translate(
                    num!(0.5) * (self.length - grid_unit),
                    Length::zero(),
                    Length::zero(),
                )),
                children: Some(vec![]),
            })
    }
}

fn setup_model(mut commands: Commands, sandbox: Query<Entity, With<Sandbox>>) {
    let test = BundleOfSticks {};
    spawn_product(sandbox.single(), test.place(), &mut commands);
}
