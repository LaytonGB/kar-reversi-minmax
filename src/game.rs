use bevy::prelude::*;
use bevy_mod_picking::{debug::DebugPickingPlugin, DefaultPickingPlugins};

use crate::{
    bevy_game_state::GameState,
    bevy_interactions::{bot_delay_reset, bot_make_move},
    /* bevy_interactions::highlight_valid_grid_squares, */ bevy_pieces::draw_pieces,
    bevy_scene::setup_scene,
    bevy_structs::{BevyAiDelay, BevyReversi},
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
        .init_resource::<Time>()
        .init_resource::<BevyAiDelay>()
        .add_state::<GameState>()
        .add_systems(Startup, setup_scene)
        .add_systems(Update, draw_pieces)
        .add_systems(OnEnter(GameState::AiTurn), bot_delay_reset)
        .add_systems(Update, bot_make_move.run_if(in_state(GameState::AiTurn)))
        // .add_systems(
        //     Update,
        //     highlight_valid_grid_squares.run_if(resource_changed::<BevyReversi>()),
        // )
        .run();
}
