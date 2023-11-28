use crate::{bot_algorithm::BotAlgorithm, reversi::Reversi};

#[derive(Clone, Debug)]
pub struct Bot {
    algorithm: BotAlgorithm,
    max_depth: Option<usize>,
    expansions: usize,
    comparisons: usize,
}

impl Default for Bot {
    fn default() -> Self {
        Self {
            algorithm: BotAlgorithm::MinMax,
            max_depth: Default::default(),
            expansions: Default::default(),
            comparisons: Default::default(),
        }
    }
}

impl Bot {
    pub fn new(algorithm: BotAlgorithm, max_depth: Option<usize>) -> Self {
        Self {
            algorithm,
            max_depth,
            ..Default::default()
        }
    }

    pub fn get_move(&mut self, game: Reversi) -> (usize, usize) {
        self.expansions = 0;
        self.comparisons = 0;
        match self.algorithm {
            BotAlgorithm::MinMax => self.get_move_minmax(game),
            BotAlgorithm::AlphaBeta => self.get_move_alphabeta(game),
            BotAlgorithm::NegaMax => self.get_move_negamax(game),
        }
    }

    #[cfg(feature = "terminal")]
    pub fn show_metrics(&self) {
        println!(
            "This turn's metrics:\n\
            Expansions: {} | Comparisons: {}",
            self.expansions, self.comparisons
        );
    }

    fn eval(game: &Reversi) -> i64 {
        let bot_player = game.bot_player().unwrap().0;
        game.board().pieces_for_player(bot_player).count() as i64
            - game.board().pieces_for_player(bot_player.other()).count() as i64
    }

    fn get_move_minmax(&mut self, mut game: Reversi) -> (usize, usize) {
        self.minmax(&mut game, 0).1.unwrap()
    }

    fn get_move_alphabeta(&mut self, mut game: Reversi) -> (usize, usize) {
        self.alphabeta(&mut game, 0, i64::MIN, i64::MAX).1.unwrap()
    }

    fn get_move_negamax(&mut self, mut game: Reversi) -> (usize, usize) {
        self.negamax(&mut game, 0, i64::MIN, i64::MAX).1.unwrap()
    }

    fn minmax(&mut self, game: &mut Reversi, depth: usize) -> (i64, Option<(usize, usize)>) {
        if !game.anyone_can_move() || self.max_depth.is_some_and(|md| depth >= md) {
            return (Self::eval(game), None);
        }

        self.expansions += 1;
        if !game.can_move(game.current_player()) {
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
            let mut coord = None;
            for m in game.valid_moves().to_vec() {
                game.place_piece(m);
                game.switch_players();
                game.update_valid_moves();
                let (new_score, _) = self.minmax(game, depth + 1);
                game.undo_turn();
                game.update_valid_moves();

                self.comparisons += 1;
                if score_compare(new_score, score) {
                    score = new_score;
                    coord = Some(m);
                }
            }
            (score, coord)
        }
    }

    fn alphabeta(
        &mut self,
        game: &mut Reversi,
        depth: usize,
        mut alpha: i64,
        mut beta: i64,
    ) -> (i64, Option<(usize, usize)>, (i64, i64)) {
        if !game.anyone_can_move() || self.max_depth.is_some_and(|md| depth >= md) {
            return (Self::eval(game), None, (alpha, beta));
        }

        self.expansions += 1;
        if !game.can_move(game.current_player()) {
            game.switch_players();
            game.update_valid_moves();
            let res = self.alphabeta(game, depth + 1, alpha, beta);
            game.switch_players();
            game.update_valid_moves();
            return res;
        } else {
            let (mut score, is_newscore_better, do_alphabeta_prune): (
                _,
                Box<dyn Fn(i64, i64) -> bool>,
                Box<dyn Fn(i64, i64) -> bool>,
            ) = if game.current_player() == game.bot_player().unwrap().0 {
                (
                    i64::MIN,
                    Box::new(|new_score: i64, score: i64| new_score > score),
                    Box::new(|alpha: i64, beta: i64| alpha > beta),
                )
            } else {
                (
                    i64::MAX,
                    Box::new(|new_score: i64, score: i64| new_score < score),
                    Box::new(|alpha: i64, beta: i64| alpha < beta),
                )
            };
            let mut coord = None;
            for m in game.valid_moves().to_vec() {
                game.place_piece(m);
                game.switch_players();
                game.update_valid_moves();
                let (new_score, _, (new_alpha, new_beta)) =
                    self.alphabeta(game, depth + 1, alpha, beta);
                (alpha, beta) = (alpha.max(new_alpha), beta.min(new_beta));
                game.undo_turn();
                game.update_valid_moves();

                self.comparisons += 1;
                if is_newscore_better(new_score, score) {
                    score = new_score;
                    coord = Some(m);
                }
                if do_alphabeta_prune(alpha, beta) {
                    break;
                }
            }
            (score, coord, (alpha, beta))
        }
    }

    fn negamax(
        &mut self,
        game: &mut Reversi,
        depth: usize,
        mut alpha: i64,
        mut beta: i64,
    ) -> (i64, Option<(usize, usize)>, (i64, i64)) {
        if !game.anyone_can_move() || self.max_depth.is_some_and(|md| depth >= md) {
            return (Self::eval(game), None, (alpha, beta));
        }

        self.expansions += 1;
        if !game.can_move(game.current_player()) {
            game.switch_players();
            game.update_valid_moves();
            let res = self.alphabeta(game, depth + 1, alpha, beta);
            game.switch_players();
            game.update_valid_moves();
            return res;
        } else {
            let mut score = i64::MIN;
            let mut coord = None;
            for m in game.valid_moves().to_vec() {
                game.place_piece(m);
                game.switch_players();
                game.update_valid_moves();
                let (new_score, _, (new_alpha, new_beta)) =
                    self.alphabeta(game, depth + 1, alpha, beta);
                (alpha, beta) = (alpha.max(new_alpha), beta.min(new_beta));
                game.undo_turn();
                game.update_valid_moves();

                self.comparisons += 1;
                if new_score > score {
                    score = new_score;
                    coord = Some(m);
                }
                if alpha > beta {
                    break;
                }
            }
            (-score, coord, (beta, alpha))
        }
    }
}
