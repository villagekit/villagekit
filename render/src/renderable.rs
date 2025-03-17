use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::{Instance, Material, MaterialId, Mesh, MeshId};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Renderable {
    pub meshes: BTreeMap<MeshId, Mesh>,
    pub materials: BTreeMap<MaterialId, Material>,
    pub instances: Vec<Instance>,
}

impl Renderable {
    pub fn insert_mesh(&mut self, key: &str, mesh: Mesh) -> MeshId {
        let id = MeshId::new(key);
        self.meshes.insert(id.clone(), mesh);
        id
    }
    pub fn insert_material(&mut self, key: &str, material: Material) -> MaterialId {
        let id = MaterialId::new(key);
        self.materials.insert(id.clone(), material);
        id
    }
    pub fn insert_instance(&mut self, instance: Instance) {
        // TODO check that meshes and materials exist, for children too.
        self.instances.push(instance);
    }
}
