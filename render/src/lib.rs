mod color;
mod image;
mod instance;
mod material;
mod mesh;
mod renderable;
mod transform;

pub use color::*;
pub use image::*;
pub use instance::*;
pub use material::*;
pub use mesh::*;
pub use renderable::*;
pub use transform::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}
