mod renderable;
mod transform;

pub use renderable::{
    Renderable, RenderableColor, RenderableInstance, RenderableMaterial, RenderableMesh,
};
pub use transform::Transform;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}
