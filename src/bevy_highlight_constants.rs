use bevy::{math::vec4, pbr::StandardMaterial};
use bevy_mod_picking::highlight::{Highlight, HighlightKind};

pub(crate) const GRID_HIGHLIGHT: Highlight<StandardMaterial> = Highlight {
    hovered: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + vec4(0.35, 0.35, 0.35, 0.0),
        ..matl.to_owned()
    })),
    pressed: Some(HighlightKind::new_dynamic(|matl| matl.to_owned())),
    selected: Some(HighlightKind::new_dynamic(|matl| matl.to_owned())),
};
