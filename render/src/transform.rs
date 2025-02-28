use bevy_transform::components::Transform as BevyTransform;
use serde::{Deserialize, Serialize};
use villagekit_math::{Quaternion, Vector3};
use villagekit_number::Number;
use villagekit_unit::Length;

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
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

impl From<Transform> for BevyTransform {
    fn from(value: Transform) -> Self {
        let Transform {
            translation,
            rotation,
            scale,
        } = value;
        BevyTransform {
            translation: translation.into(),
            rotation: rotation.into(),
            scale: scale.into(),
        }
    }
}
