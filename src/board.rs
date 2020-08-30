//! Board representation

use std::fmt;

const NO_SQUARES: BitBoard = BitBoard::new(0x0000000000000000);
const ALL_SQUARES: BitBoard = BitBoard::new(0xFFFFFFFFFFFFFFFF);
const A_FILE: BitBoard = BitBoard::new(0x0101010101010101);
const NOT_A_FILE: BitBoard = A_FILE.complement();
const H_FILE: BitBoard = BitBoard::new(0x8080808080808080);
const NOT_H_FILE: BitBoard = H_FILE.complement();
const RANK_1: BitBoard = BitBoard::new(0x00000000000000FF);
const NOT_RANK_1: BitBoard = RANK_1.complement();
const RANK_8: BitBoard = BitBoard::new(0xFF00000000000000);
const NOT_RANK_8: BitBoard = RANK_8.complement();
const DIAGONAL_A1_H8: BitBoard = BitBoard::new(0x8040201008040201);
const DIAGONAL_H1_A8: BitBoard = BitBoard::new(0x0102040810204080);
const LIGHT_SQUARES: BitBoard = BitBoard::new(0x55AA55AA55AA55AA);
const DARK_SQUARES: BitBoard = BitBoard::new(0xAA55AA55AA55AA55);

#[derive(Clone, Copy)]
struct Shift {
    distance: u8,
}
impl Shift {
    /// Construct a new Shift
    ///
    /// Generally the distance should be < 8. If it is not, the shift distance
    /// will be coerced to be < 8 using % 8.
    const fn new(distance: u8) -> Self {
        Shift {
            distance: distance % 8,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Square {
    // Rank 1
    A1 = 0,
    B1 = 1,
    C1 = 2,
    D1 = 3,
    E1 = 4,
    F1 = 5,
    G1 = 6,
    H1 = 7,
    // Rank 2
    A2 = 8,
    B2 = 9,
    C2 = 10,
    D2 = 11,
    E2 = 12,
    F2 = 13,
    G2 = 14,
    H2 = 15,
    // Rank 3
    A3 = 16,
    B3 = 17,
    C3 = 18,
    D3 = 19,
    E3 = 20,
    F3 = 21,
    G3 = 22,
    H3 = 23,
    // Rank 4
    A4 = 24,
    B4 = 25,
    C4 = 26,
    D4 = 27,
    E4 = 28,
    F4 = 29,
    G4 = 30,
    H4 = 31,
    // Rank 5
    A5 = 32,
    B5 = 33,
    C5 = 34,
    D5 = 35,
    E5 = 36,
    F5 = 37,
    G5 = 38,
    H5 = 39,
    // Rank 6
    A6 = 40,
    B6 = 41,
    C6 = 42,
    D6 = 43,
    E6 = 44,
    F6 = 45,
    G6 = 46,
    H6 = 47,
    // Rank 7
    A7 = 48,
    B7 = 49,
    C7 = 50,
    D7 = 51,
    E7 = 52,
    F7 = 53,
    G7 = 54,
    H7 = 55,
    // Rank 8
    A8 = 56,
    B8 = 57,
    C8 = 58,
    D8 = 59,
    E8 = 60,
    F8 = 61,
    G8 = 62,
    H8 = 63,
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

const SQUARES: [Square; 64] = [
    Square::A1,
    Square::A2,
    Square::A3,
    Square::A4,
    Square::A5,
    Square::A6,
    Square::A7,
    Square::A8,
    Square::B1,
    Square::B2,
    Square::B3,
    Square::B4,
    Square::B5,
    Square::B6,
    Square::B7,
    Square::B8,
    Square::C1,
    Square::C2,
    Square::C3,
    Square::C4,
    Square::C5,
    Square::C6,
    Square::C7,
    Square::C8,
    Square::D1,
    Square::D2,
    Square::D3,
    Square::D4,
    Square::D5,
    Square::D6,
    Square::D7,
    Square::D8,
    Square::E1,
    Square::E2,
    Square::E3,
    Square::E4,
    Square::E5,
    Square::E6,
    Square::E7,
    Square::E8,
    Square::F1,
    Square::F2,
    Square::F3,
    Square::F4,
    Square::F5,
    Square::F6,
    Square::F7,
    Square::F8,
    Square::G1,
    Square::G2,
    Square::G3,
    Square::G4,
    Square::G5,
    Square::G6,
    Square::G7,
    Square::G8,
    Square::H1,
    Square::H2,
    Square::H3,
    Square::H4,
    Square::H5,
    Square::H6,
    Square::H7,
    Square::H8,
];

struct BitBoard {
    positions: u64,
}
/// Constructors
impl BitBoard {
    const fn new(positions: u64) -> Self {
        BitBoard { positions }
    }
    const fn from_square(square: Square) -> Self {
        BitBoard::new(1 << square as u8)
    }
    fn from_squares(squares: &Vec<Square>) -> Self {
        squares
            .into_iter()
            .map(|square| BitBoard::from_square(*square))
            .fold(NO_SQUARES, |result, board| result.union(&board))
    }
}
/// Containment and Combination Methods
impl BitBoard {
    const fn is_empty(&self) -> bool {
        self.positions == 0
    }
    const fn contains(&self, square: Square) -> bool {
        !Self::from_square(square).intersection(self).is_empty()
    }
    fn occupied_squares(&self) -> Vec<Square> {
        SQUARES
            .iter()
            .map(|s| *s)
            .filter(|s| self.contains(*s))
            .collect()
    }

    // only (a and b)
    const fn intersection(&self, other: &Self) -> BitBoard {
        BitBoard::new(self.positions & other.positions)
    }
    const fn intersects(&self, other: &Self) -> bool {
        !self.is_disjoint_with(other)
    }
    const fn is_disjoint_with(&self, other: &Self) -> bool {
        self.intersection(other).is_empty()
    }
    // all a and all b
    const fn union(&self, other: &Self) -> BitBoard {
        BitBoard::new(self.positions | other.positions)
    }
    // anything but a
    const fn complement(&self) -> BitBoard {
        BitBoard::new(!self.positions)
    }
    // only b by itself
    const fn relative_complement(&self, other: &Self) -> BitBoard {
        BitBoard::new(!self.positions & other.positions)
    }
    // everything but (a by itself)
    const fn implication(&self, other: &Self) -> BitBoard {
        BitBoard::new(!self.positions | other.positions)
    }
    // one or the other, not both
    const fn exclusive_or(&self, other: &Self) -> BitBoard {
        BitBoard::new(self.positions ^ other.positions)
    }
    // both or neither
    const fn iff(&self, other: &Self) -> BitBoard {
        BitBoard::new(!(self.positions ^ other.positions))
    }
}
/// Shift Functions
///
/// All shift functions drop anything that goes off the board.
impl BitBoard {
    const fn shift_east(&self) -> BitBoard {
        BitBoard::new(self.intersection(&NOT_H_FILE).positions << 1)
    }
    const fn shift_northeast(&self) -> BitBoard {
        BitBoard::new(self.intersection(&NOT_H_FILE).positions << 9)
    }
    const fn shift_north(&self) -> BitBoard {
        BitBoard::new(self.positions << 8)
    }
    const fn shift_northwest(&self) -> BitBoard {
        BitBoard::new(self.intersection(&NOT_A_FILE).positions << 7)
    }
    const fn shift_west(&self) -> BitBoard {
        BitBoard::new(self.intersection(&NOT_A_FILE).positions >> 1)
    }
    const fn shift_southwest(&self) -> BitBoard {
        BitBoard::new(self.intersection(&NOT_A_FILE).positions >> 9)
    }
    const fn shift_south(&self) -> BitBoard {
        BitBoard::new(self.positions >> 8)
    }
    const fn shift_southeast(&self) -> BitBoard {
        BitBoard::new(self.intersection(&NOT_H_FILE).positions >> 7)
    }

    const fn shift_east_by(&self, shift: Shift) -> BitBoard {
        BitBoard::new(self.intersection(&NOT_H_FILE).positions << shift.distance)
    }
    const fn shift_northeast_by(&self, shift: Shift) -> BitBoard {
        BitBoard::new(self.intersection(&NOT_H_FILE).positions << (9 * shift.distance))
    }
    const fn shift_north_by(&self, shift: Shift) -> BitBoard {
        BitBoard::new(self.positions << (8 * shift.distance))
    }
    const fn shift_northwest_by(&self, shift: Shift) -> BitBoard {
        BitBoard::new(self.intersection(&NOT_A_FILE).positions << (7 * shift.distance))
    }
    const fn shift_west_by(&self, shift: Shift) -> BitBoard {
        BitBoard::new(self.intersection(&NOT_A_FILE).positions >> shift.distance)
    }
    const fn shift_southwest_by(&self, shift: Shift) -> BitBoard {
        BitBoard::new(self.intersection(&NOT_A_FILE).positions >> (9 * shift.distance))
    }
    const fn shift_south_by(&self, shift: Shift) -> BitBoard {
        BitBoard::new(self.positions >> (8 * shift.distance))
    }
    const fn shift_southeast_by(&self, shift:Shift) -> BitBoard {
        BitBoard::new(self.intersection(&NOT_H_FILE).positions >> (7 * shift.distance))
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
        write!(f, "BitBoard(positions: {:x}, {})", self.positions, self)
    }
}
/// Display for a bitboard indicating occupied squares
impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let squares = self.occupied_squares()
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
}

enum Color {
    White,
    Black,
}

struct Pieces {
    bishops: BitBoard,
    king: BitBoard,
    knights: BitBoard,
    pawns: BitBoard,
    queens: BitBoard,
    rooks: BitBoard,
}
impl PartialEq for Pieces {
    fn eq(&self, other: &Self) -> bool {
        self.pawns == other.pawns
            && self.knights == other.knights
            && self.bishops == other.bishops
            && self.rooks == other.rooks
            && self.queens == other.queens
            && self.king == other.king
    }
}
impl Eq for Pieces {}

struct Board {
    white_pieces: Pieces,
    black_pieces: Pieces,
}
impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.white_pieces == other.white_pieces && self.black_pieces == other.black_pieces
    }
}
impl Eq for Board {}
