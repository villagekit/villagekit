mod color;
mod image;
mod instance;
mod material;
mod renderable;
mod shape;
mod transform;

pub use color::*;
pub use image::*;
pub use instance::*;
pub use material::*;
pub use renderable::*;
pub use shape::*;
pub use transform::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}
