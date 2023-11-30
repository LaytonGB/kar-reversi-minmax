use async_recursion::async_recursion;
use futures::{executor::block_on, future::join_all};
use std::sync::{Arc, RwLock};

use crate::{
    board::Board, bot_algorithm::BotAlgorithm, bot_heuristic::BotHeuristic, player::Player,
    reversi::Reversi,
};

#[derive(Clone, Debug)]
pub struct Bot {
    algorithm: BotAlgorithm,
    max_depth: Option<usize>,
    heuristic: BotHeuristic,
    expansions: usize,
    comparisons: usize,
}

impl Default for Bot {
    fn default() -> Self {
        Self {
            algorithm: BotAlgorithm::MinMax,
            max_depth: Default::default(),
            heuristic: BotHeuristic::UniformWeighting,
            expansions: Default::default(),
            comparisons: Default::default(),
        }
    }
}

impl Bot {
    pub fn new(algorithm: BotAlgorithm, max_depth: Option<usize>, heuristic: BotHeuristic) -> Self {
        Self {
            algorithm,
            max_depth,
            heuristic,
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
            BotAlgorithm::Async => self.get_move_async(game),
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

    fn eval(heuristic: BotHeuristic, board: &Board, player: Player) -> i64 {
        match heuristic {
            BotHeuristic::UniformWeighting => Self::uniform_eval(board, player),
            BotHeuristic::TacticalWeighting => Self::tactical_eval(board, player),
        }
    }

    fn uniform_eval(board: &Board, player: Player) -> i64 {
        board.pieces_for_player(player).count() as i64
            - board.pieces_for_player(player.other()).count() as i64
    }

    fn tactical_eval(board: &Board, player: Player) -> i64 {
        board
            .pieces_for_player(player)
            .map(|coord| Self::get_tactical_eval_score_for_coord(board, coord))
            .sum::<i64>()
            - board
                .pieces_for_player(player.other())
                .map(|coord| Self::get_tactical_eval_score_for_coord(board, coord))
                .sum::<i64>()
    }

    fn get_tactical_eval_score_for_coord(board: &Board, coord: (usize, usize)) -> i64 {
        match (
            coord.0 == 0 || coord.0 == board.size() - 1,
            coord.1 == 0 || coord.1 == board.size() - 1,
        ) {
            (true, true) => 9,
            (true, false) => 3,
            (false, true) => 3,
            (false, false) => 1,
        }
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

    fn get_move_async(&mut self, game: Reversi) -> (usize, usize) {
        let self_arc = Arc::new(RwLock::new(self.clone()));
        let res = block_on(Self::async_negamax(
            self_arc.clone(),
            game.board().clone(),
            self.heuristic,
            game.current_player(),
            game.bot_player().unwrap().0,
            self.max_depth,
            0,
            i64::MIN,
            i64::MAX,
        ));
        *self = self_arc.read().unwrap().clone();
        res.1.unwrap()
    }

    fn minmax(&mut self, game: &mut Reversi, depth: usize) -> (i64, Option<(usize, usize)>) {
        if !Reversi::anyone_can_move(game.board()) || self.max_depth.is_some_and(|md| depth >= md) {
            return (
                Self::eval(
                    self.heuristic,
                    game.board(),
                    game.bot_player().as_ref().unwrap().0,
                ),
                None,
            );
        }

        self.expansions += 1;
        if !Reversi::can_move(game.board(), game.current_player()) {
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
                game.place_piece_and_add_history(m);
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
        if !Reversi::anyone_can_move(game.board()) || self.max_depth.is_some_and(|md| depth >= md) {
            return (
                Self::eval(
                    self.heuristic,
                    game.board(),
                    game.bot_player().as_ref().unwrap().0,
                ),
                None,
                (alpha, beta),
            );
        }

        self.expansions += 1;
        if !Reversi::can_move(game.board(), game.current_player()) {
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
                game.place_piece_and_add_history(m);
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
        if !Reversi::anyone_can_move(game.board()) || self.max_depth.is_some_and(|md| depth >= md) {
            return (
                Self::eval(self.heuristic, game.board(), game.current_player()),
                None,
                (alpha, beta),
            );
        }

        self.expansions += 1;
        if !Reversi::can_move(game.board(), game.current_player()) {
            game.switch_players();
            game.update_valid_moves();
            let res = self.negamax(game, depth + 1, alpha, beta);
            game.switch_players();
            game.update_valid_moves();
            return res;
        } else {
            let mut score = i64::MIN;
            let mut coord = None;
            for m in game.valid_moves().to_vec() {
                game.place_piece_and_add_history(m);
                game.switch_players();
                game.update_valid_moves();
                let (mut new_score, _, (new_alpha, new_beta)) =
                    self.negamax(game, depth + 1, alpha, beta);
                new_score *= -1;
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
            (score, coord, (beta, alpha))
        }
    }

    #[async_recursion]
    async fn async_negamax(
        bot: Arc<RwLock<Bot>>,
        board: Board,
        heuristic: BotHeuristic,
        current_player: Player,
        bot_player: Player,
        max_depth: Option<usize>,
        depth: usize,
        mut alpha: i64,
        mut beta: i64,
    ) -> (i64, Option<(usize, usize)>, (i64, i64)) {
        if !Reversi::anyone_can_move(&board) || max_depth.is_some_and(|md| depth >= md) {
            return (
                Self::eval(heuristic, &board, current_player),
                None,
                (alpha, beta),
            );
        }

        {
            let mut bot = bot.write().unwrap();
            bot.expansions += 1;
            if bot.expansions % 10000 == 0 {
                println!("{}", bot.expansions);
            }
        }
        if !Reversi::can_move(&board, current_player) {
            Self::async_negamax(
                bot,
                board,
                heuristic,
                current_player.other(),
                bot_player,
                max_depth,
                depth + 1,
                beta,
                alpha,
            )
            .await
        } else {
            let mut score = i64::MIN;
            let mut coord = None;
            let valid_moves: Vec<_> =
                Reversi::get_valid_moves_for_player(&board, current_player).collect();
            let mut futures = Vec::new();
            for &m in &valid_moves {
                let mut new_board = board.clone();
                Reversi::place_piece_on_board(&mut new_board, m, current_player);
                futures.push(Self::async_negamax(
                    bot.clone(),
                    new_board,
                    heuristic,
                    current_player.other(),
                    bot_player,
                    max_depth,
                    depth + 1,
                    beta,
                    alpha,
                ));
            }

            let results = valid_moves
                .into_iter()
                .zip(join_all(futures.into_iter()).await.into_iter());
            for (m, (mut new_score, _, (new_alpha, new_beta))) in results {
                {
                    let mut bot = bot.write().unwrap();
                    bot.comparisons += 1;
                }
                new_score *= -1;
                (alpha, beta) = (alpha.max(new_alpha), beta.min(new_beta));
                if new_score > score {
                    score = new_score;
                    coord = Some(m);
                }
                if alpha > beta {
                    break;
                }
            }

            (score, coord, (beta, alpha))
        }
    }

    pub fn get_metrics(&self) -> (usize, usize) {
        (self.expansions, self.comparisons)
    }
}
