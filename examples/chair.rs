use villagekit::prelude::*;

fn main() {
    let chair = Chair {
        width: qty!(10 m),
        depth: qty!(10 m),
        height: qty!(10 m),
    };
    setup_assembly(chair);
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
            Beam::z(qty!(0 m), qty!(0 m), (qty!(0 m), height)),
            Beam::z(width - qty!(1 m), qty!(0 m), (qty!(0 m), height)),
            Beam::z(qty!(0 m), depth - qty!(1 m), (qty!(0 m), height)),
            Beam::z(width - qty!(1 m), depth - qty!(1 m), (qty!(0 m), height)),
            Beam::x((qty!(0 m), width), qty!(1 m), height - qty!(2 m)),
            Beam::x((qty!(0 m), width), depth - qty!(2 m), height - qty!(2 m)),
            Beam::y(qty!(1 m), (qty!(0 m), depth), height - qty!(1 m)),
            Beam::y(width - qty!(2 m), (qty!(0 m), depth), height - qty!(1 m)),
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
            beam = beam.rotate(Y_AXIS, Rotations::HALF, None)
        }

        beam.translate(x.0, y, z)
    }

    fn y(x: Length, y: (Length, Length), z: Length) -> Product {
        let length = (y.0 - y.1).abs();

        let mut beam = Self { length }.place();

        beam = beam.rotate(Z_AXIS, Rotations::QUARTER, None);

        if y.0 > y.1 {
            beam = beam.rotate(X_AXIS, Rotations::HALF, None)
        }

        beam.translate(x, y.0, z)
    }

    fn z(x: Length, y: Length, z: (Length, Length)) -> Product {
        let length = (z.0 - z.1).abs();

        let mut beam = Self { length }.place();

        beam = beam.rotate(Y_AXIS, -Rotations::QUARTER, None);

        if z.0 > z.1 {
            beam = beam.rotate(X_AXIS, Rotations::HALF, None)
        }

        beam.translate(x, y, z.0)
    }
}

impl Stock for Beam {
    fn render(&self) -> Renderable {
        let grid_unit: Length = qty!(1 m);

        let mut r = Renderable::default();
        let cube = r.insert_mesh(
            "cube",
            Mesh::Cuboid {
                x_length: self.length,
                y_length: grid_unit,
                z_length: grid_unit,
            },
        );
        let wood = r.insert_material(
            "wood",
            Material {
                base_color_texture: Some(image!("./textures/wood.jpg")),
                ..Default::default()
            },
        );
        r.insert_instance(Instance {
            mesh: cube,
            material: wood,
            transform: Transform::default().translate(
                num!(0.5) * (self.length - grid_unit),
                Length::zero(),
                Length::zero(),
            ),
            children: vec![],
        });
        r
    }
}
