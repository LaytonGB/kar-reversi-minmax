use crate::bevy_systems::draw_board;

pub fn run_game() {
    use bevy::prelude::*;

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, draw_board)
        .run();
}
