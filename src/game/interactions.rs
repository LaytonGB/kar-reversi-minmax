use bevy::{
    ecs::system::{EntityCommands, RunSystemOnce},
    prelude::*,
};
use bevy_mod_picking::prelude::{EntityEvent, ListenerInput};
use strum::IntoEnumIterator;

use crate::game::{
    states::GameState,
    structs::{BevyAiDelay, BevyCurrentPlayer, BevyPlayerScore, BevyReversi},
    utils::*,
};

use crate::player::Player;

pub fn click_grid_square<E>(_: &ListenerInput<E>, commands: &mut EntityCommands)
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
                let coord = game_coord_to_reversi_coord((x, z));
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

pub fn into_ai_turn_state(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::AiTurn);
}

pub fn bot_delay_reset(mut timer: ResMut<BevyAiDelay>) {
    let timer = &mut timer.0;
    timer.reset();
}

pub fn bot_make_move(
    mut game: ResMut<BevyReversi>,
    time: Res<Time>,
    mut timer: ResMut<BevyAiDelay>,
    mut state: ResMut<NextState<GameState>>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        if game.0.can_move(Player::Red) {
            let bot_move_coord = {
                let game = game.0.clone();
                let mut bot = game.bot_player().expect("bot").1.clone();
                bot.get_move(game)
            };
            game.0.place_piece(bot_move_coord);
        }
        game.0.switch_players();
        game.0.update_valid_moves();
        state.set(GameState::PlayerTurn);
    }
}

pub fn update_current_player(
    game: Res<BevyReversi>,
    mut query: Query<&mut Text, With<BevyCurrentPlayer>>,
) {
    if game.is_changed() {
        for mut text in &mut query {
            text.set(Box::new(Text::from_section(
                format!("Turn: {}", game.0.current_player()),
                TextStyle {
                    font: default(),
                    font_size: 26.0,
                    color: Color::WHITE,
                },
            )))
            .ok();
        }
    }
}

pub fn update_player_scores(
    mut game: ResMut<BevyReversi>,
    mut query: Query<&mut BevyPlayerScore>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if game.is_changed() {
        for mut bps in &mut query {
            let pc = &mut bps.piece_counts;
            for player in Player::iter() {
                pc.set(player, game.0.board().pieces_for_player(player).count());
            }
        }
        if !game.0.anyone_can_move() {
            next_state.set(GameState::End);
            todo!("game over");
        } else if *state.get() == GameState::PlayerTurn && !game.0.can_move(Player::Green) {
            game.0.switch_players();
            game.0.update_valid_moves();
            next_state.set(GameState::AiTurn);
        }
    }
}

pub fn maintain_score_display(mut commands: Commands, query: Query<(Entity, &BevyPlayerScore)>) {
    for (
        entity,
        BevyPlayerScore {
            player,
            text_style,
            piece_counts,
        },
    ) in &query
    {
        let mut text = commands.entity(entity);
        text.insert(TextBundle {
            text: Text::from_section(
                format!("{}: {}", player, piece_counts.get(*player)),
                text_style.clone(),
            ),
            ..Default::default()
        });
    }
}
