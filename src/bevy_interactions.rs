use bevy::{
    ecs::system::{EntityCommands, RunSystemOnce},
    prelude::*,
};
use bevy_mod_picking::prelude::{EntityEvent, ListenerInput};

use crate::{
    bevy_game_state::GameState,
    bevy_structs::{BevyAiDelay, BevyReversi},
    bevy_utils::*,
    player::Player,
};

// pub(crate) fn highlight_valid_grid_squares(
//     mut commands: Commands,
//     queries: Query<(Entity, &Transform), With<BevySquare>>,
//     game: Res<BevyReversi>,
// ) {
//     // FIXME sometimes squares get highlighted green
//     for (square, transform) in &queries {
//         let mut square = commands.entity(square);
//         let Vec3 { x, z, .. } = transform.translation;
//         let coord = game_coord_to_reversi_coord((x, z));
//         if game.0.valid_moves().contains(&coord) {
//             square.insert((PickHighlight, GRID_HIGHLIGHT));
//         } else {
//             square.remove::<(PickHighlight, Highlight<StandardMaterial>)>();
//             square.log_components();
//         }
//     }
// }

pub(crate) fn click_grid_square<E>(_: &ListenerInput<E>, commands: &mut EntityCommands)
where
    E: EntityEvent,
{
    commands.add(|mut world_entity: EntityWorldMut<'_>| unsafe {
        let transform = world_entity.get::<Transform>().cloned();
        let mut game = world_entity.world_mut().resource_mut::<BevyReversi>();
        // TODO replace with GameState check ?
        if game.0.current_player() == Player::Green {
            if let Some(transform) = transform {
                let Vec3 { x, z, .. } = transform.translation;
                let coord = dbg!(game_coord_to_reversi_coord((x, z)));
                if game.0.valid_moves().contains(&coord) {
                    place_piece(&mut game, coord);
                    world_entity.world_mut().run_system_once(into_ai_turn_state);
                }
            }
        }
    });
}

fn place_piece(game: &mut Mut<'_, BevyReversi>, coord: (usize, usize)) {
    game.0.place_piece(coord);
    game.0.switch_players();
    game.0.update_valid_moves();
}

pub(crate) fn into_ai_turn_state(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::AiTurn);
}

pub(crate) fn bot_delay_reset(mut timer: ResMut<BevyAiDelay>) {
    let timer = &mut timer.0;
    timer.reset();
}

pub(crate) fn bot_make_move(
    mut game: ResMut<BevyReversi>,
    time: Res<Time>,
    mut timer: ResMut<BevyAiDelay>,
    mut state: ResMut<NextState<GameState>>,
) {
    dbg!(&timer);
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        let bot_move_coord = {
            let game = game.0.clone();
            let mut bot = game.bot_player().expect("bot").1.clone();
            bot.get_move(game)
        };
        game.0.place_piece(bot_move_coord);
        game.0.switch_players();
        game.0.update_valid_moves();
        state.set(GameState::PlayerTurn);
    }
}
