use villagekit_render::{Renderable, Transform};
use villagekit_unit::Length;

pub trait Stock {
    fn render(&self) -> Renderable;
    fn to_product(self) -> Product
    where
        Self: Sized + Send + Sync + 'static,
    {
        Product::new(ProductKind::Stock(Box::new(self)))
    }
}

pub trait Assembly {
    fn products(&self) -> Vec<Product>;
}

#[derive(Default)]
pub struct Group(pub Vec<Product>);

#[derive(Default)]
pub enum ProductKind {
    Stock(Box<dyn Stock + Send + Sync>),
    Assembly(Box<dyn Assembly + Send + Sync>),
    Group(Group),
    #[default]
    None,
}

impl From<Option<ProductKind>> for ProductKind {
    fn from(value: Option<ProductKind>) -> Self {
        match value {
            Some(kind) => kind,
            None => ProductKind::None,
        }
    }
}

#[derive(Default)]
pub struct Product {
    pub kind: ProductKind,
    pub transform: Transform,
}

impl Product {
    pub fn new(kind: ProductKind) -> Self {
        Self {
            kind,
            transform: Transform::default(),
        }
    }

    pub fn translate(self, x: Length, y: Length, z: Length) -> Self {
        Self {
            transform: self.transform.translate(x, y, z),
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}
