use bevy::{math::vec4, pbr::StandardMaterial, render::color::Color};
use bevy_mod_picking::highlight::{Highlight, HighlightKind};

pub const GRID_HIGHLIGHT: Highlight<StandardMaterial> = Highlight {
    hovered: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + vec4(0.35, 0.35, 0.35, 0.0),
        ..matl.to_owned()
    })),
    pressed: Some(HighlightKind::new_dynamic(|matl| matl.to_owned())),
};

pub const BUTTON_DEFAULT: Color = Color::BLACK;
pub const BUTTON_HOVERED: Color = Color::Hsla {
    hue: 0.0,
    saturation: 0.0,
    lightness: 0.15,
    alpha: 1.0,
};
pub const BUTTON_SELECTED: Color = Color::Hsla {
    hue: 110.0,
    saturation: 1.0,
    lightness: 0.15,
    alpha: 1.0,
};
