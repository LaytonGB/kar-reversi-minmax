use crate::{bot_algorithm::BotAlgorithm, reversi::Reversi};

#[derive(Clone, Debug)]
pub struct Bot {
    algorithm: BotAlgorithm,
    max_depth: Option<usize>,
    expansions: usize,
    exploration: usize,
}

impl Default for Bot {
    fn default() -> Self {
        Self {
            algorithm: BotAlgorithm::MinMax,
            max_depth: Default::default(),
            expansions: Default::default(),
            exploration: Default::default(),
        }
    }
}

impl Bot {
    pub(crate) fn new(algorithm: BotAlgorithm, max_depth: Option<usize>) -> Self {
        Self {
            algorithm,
            max_depth,
            ..Default::default()
        }
    }

    pub(crate) fn get_move(&mut self, game: Reversi) -> (usize, usize) {
        match self.algorithm {
            BotAlgorithm::MinMax => self.get_move_minmax(game),
            BotAlgorithm::AlphaBeta => todo!(),
            BotAlgorithm::NegaMax => todo!(),
        }
    }

    fn eval(game: &Reversi) -> i64 {
        let bot_player = game.bot_player().unwrap().0;
        game.board().pieces_for_player(bot_player).count() as i64
            - game.board().pieces_for_player(bot_player.other()).count() as i64
    }

    fn get_move_minmax(&mut self, mut game: Reversi) -> (usize, usize) {
        self.expansions = 0;
        self.exploration = 0;
        self.minmax(&mut game, 0).1.unwrap()
    }

    fn minmax(&mut self, game: &mut Reversi, depth: usize) -> (i64, Option<(usize, usize)>) {
        if !game.anyone_can_move() || self.max_depth.is_some_and(|md| depth >= md) {
            (Self::eval(game), None)
        } else if !game.can_move(game.current_player()) {
            self.expansions += 1;
            game.switch_players();
            game.update_valid_moves();
            let res = self.minmax(game, depth + 1);
            game.switch_players();
            game.update_valid_moves();
            return res;
        } else {
            let (mut score, score_compare): (_, Box<dyn Fn(i64, i64) -> bool>) =
                if game.current_player() == game.bot_player().unwrap().0 {
                    (
                        i64::MIN,
                        Box::new(|new_score: i64, score: i64| new_score > score),
                    )
                } else {
                    (
                        i64::MAX,
                        Box::new(|new_score: i64, score: i64| new_score < score),
                    )
                };
            self.expansions += 1;
            let mut coord = None;
            for m in game.valid_moves().to_vec() {
                game.place_piece(m);
                game.switch_players();
                game.update_valid_moves();
                let (new_score, _) = self.minmax(game, depth + 1);
                game.undo_turn();
                game.update_valid_moves();

                if score_compare(new_score, score) {
                    score = new_score;
                    coord = Some(m);
                }
            }
            (score, coord)
        }
    }
}
