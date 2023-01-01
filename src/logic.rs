use std::fmt::Display;

use crate::common::Location;

use self::calculated::LINES;

pub mod computer_player;

const SIZE: usize = 4;

mod calculated;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Player {
    A,
    B,
}

impl Player {
    pub fn other_player(&self) -> Self {
        if *self == Player::A {
            Player::B
        } else {
            Player::A
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Player::A => "Player A",
            Player::B => "Player B",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    pub spots: [[[Option<Player>; SIZE]; SIZE]; SIZE],
}

impl Board {
    pub fn new() -> Self {
        Self {
            spots: [[[None; 4]; 4]; 4],
        }
    }

    pub fn place(&mut self, player: Player, loc: Location) -> Result<PlaceResult, PlaceErr> {
        if self.spots[loc.z][loc.y][loc.x].is_some() {
            Err(PlaceErr::Occupied)
        } else {
            self.spots[loc.z][loc.y][loc.x] = Some(player);
            if LINES.iter().any(|sol| {
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
pub struct TTTTState {
    pub board: Board,
    pub status: GamePlayStatus,
    pub players: Vec<Player>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePlayStatus {
    Playing(Player),
    Draw,
    Win(Player),
}

impl TTTTState {
    pub fn play(&mut self, loc: Location) {
        match self.status {
            GamePlayStatus::Playing(player) => match self.board.place(player, loc) {
                Ok(PlaceResult::Continue) => {
                    self.status = GamePlayStatus::Playing(player.other_player());
                }
                Ok(PlaceResult::GameOver) => self.status = GamePlayStatus::Win(player),
                Err(PlaceErr::Occupied) => (),
            },
            GamePlayStatus::Draw => (),
            GamePlayStatus::Win(_) => (),
        }
    }

    pub fn turn(&self) -> usize {
        self.board.spots.iter().fold(0, |prev, plane| {
            plane.iter().fold(prev, |prev, row| {
                prev + row.iter().filter(|p| p.is_some()).count()
            })
        })
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
        let loc = (0, 1, 2).into();
        board.place(Player::A, loc).unwrap();
        let result = board.place(Player::A, loc);
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
