mod object3d;
mod renderable;

pub use object3d::{Object3d, Transform};
pub use renderable::{
    Renderable, RenderableColor, RenderableInstance, RenderableMaterial, RenderableMesh,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}
