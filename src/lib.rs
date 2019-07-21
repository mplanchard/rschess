
use std::error;
use std::fmt;
#[derive(Debug, PartialEq)]
enum Error {
    OffBoard,
}
impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

type BitIndex = u64;


#[derive(Debug)]
enum Compass {
    North,
    East,
    South,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}


// The integer associations are bit indexes on the 64 bit position.
#[derive(Debug, PartialEq)]
enum Square {
    A1 = 0,
    B1 = 1,
    C1 = 2,
    D1 = 3,
    E1 = 4,
    F1 = 5,
    G1 = 6,
    H1 = 7,
    A2 = 8,
    B2 = 9,
    C2 = 10,
    D2 = 11,
    E2 = 12,
    F2 = 13,
    G2 = 14,
    H2 = 15,
    A3 = 16,
    B3 = 17,
    C3 = 18,
    D3 = 19,
    E3 = 20,
    F3 = 21,
    G3 = 22,
    H3 = 23,
    A4 = 24,
    B4 = 25,
    C4 = 26,
    D4 = 27,
    E4 = 28,
    F4 = 29,
    G4 = 30,
    H4 = 31,
    A5 = 32,
    B5 = 33,
    C5 = 34,
    D5 = 35,
    E5 = 36,
    F5 = 37,
    G5 = 38,
    H5 = 39,
    A6 = 40,
    B6 = 41,
    C6 = 42,
    D6 = 43,
    E6 = 44,
    F6 = 45,
    G6 = 46,
    H6 = 47,
    A7 = 48,
    B7 = 49,
    C7 = 50,
    D7 = 51,
    E7 = 52,
    F7 = 53,
    G7 = 54,
    H7 = 55,
    A8 = 56,
    B8 = 57,
    C8 = 58,
    D8 = 59,
    E8 = 60,
    F8 = 61,
    G8 = 62,
    H8 = 63,
}
impl Into<u64> for &Square {
    fn into(self) -> BitIndex {
        match self {
            Square::A1 => 0,
            Square::B1 => 1,
            Square::C1 => 2,
            Square::D1 => 3,
            Square::E1 => 4,
            Square::F1 => 5,
            Square::G1 => 6,
            Square::H1 => 7,
            Square::A2 => 8,
            Square::B2 => 9,
            Square::C2 => 10,
            Square::D2 => 11,
            Square::E2 => 12,
            Square::F2 => 13,
            Square::G2 => 14,
            Square::H2 => 15,
            Square::A3 => 16,
            Square::B3 => 17,
            Square::C3 => 18,
            Square::D3 => 19,
            Square::E3 => 20,
            Square::F3 => 21,
            Square::G3 => 22,
            Square::H3 => 23,
            Square::A4 => 24,
            Square::B4 => 25,
            Square::C4 => 26,
            Square::D4 => 27,
            Square::E4 => 28,
            Square::F4 => 29,
            Square::G4 => 30,
            Square::H4 => 31,
            Square::A5 => 32,
            Square::B5 => 33,
            Square::C5 => 34,
            Square::D5 => 35,
            Square::E5 => 36,
            Square::F5 => 37,
            Square::G5 => 38,
            Square::H5 => 39,
            Square::A6 => 40,
            Square::B6 => 41,
            Square::C6 => 42,
            Square::D6 => 43,
            Square::E6 => 44,
            Square::F6 => 45,
            Square::G6 => 46,
            Square::H6 => 47,
            Square::A7 => 48,
            Square::B7 => 49,
            Square::C7 => 50,
            Square::D7 => 51,
            Square::E7 => 52,
            Square::F7 => 53,
            Square::G7 => 54,
            Square::H7 => 55,
            Square::A8 => 56,
            Square::B8 => 57,
            Square::C8 => 58,
            Square::D8 => 59,
            Square::E8 => 60,
            Square::F8 => 61,
            Square::G8 => 62,
            Square::H8 => 63,
        }
    }
}
impl Square {
    // Constants courtesy chessprogramming.org
    // const A_FILE: u64 = 0x0101010101010101;
    // const H_FILE: u64 = 0x8080808080808080;
    // const RANK_1: u64 = 0x00000000000000FF;
    // const RANK_8: u64 = 0xFF00000000000000;

    const A_FILE: u64 = Square::A1 as u64
        | Square::A2 as u64
        | Square::A3 as u64
        | Square::A4 as u64
        | Square::A5 as u64
        | Square::A6 as u64
        | Square::A7 as u64
        | Square::A8 as u64;
    const H_FILE: u64 = Square::H1 as u64
        | Square::H2 as u64
        | Square::H3 as u64
        | Square::H4 as u64
        | Square::H5 as u64
        | Square::H6 as u64
        | Square::H7 as u64
        | Square::H8 as u64;

