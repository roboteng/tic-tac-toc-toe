use crate::common::Location;

pub mod computer_player;

const SIZE: usize = 4;
fn generate_lines() -> Vec<Vec<Location>> {
    let fs: [fn(usize) -> usize; 6] = [|_| 0, |_| 1, |_| 2, |_| 3, |x| x, |x| 3 - x];
    let mut triples: Vec<Vec<Location>> = Vec::new();
    for fx in fs {
        for fy in fs {
            for fz in fs {
                let mut sol = Vec::new();
                for i in 0..SIZE {
                    sol.push(Location::new(fx(i), fy(i), fz(i)));
                }
                if !sol.iter().skip(1).any(|p| *p == sol[0])
                    && !triples
                        .iter()
                        .any(|known_sol| known_sol[0] == sol[3] && known_sol[3] == sol[0])
                {
                    triples.push(sol);
                }
            }
        }
    }
    triples
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Player {
    A,
    B,
}

#[derive(Debug, Clone)]
pub struct Board {
    pub spots: [[[Option<Player>; SIZE]; SIZE]; SIZE],
    lines: Vec<Vec<Location>>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            lines: generate_lines(),
            spots: [
                [
                    [None, None, None, None],
                    [None, None, None, None],
                    [None, None, None, None],
                    [None, None, None, None],
                ],
                [
                    [None, None, None, None],
                    [None, None, None, None],
                    [None, None, None, None],
                    [None, None, None, None],
                ],
                [
                    [None, None, None, None],
                    [None, None, None, None],
                    [None, None, None, None],
                    [None, None, None, None],
                ],
                [
                    [None, None, None, None],
                    [None, None, None, None],
                    [None, None, None, None],
                    [None, None, None, None],
                ],
            ],
        }
    }

    pub fn place(&mut self, player: Player, loc: Location) -> Result<PlaceResult, PlaceErr> {
        if self.spots[loc.z][loc.y][loc.x].is_some() {
            Err(PlaceErr::Occupied)
        } else {
            self.spots[loc.z][loc.y][loc.x] = Some(player);
            if self.lines.iter().any(|sol| {
                sol.iter().all(|p| {
                    if let Some(p) = self.at(*p) {
                        p == player
                    } else {
                        false
                    }
                })
            }) {
                Ok(PlaceResult::GameOver)
            } else {
                Ok(PlaceResult::Continue)
            }
        }
    }

    pub fn at(&self, loc: Location) -> Option<Player> {
        self.spots[loc.z][loc.y][loc.x]
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlaceResult {
    Continue,
    GameOver,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlaceErr {
    Occupied,
}

#[derive(Debug, Clone)]
pub struct Game {
    pub board: Board,
    pub status: GamePlayStatus,
    pub players: Vec<Player>,
    pub turn: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePlayStatus {
    Playing(Player),
    Draw,
    Win(Player),
}

impl Game {
    pub fn play(&mut self, loc: Location) {
        match self.status {
            GamePlayStatus::Playing(player) => match self.board.place(player, loc) {
                Ok(PlaceResult::Continue) => {
                    self.turn += 1;
                    let next_player = self.players[self.turn % self.players.len()];
                    self.status = GamePlayStatus::Playing(next_player);
                }
                Ok(PlaceResult::GameOver) => self.status = GamePlayStatus::Win(player),
                Err(PlaceErr::Occupied) => (),
            },
            GamePlayStatus::Draw => (),
            GamePlayStatus::Win(p) => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placing_on_an_empty_board_is_successful() {
        let mut board = Board::new();
        let result = board.place(Player::A, Location { x: 0, y: 1, z: 2 });
        assert_eq!(result, Ok(PlaceResult::Continue));
    }

    #[test]
    fn cannot_place_on_the_same_spot_twice() {
        let mut board = Board::new();
        board.place(Player::A, Location::new(0, 1, 2)).unwrap();
        let result = board.place(Player::A, Location::new(0, 1, 2));
        assert_eq!(result, Err(PlaceErr::Occupied));
    }

    #[test]
    fn placing_on_a_second_empty_spot_is_valid() {
        let mut board = Board::new();
        board.place(Player::A, Location::new(0, 1, 2)).unwrap();
        let result = board.place(Player::A, Location::new(1, 2, 3));
        assert_eq!(result, Ok(PlaceResult::Continue));
    }

    #[test]
    fn the_game_is_over_when_4_are_placed_in_a_line() {
        let mut board = Board::new();
        board.place(Player::A, Location::new(0, 0, 0)).unwrap();
        board.place(Player::A, Location::new(1, 0, 0)).unwrap();
        board.place(Player::A, Location::new(2, 0, 0)).unwrap();
        let result = board.place(Player::A, Location::new(3, 0, 0));
        assert_eq!(result, Ok(PlaceResult::GameOver));
    }

    #[test]
    fn the_game_continues_when_4_are_not_placed_in_a_line() {
        let mut board = Board::new();
        board.place(Player::A, Location::new(0, 0, 0)).unwrap();
        board.place(Player::A, Location::new(1, 0, 0)).unwrap();
        board.place(Player::A, Location::new(2, 0, 0)).unwrap();
        let result = board.place(Player::A, Location::new(0, 1, 0));
        assert_eq!(result, Ok(PlaceResult::Continue));
    }

    #[test]
    fn the_game_continues_when_4_in_a_row_arent_the_same_player() {
        let mut board = Board::new();
        board.place(Player::A, Location::new(0, 0, 0)).unwrap();
        board.place(Player::A, Location::new(1, 0, 0)).unwrap();
        board.place(Player::A, Location::new(2, 0, 0)).unwrap();
        let result = board.place(Player::B, Location::new(3, 0, 0));
        assert_eq!(result, Ok(PlaceResult::Continue));
    }
}
