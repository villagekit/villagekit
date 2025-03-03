mod matrix3;
mod quaternion;
mod transform3;
mod vector3;

pub use matrix3::*;
pub use quaternion::*;
pub use transform3::*;
pub use vector3::*;

#[cfg(test)]
mod tests {
    use villagekit_number::{num, ops::Sqrt, Number};
    use villagekit_unit::Length;

    use super::*;

    #[test]
    fn magnitude_of_vector_with_units() {
        let expected = Length(num!(10) * num!(3).sqrt());
        let v = Vector3::new(Length(num!(10)), Length(num!(10)), Length(num!(10)));
        let actual = v.magnitude();
        assert_eq!(expected, actual);
    }
}
