use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(
    Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize,
)]
pub struct Vector2<N> {
    pub x: N,
    pub y: N,
}

impl<N> Vector2<N> {
    pub const fn new(x: N, y: N) -> Self {
        Self { x, y }
    }
}

impl<N: Display> Display for Vector2<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<N> From<Vector2<N>> for glam::Vec2
where
    N: Into<f32>,
{
    fn from(value: Vector2<N>) -> Self {
        let Vector2 { x, y } = value;
        glam::Vec2::new(x.into(), y.into())
    }
}
