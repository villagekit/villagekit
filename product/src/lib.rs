use dyn_clone::DynClone;
use villagekit_math::Vector3;
use villagekit_number::Number;
use villagekit_render::{Renderable, Transform};
use villagekit_unit::{Angle, Length};

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
        self.update_transform(|t| t.translate(x, y, z))
    }

    pub fn rotate(
        self,
        axis: Vector3<Number>,
        angle: Angle,
        origin: Option<Vector3<Length>>,
    ) -> Self {
        self.update_transform(|t| t.rotate(axis, angle, origin))
    }

    fn update_transform(self, updater: impl Fn(Transform) -> Transform) -> Self {
        Self {
            transform: updater(self.transform),
            ..self
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert!(true);
    }
}
