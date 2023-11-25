// // This system updates the settings when a new value for a setting is selected, and marks
// // the button as the one currently selected
// fn setting_button<T: Resource + Component + PartialEq + Copy>(
//     interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
//     mut selected_query: Query<(Entity, &mut BackgroundColor), With<SelectedOption>>,
//     mut commands: Commands,
//     mut setting: ResMut<T>,
// ) {
//     for (interaction, button_setting, entity) in &interaction_query {
//         if *interaction == Interaction::Pressed && *setting != *button_setting {
//             let (previous_button, mut previous_color) = selected_query.single_mut();
//             *previous_color = NORMAL_BUTTON.into();
//             commands.entity(previous_button).remove::<SelectedOption>();
//             commands.entity(entity).insert(SelectedOption);
//             *setting = *button_setting;
//         }
//     }
// }

use std::{thread, time};

use bevy::prelude::*;

use crate::{
    bevy_structs::{BevyReversi, BevySquare},
    bevy_utils::*,
};

pub(crate) fn interactions(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &Transform), (Changed<Interaction>, With<BevySquare>)>,
    mut game: ResMut<BevyReversi>,
) {
    for (&interaction, transform) in &interaction_query {
        if interaction == Interaction::Pressed {
            let Vec3 { x, z, .. } = transform.translation;
            let coord = game_coord_to_reversi_coord((x, z));
            game.0.place_piece(coord);
            game.0.switch_players();

            let sleep_time = time::Duration::from_millis(1500);
            thread::sleep(sleep_time);
            let game = game.0.clone();
            let (_, bot) = game.0.bot_player.as_mut().unwrap();
            let coord = bot.get_move(game);
            game.0.place_piece(coord);
            game.0.switch_players();
        }
    }
}
