use bevy::prelude::*;
use if_chain::if_chain;

use crate::{
    bot_algorithm::BotAlgorithm,
    bot_difficulty::BotDifficulty,
    game::{
        highlight_constants::{BUTTON_DEFAULT, BUTTON_HOVERED},
        states::GameState,
    },
    player::Player,
    reversi::Reversi,
};

use super::{
    highlight_constants::{BUTTON_SELECTED, DANGER_DEFAULT, DANGER_HOVERED},
    structs::{
        BevyBotAlgorithm, BevyBotDifficulty, BevyBotHeuristic, BevyMenuContent, BevyPlayButton,
        BevyReversi,
    },
};

pub fn handle_difficulty_buttons(
    mut config: ResMut<BevyMenuContent>,
    mut query: Query<(&Interaction, &mut BackgroundColor, &BevyBotDifficulty), With<Button>>,
) {
    for (interaction, mut background_color, difficulty) in &mut query {
        if config.config.difficulty.is_some_and(|d| d == difficulty.0) {
            *background_color = BackgroundColor(BUTTON_SELECTED);
        } else {
            let is_insane = difficulty.0 == BotDifficulty::Insane;
            match interaction {
                Interaction::Pressed => config.config.difficulty = Some(difficulty.0),
                Interaction::Hovered => {
                    *background_color = BackgroundColor(if is_insane {
                        DANGER_HOVERED
                    } else {
                        BUTTON_HOVERED
                    })
                }
                Interaction::None => {
                    *background_color = BackgroundColor(if is_insane {
                        DANGER_DEFAULT
                    } else {
                        BUTTON_DEFAULT
                    })
                }
            }
        }
    }
}

pub fn handle_algorithm_buttons(
    mut config: ResMut<BevyMenuContent>,
    mut query: Query<(&Interaction, &mut BackgroundColor, &BevyBotAlgorithm), With<Button>>,
) {
    for (interaction, mut background_color, algorithm) in &mut query {
        if config.config.algorithm.is_some_and(|a| a == algorithm.0) {
            *background_color = BackgroundColor(BUTTON_SELECTED);
        } else {
            let is_async = algorithm.0 == BotAlgorithm::Async;
            match interaction {
                Interaction::Pressed => config.config.algorithm = Some(algorithm.0),
                Interaction::Hovered => {
                    *background_color = BackgroundColor(if is_async {
                        DANGER_HOVERED
                    } else {
                        BUTTON_HOVERED
                    })
                }
                Interaction::None => {
                    *background_color = BackgroundColor(if is_async {
                        DANGER_DEFAULT
                    } else {
                        BUTTON_DEFAULT
                    })
                }
            }
        }
    }
}

pub fn handle_heuristic_buttons(
    mut config: ResMut<BevyMenuContent>,
    mut query: Query<(&Interaction, &mut BackgroundColor, &BevyBotHeuristic), With<Button>>,
) {
    for (interaction, mut background_color, heuristic) in &mut query {
        if config.config.heuristic.is_some_and(|a| a == heuristic.0) {
            *background_color = BackgroundColor(BUTTON_SELECTED);
        } else {
            match interaction {
                Interaction::Pressed => config.config.heuristic = Some(heuristic.0),
                Interaction::Hovered => *background_color = BackgroundColor(BUTTON_HOVERED),
                Interaction::None => *background_color = BackgroundColor(BUTTON_DEFAULT),
            }
        }
    }
}

pub fn handle_play_button(
    mut game: ResMut<BevyReversi>,
    mut state: ResMut<NextState<GameState>>,
    config: Res<BevyMenuContent>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<Button>, With<BevyPlayButton>),
    >,
    mut text_query: Query<&mut Text, With<BevyPlayButton>>,
) {
    for (interaction, mut background_color) in &mut button_query {
        if_chain!(
            if let Some(_) = config.config.algorithm;
            if let Some(_) = config.config.difficulty;
            then {
                for mut text in &mut text_query {
                    text.sections[0].style.color.set_l(0.85);
                }
                match interaction {
                    Interaction::Pressed => start_game(&mut game, &mut state, &config),
                    Interaction::Hovered => *background_color = BackgroundColor(BUTTON_HOVERED),
                    Interaction::None => *background_color = BackgroundColor(BUTTON_DEFAULT),
                }
            } else {
                for mut text in &mut text_query {
                    text.sections[0].style.color.set_l(0.3);
                }
            }
        );
    }
}

fn start_game(
    game: &mut ResMut<BevyReversi>,
    state: &mut ResMut<NextState<GameState>>,
    config: &Res<BevyMenuContent>,
) {
    state.set(GameState::PlayerTurn);
    game.0 = Reversi::new(Some((
        Player::Red,
        config.config.difficulty.unwrap(),
        config.config.algorithm.unwrap(),
        config.config.heuristic.unwrap(),
    )));
    game.0.update_valid_moves();
}
