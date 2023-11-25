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

use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_mod_picking::prelude::{EntityEvent, ListenerInput};

use crate::{bevy_structs::BevyReversi, bevy_utils::*, player::Player};

pub(crate) fn click_grid_square<E>(_: &ListenerInput<E>, commands: &mut EntityCommands)
where
    E: EntityEvent,
{
    commands.add(|mut world_entity: EntityWorldMut<'_>| unsafe {
        let transform = world_entity.get::<Transform>().cloned();
        let game = world_entity.world_mut().resource_mut::<BevyReversi>();
        if game.0.current_player() == Player::Green {
            if let Some(transform) = transform {
                let Vec3 { x, z, .. } = transform.translation;
                let coord = dbg!(game_coord_to_reversi_coord((x, z)));
                if dbg!(dbg!(game.0.valid_moves()).contains(&coord)) {
                    place_piece(game, coord);
                }
            }
        }
    });
}

unsafe fn place_piece(mut game: Mut<'_, BevyReversi>, coord: (usize, usize)) {
    game.0.place_piece(coord);
    game.0.switch_players();
    game.0.update_valid_moves();
    bot_make_move(game);
}

unsafe fn bot_make_move(mut game: Mut<'_, BevyReversi>) {
    // TODO make "bot get move with delay" function
    let sleep_time = time::Duration::from_millis(1500);
    thread::sleep(sleep_time);
    let bot_move_coord = {
        let game = game.0.clone();
        let mut bot = game.bot_player().expect("bot").1.clone();
        bot.get_move(game)
    };
    game.0.place_piece(bot_move_coord);
    game.0.switch_players();
    game.0.update_valid_moves();
}
