use bevy::prelude::*;

use crate::game::{
    game_state::GameState,
    highlight_constants::{BUTTON_DEFAULT, BUTTON_HOVERED},
};

pub fn handle_menu_buttons(
    mut state: ResMut<NextState<GameState>>,
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut background_color) in &mut query {
        match interaction {
            Interaction::Pressed => state.set(GameState::PlayerTurn),
            Interaction::Hovered => *background_color = BackgroundColor(BUTTON_HOVERED),
            Interaction::None => *background_color = BackgroundColor(BUTTON_DEFAULT),
        }
    }
}
