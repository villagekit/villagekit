// https://github.com/villagekit/villagekit-rust/blob/6b8fd203e76d7fdfb149a1d7cacde785aa80c21e/math/src/affine3.rs

use serde::{Deserialize, Serialize};
use villagekit_unit::{num, Number};

use crate::{Matrix2, Vector2};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Affine2<N> {
    pub matrix2: Matrix2,
    pub translation: Vector2<N>,
}

impl<N> Affine2<N>
where
    N: Default,
{
    pub fn from_scale(scale: Vector2<Number>) -> Self {
        Self {
            matrix2: Matrix2::from_diagonal(scale),
            translation: Vector2::default(),
        }
    }

    pub fn identity() -> Self {
        Self::from_scale(Vector2::new(num!(1), num!(1)))
    }
}

impl<N> From<Affine2<N>> for glam::Affine2
where
    N: Into<f32>,
{
    fn from(value: Affine2<N>) -> Self {
        let Affine2 {
            matrix2,
            translation,
        } = value;
        glam::Affine2::from_mat2_translation(matrix2.into(), translation.into())
    }
}