    // Get the square associated with a given bit index.
    fn from_bitindex(bi: BitIndex) -> Result<Square, Error> {
        match bi {
            0 => Ok(Square::A1),
            1 => Ok(Square::B1),
            2 => Ok(Square::C1),
            3 => Ok(Square::D1),
            4 => Ok(Square::E1),
            5 => Ok(Square::F1),
            6 => Ok(Square::G1),
            7 => Ok(Square::H1),
            8 => Ok(Square::A2),
            9 => Ok(Square::B2),
            10 => Ok(Square::C2),
            11 => Ok(Square::D2),
            12 => Ok(Square::E2),
            13 => Ok(Square::F2),
            14 => Ok(Square::G2),
            15 => Ok(Square::H2),
            16 => Ok(Square::A3),
            17 => Ok(Square::B3),
            18 => Ok(Square::C3),
            19 => Ok(Square::D3),
            20 => Ok(Square::E3),
            21 => Ok(Square::F3),
            22 => Ok(Square::G3),
            23 => Ok(Square::H3),
            24 => Ok(Square::A4),
            25 => Ok(Square::B4),
            26 => Ok(Square::C4),
            27 => Ok(Square::D4),
            28 => Ok(Square::E4),
            29 => Ok(Square::F4),
            30 => Ok(Square::G4),
            31 => Ok(Square::H4),
            32 => Ok(Square::A5),
            33 => Ok(Square::B5),
            34 => Ok(Square::C5),
            35 => Ok(Square::D5),
            36 => Ok(Square::E5),
            37 => Ok(Square::F5),
            38 => Ok(Square::G5),
            39 => Ok(Square::H5),
            40 => Ok(Square::A6),
            41 => Ok(Square::B6),
            42 => Ok(Square::C6),
            43 => Ok(Square::D6),
            44 => Ok(Square::E6),
            45 => Ok(Square::F6),
            46 => Ok(Square::G6),
            47 => Ok(Square::H6),
            48 => Ok(Square::A7),
            49 => Ok(Square::B7),
            50 => Ok(Square::C7),
            51 => Ok(Square::D7),
            52 => Ok(Square::E7),
            53 => Ok(Square::F7),
            54 => Ok(Square::G7),
            55 => Ok(Square::H7),
            56 => Ok(Square::A8),
            57 => Ok(Square::B8),
            58 => Ok(Square::C8),
            59 => Ok(Square::D8),
            60 => Ok(Square::E8),
            61 => Ok(Square::F8),
            62 => Ok(Square::G8),
            63 => Ok(Square::H8),
            _ => Err(Error::OffBoard),
        }
    }
    fn go(&self, direction: Compass) -> Result<Self, Error> {
        let cur: u64 = self.into();
        let new_bitindex_opt = match direction {
            Compass::North => cur.checked_add(8),
            Compass::South => cur.checked_sub(8),
            Compass::East => {
                if self.on_h_file() {
                    return Err(Error::OffBoard);
                } else {
                    cur.checked_add(1)
                }
            }
            Compass::West => {
                if self.on_a_file() {
                    return Err(Error::OffBoard);
                } else {
                    cur.checked_sub(1)
                }
            }
            Compass::NorthEast => {
                if self.on_h_file() {
                    return Err(Error::OffBoard);
                } else {
                    cur.checked_add(9)
                }
            }
            Compass::SouthWest => {
                if self.on_a_file() {
                    return Err(Error::OffBoard);
                } else {
                    cur.checked_sub(9)
                }
            }
            Compass::NorthWest => {
                if self.on_a_file() {
                    return Err(Error::OffBoard);
                } else {
                    cur.checked_add(7)
                }
            }
            Compass::SouthEast => {
                if self.on_h_file() {
                    return Err(Error::OffBoard);
                } else {
                    cur.checked_sub(7)
                }
            }
        };
        let new_bitindex = new_bitindex_opt.ok_or(Error::OffBoard)?;
        Self::from_bitindex(new_bitindex)
    }
    fn on_a_file(&self) -> bool {
        let cur: u64 = self.into();
        cur % 8 == 0
    }
    fn on_h_file(&self) -> bool {
        let cur: u64 = self.into();
        cur % 8 == 7
    }
}

struct Positions(u64);
impl Positions {
    fn new() -> Self {
        Self(0)
    }
    fn from<T: Into<u64>>(position: T) -> Self {
        Self(position.into())
    }
}


enum Color {
    White,
    Black,
}

enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}


struct Pieces {
    color: Color,
    piece_type: PieceType,
    positions: Positions,
}
impl Pieces {
    fn new(color: Color, piece_type: PieceType, positions: Positions) -> Self {
        Self {
            color,
            piece_type,
            positions,
        }
    }
}

