use villagekit_render::{Renderable, Transform};
use villagekit_unit::Length;

pub trait Stock {
    fn render(&self) -> Renderable;
    fn product(self) -> Product
    where
        Self: Sized + 'static,
    {
        Product::new(ProductKind::Stock(Box::new(self)))
    }
}

pub trait Assembly {
    fn products(&self) -> Vec<Product>;
}

pub struct Group(pub Vec<Product>);

pub enum ProductKind {
    Stock(Box<dyn Stock>),
    Assembly(Box<dyn Assembly>),
    Group(Group),
    None,
}

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
