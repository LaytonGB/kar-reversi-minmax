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
                let coord = game_coord_to_reversi_coord((x, z));
                if game.0.valid_moves().contains(&coord) {
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
