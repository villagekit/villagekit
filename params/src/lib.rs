// https://crates.io/crates/valico

#[derive(Debug, Clone)]
pub struct ParamsValue {}

#[derive(Debug, Clone)]
pub struct ParamsDef {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
