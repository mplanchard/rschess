use std::convert::TryFrom;
use std::error;
use std::fmt;
use std::ops;


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

trait SelfReturningCheckedAdd<T>
where
    Self: Sized,
{
    fn checked_add(self, rhs: T) -> Option<Self>;
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct BitIndex(u8);
impl From<Square> for BitIndex {
    fn from(sq: Square) -> Self {
        Self::sq_to_bitindex(&sq)
    }
}
impl From<&Square> for BitIndex {
    fn from(sq: &Square) -> Self {
        Self::sq_to_bitindex(sq)
    }
}
impl Into<u8> for BitIndex {
    fn into(self) -> u8 {
        self.0
    }
}
impl Into<u8> for &BitIndex {
    fn into(self) -> u8 {
        self.0
    }
}
impl ops::Rem<u8> for BitIndex {
    type Output = u8;

    fn rem(self, other: u8) -> u8 {
        self.0 % other
    }
}
impl ops::Rem<u8> for &BitIndex {
    type Output = u8;

    fn rem(self, other: u8) -> u8 {
        self.0 % other
    }
}
impl ops::Rem<BitIndex> for &BitIndex {
    type Output = u8;

    fn rem(self, other: BitIndex) -> u8 {
        self.0 % other.0
    }
}
impl ops::Rem<&BitIndex> for &BitIndex {
    type Output = u8;

    fn rem(self, other: &BitIndex) -> u8 {
        self.0 % other.0
    }
}
impl SelfReturningCheckedAdd<u8> for BitIndex {
    /// Do addition, and return None on failure
    ///
    /// ```rs
    /// assert_eq!(BitIndex::from(8).checked_add(8), Some(BitIndex(16)));
    /// assert_eq!(BitIndex::from(255).checked_add(255), None);
    /// ```
    fn checked_add(self, rhs: u8) -> Option<BitIndex> {
        Some(Self(self.0.checked_add(rhs)?))
    }
}
impl SelfReturningCheckedAdd<i8> for BitIndex {
    /// Add a potentially negative i8
    ///
    /// Will return `None` if the absolute value of the i8 cannot be converted
    /// to a u8 (although this should always be safe), or if the resultant
    /// addition/subtraction goes outside of the u8 bounds.
    ///
    /// ```rs
    /// assert_eq!(BitIndex::from(8).checked_add(2), Some(BitIndex(10)));
    /// assert_eq!(BitIndex::from(8).checked_add(-2), Some(BitIndex(6)));
    /// assert_eq!(BitIndex::from(0).checked_add(-2), None);
    /// assert_eq!(BitIndex::from(255).checked_add(255), None);
    /// ```
    fn checked_add(self, rhs: i8) -> Option<BitIndex> {
        let func = if rhs < 0 {
            u8::checked_sub
        } else {
            u8::checked_add
        };
        let conv = u8::try_from(rhs.abs()).ok()?;
        Some(Self(func(self.0, conv)?))
    }
}
impl BitIndex {
    fn from<T: Into<u8>>(t: T) -> Option<Self> {
        let val = t.into();
        return if val < 64 { Some(Self(val)) } else { None };
    }
    fn sq_to_bitindex(sq: &Square) -> BitIndex {
        match sq {
            Square::A1 => BitIndex(0),
            Square::B1 => BitIndex(1),
            Square::C1 => BitIndex(2),
            Square::D1 => BitIndex(3),
            Square::E1 => BitIndex(4),
            Square::F1 => BitIndex(5),
            Square::G1 => BitIndex(6),
            Square::H1 => BitIndex(7),
            Square::A2 => BitIndex(8),
            Square::B2 => BitIndex(9),
            Square::C2 => BitIndex(10),
            Square::D2 => BitIndex(11),
            Square::E2 => BitIndex(12),
            Square::F2 => BitIndex(13),
            Square::G2 => BitIndex(14),
            Square::H2 => BitIndex(15),
            Square::A3 => BitIndex(16),
            Square::B3 => BitIndex(17),
            Square::C3 => BitIndex(18),
            Square::D3 => BitIndex(19),
            Square::E3 => BitIndex(20),
            Square::F3 => BitIndex(21),
            Square::G3 => BitIndex(22),
            Square::H3 => BitIndex(23),
            Square::A4 => BitIndex(24),
            Square::B4 => BitIndex(25),
            Square::C4 => BitIndex(26),
            Square::D4 => BitIndex(27),
            Square::E4 => BitIndex(28),
            Square::F4 => BitIndex(29),
            Square::G4 => BitIndex(30),
            Square::H4 => BitIndex(31),
            Square::A5 => BitIndex(32),
            Square::B5 => BitIndex(33),
            Square::C5 => BitIndex(34),
            Square::D5 => BitIndex(35),
            Square::E5 => BitIndex(36),
            Square::F5 => BitIndex(37),
            Square::G5 => BitIndex(38),
            Square::H5 => BitIndex(39),
            Square::A6 => BitIndex(40),
            Square::B6 => BitIndex(41),
            Square::C6 => BitIndex(42),
            Square::D6 => BitIndex(43),
            Square::E6 => BitIndex(44),
            Square::F6 => BitIndex(45),
            Square::G6 => BitIndex(46),
            Square::H6 => BitIndex(47),
            Square::A7 => BitIndex(48),
            Square::B7 => BitIndex(49),
            Square::C7 => BitIndex(50),
            Square::D7 => BitIndex(51),
            Square::E7 => BitIndex(52),
            Square::F7 => BitIndex(53),
            Square::G7 => BitIndex(54),
            Square::H7 => BitIndex(55),
            Square::A8 => BitIndex(56),
            Square::B8 => BitIndex(57),
            Square::C8 => BitIndex(58),
            Square::D8 => BitIndex(59),
            Square::E8 => BitIndex(60),
            Square::F8 => BitIndex(61),
            Square::G8 => BitIndex(62),
            Square::H8 => BitIndex(63),
        }
    }
}


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
impl Compass {
    fn to_offset(&self) -> i8 {
        match self {
            Compass::East => 1,
            Compass::West => -1,

            Compass::North => 8,
            Compass::South => -8,

            Compass::NorthEast => 9,
            Compass::SouthWest => -9,

            Compass::NorthWest => 7,
            Compass::SouthEast => -7,
        }
    }
}


// The integer associations are bit indexes on the 64 bit position.
#[derive(Debug, PartialEq)]
#[cfg_attr(rustfmt, rustfmt_skip)]
enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}
impl Square {
    // Constants courtesy chessprogramming.org
    // const A_FILE: u64 = 0x0101010101010101;
    // const H_FILE: u64 = 0x8080808080808080;
    // const RANK_1: u64 = 0x00000000000000FF;
    // const RANK_8: u64 = 0xFF00000000000000;

