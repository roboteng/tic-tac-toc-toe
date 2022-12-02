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
            Some((x, y, z))
        } else {
            None
        }
    }
}

fn next(player: Player, board: &Board) -> Location {
    for loc in Spots::default() {
        let mut board = board.clone();
        if let Ok(res) = board.place(player, loc.0, loc.1, loc.2) {
            if res == PlaceResult::GameOver {
                return loc;
            }
        }
    }
    (0, 0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_a_one_step_look_ahead_ai_should_win_if_possible() {
        let mut board = Board::new();
        board.place(Player::A, 0, 0, 0).unwrap();
        board.place(Player::A, 1, 0, 0).unwrap();
        board.place(Player::A, 2, 0, 0).unwrap();

        let next_move = next(Player::A, &board);
        assert_eq!(next_move, (3, 0, 0));
    }
}
