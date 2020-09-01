//! Board representation
//!
//! Uses a little-endian rank-file (LRF) bitboard approach

use crate::square::{Square, SquarePosition, SQUARES};
use std::fmt;

// Precalculated bitboards corresponding to a variety of board positions
pub const NO_SQUARES: BitBoard = BitBoard::new(0x0000000000000000);
pub const ALL_SQUARES: BitBoard = BitBoard::new(0xFFFFFFFFFFFFFFFF);

pub const A_FILE: BitBoard = BitBoard::new(0x0101010101010101);
pub const NOT_A_FILE: BitBoard = A_FILE.complement();

pub const H_FILE: BitBoard = BitBoard::new(0x8080808080808080);
pub const NOT_H_FILE: BitBoard = H_FILE.complement();

pub const RANK_1: BitBoard = BitBoard::new(0x00000000000000FF);
pub const NOT_RANK_1: BitBoard = RANK_1.complement();

pub const RANK_8: BitBoard = BitBoard::new(0xFF00000000000000);
pub const NOT_RANK_8: BitBoard = RANK_8.complement();

pub const DIAGONAL_A1_H8: BitBoard = BitBoard::new(0x8040201008040201);
pub const DIAGONAL_H1_A8: BitBoard = BitBoard::new(0x0102040810204080);

pub const LIGHT_SQUARES: BitBoard = BitBoard::new(0x55AA55AA55AA55AA);
pub const DARK_SQUARES: BitBoard = BitBoard::new(0xAA55AA55AA55AA55);

/// A representation of a shift
#[derive(Clone, Copy)]
pub struct Shift {
    distance: u8,
}
impl Shift {
    /// Construct a new Shift
    ///
    /// Generally the distance should be < 8. If it is not, the shift distance
    /// will be coerced to be < 8 using % 8.
    pub const fn new(distance: u8) -> Self {
        Shift {
            distance: distance % 8,
        }
    }
}

/// A little-endian rank-file (LRF) bitboard
#[derive(Clone, Copy)]
pub struct BitBoard {
    positions: u64,
}
/// Constructors
impl BitBoard {
    pub const fn new(positions: u64) -> Self {
        BitBoard { positions }
    }
    pub const fn from_square(square: Square) -> Self {
        BitBoard::new(SquarePosition::from_square(square) as u64)
    }
    pub fn from_squares(squares: &Vec<Square>) -> Self {
        squares
            .iter()
            .map(|square| BitBoard::from_square(*square))
            .fold(NO_SQUARES, |result, board| result.union(&board))
    }
    pub fn from_boards<T: AsRef<Self>>(boards: &Vec<T>) -> Self {
        boards
            .iter()
            .fold(NO_SQUARES, |result, board| result.union(board.as_ref()))
    }
}
/// Containment and Combination Methods
impl BitBoard {
    pub const fn is_empty(&self) -> bool {
        self.positions == 0
    }
    pub const fn occupied(&self, square: Square) -> bool {
        !Self::from_square(square).intersection(self).is_empty()
    }
    pub fn occupied_squares(&self) -> Vec<Square> {
        SQUARES
            .iter()
            .map(|s| *s)
            .filter(|s| self.occupied(*s))
            .collect()
    }

    // only (a and b)
    pub const fn intersection(&self, other: &Self) -> BitBoard {
        BitBoard::new(self.positions & other.positions)
    }
    pub const fn intersects(&self, other: &Self) -> bool {
        !self.is_disjoint_with(other)
    }
    pub const fn is_disjoint_with(&self, other: &Self) -> bool {
        self.intersection(other).is_empty()
    }
    // all a and all b
    pub const fn union(&self, other: &Self) -> BitBoard {
        BitBoard::new(self.positions | other.positions)
    }
    // anything but a
    pub const fn complement(&self) -> BitBoard {
        BitBoard::new(!self.positions)
    }
    // only b by itself
    pub const fn relative_complement(&self, other: &Self) -> BitBoard {
        BitBoard::new(!self.positions & other.positions)
    }
    // everything but (a by itself)
    pub const fn implication(&self, other: &Self) -> BitBoard {
        BitBoard::new(!self.positions | other.positions)
    }
    // one or the other, not both
    pub const fn exclusive_or(&self, other: &Self) -> BitBoard {
        BitBoard::new(self.positions ^ other.positions)
    }
    // both or neither
    pub const fn iff(&self, other: &Self) -> BitBoard {
        BitBoard::new(!(self.positions ^ other.positions))
    }
}
/// Shift Functions
///
/// All shift functions drop anything that goes off the board.
impl BitBoard {
    pub const fn shift_east(&self) -> BitBoard {
        BitBoard::new(self.intersection(&NOT_H_FILE).positions << 1)
    }
    pub const fn shift_northeast(&self) -> BitBoard {
        BitBoard::new(self.intersection(&NOT_H_FILE).positions << 9)
    }
    pub const fn shift_north(&self) -> BitBoard {
        BitBoard::new(self.positions << 8)
    }
    pub const fn shift_northwest(&self) -> BitBoard {
        BitBoard::new(self.intersection(&NOT_A_FILE).positions << 7)
    }
    pub const fn shift_west(&self) -> BitBoard {
        BitBoard::new(self.intersection(&NOT_A_FILE).positions >> 1)
    }
    pub const fn shift_southwest(&self) -> BitBoard {
        BitBoard::new(self.intersection(&NOT_A_FILE).positions >> 9)
    }
    pub const fn shift_south(&self) -> BitBoard {
        BitBoard::new(self.positions >> 8)
    }
    pub const fn shift_southeast(&self) -> BitBoard {
        BitBoard::new(self.intersection(&NOT_H_FILE).positions >> 7)
    }

