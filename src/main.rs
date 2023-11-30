use kar_reversi_minmax::bot_heuristic::BotHeuristic;

fn main() {
    #[cfg(all(feature = "terminal", feature = "game"))]
    compile_error!("cannot enable both terminal and game");

    #[cfg(feature = "terminal")]
    {
        use kar_reversi_minmax::{
            bot_algorithm::BotAlgorithm, bot_difficulty::BotDifficulty, player::Player,
            reversi::Reversi, utils::clear_terminal,
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
            clear_terminal();
        }

        let mut algorithm: Option<BotAlgorithm> = None;
        while algorithm.is_none() {
            println!("Enter an algorithm (caps matter):");
            for d in BotAlgorithm::iter() {
                println!("{}", d);
            }
            println!();
            algorithm = try_read!().ok();
            clear_terminal();
        }

        let mut heuristic: Option<BotHeuristic> = None;
        while heuristic.is_none() {
            println!("Enter a heuristic method (caps matter):");
            for h in BotHeuristic::iter() {
                println!("{}", h);
            }
            println!();
            heuristic = try_read!().ok();
            clear_terminal();
        }

        let mut game = Reversi::new(Some((
            Player::Red,
            difficulty.unwrap(),
            algorithm.unwrap(),
            heuristic.unwrap(),
        )));
        game.start()
    }

    #[cfg(feature = "game")]
    {
        use kar_reversi_minmax::game::game;

        game::run_game();
    }
}
