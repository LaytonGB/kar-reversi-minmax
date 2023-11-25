use bevy::prelude::*;
use bevy_mod_picking::{debug::DebugPickingPlugin, DefaultPickingPlugins};

use crate::{
    bevy_interactions::highlight_valid_grid_squares, bevy_pieces::draw_pieces,
    bevy_scene::setup_scene, bevy_structs::BevyReversi,
};

pub fn run_game() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            DefaultPickingPlugins
                .build()
                .disable::<DebugPickingPlugin>(),
        ))
        .init_resource::<BevyReversi>()
        .add_systems(Startup, setup_scene)
        .add_systems(Update, draw_pieces)
        .add_systems(PostUpdate, highlight_valid_grid_squares)
        .run();
}
