use std::any::Any;

use bevy::prelude::*;
use villagekit_product::{Product, ProductKind};

use crate::spawn_renderable;

#[derive(Component, Default)]
#[require(Transform, Visibility)]
pub struct ProductObject(ProductKind);

pub fn spawn_product(parent: Entity, product: Product, commands: &mut Commands) {
    let Product { kind, transform } = product;
    let transform: Transform = transform.into();
    commands.entity(parent).with_children(|p| {
        p.spawn((ProductObject(kind), transform));
    });
}

pub(crate) fn process_products(
    mut commands: Commands,
    query: Query<(Entity, &ProductObject), Added<ProductObject>>,
) {
    for (entity, object) in query.iter() {
        let kind: &ProductKind = &object.0;
        match kind {
            ProductKind::Stock(stock) => {
                let renderable = stock.render();
                spawn_renderable(entity, renderable, &mut commands);
            }
            ProductKind::Assembly(assembly) => {
                let products = assembly.products();
                for product in products {
                    spawn_product(entity, product, &mut commands);
                }
            }
            ProductKind::Group(group) => {
                for product in &group.0 {
                    spawn_product(entity, product.clone(), &mut commands);
                }
            }
            ProductKind::None => {}
        }
    }
}
