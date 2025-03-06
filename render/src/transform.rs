use bevy_transform::components::Transform as BevyTransform;
use serde::{Deserialize, Serialize};
use villagekit_math::{Quaternion, Vector3};
use villagekit_number::Number;
use villagekit_unit::Length;

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
        angle: Number,
        origin: Option<Vector3<Length>>,
    ) -> Self {
        let origin = origin.unwrap_or_default();
        let rotation = Quaternion::from_axis_angle(axis, angle);

        Self {
            // - Translate so that `origin` is at the origin.
            // - Rotate the translation (in length units) about the new origin
            // - Translate back to the original pivot by adding `origin` again
            translation: (self.translation - origin) * rotation + origin,
            // Apply the rotation
            rotation: self.rotation * rotation,
        }
    }
}

impl From<Transform> for BevyTransform {
    fn from(value: Transform) -> Self {
        BevyTransform {
            translation: value.translation.into(),
            rotation: value.rotation.into(),
            ..Default::default()
        }
    }
}
#[cfg(test)]
mod tests {
    use villagekit_number::{num, traits::ApproxEq};

    use super::*;

    #[test]
    fn test_apply_translation() {
        let initial = Transform {
            translation: Vector3::new(Length(num!(1)), Length(num!(2)), Length(num!(3))),
            rotation: Quaternion::default(),
        };
        let delta = Vector3::new(Length(num!(4)), Length(num!(-2)), Length(num!(0)));
        let result = initial.apply_translation(delta);
        // Expected translation is (1+4, 2-2, 3+0) = (5, 0, 3)
        assert_eq!(
            result.translation,
            Vector3::new(Length(num!(5)), Length(num!(0)), Length(num!(3)))
        );
        // Rotation remains unchanged (identity)
        assert_eq!(result.rotation, Quaternion::default());
    }

    #[test]
    fn test_translate() {
        let initial = Transform::default();
        let result = initial.translate(Length(num!(1)), Length(num!(2)), Length(num!(3)));
        assert_eq!(
            result.translation,
            Vector3::new(Length(num!(1)), Length(num!(2)), Length(num!(3)))
        );
    }

    #[test]
    fn test_apply_rotation() {
        // Start with a non-default rotation: 45° around the y-axis.
        let initial = Transform {
            translation: Vector3::new(Length(num!(1)), Length(num!(0)), Length(num!(0))),
            rotation: Quaternion::from_axis_angle(
                Vector3::new(num!(0), num!(1), num!(0)),
                Number::FRAC_PI_4,
            ),
        };
        // Apply a rotation of 90° about the z-axis.
        let axis = Vector3::new(num!(0), num!(0), num!(1));
        let angle = Number::FRAC_PI_2;
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
            translation: Vector3::new(Length(num!(1)), Length(num!(0)), Length(num!(0))),
            rotation: Quaternion::default(),
        };
        let axis = Vector3::new(num!(0), num!(0), num!(1));
        let angle = Number::FRAC_PI_2;
        let result = initial.rotate(axis, angle, None);
        // (1,0,0) rotated about (0,0,0) by 90° should become (0,1,0)
        assert_approx_eq!(
            &result.translation,
            &Vector3::new(Length(num!(0)), Length(num!(1)), Length(num!(0)))
        );
    }

    #[test]
    fn test_rotate_with_origin() {
        // Rotate a point at (2,0,0) 90° about the z-axis with pivot (1,0,0).
        let initial = Transform {
            translation: Vector3::new(Length(num!(2)), Length(num!(0)), Length(num!(0))),
            rotation: Quaternion::default(),
        };
        let axis = Vector3::new(num!(0), num!(0), num!(1));
        let angle = Number::FRAC_PI_2;
        let origin = Vector3::new(Length(num!(1)), Length(num!(0)), Length(num!(0)));
        let result = initial.rotate(axis, angle, Some(origin));
        // Calculation:
        // (2,0,0) - (1,0,0) = (1,0,0) rotated 90° gives (0,1,0), then add origin -> (1,1,0)
        assert_approx_eq!(
            &result.translation,
            &Vector3::new(Length(num!(1)), Length(num!(1)), Length(num!(0)))
        );
    }
}
