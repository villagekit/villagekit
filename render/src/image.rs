use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct ImageId(PathBuf);

impl ImageId {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self(path.as_ref().to_path_buf())
    }
}

impl AsRef<Path> for ImageId {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}
