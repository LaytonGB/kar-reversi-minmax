use bevy::prelude::*;

use crate::{
    bot_algorithm::BotAlgorithm, bot_difficulty::BotDifficulty, player::Player, reversi::Reversi,
};

#[derive(Component)]
pub(crate) struct BevySquare;

#[derive(Component)]
pub(crate) struct BevyPiece;

#[derive(Resource, Debug)]
pub(crate) struct BevyReversi(pub Reversi);

impl Default for BevyReversi {
    fn default() -> Self {
        let mut game = Reversi::new(Some((
            Player::Red,
            BotAlgorithm::MinMax,
            BotDifficulty::Easy,
        )));
        game.update_valid_moves();

        Self(game)
    }
}
