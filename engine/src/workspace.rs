use std::path::PathBuf;

use bevy::{prelude::*, utils::HashMap};

#[derive(Resource)]
pub struct Workspace {
    pub products: HashMap<PathBuf, String>,
    pub dependencies: HashMap<PathBuf, Vec<PathBuf>>,
}
