use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::{Instance, Material, MaterialId, Shape3d, Shape3dId};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Renderable {
    pub shapes: BTreeMap<Shape3dId, Shape3d>,
    pub materials: BTreeMap<MaterialId, Material>,
    pub instances: Vec<Instance>,
}

impl Renderable {
    pub fn insert_shape(&mut self, key: &str, shape: Shape3d) -> Shape3dId {
        let id = Shape3dId::new(key);
        self.shapes.insert(id.clone(), shape);
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