    fn to_bitindex(&self) -> BitIndex {
        self.into()
    }

    fn next_square_on_board(&self, direction: &Compass) -> bool {
        match direction {
            Compass::North => self.north_on_board(),
            Compass::South => self.south_on_board(),
            Compass::East => self.east_on_board(),
            Compass::West => self.west_on_board(),
            Compass::NorthEast => self.north_on_board() && self.east_on_board(),
            Compass::NorthWest => self.north_on_board() && self.west_on_board(),
            Compass::SouthEast => self.south_on_board() && self.east_on_board(),
            Compass::SouthWest => self.south_on_board() && self.west_on_board(),
        }
    }

    fn next_square(&self, direction: Compass) -> Result<Self, Error> {
        if !self.next_square_on_board(&direction) {
            return Err(Error::OffBoard);
        }
        let cur: BitIndex = self.into();
        let offset = direction.to_offset();
        let new_bitindex = cur.checked_add(offset).ok_or(Error::OffBoard)?;
        Self::try_from(new_bitindex)
    }
    fn on_a_file(&self) -> bool {
        self.to_bitindex() % 8 == 0
    }
    fn on_first_rank(&self) -> bool {
        let bi: u8 = self.to_bitindex().into();
        bi < 8 as u8
    }
    fn on_last_rank(&self) -> bool {
        let bi: u8 = self.to_bitindex().into();
        bi > 55 as u8
    }
    fn on_h_file(&self) -> bool {
        self.to_bitindex() % 8 == 7
    }

