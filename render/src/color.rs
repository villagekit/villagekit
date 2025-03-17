use bevy_color::{Color as BevyColor, LinearRgba};
use serde::{Deserialize, Serialize};
use villagekit_unit::{num, Number};

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
    LinearRgba {
        red: Number,
        green: Number,
        blue: Number,
        alpha: Number,
    },
}

impl Color {
    pub const WHITE: Color = Color::LinearRgba {
        red: num!(1),
        green: num!(1),
        blue: num!(1),
        alpha: num!(1),
    };
    pub const BLACK: Color = Color::LinearRgba {
        red: num!(0),
        green: num!(0),
        blue: num!(0),
        alpha: num!(0),
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
            Color::LinearRgba {
                red,
                green,
                blue,
                alpha,
            } => BevyColor::linear_rgba(red.into(), green.into(), blue.into(), alpha.into()),
        }
    }
}

impl From<Color> for LinearRgba {
    fn from(value: Color) -> Self {
        BevyColor::from(value).to_linear()
    }
}
