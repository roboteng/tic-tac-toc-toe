struct Board(bool);

impl Board {
    fn new() -> Self {
        Self(false)
    }

    fn place(&mut self, _: usize, _: usize, _: usize) -> Result<PlaceResult, PlaceErr> {
        if !self.0 {
            self.0 = true;
            Ok(PlaceResult::Continue)
        } else {
            Err(PlaceErr::Occupied)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum PlaceResult {
    Continue,
}

#[derive(Debug, PartialEq, Eq)]
enum PlaceErr {
    Occupied,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placing_on_an_empty_board_is_successful() {
        let mut board = Board::new();
        let result = board.place(0, 1, 2);
        assert_eq!(result, Ok(PlaceResult::Continue));
    }

    #[test]
    fn cannot_place_on_the_same_spot_twice() {
        let mut board = Board::new();
        board.place(0, 1, 2).unwrap();
        let result = board.place(0, 1, 2);
        assert_eq!(result, Err(PlaceErr::Occupied));
    }
}
