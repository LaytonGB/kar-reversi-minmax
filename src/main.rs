fn main() {
    #[cfg(all(feature = "terminal", feature = "game"))]
    compile_error!("cannot enable both terminal and game");

    #[cfg(feature = "terminal")]
    {
        use kar_reversi_minmax::{
            bot_algorithm::BotAlgorithm, bot_difficulty::BotDifficulty, player::Player,
            reversi::Reversi,
        };
        use strum::IntoEnumIterator;
        use text_io::try_read;

        let mut difficulty: Option<BotDifficulty> = None;
        while difficulty.is_none() {
            println!("Enter a difficulty (caps matter):");
            for d in BotDifficulty::iter() {
                println!("{}", d);
            }
            println!();
            difficulty = try_read!().ok();
        }

        let mut game = Reversi::new(Some((
            Player::Red,
            BotAlgorithm::MinMax,
            difficulty.unwrap(),
        )));
        game.start()
    }

    #[cfg(feature = "game")]
    {
        use kar_reversi_minmax::game;

        game::run_game();
    }
}
