use bevy::prelude::*;
use serde_json::Value;
use villagekit_product::Product;

#[derive(Component, Default)]
struct ProductComponent(Product);
