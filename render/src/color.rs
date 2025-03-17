use bevy_color::Color as BevyColor;
use serde::{Deserialize, Serialize};
use villagekit_unit::{num, Number};

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum Color {
    Hsla {
        hue: Number,
        saturation: Number,
        lightness: Number,
        alpha: Number,
    },
    Srgba {
        red: Number,
        green: Number,
        blue: Number,
        alpha: Number,
    },
}

impl Color {
    pub const WHITE: Color = Color::Srgba {
        red: num!(1),
        green: num!(1),
        blue: num!(1),
        alpha: num!(1),
    };
}

impl From<Color> for BevyColor {
    fn from(value: Color) -> Self {
        match value {
            Color::Hsla {
                hue,
                saturation,
                lightness,
                alpha,
            } => BevyColor::hsla(
                hue.into(),
                saturation.into(),
                lightness.into(),
                alpha.into(),
            ),
            Color::Srgba {
                red,
                green,
                blue,
                alpha,
            } => BevyColor::hsla(red.into(), green.into(), blue.into(), alpha.into()),
        }
    }
}
