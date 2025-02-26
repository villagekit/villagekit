use num_traits::real::Real;

pub struct Vector3<N: Real> {
    x: N,
    y: N,
    z: N,
}

impl<N: Real> Vector3<N> {
    fn length(self) -> N {
        let Self { x, y, z } = self;
        (x * x + y * y + z * z).sqrt()
    }
}

pub struct Quaternion {
    x: Number,
    y: Number,
    z: Number,
    w: Number,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}
