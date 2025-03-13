mod constants;
mod quaternion;
mod vector3;

pub use constants::*;
pub use quaternion::*;
pub use vector3::*;

#[cfg(test)]
mod tests {
    use villagekit_number::{num, traits::Sqrt};
    use villagekit_unit::{qty, Length};

    use super::*;

    #[test]
    fn length_of_vector_with_units() {
        let expected = qty!(10 m) * num!(3).sqrt();
        let v = Vector3::new(qty!(10 m), qty!(10 m), qty!(10 m));
        let actual = v.length();
        assert_eq!(expected, actual);
    }
}
