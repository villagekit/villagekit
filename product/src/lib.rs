use dyn_clone::DynClone;
use villagekit_render::{Renderable, Transform};
use villagekit_unit::Length;

pub trait Stock: DynClone {
    fn render(&self) -> Renderable;
    fn place(self) -> Product
    where
        Self: Sized + Send + Sync + 'static,
    {
        Product::new(ProductKind::Stock(Box::new(self)))
    }
}

pub trait Assembly: DynClone {
    fn products(&self) -> Vec<Product>;
    fn place(self) -> Product
    where
        Self: Sized + Send + Sync + 'static,
    {
        Product::new(ProductKind::Assembly(Box::new(self)))
    }
}

dyn_clone::clone_trait_object!(Stock);
dyn_clone::clone_trait_object!(Assembly);

#[derive(Default, Clone)]
pub struct Group(pub Vec<Product>);

#[derive(Default, Clone)]
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

#[derive(Default, Clone)]
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