    fn east_on_board(&self) -> bool {
        !self.on_h_file()
    }

    fn west_on_board(&self) -> bool {
        !self.on_a_file()
    }

    fn north_on_board(&self) -> bool {
        !self.on_last_rank()
    }

    fn south_on_board(&self) -> bool {
        !self.on_first_rank()
    }
}
impl TryFrom<BitIndex> for Square {
    type Error = Error;

    fn try_from(bi: BitIndex) -> Result<Self, Self::Error> {
        let val: u8 = bi.into();
        match val {
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
}

/// Piece Positions, using a bitboard representation
struct BitBoard(u64);
impl BitBoard {
    /// Create a new bitboard with no pieces
    fn new() -> Self {
        Self(0)
    }
    fn from<T: Into<u64>>(position: T) -> Self {
        Self(position.into())
    }
}
impl Into<u64> for BitBoard {
    fn into(self) -> u64 {
        self.0
    }
}
impl Into<u64> for &BitBoard {
    fn into(self) -> u64 {
        self.0
    }
}
impl fmt::Binary for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:b}", self.0)
    }
}
impl ops::BitAnd for BitBoard {
    type Output = BitBoard;

    fn bitand(self, other: BitBoard) -> BitBoard {
        Self::from(self.0 & other.0)
    }
}
impl ops::BitAnd for &BitBoard {
    type Output = BitBoard;

