use crate::bevy_scene::setup_scene;

pub fn run_game() {
    use bevy::prelude::*;

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_scene)
        .run();
}
