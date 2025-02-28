use bevy::prelude::*;

use serde_json::{from_str, from_value};
use villagekit_engine::*;

fn main() {
    App::new()
        .add_plugins(EnginePlugin)
        .add_systems(PostStartup, setup_model)
        .run();
}

fn setup_model(mut commands: Commands, sandbox: Query<Entity, With<Sandbox>>) {
    let renderable_json = r#"
        {
            "meshes": {
                "cube": {
                    "type": "Cuboid",
                    "x_length": "1",
                    "y_length": "1",
                    "z_length": "10"
                }
            },
            "materials": {
                "red": {
                    "type": "Color",
                    "color": {
                        "type": "Hsla",
                        "hue": "0",
                        "saturation": "1",
                        "lightness": "0.5",
                        "alpha": "1"
                    }
                }
            },
            "instances": [
                {
                    "mesh": "cube",
                    "material": "red",
                    "transform": {
                        "translation": ["0", "0", "0"],
                        "rotation": ["0", "0", "0", "1"],
                        "scale": ["1", "1", "1"]
                    }
                }
            ]
        }"#;
    let renderable_value = from_str(renderable_json).unwrap();
    let renderable: Renderable = from_value(renderable_value).unwrap();

    spawn_renderable(sandbox.single(), renderable, commands.reborrow());
}
