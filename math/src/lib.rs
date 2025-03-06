mod constants;
mod quaternion;
mod vector3;

pub use constants::*;
pub use quaternion::*;
pub use vector3::*;

#[cfg(test)]
mod tests {
    use villagekit_number::{num, traits::Sqrt, Number};
    use villagekit_unit::Length;

    use super::*;

    #[test]
    fn length_of_vector_with_units() {
        let expected = Length(num!(10) * num!(3).sqrt());
        let v = Vector3::new(Length(num!(10)), Length(num!(10)), Length(num!(10)));
        let actual = v.length();
        assert_eq!(expected, actual);
    }
}
