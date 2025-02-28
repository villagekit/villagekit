use bevy::prelude::*;
use villagekit_product::ProductKind;

#[derive(Component, Default)]
#[require(Transform, Visibility)]
pub struct ProductObject(ProductKind);