    fn bitand(self, other: &BitBoard) -> BitBoard {
        BitBoard::from(self.0 & other.0)
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
    positions: BitBoard,
}
impl Pieces {
    fn new(color: Color, piece_type: PieceType, positions: BitBoard) -> Self {
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
            white_king: Pieces::new(Color::White, PieceType::King, BitBoard::new()),
            white_queens: Pieces::new(Color::White, PieceType::Queen, BitBoard::new()),
            white_rooks: Pieces::new(Color::White, PieceType::Rook, BitBoard::new()),
            white_bishops: Pieces::new(Color::White, PieceType::Bishop, BitBoard::new()),
            white_knights: Pieces::new(Color::White, PieceType::Knight, BitBoard::new()),
            white_pawns: Pieces::new(Color::White, PieceType::Pawn, BitBoard::new()),
            black_king: Pieces::new(Color::White, PieceType::King, BitBoard::new()),
            black_queens: Pieces::new(Color::White, PieceType::Queen, BitBoard::new()),
            black_rooks: Pieces::new(Color::White, PieceType::Rook, BitBoard::new()),
            black_bishops: Pieces::new(Color::White, PieceType::Bishop, BitBoard::new()),
            black_knights: Pieces::new(Color::White, PieceType::Knight, BitBoard::new()),
            black_pawns: Pieces::new(Color::White, PieceType::Pawn, BitBoard::new()),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_north() {
        let cur_pos = Square::A1;
        let next_pos = cur_pos.next_square(Compass::North).unwrap();
        assert_eq!(next_pos, Square::A2);
    }

    #[test]
    fn move_south() {
        let cur_pos = Square::B2;
        let next_pos = cur_pos.next_square(Compass::South).unwrap();
        assert_eq!(next_pos, Square::B1);
    }

    #[test]
    fn move_east() {
        let cur_pos = Square::B1;
        let next_pos = cur_pos.next_square(Compass::East).unwrap();
        assert_eq!(next_pos, Square::C1);
    }

    #[test]
    fn move_west() {
        let cur_pos = Square::B2;
        let next_pos = cur_pos.next_square(Compass::West).unwrap();
        assert_eq!(next_pos, Square::A2);
    }

    #[test]
    fn move_northwest() {
        let cur_pos = Square::B2;
        let next_pos = cur_pos.next_square(Compass::NorthWest).unwrap();
        assert_eq!(next_pos, Square::A3);
    }

    #[test]
    fn move_northeast() {
        let cur_pos = Square::B2;
        let next_pos = cur_pos.next_square(Compass::NorthEast).unwrap();
        assert_eq!(next_pos, Square::C3);
    }

    #[test]
    fn move_southwest() {
        let cur_pos = Square::B2;
        let next_pos = cur_pos.next_square(Compass::SouthWest).unwrap();
        assert_eq!(next_pos, Square::A1);
    }

    #[test]
    fn move_southeast() {
        let cur_pos = Square::B2;
        let next_pos = cur_pos.next_square(Compass::SouthEast).unwrap();
        assert_eq!(next_pos, Square::C1);
    }

    #[test]
    fn move_offboard_south() {
        let cur_pos = Square::A1;
        let next_pos = cur_pos.next_square(Compass::South);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_north() {
        let cur_pos = Square::A8;
        let next_pos = cur_pos.next_square(Compass::North);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_east() {
        let cur_pos = Square::H5;
        let next_pos = cur_pos.next_square(Compass::East);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_west() {
        let cur_pos = Square::A5;
        let next_pos = cur_pos.next_square(Compass::West);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_northwest_north() {
        let cur_pos = Square::H8;
        let next_pos = cur_pos.next_square(Compass::NorthWest);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_northwest_west() {
        let cur_pos = Square::A1;
        let next_pos = cur_pos.next_square(Compass::NorthWest);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_northwest_northwest() {
        let cur_pos = Square::A8;
        let next_pos = cur_pos.next_square(Compass::NorthWest);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_southwest_south() {
        let cur_pos = Square::H1;
        let next_pos = cur_pos.next_square(Compass::SouthWest);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_southwest_west() {
        let cur_pos = Square::A8;
        let next_pos = cur_pos.next_square(Compass::NorthWest);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_southwest_southwest() {
        let cur_pos = Square::A1;
        let next_pos = cur_pos.next_square(Compass::NorthWest);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_northeast_north() {
        let cur_pos = Square::A8;
        let next_pos = cur_pos.next_square(Compass::NorthEast);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_northeast_east() {
        let cur_pos = Square::H1;
        let next_pos = cur_pos.next_square(Compass::NorthEast);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_northeast_northeast() {
        let cur_pos = Square::H8;
        let next_pos = cur_pos.next_square(Compass::NorthEast);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_southeast_south() {
        let cur_pos = Square::A1;
        let next_pos = cur_pos.next_square(Compass::SouthEast);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_southeast_east() {
        let cur_pos = Square::H8;
        let next_pos = cur_pos.next_square(Compass::SouthEast);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    #[test]
    fn move_offboard_southeast_southeast() {
        let cur_pos = Square::H1;
        let next_pos = cur_pos.next_square(Compass::SouthEast);
        assert_eq!(next_pos, Err(Error::OffBoard));
    }

    // fn get_legal_moves(sq: &Square) -> Vec<Compass> {
    //     let idx = sq.to_bitindex();
    //     let legals = Vec::new();

    //     // Correspond to being on the first/last file/rank
    //     let east_illegal = |i| i % 8 == 7;
    //     let west_illegal = |i| i % 8 == 0;
    //     let north_illegal = |i| i > 55;
    //     let south_illegal = |i| i < 8;
    //     let northwest_illegal = |i| north_illegal(i) || west_illegal(i);
    //     let northeast_illegal = |i| north_illegal(i) || east_illegal(i);
    //     let southwest_illegal = |i| south_illegal(i) || west_illegal(i);
    //     let southeast_illegal = |i| south_illegal(i) || east_illegal(i);

    //     //
    // }

}
