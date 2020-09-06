//! Square constants and enumerations

use std::fmt;

pub enum SquareColor {
    White,
    Black,
}

/// Square indices for a bitboard in little-endian rank-file (LRF) mapping
///
/// LRF mapping makes A1 (the bottom left square) 0, and increases index
/// going right across the files. The enum below is formatted as though
/// it were a chess board.
#[rustfmt::skip]
#[derive(Clone, Copy, Debug)]
pub enum Square {
    A8 = 56, B8 = 57, C8 = 58, D8 = 59, E8 = 60, F8 = 61, G8 = 62, H8 = 63,
    A7 = 48, B7 = 49, C7 = 50, D7 = 51, E7 = 52, F7 = 53, G7 = 54, H7 = 55,
    A6 = 40, B6 = 41, C6 = 42, D6 = 43, E6 = 44, F6 = 45, G6 = 46, H6 = 47,
    A5 = 32, B5 = 33, C5 = 34, D5 = 35, E5 = 36, F5 = 37, G5 = 38, H5 = 39,
    A4 = 24, B4 = 25, C4 = 26, D4 = 27, E4 = 28, F4 = 29, G4 = 30, H4 = 31,
    A3 = 16, B3 = 17, C3 = 18, D3 = 19, E3 = 20, F3 = 21, G3 = 22, H3 = 23,
    A2 = 8,  B2 =  9, C2 = 10, D2 = 11, E2 = 12, F2 = 13, G2 = 14, H2 = 15,
    A1 = 0,  B1 =  1, C1 =  2, D1 =  3, E1 =  4, F1 =  5, G1 =  6, H1 =  7,
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Square {
    pub fn color(&self) -> SquareColor {
        match self {
            // Rank 1
            Square::A1 => SquareColor::Black,
            Square::B1 => SquareColor::White,
            Square::C1 => SquareColor::Black,
            Square::D1 => SquareColor::White,
            Square::E1 => SquareColor::Black,
            Square::F1 => SquareColor::White,
            Square::G1 => SquareColor::Black,
            Square::H1 => SquareColor::White,
            // Rank 2
            Square::A2 => SquareColor::White,
            Square::B2 => SquareColor::Black,
            Square::C2 => SquareColor::White,
            Square::D2 => SquareColor::Black,
            Square::E2 => SquareColor::White,
            Square::F2 => SquareColor::Black,
            Square::G2 => SquareColor::White,
            Square::H2 => SquareColor::Black,
            // Rank 3
            Square::A3 => SquareColor::Black,
            Square::B3 => SquareColor::White,
            Square::C3 => SquareColor::Black,
            Square::D3 => SquareColor::White,
            Square::E3 => SquareColor::Black,
            Square::F3 => SquareColor::White,
            Square::G3 => SquareColor::Black,
            Square::H3 => SquareColor::White,
            // Rank 4
            Square::A4 => SquareColor::White,
            Square::B4 => SquareColor::Black,
            Square::C4 => SquareColor::White,
            Square::D4 => SquareColor::Black,
            Square::E4 => SquareColor::White,
            Square::F4 => SquareColor::Black,
            Square::G4 => SquareColor::White,
            Square::H4 => SquareColor::Black,
            // Rank 5
            Square::A5 => SquareColor::Black,
            Square::B5 => SquareColor::White,
            Square::C5 => SquareColor::Black,
            Square::D5 => SquareColor::White,
            Square::E5 => SquareColor::Black,
            Square::F5 => SquareColor::White,
            Square::G5 => SquareColor::Black,
            Square::H5 => SquareColor::White,
            // Rank 6
            Square::A6 => SquareColor::White,
            Square::B6 => SquareColor::Black,
            Square::C6 => SquareColor::White,
            Square::D6 => SquareColor::Black,
            Square::E6 => SquareColor::White,
            Square::F6 => SquareColor::Black,
            Square::G6 => SquareColor::White,
            Square::H6 => SquareColor::Black,
            // Rank 7
            Square::A7 => SquareColor::Black,
            Square::B7 => SquareColor::White,
            Square::C7 => SquareColor::Black,
            Square::D7 => SquareColor::White,
            Square::E7 => SquareColor::Black,
            Square::F7 => SquareColor::White,
            Square::G7 => SquareColor::Black,
            Square::H7 => SquareColor::White,
            // Rank 8
            Square::A8 => SquareColor::White,
            Square::B8 => SquareColor::Black,
            Square::C8 => SquareColor::White,
            Square::D8 => SquareColor::Black,
            Square::E8 => SquareColor::White,
            Square::F8 => SquareColor::Black,
            Square::G8 => SquareColor::White,
            Square::H8 => SquareColor::Black,
        }
    }
}

/// Precalculated Square (index) positions (LRF bitboard positions)
///
/// Shift is pretty cheap, but no reason not to stash these rather than
/// calculating on demand.
#[derive(Clone, Copy, Debug)]
pub enum SquarePosition {
    // Don't worry, I did this with vim macros :)
    A1 = 1 << Square::A1 as u8,
    A2 = 1 << Square::A2 as u8,
    A3 = 1 << Square::A3 as u8,
    A4 = 1 << Square::A4 as u8,
    A5 = 1 << Square::A5 as u8,
    A6 = 1 << Square::A6 as u8,
    A7 = 1 << Square::A7 as u8,
    A8 = 1 << Square::A8 as u8,
    B1 = 1 << Square::B1 as u8,
    B2 = 1 << Square::B2 as u8,
    B3 = 1 << Square::B3 as u8,
    B4 = 1 << Square::B4 as u8,
    B5 = 1 << Square::B5 as u8,
    B6 = 1 << Square::B6 as u8,
    B7 = 1 << Square::B7 as u8,
    B8 = 1 << Square::B8 as u8,
    C1 = 1 << Square::C1 as u8,
    C2 = 1 << Square::C2 as u8,
    C3 = 1 << Square::C3 as u8,
    C4 = 1 << Square::C4 as u8,
    C5 = 1 << Square::C5 as u8,
    C6 = 1 << Square::C6 as u8,
    C7 = 1 << Square::C7 as u8,
    C8 = 1 << Square::C8 as u8,
    D1 = 1 << Square::D1 as u8,
    D2 = 1 << Square::D2 as u8,
    D3 = 1 << Square::D3 as u8,
    D4 = 1 << Square::D4 as u8,
    D5 = 1 << Square::D5 as u8,
    D6 = 1 << Square::D6 as u8,
    D7 = 1 << Square::D7 as u8,
    D8 = 1 << Square::D8 as u8,
    E1 = 1 << Square::E1 as u8,
    E2 = 1 << Square::E2 as u8,
    E3 = 1 << Square::E3 as u8,
    E4 = 1 << Square::E4 as u8,
    E5 = 1 << Square::E5 as u8,
    E6 = 1 << Square::E6 as u8,
    E7 = 1 << Square::E7 as u8,
    E8 = 1 << Square::E8 as u8,
    F1 = 1 << Square::F1 as u8,
    F2 = 1 << Square::F2 as u8,
    F3 = 1 << Square::F3 as u8,
    F4 = 1 << Square::F4 as u8,
    F5 = 1 << Square::F5 as u8,
    F6 = 1 << Square::F6 as u8,
    F7 = 1 << Square::F7 as u8,
    F8 = 1 << Square::F8 as u8,
    G1 = 1 << Square::G1 as u8,
    G2 = 1 << Square::G2 as u8,
    G3 = 1 << Square::G3 as u8,
    G4 = 1 << Square::G4 as u8,
    G5 = 1 << Square::G5 as u8,
    G6 = 1 << Square::G6 as u8,
    G7 = 1 << Square::G7 as u8,
    G8 = 1 << Square::G8 as u8,
    H1 = 1 << Square::H1 as u8,
    H2 = 1 << Square::H2 as u8,
    H3 = 1 << Square::H3 as u8,
    H4 = 1 << Square::H4 as u8,
    H5 = 1 << Square::H5 as u8,
    H6 = 1 << Square::H6 as u8,
    H7 = 1 << Square::H7 as u8,
    H8 = 1 << Square::H8 as u8,
}
impl SquarePosition {
    /// Match a square index to its LRF bitboard position
    pub const fn from_square(square: &Square) -> Self {
        match square {
            Square::A1 => Self::A1,
            Square::A2 => Self::A2,
            Square::A3 => Self::A3,
            Square::A4 => Self::A4,
            Square::A5 => Self::A5,
            Square::A6 => Self::A6,
            Square::A7 => Self::A7,
            Square::A8 => Self::A8,
            Square::B1 => Self::B1,
            Square::B2 => Self::B2,
            Square::B3 => Self::B3,
            Square::B4 => Self::B4,
            Square::B5 => Self::B5,
            Square::B6 => Self::B6,
            Square::B7 => Self::B7,
            Square::B8 => Self::B8,
            Square::C1 => Self::C1,
            Square::C2 => Self::C2,
            Square::C3 => Self::C3,
            Square::C4 => Self::C4,
            Square::C5 => Self::C5,
            Square::C6 => Self::C6,
            Square::C7 => Self::C7,
            Square::C8 => Self::C8,
            Square::D1 => Self::D1,
            Square::D2 => Self::D2,
            Square::D3 => Self::D3,
            Square::D4 => Self::D4,
            Square::D5 => Self::D5,
            Square::D6 => Self::D6,
            Square::D7 => Self::D7,
            Square::D8 => Self::D8,
            Square::E1 => Self::E1,
            Square::E2 => Self::E2,
            Square::E3 => Self::E3,
            Square::E4 => Self::E4,
            Square::E5 => Self::E5,
            Square::E6 => Self::E6,
            Square::E7 => Self::E7,
            Square::E8 => Self::E8,
            Square::F1 => Self::F1,
            Square::F2 => Self::F2,
            Square::F3 => Self::F3,
            Square::F4 => Self::F4,
            Square::F5 => Self::F5,
            Square::F6 => Self::F6,
            Square::F7 => Self::F7,
            Square::F8 => Self::F8,
            Square::G1 => Self::G1,
            Square::G2 => Self::G2,
            Square::G3 => Self::G3,
            Square::G4 => Self::G4,
            Square::G5 => Self::G5,
            Square::G6 => Self::G6,
            Square::G7 => Self::G7,
            Square::G8 => Self::G8,
            Square::H1 => Self::H1,
            Square::H2 => Self::H2,
            Square::H3 => Self::H3,
            Square::H4 => Self::H4,
            Square::H5 => Self::H5,
            Square::H6 => Self::H6,
            Square::H7 => Self::H7,
            Square::H8 => Self::H8,
        }
    }
}

/// An array of all squares in index order
#[rustfmt::skip]
pub const SQUARES: [Square; 64] = [
    Square::A1, Square::B1, Square::C1, Square::D1, Square::E1, Square::F1, Square::G1, Square::H1,
    Square::A2, Square::B2, Square::C2, Square::D2, Square::E2, Square::F2, Square::G2, Square::H2,
    Square::A3, Square::B3, Square::C3, Square::D3, Square::E3, Square::F3, Square::G3, Square::H3,
    Square::A4, Square::B4, Square::C4, Square::D4, Square::E4, Square::F4, Square::G4, Square::H4,
    Square::A5, Square::B5, Square::C5, Square::D5, Square::E5, Square::F5, Square::G5, Square::H5,
    Square::A6, Square::B6, Square::C6, Square::D6, Square::E6, Square::F6, Square::G6, Square::H6,
    Square::A7, Square::B7, Square::C7, Square::D7, Square::E7, Square::F7, Square::G7, Square::H7,
    Square::A8, Square::B8, Square::C8, Square::D8, Square::E8, Square::F8, Square::G8, Square::H8,
];

/// An array of all square positions
pub const SQUARE_POSITIONS: [SquarePosition; 64] = [
    SquarePosition::A1,
    SquarePosition::A2,
    SquarePosition::A3,
    SquarePosition::A4,
    SquarePosition::A5,
    SquarePosition::A6,
    SquarePosition::A7,
    SquarePosition::A8,
    SquarePosition::B1,
    SquarePosition::B2,
    SquarePosition::B3,
    SquarePosition::B4,
    SquarePosition::B5,
    SquarePosition::B6,
    SquarePosition::B7,
    SquarePosition::B8,
    SquarePosition::C1,
    SquarePosition::C2,
    SquarePosition::C3,
    SquarePosition::C4,
    SquarePosition::C5,
    SquarePosition::C6,
    SquarePosition::C7,
    SquarePosition::C8,
    SquarePosition::D1,
    SquarePosition::D2,
    SquarePosition::D3,
    SquarePosition::D4,
    SquarePosition::D5,
    SquarePosition::D6,
    SquarePosition::D7,
    SquarePosition::D8,
    SquarePosition::E1,
    SquarePosition::E2,
    SquarePosition::E3,
    SquarePosition::E4,
    SquarePosition::E5,
    SquarePosition::E6,
    SquarePosition::E7,
    SquarePosition::E8,
    SquarePosition::F1,
    SquarePosition::F2,
    SquarePosition::F3,
    SquarePosition::F4,
    SquarePosition::F5,
    SquarePosition::F6,
    SquarePosition::F7,
    SquarePosition::F8,
    SquarePosition::G1,
    SquarePosition::G2,
    SquarePosition::G3,
    SquarePosition::G4,
    SquarePosition::G5,
    SquarePosition::G6,
    SquarePosition::G7,
    SquarePosition::G8,
    SquarePosition::H1,
    SquarePosition::H2,
    SquarePosition::H3,
    SquarePosition::H4,
    SquarePosition::H5,
    SquarePosition::H6,
    SquarePosition::H7,
    SquarePosition::H8,
];
