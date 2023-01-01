use super::*;
use crate::common::Location;
use minimax::{self, Strategy};

#[derive(Debug, Clone, Copy)]
struct TTTT;
impl minimax::Game for TTTT {
    type S = TTTTState;
    type M = TTTTMove;

    fn generate_moves(state: &TTTTState, moves: &mut Vec<TTTTMove>) {
        match state.status {
            GamePlayStatus::Playing(player) => {
                let spots = state.board.spots;
                for (z, plane) in spots.iter().enumerate() {
                    for (y, row) in plane.iter().enumerate() {
                        for (x, spot) in row.iter().enumerate() {
                            if spot.is_none() {
                                let m = TTTTMove {
                                    player,
                                    loc: Location::new(x, y, z),
                                };
                                moves.push(m);
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }

    fn get_winner(state: &TTTTState) -> Option<minimax::Winner> {
        match state.status {
            GamePlayStatus::Playing(_) => None,
            GamePlayStatus::Draw => Some(minimax::Winner::Draw),
            GamePlayStatus::Win(_) => Some(minimax::Winner::PlayerJustMoved),
        }
    }
}

impl minimax::Move for TTTTMove {
    type G = TTTT;

    fn apply(&self, state: &mut TTTTState) {
        state.board.place(self.player, self.loc).unwrap();
    }

    fn undo(&self, state: &mut TTTTState) {
        let Location { x, y, z } = self.loc;
        state.board.spots[z][y][x] = None;
        state.status = GamePlayStatus::Playing(self.player);
    }
}

#[derive(Debug, Clone, Copy)]
struct TTTTMove {
    player: Player,
    loc: Location,
}

struct Eval;
impl minimax::Evaluator for Eval {
    type G = TTTT;

    fn evaluate(&self, s: &TTTTState) -> minimax::Evaluation {
        match s.status {
            GamePlayStatus::Playing(_) => -eval(&s.board) as minimax::Evaluation,
            GamePlayStatus::Draw => 0 as minimax::Evaluation,
            GamePlayStatus::Win(player) => {
                if player == s.players[0] {
                    minimax::BEST_EVAL
                } else {
                    minimax::WORST_EVAL
                }
            }
        }
    }
}

#[derive(Default)]
struct Spots {
    i: usize,
}

impl Iterator for Spots {
    type Item = Location;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.i % 4;
        let y = self.i % 16 / 4;
        let z = self.i / 16;
        if x < 4 && y < 4 && z < 4 {
            self.i += 1;
            Some(Location { x, y, z })
        } else {
            None
        }
    }
}

fn next(player: Player, board: &Board, look_ahead: u8) -> Location {
    let start = TTTTState {
        board: board.clone(),
        status: GamePlayStatus::Playing(player),
        players: vec![Player::A, Player::B],
    };
    let mut strategy = minimax::Negamax::new(Eval, look_ahead);
    let k = strategy.choose_move(&start);

    k.unwrap().loc
}

fn eval(board: &Board) -> f32 {
    eval_sub(Player::A, board) - eval_sub(Player::B, board)
}

fn eval_sub(player: Player, board: &Board) -> f32 {
    LINES
        .iter()
        .map(|l| {
            let mut score = 1.0_f32;
            for spot in l {
                if let Some(p) = board.at(*spot) {
                    if p != player {
                        return 0.0;
                    }
                    score *= 2.0;
                }
            }
            score
        })
        .reduce(|a, b| a + b)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_a_one_step_look_ahead_ai_should_win_if_possible() {
        let mut board = Board::new();
        board.place(Player::A, Location::new(0, 0, 0)).unwrap();
        board.place(Player::A, Location::new(1, 0, 0)).unwrap();
        board.place(Player::A, Location::new(2, 0, 0)).unwrap();

        let next_move = next(Player::A, &board, 1);
        assert_eq!(next_move, Location::new(3, 0, 0));
    }

    #[test]
    fn should_block_move_if_opponent_would_win() {
        let mut board = Board::new();
        board.place(Player::A, Location::new(0, 0, 0)).unwrap();
        board.place(Player::A, Location::new(1, 0, 0)).unwrap();
        board.place(Player::A, Location::new(2, 0, 0)).unwrap();

        let next_move = next(Player::B, &board, 2);
        assert_eq!(next_move, Location::new(3, 0, 0));
    }

    #[test]
    fn board_with_line_blocked_should_be_worse_than_unblocked() {
        let mut good_board = Board::new();
        good_board.place(Player::A, Location::new(0, 0, 0)).unwrap();
        good_board.place(Player::A, Location::new(3, 0, 0)).unwrap();
        good_board.place(Player::B, Location::new(0, 2, 0)).unwrap();

        let mut bad_board = Board::new();
        bad_board.place(Player::A, Location::new(0, 0, 0)).unwrap();
        bad_board.place(Player::A, Location::new(3, 0, 0)).unwrap();
        bad_board.place(Player::B, Location::new(2, 0, 0)).unwrap();

        assert!(eval(&good_board) > eval(&bad_board));
    }
}
