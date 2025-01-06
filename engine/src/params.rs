use bevy::prelude::*;
use serde_json::Value;

#[derive(Component, Default)]
pub(crate) struct Params(Value);
