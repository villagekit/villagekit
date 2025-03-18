use villagekit::prelude::*;

fn main() {
    let chair = Chair {
        width: num!(10),
        depth: num!(10),
        height: num!(10),
    };
    setup_assembly(chair);
}

#[derive(Clone)]
struct Chair {
    width: Number,
    depth: Number,
    height: Number,
}

impl Assembly for Chair {
    fn products(&self) -> Vec<Product> {
        let Self {
            width,
            depth,
            height,
        } = *self;

        vec![
            GridBeam::z(num!(0), num!(0), (num!(0), height)),
            GridBeam::z(width - num!(1), num!(0), (num!(0), height)),
            GridBeam::z(num!(0), depth - num!(1), (num!(0), height)),
            GridBeam::z(width - num!(1), depth - num!(1), (num!(0), height)),
            GridBeam::x((num!(0), width), num!(1), height - num!(2)),
            GridBeam::x((num!(0), width), depth - num!(2), height - num!(2)),
            GridBeam::y(num!(1), (num!(0), depth), height - num!(1)),
            GridBeam::y(width - num!(2), (num!(0), depth), height - num!(1)),
        ]
    }
}

// TODO Add support for variants with custom grid units.
#[derive(Clone)]
struct GridBeam {
    length: Number,
}

impl GridBeam {
    const GRID_UNIT: Length = qty!(40 mm);

    fn x(x: (Number, Number), y: Number, z: Number) -> Product {
        let length = (x.0 - x.1).abs();

        let mut beam = Self { length }.place();

        if x.0 > x.1 {
            beam = beam.rotate(Y_AXIS, Rotations::HALF, None)
        }

        beam.translate(
            x.0 * Self::GRID_UNIT,
            y * Self::GRID_UNIT,
            z * Self::GRID_UNIT,
        )
    }

    fn y(x: Number, y: (Number, Number), z: Number) -> Product {
        let length = (y.0 - y.1).abs();

        let mut beam = Self { length }.place();

        beam = beam.rotate(Z_AXIS, Rotations::QUARTER, None);

        if y.0 > y.1 {
            beam = beam.rotate(X_AXIS, Rotations::HALF, None)
        }

        beam.translate(
            x * Self::GRID_UNIT,
            y.0 * Self::GRID_UNIT,
            z * Self::GRID_UNIT,
        )
    }

    fn z(x: Number, y: Number, z: (Number, Number)) -> Product {
        let length = (z.0 - z.1).abs();

        let mut beam = Self { length }.place();

        beam = beam.rotate(Y_AXIS, -Rotations::QUARTER, None);

        if z.0 > z.1 {
            beam = beam.rotate(X_AXIS, Rotations::HALF, None)
        }

        beam.translate(
            x * Self::GRID_UNIT,
            y * Self::GRID_UNIT,
            z.0 * Self::GRID_UNIT,
        )
    }
}

impl Stock for GridBeam {
    fn render(&self) -> Renderable {
        let mut r = Renderable::default();
        let cube = r.insert_mesh(
            "cube",
            Mesh::Cuboid {
                x_length: self.length * Self::GRID_UNIT,
                y_length: Self::GRID_UNIT,
                z_length: Self::GRID_UNIT,
            },
        );
        let wood = r.insert_material(
            "wood",
            Material {
                base_color_texture: Some(ImageId::new("./textures/wood.jpg")),
                normal_map_texture: Some(ImageId::new("./textures/wood-normals.jpg")),
                metallic: num!(0),
                perceptual_roughness: num!(0.7),
                // Repeat texture every 2.5 meters
                uv_transform: Affine2::from_scale(Vector2::new(num!(0.4), num!(0.4))),
                ..Default::default()
            },
        );
        r.insert_instance(Instance {
            mesh: cube,
            material: wood,
            transform: Transform::default().translate(
                num!(0.5) * (self.length - num!(1)) * Self::GRID_UNIT,
                Length::zero(),
                Length::zero(),
            ),
            children: vec![],
        });
        r
    }
}
