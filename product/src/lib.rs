use villagekit_render::Renderable;

pub trait Stock {
    fn render(self) -> Renderable;
}

pub trait Assembly {
    fn products(self) -> ProductTree;
}

pub enum Product {
    Stock(Box<dyn Stock>),
    Assembly(Box<dyn Assembly>),
}

pub enum ProductTreeItem {
    Single(Product),
    Nested(ProductTree),
    None,
}

pub struct ProductTree(Vec<ProductTreeItem>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}
