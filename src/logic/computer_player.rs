use crate::common::Location;

use super::*;

#[derive(Default)]
struct Spots {
    i: usize,
}

impl Iterator for Spots {
    type Item = Location;

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        let x = self.i % 4;
        let y = self.i % 16 / 4;
        let z = self.i / 16;
        if x < 4 && y < 4 && z < 4 {
            Some(Location { x, y, z })
        } else {
            None
        }
    }
}

fn next(player: Player, board: &Board, look_ahead: u8) -> Location {
    if look_ahead == 1 {
        return next_one(Player::A, board);
    }
    for loc in Spots::default() {
        let mut board = board.clone();
        if let Ok(res) = board.place(player, loc) {
            if res != PlaceResult::GameOver {
                return loc;
            }
        }
    }
    next_open(board).unwrap()
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
}
