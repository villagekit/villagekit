use villagekit_render::{Object3d, Renderable, Transform};

pub trait Stock: Object3d {
    fn render(self) -> Renderable;
}

pub struct Group {
    transform: Transform,
    products: Vec<Product>,
}

impl Object3d for Group {
    fn transform(self, update: impl Fn(Transform) -> Transform) -> Self {
        Self {
            transform: update(self.transform),
            ..self
        }
    }
}

pub trait Assembly: Object3d {
    fn products(self) -> Vec<Product>;
}

pub enum Product {
    Stock(Box<dyn Stock>),
    Group(Group),
    Assembly(Box<dyn Assembly>),
    None,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}
