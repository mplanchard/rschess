//! Board representation

use crate::bitboard::BitBoard;
use crate::square::Square;

// Constants for starting piece locations

const STARTING_WHITE_PAWNS: BitBoard = BitBoard::new(0x000000000000ff00);
const STARTING_BLACK_PAWNS: BitBoard = BitBoard::new(0x00ff000000000000);

const STARTING_WHITE_ROOKS: BitBoard = BitBoard::new(0x0000000000000081);
const STARTING_BLACK_ROOKS: BitBoard = BitBoard::new(0x8100000000000000);

const STARTING_WHITE_BISHOPS: BitBoard = BitBoard::new(0x0000000000000042);
const STARTING_BLACK_BISHOPS: BitBoard = BitBoard::new(0x4200000000000000);

const STARTING_WHITE_KNIGHTS: BitBoard = BitBoard::new(0x0000000000000024);
const STARTING_BLACK_KNIGHTS: BitBoard = BitBoard::new(0x2400000000000000);

const STARTING_WHITE_QUEEN: BitBoard = BitBoard::from_square(Square::D1);
const STARTING_BLACK_QUEEN: BitBoard = BitBoard::from_square(Square::D8);

const STARTING_WHITE_KING: BitBoard = BitBoard::from_square(Square::E1);
const STARTING_BLACK_KING: BitBoard = BitBoard::from_square(Square::E8);

// Precalculating starting board
const STARTING_BOARD: Board = Board::fresh_game();

/// A representation of a side's pieces
pub struct Pieces {
    pub king: BitBoard,
    pub queens: BitBoard,
    pub rooks: BitBoard,
    pub bishops: BitBoard,
    pub knights: BitBoard,
    pub pawns: BitBoard,
}
impl Pieces {
    // Construct a new set of pieces
    pub const fn new(
        king: BitBoard,
        queens: BitBoard,
        rooks: BitBoard,
        bishops: BitBoard,
        knights: BitBoard,
        pawns: BitBoard,
    ) -> Self {
        Self {
            king,
            queens,
            rooks,
            bishops,
            knights,
            pawns,
        }
    }
    /// Retrieve a bitboard indicating the position of all the pieces
    pub fn all_pieces(&self) -> BitBoard {
        // lazily set the union on first access. Otherwise don't bother.
        self.king
            .union(&self.queens)
            .union(&self.rooks)
            .union(&self.bishops)
            .union(&self.knights)
            .union(&self.pawns)
    }
}

/// A representation of the game board
pub struct Board {
    pub white: Pieces,
    pub black: Pieces,
}
impl Board {
    /// Construct a new side from a set of pieces
    pub const fn new(white: Pieces, black: Pieces) -> Self {
        Self {
            white,
            black,
        }
    }
    /// Construct a new board with typical starting positions
    pub const fn fresh_game() -> Self {
        Self {
            white: Pieces::new(
                STARTING_WHITE_KING,
                STARTING_WHITE_QUEEN,
                STARTING_WHITE_ROOKS,
                STARTING_WHITE_BISHOPS,
                STARTING_WHITE_KNIGHTS,
                STARTING_WHITE_PAWNS,
            ),
            black: Pieces::new(
                STARTING_BLACK_KING,
                STARTING_BLACK_QUEEN,
                STARTING_BLACK_ROOKS,
                STARTING_BLACK_BISHOPS,
                STARTING_BLACK_KNIGHTS,
                STARTING_BLACK_PAWNS,
            ),
        }
    }
    /// Return a bitboard corresponding to all the pieces on both sides
    pub fn all_pieces(&mut self) -> BitBoard {
        self.white.all_pieces().union(&self.black.all_pieces())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bitboard;

    // *****************************************************************
    // Test starting constants
    // *****************************************************************

    #[test]
    fn test_starting_white_pawns() {
        assert!(
            STARTING_WHITE_PAWNS
                == BitBoard::from_squares(&vec![
                    Square::A2,
                    Square::B2,
                    Square::C2,
                    Square::D2,
                    Square::E2,
                    Square::F2,
                    Square::G2,
                    Square::H2,
                ])
        );
    }

    #[test]
    fn test_starting_black_pawns() {
        assert!(
            STARTING_BLACK_PAWNS
                == BitBoard::from_squares(&vec![
                    Square::A7,
                    Square::B7,
                    Square::C7,
                    Square::D7,
                    Square::E7,
                    Square::F7,
                    Square::G7,
                    Square::H7,
                ])
        );
    }

    #[test]
    fn test_starting_white_rooks() {
        assert!(STARTING_WHITE_ROOKS == BitBoard::from_squares(&vec![Square::A1, Square::H1]));
    }

    #[test]
    fn test_starting_black_rooks() {
        assert!(STARTING_BLACK_ROOKS == BitBoard::from_squares(&vec![Square::A8, Square::H8]));
    }

    #[test]
    fn test_starting_white_bishops() {
        assert!(STARTING_WHITE_BISHOPS == BitBoard::from_squares(&vec![Square::B1, Square::G1]));
    }

    #[test]
    fn test_starting_black_bishops() {
        assert!(STARTING_BLACK_BISHOPS == BitBoard::from_squares(&vec![Square::B8, Square::G8]));
    }

    #[test]
    fn test_starting_white_knights() {
        assert!(STARTING_WHITE_KNIGHTS == BitBoard::from_squares(&vec![Square::C1, Square::F1]));
    }

    #[test]
    fn test_starting_black_knights() {
        assert!(STARTING_BLACK_KNIGHTS == BitBoard::from_squares(&vec![Square::C8, Square::F8]));
    }

    // *****************************************************************
    // Test board
    // *****************************************************************

    #[test]
    fn test_fresh_game_all_pieces() {
        assert!(
            Board::fresh_game().all_pieces()
                == bitboard::RANK_1
                    .union(&bitboard::RANK_1.shift_north())
                    .union(&bitboard::RANK_8)
                    .union(&bitboard::RANK_8.shift_south())
        );
    }

    #[test]
    fn test_fresh_game_white_pieces() {
        assert!(
            Board::fresh_game().white.all_pieces()
                == bitboard::RANK_1.union(&bitboard::RANK_1.shift_north())
        );
    }

    #[test]
    fn test_fresh_game_black_pieces() {
        assert!(
            Board::fresh_game().black.all_pieces()
                == bitboard::RANK_8.union(&bitboard::RANK_8.shift_south())
        );
    }
}