    pub const fn shift_east_by(&self, shift: Shift) -> BitBoard {
        BitBoard::new(self.intersection(&NOT_H_FILE).positions << shift.distance)
    }
    pub const fn shift_northeast_by(&self, shift: Shift) -> BitBoard {
        BitBoard::new(self.intersection(&NOT_H_FILE).positions << (9 * shift.distance))
    }
    pub const fn shift_north_by(&self, shift: Shift) -> BitBoard {
        BitBoard::new(self.positions << (8 * shift.distance))
    }
    pub const fn shift_northwest_by(&self, shift: Shift) -> BitBoard {
        BitBoard::new(self.intersection(&NOT_A_FILE).positions << (7 * shift.distance))
    }
    pub const fn shift_west_by(&self, shift: Shift) -> BitBoard {
        BitBoard::new(self.intersection(&NOT_A_FILE).positions >> shift.distance)
    }
    pub const fn shift_southwest_by(&self, shift: Shift) -> BitBoard {
        BitBoard::new(self.intersection(&NOT_A_FILE).positions >> (9 * shift.distance))
    }
    pub const fn shift_south_by(&self, shift: Shift) -> BitBoard {
        BitBoard::new(self.positions >> (8 * shift.distance))
    }
    pub const fn shift_southeast_by(&self, shift: Shift) -> BitBoard {
        BitBoard::new(self.intersection(&NOT_H_FILE).positions >> (7 * shift.distance))
    }
}
/// Update methods
impl BitBoard {
    /// Ensure a given square is set
    pub const fn set(&self, square: Square) -> Self {
        self.union(&Self::from_square(square))
    }
    /// Toggle a given square
    pub const fn toggle(&self, square: Square) -> Self {
        self.exclusive_or(&Self::from_square(square))
    }
    /// Unset a given square
    pub const fn unset(&self, square: Square) -> Self {
        self.intersection(&Self::from_square(square).complement())
    }
}
/// Return a reference from a value or a reference
impl AsRef<Self> for BitBoard {
    fn as_ref(&self) -> &Self {
        self
    }
}
/// Bitboards are equal if their positions are equal
impl Eq for BitBoard {}
/// Bitboards are equal if their positions are equal
impl PartialEq for BitBoard {
    fn eq(&self, other: &Self) -> bool {
        self.positions == other.positions
    }
}
/// Debug display for a bitboard containing position and occupied squares
impl fmt::Debug for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BitBoard(positions: {:x?}, {})", self.positions, self)
    }
}
/// Display for a bitboard indicating occupied squares
impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let squares = self
            .occupied_squares()
            .iter()
            .map(|s| format!("{}", s))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "[{}]", squares)
    }
}

#[cfg(test)]
mod test_bitboard {
    use super::*;

    #[test]
    fn test_shift_north_overflow() {
        assert!(RANK_8.shift_north() == NO_SQUARES);
    }

    #[test]
    fn test_shift_south_overflow() {
        assert!(RANK_1.shift_south() == NO_SQUARES);
    }

    #[test]
    fn test_shift_east_overflow() {
        assert!(H_FILE.shift_east() == NO_SQUARES);
    }

    #[test]
    fn test_shift_west_overflow() {
        assert!(A_FILE.shift_west() == NO_SQUARES);
    }

    #[test]
    fn test_shift_north() {
        let mut board = RANK_1;
        for _ in 0..7 {
            // shift north 7 times
            board = board.shift_north()
        }
        assert!(board == RANK_8);
    }

    #[test]
    fn test_shift_south() {
        let mut board = RANK_8;
        for _ in 0..7 {
            // shift south 7 times
            board = board.shift_south()
        }
        assert!(board == RANK_1);
    }