struct Board {
    white_king: Pieces,
    white_queens: Pieces,
    white_rooks: Pieces,
    white_bishops: Pieces,
    white_knights: Pieces,
    white_pawns: Pieces,
    black_king: Pieces,
    black_queens: Pieces,
    black_rooks: Pieces,
    black_bishops: Pieces,
    black_knights: Pieces,
    black_pawns: Pieces,
}
impl Board {
    fn new() -> Self {
        Self {
            white_king: Pieces::new(Color::White, PieceType::King, Positions::new()),
            white_queens: Pieces::new(Color::White, PieceType::Queen, Positions::new()),
            white_rooks: Pieces::new(Color::White, PieceType::Rook, Positions::new()),
            white_bishops: Pieces::new(Color::White, PieceType::Bishop, Positions::new()),
            white_knights: Pieces::new(Color::White, PieceType::Knight, Positions::new()),
            white_pawns: Pieces::new(Color::White, PieceType::Pawn, Positions::new()),
            black_king: Pieces::new(Color::White, PieceType::King, Positions::new()),
            black_queens: Pieces::new(Color::White, PieceType::Queen, Positions::new()),
            black_rooks: Pieces::new(Color::White, PieceType::Rook, Positions::new()),
            black_bishops: Pieces::new(Color::White, PieceType::Bishop, Positions::new()),
            black_knights: Pieces::new(Color::White, PieceType::Knight, Positions::new()),
            black_pawns: Pieces::new(Color::White, PieceType::Pawn, Positions::new()),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_north() {
        let cur_pos = Square::A1;
        let next_pos = cur_pos.go(Compass::North).unwrap();
        assert_eq!(next_pos, Square::A2);
    }

    #[test]
    fn move_south() {
        let cur_pos = Square::B2;
        let next_pos = cur_pos.go(Compass::South).unwrap();
        assert_eq!(next_pos, Square::B1);
    }

    #[test]
    fn move_east() {
        let cur_pos = Square::B1;
        let next_pos = cur_pos.go(Compass::East).unwrap();
        assert_eq!(next_pos, Square::C1);
    }

    #[test]
    fn move_west() {
        let cur_pos = Square::B2;
        let next_pos = cur_pos.go(Compass::West).unwrap();
        assert_eq!(next_pos, Square::A2);
    }

    #[test]
    fn move_northwest() {
        let cur_pos = Square::B2;
        let next_pos = cur_pos.go(Compass::NorthWest).unwrap();
        assert_eq!(next_pos, Square::A3);
    }

    #[test]
    fn move_northeast() {
        let cur_pos = Square::B2;
        let next_pos = cur_pos.go(Compass::NorthEast).unwrap();
        assert_eq!(next_pos, Square::C3);
    }

    #[test]
    fn move_southwest() {
        let cur_pos = Square::B2;
        let next_pos = cur_pos.go(Compass::SouthWest).unwrap();
        assert_eq!(next_pos, Square::A1);
    }

    #[test]
    fn move_southeast() {
        let cur_pos = Square::B2;
        let next_pos = cur_pos.go(Compass::SouthEast).unwrap();
        assert_eq!(next_pos, Square::C1);
    }

    #[test]
    fn move_offboard_south() {
        let cur_pos = Square::A1;
        let next_pos = cur_pos.go(Compass::South);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_north() {
        let cur_pos = Square::A8;
        let next_pos = cur_pos.go(Compass::North);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_east() {
        let cur_pos = Square::H5;
        let next_pos = cur_pos.go(Compass::East);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_west() {
        let cur_pos = Square::A5;
        let next_pos = cur_pos.go(Compass::West);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_northwest_north() {
        let cur_pos = Square::H8;
        let next_pos = cur_pos.go(Compass::NorthWest);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_northwest_west() {
        let cur_pos = Square::A1;
        let next_pos = cur_pos.go(Compass::NorthWest);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_northwest_northwest() {
        let cur_pos = Square::A8;
        let next_pos = cur_pos.go(Compass::NorthWest);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_southwest_south() {
        let cur_pos = Square::H1;
        let next_pos = cur_pos.go(Compass::SouthWest);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_southwest_west() {
        let cur_pos = Square::A8;
        let next_pos = cur_pos.go(Compass::NorthWest);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_southwest_southwest() {
        let cur_pos = Square::A1;
        let next_pos = cur_pos.go(Compass::NorthWest);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_northeast_north() {
        let cur_pos = Square::A8;
        let next_pos = cur_pos.go(Compass::NorthEast);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_northeast_east() {
        let cur_pos = Square::H1;
        let next_pos = cur_pos.go(Compass::NorthEast);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_northeast_northeast() {
        let cur_pos = Square::H8;
        let next_pos = cur_pos.go(Compass::NorthEast);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_southeast_south() {
        let cur_pos = Square::A1;
        let next_pos = cur_pos.go(Compass::SouthEast);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_southeast_east() {
        let cur_pos = Square::H8;
        let next_pos = cur_pos.go(Compass::SouthEast);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_southeast_southeast() {
        let cur_pos = Square::H1;
        let next_pos = cur_pos.go(Compass::SouthEast);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

}
