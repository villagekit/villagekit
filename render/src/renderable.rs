use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::{Instance, Material, MaterialId, Shape, ShapeEnum, ShapeId};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Renderable {
    pub shapes: BTreeMap<ShapeId, ShapeEnum>,
    pub materials: BTreeMap<MaterialId, Material>,
    pub instances: Vec<Instance>,
}

impl Renderable {
    pub fn insert_shape(&mut self, key: &str, shape: impl Shape) -> ShapeId {
        let id = ShapeId::new(key);
        self.shapes.insert(id.clone(), shape.into());
        id
    }
    pub fn insert_material(&mut self, key: &str, material: Material) -> MaterialId {
        let id = MaterialId::new(key);
        self.materials.insert(id.clone(), material);
        id
    }
    pub fn insert_instance(&mut self, instance: Instance) {
        // TODO check that shapes and materials exist, for children too.
        self.instances.push(instance);
    }
}
