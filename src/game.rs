use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;

use crate::{bevy_pieces::draw_pieces, bevy_scene::setup_scene, bevy_structs::BevyReversi};

pub fn run_game() {
    App::new()
        .add_plugins((DefaultPlugins, DefaultPickingPlugins))
        .init_resource::<BevyReversi>()
        .add_systems(Startup, setup_scene)
        .add_systems(Update, draw_pieces)
        .run();
}
