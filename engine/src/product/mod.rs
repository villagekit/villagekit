use bevy::prelude::*;
use bevy::utils::HashMap;
use serde_json::Value;
use std::path::PathBuf;

use crate::params::Params;
use crate::render::Renderable;

#[derive(Component, Default)]
#[require(Transform, Visibility)]
pub struct ProductObject;

#[derive(Component, Default)]
#[require(ProductDesignEntry, Params)]
pub struct ProductDesign;

#[derive(Component)]
#[require(ProductDesign, ProductDesignType(init_product_design_type_stock))]
pub struct StockDesign;
#[derive(Component)]
#[require(ProductDesign, ProductDesignType(init_product_design_type_sheet))]
pub struct SheetDesign;
#[derive(Component)]
#[require(ProductDesign, ProductDesignType(init_product_design_type_solid))]
pub struct SolidDesign;
#[derive(Component)]
#[require(ProductDesign, ProductDesignType(init_product_design_type_assembly))]
pub struct AssemblyDesign;

#[derive(Component)]
pub enum ProductDesignType {
    Stock,
    Sheet,
    Solid,
    Assembly,
}

fn init_product_design_type_stock() -> ProductDesignType {
    ProductDesignType::Stock
}
fn init_product_design_type_sheet() -> ProductDesignType {
    ProductDesignType::Sheet
}
fn init_product_design_type_solid() -> ProductDesignType {
    ProductDesignType::Solid
}
fn init_product_design_type_assembly() -> ProductDesignType {
    ProductDesignType::Assembly
}

#[derive(Component, Default)]
pub struct ProductDesignEntry {
    path: PathBuf,
}

pub struct ProductDesignRenderer {
    render: Box<dyn Fn(&Value) -> Renderable + Send + Sync>,
}

#[derive(Resource)]
pub struct ProductRenderers(HashMap<ProductDesignEntry, ProductDesignRenderer>);