    #[test]
    fn test_shift_east() {
        let mut board = A_FILE;
        for _ in 0..7 {
            // shift east 7 times
            board = board.shift_east()
        }
        assert!(board == H_FILE);
    }

    #[test]
    fn test_shift_west() {
        let mut board = H_FILE;
        for _ in 0..7 {
            // shift west 7 times
            board = board.shift_west()
        }
        assert!(board == A_FILE);
    }

    #[test]
    fn test_shift_northest() {
        let mut board = BitBoard::from_square(Square::A1);
        for _ in 0..7 {
            // shift west 7 times
            board = board.shift_northeast()
        }
        assert!(board == BitBoard::from_square(Square::H8));
    }

    #[test]
    fn test_shift_northwest() {
        let mut board = BitBoard::from_square(Square::H1);
        for _ in 0..7 {
            // shift west 7 times
            board = board.shift_northwest()
        }
        assert!(board == BitBoard::from_square(Square::A8));
    }

    #[test]
    fn test_shift_southeast() {
        let mut board = BitBoard::from_square(Square::A8);
        for _ in 0..7 {
            // shift west 7 times
            board = board.shift_southeast()
        }
        assert!(board == BitBoard::from_square(Square::H1));
    }

    #[test]
    fn test_shift_southwest() {
        let mut board = BitBoard::from_square(Square::H8);
        for _ in 0..7 {
            // shift west 7 times
            board = board.shift_southwest()
        }
        assert!(board == BitBoard::from_square(Square::A1));
    }

    #[test]
    fn test_shift_west_by() {
        assert!(H_FILE.shift_west_by(Shift::new(7)) == A_FILE);
    }

    #[test]
    fn test_shift_east_by() {
        assert!(A_FILE.shift_east_by(Shift::new(7)) == H_FILE);
    }

    #[test]
    fn test_shift_north_by() {
        assert!(RANK_1.shift_north_by(Shift::new(7)) == RANK_8);
    }

    #[test]
    fn test_shift_south_by() {
        assert!(RANK_8.shift_south_by(Shift::new(7)) == RANK_1);
    }

    #[test]
    fn test_shift_northeast_by() {
        assert!(
            BitBoard::from_square(Square::A1).shift_northeast_by(Shift::new(7))
                == BitBoard::from_square(Square::H8)
        );
    }

    #[test]
    fn test_shift_northwest_by() {
        assert!(
            BitBoard::from_square(Square::H1).shift_northwest_by(Shift::new(7))
                == BitBoard::from_square(Square::A8)
        );
    }

    #[test]
    fn test_shift_southeast_by() {
        assert!(
            BitBoard::from_square(Square::A8).shift_southeast_by(Shift::new(7))
                == BitBoard::from_square(Square::H1)
        );
    }

    #[test]
    fn test_shift_southwest_by() {
        assert!(
            BitBoard::from_square(Square::H8).shift_southwest_by(Shift::new(7))
                == BitBoard::from_square(Square::A1)
        );
    }

    #[test]
    fn test_from_square() {
        assert!(BitBoard::from_square(Square::A1).shift_south() == NO_SQUARES);
        assert!(BitBoard::from_square(Square::A1).shift_west() == NO_SQUARES);
        assert!(BitBoard::from_square(Square::A8).shift_north() == NO_SQUARES);
        assert!(BitBoard::from_square(Square::A8).shift_west() == NO_SQUARES);
        assert!(BitBoard::from_square(Square::H1).shift_east() == NO_SQUARES);
        assert!(BitBoard::from_square(Square::H1).shift_south() == NO_SQUARES);
        assert!(BitBoard::from_square(Square::H8).shift_east() == NO_SQUARES);
        assert!(BitBoard::from_square(Square::H8).shift_north() == NO_SQUARES);
    }

    #[test]
    fn test_from_squares() {
        assert!(
            BitBoard::from_squares(&vec![
                Square::A1,
                Square::B1,
                Square::C1,
                Square::D1,
                Square::E1,
                Square::F1,
                Square::G1,
                Square::H1
            ]) == RANK_1
        )
    }

    #[test]
    fn test_set() {
        assert!(NO_SQUARES.set(Square::A1) == BitBoard::from_square(Square::A1))
    }

    #[test]
    fn test_unset() {
        assert!(BitBoard::from_square(Square::A1).unset(Square::A1) == NO_SQUARES)
    }

    #[test]
    fn test_toggle_on() {
        assert!(NO_SQUARES.toggle(Square::A1) == BitBoard::from_square(Square::A1))
    }

    #[test]
    fn test_toggle_off() {
        assert!(BitBoard::from_square(Square::A1).toggle(Square::A1) == NO_SQUARES)
    }
}
