use kar_reversi_minmax::{
    bot_algorithm::BotAlgorithm, bot_difficulty::BotDifficulty, player::Player, reversi::Reversi,
};
use strum::IntoEnumIterator;
use text_io::try_read;

fn main() {
    let mut difficulty: Option<BotDifficulty> = None;
    while difficulty.is_none() {
        println!("{}[2J", 27 as char);
        println!("Enter a difficulty (caps matter):");
        for d in BotDifficulty::iter() {
            println!("{}", d);
        }
        println!();
        difficulty = try_read!().ok();
    }
    let mut game = Reversi::new(Some((
        Player::Black,
        BotAlgorithm::MinMax,
        difficulty.unwrap(),
    )));
    game.start()
}
