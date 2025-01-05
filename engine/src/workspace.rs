#[derive(Resource)]
pub struct Workspace {
    pub parts: HashMap<PathBuf, String>,
    pub products: HashMap<PathBuf, String>,
    pub dependencies: HashMap<PathBuf, Vec<PathBuf>>,
}
