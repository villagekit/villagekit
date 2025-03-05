use bevy::prelude::*;

use villagekit_engine::{traits::*, Transform, *};

fn main() {
    App::new()
        .add_plugins(EnginePlugin)
        .add_systems(PostStartup, setup_model)
        .run();
}

#[derive(Clone)]
struct Chair {
    width: Length,
    depth: Length,
    height: Length,
}

impl Assembly for Chair {
    fn products(&self) -> Vec<Product> {
        let Self {
            width,
            depth,
            height,
        } = *self;

        vec![
            Beam::z(Length(num!(0)), Length(num!(0)), (Length(num!(0)), height)),
            Beam::z(
                width - Length(num!(1)),
                Length(num!(0)),
                (Length(num!(0)), height),
            ),
            Beam::z(
                Length(num!(0)),
                depth - Length(num!(1)),
                (Length(num!(0)), height),
            ),
            Beam::z(
                width - Length(num!(1)),
                depth - Length(num!(1)),
                (Length(num!(0)), height),
            ),
            Beam::x(
                (Length(num!(0)), width),
                Length(num!(1)),
                height - Length(num!(2)),
            ),
            Beam::x(
                (Length(num!(0)), width),
                depth - Length(num!(2)),
                height - Length(num!(2)),
            ),
            Beam::y(
                Length(num!(1)),
                (Length(num!(0)), depth),
                height - Length(num!(1)),
            ),
            Beam::y(
                width - Length(num!(2)),
                (Length(num!(0)), depth),
                height - Length(num!(1)),
            ),
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
            beam = beam.rotate(Y_AXIS, HALF_ROTATION, None)
        }

        beam.translate(x.0, y, z)
    }

    fn y(x: Length, y: (Length, Length), z: Length) -> Product {
        let length = (y.0 - y.1).abs();

        let mut beam = Self { length }.place();

        beam = beam.rotate(Z_AXIS, QUARTER_ROTATION, None);

        if y.0 > y.1 {
            beam = beam.rotate(X_AXIS, HALF_ROTATION, None)
        }

        beam.translate(x, y.0, z)
    }

    fn z(x: Length, y: Length, z: (Length, Length)) -> Product {
        let length = (z.0 - z.1).abs();

        let mut beam = Self { length }.place();

        beam = beam.rotate(Y_AXIS, -QUARTER_ROTATION, None);

        if z.0 > z.1 {
            beam = beam.rotate(X_AXIS, HALF_ROTATION, None)
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
                "green".into(),
                RenderableMaterial::Color {
                    color: RenderableColor::Hsla {
                        hue: num!(120),
                        saturation: num!(1),
                        lightness: num!(0.5),
                        alpha: num!(1),
                    },
                },
            )
            .insert_instance(RenderableInstance {
                mesh: Some("cube".into()),
                material: Some("green".into()),
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
    let test = Chair {
        width: Length(num!(10)),
        depth: Length(num!(10)),
        height: Length(num!(10)),
    };
    spawn_product(sandbox.single(), test.place(), &mut commands);
}
