use bevy::prelude::*;

use crate::{
    bot_algorithm::BotAlgorithm,
    game::{
        highlight_constants::{BUTTON_DEFAULT, BUTTON_HOVERED},
        states::GameState,
    },
    player::Player,
    reversi::Reversi,
};

use super::structs::{BevyBotDifficulty, BevyReversi};

pub fn handle_menu_buttons(
    mut game: ResMut<BevyReversi>,
    mut state: ResMut<NextState<GameState>>,
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &BevyBotDifficulty),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color, difficulty) in &mut query {
        match interaction {
            Interaction::Pressed => {
                state.set(GameState::PlayerTurn);
                game.0 = Reversi::new(Some((Player::Red, BotAlgorithm::MinMax, difficulty.0)));
                game.0.update_valid_moves();
            }
            Interaction::Hovered => *background_color = BackgroundColor(BUTTON_HOVERED),
            Interaction::None => *background_color = BackgroundColor(BUTTON_DEFAULT),
        }
    }
}
