use bevy::prelude::*;

use crate::{
    bot_algorithm::BotAlgorithm, bot_difficulty::BotDifficulty, player::Player, reversi::Reversi,
};

#[derive(Default, Debug)]
pub struct BevyGameConfig {
    pub difficulty: Option<BotDifficulty>,
    pub algorithm: Option<BotAlgorithm>,
}

#[derive(Resource)]
pub struct BevyMenuContent {
    pub camera: Entity,
    pub menu: Entity,
    pub config: BevyGameConfig,
}

#[derive(Component)]
pub struct BevySquare;

#[derive(Component)]
pub struct BevyPiece;

#[derive(Component)]
pub struct BevyPlayerScore {
    pub player: Player,
    pub text_style: TextStyle,
    pub piece_counts: PieceCounts,
}

#[derive(Component)]
pub struct BevyCurrentPlayer;

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub struct PieceCounts {
    pub green: usize,
    pub red: usize,
}

impl PieceCounts {
    pub fn get(&self, player: Player) -> usize {
        match player {
            Player::Green => self.green,
            Player::Red => self.red,
        }
    }

    pub fn set(&mut self, player: Player, value: usize) {
        match player {
            Player::Green => self.green = value,
            Player::Red => self.red = value,
        }
    }
}

#[derive(Resource, Debug)]
pub struct BevyAiDelay(pub Timer);

impl Default for BevyAiDelay {
    fn default() -> Self {
        Self(Timer::from_seconds(1.5, TimerMode::Once))
    }
}

#[derive(Component, Debug)]
pub struct BevyBotDifficulty(pub BotDifficulty);

#[derive(Component, Debug)]
pub struct BevyBotAlgorithm(pub BotAlgorithm);

#[derive(Component, Debug)]
pub struct BevyPlayButton;

#[derive(Component, Default, Debug)]
pub struct BevyMetricsDisplay;

#[derive(Resource, Debug)]
pub struct BevyReversi(pub Reversi);

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
