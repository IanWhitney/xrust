extern crate queen_attack;
use queen_attack::*;

pub struct Knight {
    p: ChessPosition,
}

impl ChessPiece for Knight {
    fn valid_position(position: &ChessPosition) -> bool {
        position.rank >= 0 && position.rank <= 7 && position.file >= 0 && position.file <= 7
    }

    fn position(&self) -> &ChessPosition {
        &self.p
    }

    fn can_attack<T: ChessPiece>(&self, piece: &T) -> bool {
        (
            (self.position().rank - piece.position().rank).abs() == 1 &&
            (self.position().file - piece.position().file).abs() == 2
        ) || (
            (self.position().file - piece.position().file).abs() == 1 &&
            (self.position().rank - piece.position().rank).abs() == 2
        )
    }
}

impl Knight {
    pub fn new(position: (i8, i8)) -> Result<Knight, ()> {
        let position = ChessPosition::new::<Knight>(position);
        if position.is_ok() {
            Ok(Knight { p: position.unwrap() })
        } else {
            Err(())
        }
    }
}
