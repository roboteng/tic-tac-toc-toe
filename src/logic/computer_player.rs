use crate::common::Location;

use super::*;

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
    let mut loc = Location::new(0, 0, 0);
    let mut max_eval = f32::MIN;
    for spot in Spots::default() {
        let mut board = board.clone();
        if board.place(player, spot).is_ok() {
            let eval = minmax(player, &board, look_ahead - 1);
            println!("{spot:?} -> {eval}");
            if eval > max_eval {
                loc = spot;
                max_eval = eval;
            }
        }
    }
    loc
}

fn minmax(player: Player, board: &Board, look_ahead: u8) -> f32 {
    if look_ahead == 0 {
        return eval(player, board);
    }

    let mut max_eval = f32::MIN;
    let mut loc = None;
    for spot in Spots::default() {
        let mut board = board.clone();
        if let Ok(_) = board.place(player, spot) {
            let eval = minmax(
                if player == Player::A {
                    Player::B
                } else {
                    Player::A
                },
                &board,
                look_ahead - 1,
            );
            if -eval > max_eval {
                max_eval = eval;
                loc = Some(spot);
            }
        }
    }
    max_eval
}

fn next_one(player: Player, board: &Board) -> Location {
    for loc in Spots::default() {
        let mut board = board.clone();
        if let Ok(res) = board.place(player, loc) {
            if res == PlaceResult::GameOver {
                return loc;
            }
        }
    }
    next_open(board).unwrap()
}

fn next_open(board: &Board) -> Option<Location> {
    for loc in Spots::default() {
        let mut board = board.clone();
        if board.place(Player::A, loc).is_ok() {
            return Some(loc);
        }
    }
    None
}

fn eval(player: Player, board: &Board) -> f32 {
    board
        .lines
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
    #[ignore = "Fix later"]
    fn should_block_move_if_opponent_would_win() {
        let mut board = Board::new();
        board.place(Player::A, Location::new(0, 0, 0)).unwrap();
        board.place(Player::A, Location::new(1, 0, 0)).unwrap();
        board.place(Player::A, Location::new(2, 0, 0)).unwrap();

        let next_move = next(Player::B, &board, 2);
        assert_eq!(next_move, Location::new(3, 0, 0));
    }

    #[test]
    fn board_with_more_lines_should_be_better() {
        let mut good_board = Board::new();
        good_board.place(Player::A, Location::new(1, 0, 0)).unwrap();
        good_board.place(Player::A, Location::new(2, 0, 0)).unwrap();
        good_board.place(Player::A, Location::new(0, 1, 0)).unwrap();
        good_board.place(Player::A, Location::new(0, 2, 0)).unwrap();

        let mut bad_board = Board::new();
        bad_board.place(Player::A, Location::new(1, 0, 0)).unwrap();
        bad_board.place(Player::A, Location::new(0, 1, 0)).unwrap();
        bad_board.place(Player::A, Location::new(3, 2, 3)).unwrap();
        bad_board.place(Player::A, Location::new(0, 3, 1)).unwrap();

        assert!(eval(Player::A, &good_board) > eval(Player::A, &bad_board));
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

        assert!(eval(Player::A, &good_board) > eval(Player::A, &bad_board));
    }
}
