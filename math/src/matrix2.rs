// https://github.com/villagekit/villagekit-rust/blob/6b8fd203e76d7fdfb149a1d7cacde785aa80c21e/math/src/matrix3.rs

use serde::{Deserialize, Serialize};
use villagekit_unit::{num, Number};

use crate::Vector2;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Matrix2 {
    pub x_axis: Vector2<Number>,
    pub y_axis: Vector2<Number>,
}

impl Matrix2 {
    pub fn from_cols(x_axis: Vector2<Number>, y_axis: Vector2<Number>) -> Self {
        Self { x_axis, y_axis }
    }

    pub fn from_rows(row_1: Vector2<Number>, row_2: Vector2<Number>) -> Self {
        Self {
            x_axis: Vector2::new(row_1.x, row_2.x),
            y_axis: Vector2::new(row_1.y, row_2.y),
        }
    }

    pub fn from_diagonal(diagonal: Vector2<Number>) -> Self {
        Self::from_rows(
            Vector2::new(diagonal.x, num!(0)),
            Vector2::new(num!(0), diagonal.y),
        )
    }

    pub fn identity() -> Self {
        Self::from_diagonal(Vector2::new(num!(1), num!(1)))
    }
}

impl From<Matrix2> for glam::Mat2 {
    fn from(value: Matrix2) -> Self {
        let Matrix2 { x_axis, y_axis } = value;
        glam::Mat2::from_cols(x_axis.into(), y_axis.into())
    }
}
