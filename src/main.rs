use kar_reversi_minmax::{
    bot_algorithm::BotAlgorithm, bot_difficulty::BotDifficulty, player::Player, reversi::Reversi,
};

fn main() {
    let mut game = Reversi::new(Some((
        Player::Black,
        BotAlgorithm::MinMax,
        BotDifficulty::Easy,
    )));
    game.start()
}
