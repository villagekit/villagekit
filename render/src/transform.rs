use bevy_math::{Isometry3d, Vec3};
use bevy_transform::components::Transform as BevyTransform;
use serde::{Deserialize, Serialize};
use villagekit_math::{Quaternion, Vector3};
use villagekit_number::Number;
use villagekit_unit::{Angle, Dimension, Length};

// TODO this is the same as https://docs.rs/bevy/latest/bevy/prelude/struct.Isometry3d.html
//   So maybe should be renamed?

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Transform {
    translation: Vector3<Length>,
    rotation: Quaternion,
}

impl Transform {
    pub fn apply_translation(self, translation: Vector3<Length>) -> Self {
        Self {
            translation: self.translation + translation,
            ..self
        }
    }

    pub fn translate(self, x: Length, y: Length, z: Length) -> Self {
        self.apply_translation(Vector3::new(x, y, z))
    }

    pub fn apply_rotation(self, rotation: Quaternion) -> Self {
        Self {
            rotation: self.rotation * rotation,
            ..self
        }
    }

    /// Rotate this transform around an arbitrary axis that passes through a given origin.
    pub fn rotate(
        self,
        axis: Vector3<Number>,
        angle: Angle,
        origin: Option<Vector3<Length>>,
    ) -> Self {
        let origin = origin.unwrap_or_default();
        let rotation = Quaternion::from_axis_angle(axis, angle);

        Self {
            // - Translate so that `origin` is at the origin.
            // - Rotate the translation (in length units) about the new origin
            // - Translate back to the original pivot by adding `origin` again
            translation: (self.translation - origin).multiply_quaternion(rotation) + origin,
            // Apply the rotation
            rotation: self.rotation * rotation,
        }
    }
}

impl From<Transform> for BevyTransform {
    fn from(value: Transform) -> Self {
        BevyTransform {
            translation: value.translation.map(|v| v.canonical()).into(),
            rotation: value.rotation.into(),
            ..Default::default()
        }
    }
}

impl From<Transform> for Isometry3d {
    fn from(value: Transform) -> Self {
        let translation_vec3: Vec3 = value.translation.map(|v| v.canonical()).into();
        Isometry3d {
            translation: translation_vec3.into(),
            rotation: value.rotation.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use villagekit_number::{num, traits::ApproxEq};
    use villagekit_unit::{qty, Radians};

    use super::*;

    #[test]
    fn test_apply_translation() {
        let initial = Transform {
            translation: Vector3::new(qty!(1 m), qty!(2 m), qty!(3 m)),
            rotation: Quaternion::default(),
        };
        let delta = Vector3::new(qty!(4 m), qty!(-2 m), qty!(0 m));
        let result = initial.apply_translation(delta);
        // Expected translation is (1+4, 2-2, 3+0) = (5, 0, 3)
        assert_eq!(
            result.translation,
            Vector3::new(qty!(5 m), qty!(0 m), qty!(3 m))
        );
        // Rotation remains unchanged (identity)
        assert_eq!(result.rotation, Quaternion::default());
    }

    #[test]
    fn test_translate() {
        let initial = Transform::default();
        let result = initial.translate(qty!(1 m), qty!(2 m), qty!(3 m));
        assert_eq!(
            result.translation,
            Vector3::new(qty!(1 m), qty!(2 m), qty!(3 m))
        );
    }

    #[test]
    fn test_apply_rotation() {
        // Start with a non-default rotation: 45° around the y-axis.
        let initial = Transform {
            translation: Vector3::new(qty!(1 m), qty!(0 m), qty!(0 m)),
            rotation: Quaternion::from_axis_angle(
                Vector3::new(num!(0), num!(1), num!(0)),
                Radians::FRAC_PI_4,
            ),
        };
        // Apply a rotation of 90° about the z-axis.
        let axis = Vector3::new(num!(0), num!(0), num!(1));
        let angle = Radians::FRAC_PI_2;
        let applied_rotation = Quaternion::from_axis_angle(axis, angle);
        let result = initial.apply_rotation(applied_rotation);
        let expected_rotation = initial.rotation * applied_rotation;
        assert_eq!(result.rotation, expected_rotation);
        // Ensure the translation remains unchanged.
        assert_eq!(result.translation, initial.translation);
    }

    macro_rules! assert_approx_eq {
        ($given:expr, $expected:expr) => {
            assert!(
                ApproxEq::approx_eq($given, $expected),
                "assert_approx_eq!({}, {})

    left  = {:?}
    right = {:?}

",
                stringify!($given),
                stringify!($expected),
                $given,
                $expected
            )
        };
    }

    #[test]
    fn test_rotate_without_origin() {
        // Rotate a point at (1,0,0) 90° about the z-axis using default origin (0,0,0).
        let initial = Transform {
            translation: Vector3::new(qty!(1 m), qty!(0 m), qty!(0 m)),
            rotation: Quaternion::default(),
        };
        let axis = Vector3::new(num!(0), num!(0), num!(1));
        let angle = Radians::FRAC_PI_2;
        let result = initial.rotate(axis, angle, None);
        // (1,0,0) rotated about (0,0,0) by 90° should become (0,1,0)
        assert_approx_eq!(
            &result.translation,
            &Vector3::new(qty!(0 m), qty!(1 m), qty!(0 m))
        );
    }

    #[test]
    fn test_rotate_with_origin() {
        // Rotate a point at (2,0,0) 90° about the z-axis with pivot (1,0,0).
        let initial = Transform {
            translation: Vector3::new(qty!(2 m), qty!(0 m), qty!(0 m)),
            rotation: Quaternion::default(),
        };
        let axis = Vector3::new(num!(0), num!(0), num!(1));
        let angle = Radians::FRAC_PI_2;
        let origin = Vector3::new(qty!(1 m), qty!(0 m), qty!(0 m));
        let result = initial.rotate(axis, angle, Some(origin));
        // Calculation:
        // (2,0,0) - (1,0,0) = (1,0,0) rotated 90° gives (0,1,0), then add origin -> (1,1,0)
        assert_approx_eq!(
            &result.translation,
            &Vector3::new(qty!(1 m), qty!(1 m), qty!(0 m))
        );
    }
}
