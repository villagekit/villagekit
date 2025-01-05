#[derive(Resource)]
pub enum Entry {
    Part { path: PathBuf, params: Value },
    Product { path: PathBuf, params: Value },
}
