use bevy::prelude::*;
use bevy_mod_picking::{debug::DebugPickingPlugin, DefaultPickingPlugins};

use crate::game::{
    interactions::{
        bot_delay_reset, bot_make_move, maintain_score_display, update_current_player,
        update_player_scores,
    },
    menu_interactions::handle_difficulty_buttons,
    /* interactions::highlight_valid_grid_squares, */ pieces::draw_pieces,
    scenes::{board_setup, menu_setup, menu_teardown},
    states::GameState,
    structs::{BevyAiDelay, BevyReversi},
};

use super::{
    interactions::{display_metrics, show_game_over, update_grid_highlights},
    menu_interactions::{
        handle_algorithm_buttons, handle_gamemode_buttons, handle_heuristic_buttons,
        handle_play_button,
    },
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
        // menu
        .add_systems(OnEnter(GameState::Menu), menu_setup)
        .add_systems(
            Update,
            (
                handle_gamemode_buttons,
                handle_difficulty_buttons,
                handle_algorithm_buttons,
                handle_heuristic_buttons,
                handle_play_button,
            )
                .run_if(in_state(GameState::Menu)),
        )
        .add_systems(OnExit(GameState::Menu), menu_teardown)
        // game
        .add_systems(
            OnTransition {
                from: GameState::Menu,
                to: GameState::PlayerTurn,
            },
            board_setup,
        )
        .add_systems(
            Update,
            (
                (
                    draw_pieces,
                    update_player_scores,
                    update_current_player,
                    maintain_score_display,
                )
                    .run_if(in_state(GameState::PlayerTurn)),
                (
                    draw_pieces,
                    update_player_scores,
                    update_current_player,
                    maintain_score_display,
                )
                    .run_if(in_state(GameState::AiTurn)),
            ),
        )
        .add_systems(OnExit(GameState::AiTurn), display_metrics)
        .add_systems(
            PostUpdate,
            (
                update_grid_highlights.run_if(in_state(GameState::PlayerTurn)),
                update_grid_highlights.run_if(in_state(GameState::AiTurn)),
            ),
        )
        .add_systems(OnEnter(GameState::AiTurn), bot_delay_reset)
        .add_systems(Update, bot_make_move.run_if(in_state(GameState::AiTurn)))
        .add_systems(OnEnter(GameState::End), show_game_over)
        .run();
}
