use villagekit_math::{Quaternion, Vector3};
use villagekit_number::Number;
use villagekit_unit::Length;

pub struct Transform {
    translation: Vector3<Length>,
    rotation: Quaternion,
    scale: Vector3<Number>,
}

impl Transform {
    pub fn translate(self, x: Length, y: Length, z: Length) -> Self {
        let Self {
            translation,
            rotation,
            scale,
        } = self;
        let translation = translation + Vector3::new(x, y, z);
        Self {
            translation,
            rotation,
            scale,
        }
    }
}

pub trait Object3d: Sized {
    fn transform(self, update: impl Fn(Transform) -> Transform) -> Self;

    fn translate(self, x: Length, y: Length, z: Length) -> Self {
        self.transform(|transform| transform.translate(x, y, z))
    }
}
