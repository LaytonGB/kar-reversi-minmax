use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_mod_picking::{
    highlight::PickHighlight,
    prelude::{EntityEvent, ListenerInput},
};

use crate::{
    bevy_structs::{BevyReversi, BevySquare},
    bevy_utils::*,
    player::Player,
};

pub(crate) fn highlight_valid_grid_squares(
    mut commands: Commands,
    queries: Query<(Entity, &Transform), With<BevySquare>>,
    game: Res<BevyReversi>,
) {
    // FIXME idk whats going on here
    // squares that are not in valid moves are getting highlighted
    if game.is_changed() {
        for (square, transform) in &queries {
            let mut square = commands.entity(square);
            let Vec3 { x, z, .. } = transform.translation;
            let coord = game_coord_to_reversi_coord((x, z));
            if game.0.valid_moves().contains(&coord) {
                square.insert(PickHighlight);
            } else {
                square.remove::<PickHighlight>();
            }
        }
    }
}

pub(crate) fn click_grid_square<E>(_: &ListenerInput<E>, commands: &mut EntityCommands)
where
    E: EntityEvent,
{
    commands.add(|mut world_entity: EntityWorldMut<'_>| unsafe {
        let transform = world_entity.get::<Transform>().cloned();
        let mut game = world_entity.world_mut().resource_mut::<BevyReversi>();
        if game.0.current_player() == Player::Green {
            if let Some(transform) = transform {
                let Vec3 { x, z, .. } = transform.translation;
                let coord = dbg!(game_coord_to_reversi_coord((x, z)));
                if dbg!(game.0.valid_moves()).contains(&coord) {
                    place_piece(&mut game, coord);
                    bot_make_move(game);
                }
            }
        }
    });
}

unsafe fn place_piece(game: &mut Mut<'_, BevyReversi>, coord: (usize, usize)) {
    game.0.place_piece(coord);
    game.0.switch_players();
    game.0.update_valid_moves();
}

unsafe fn bot_make_move(mut game: Mut<'_, BevyReversi>) {
    // TODO make "bot get move with delay" function
    // let sleep_time = time::Duration::from_millis(1500);
    // thread::sleep(sleep_time);
    let bot_move_coord = {
        let game = game.0.clone();
        let mut bot = game.bot_player().expect("bot").1.clone();
        bot.get_move(game)
    };
    game.0.place_piece(bot_move_coord);
    game.0.switch_players();
    game.0.update_valid_moves();
}
