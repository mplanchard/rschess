//! Board representation

use colored::*;
use std::fmt;

use crate::bitboard::BitBoard;
use crate::square::{Square, SquareColor, SQUARES};
use crate::traits::{Movable, Occupied};

// Constants for starting piece locations

const STARTING_WHITE_PAWNS: BitBoard = BitBoard::new(0x000000000000ff00);
const STARTING_BLACK_PAWNS: BitBoard = BitBoard::new(0x00ff000000000000);

const STARTING_WHITE_ROOKS: BitBoard = BitBoard::new(0x0000000000000081);
const STARTING_BLACK_ROOKS: BitBoard = BitBoard::new(0x8100000000000000);

const STARTING_WHITE_KNIGHTS: BitBoard = BitBoard::new(0x0000000000000042);
const STARTING_BLACK_KNIGHTS: BitBoard = BitBoard::new(0x4200000000000000);

const STARTING_WHITE_BISHOPS: BitBoard = BitBoard::new(0x0000000000000024);
const STARTING_BLACK_BISHOPS: BitBoard = BitBoard::new(0x2400000000000000);

const STARTING_WHITE_QUEEN: BitBoard = BitBoard::from_square(&Square::D1);
const STARTING_BLACK_QUEEN: BitBoard = BitBoard::from_square(&Square::D8);

const STARTING_WHITE_KING: BitBoard = BitBoard::from_square(&Square::E1);
const STARTING_BLACK_KING: BitBoard = BitBoard::from_square(&Square::E8);

// Precalculating starting board
const STARTING_BOARD: Board = Board::fresh_game();

pub struct MoveComponent<'a> {
    square: &'a Square,
    board: BitBoard,
}
impl<'a> MoveComponent<'a> {
    pub const fn new(square: &'a Square) -> Self {
        Self {
            square,
            board: BitBoard::from_square(square),
        }
    }
}

pub struct Move<'a> {
    from: MoveComponent<'a>,
    to: MoveComponent<'a>,
}
impl<'a> Move<'a> {
    pub const fn new(from: &'a Square, to: &'a Square) -> Self {
        Self {
            from: MoveComponent::new(from),
            to: MoveComponent::new(to),
        }
    }
}

struct Stash {
    all_pieces: Option<BitBoard>,
}
impl Stash {
    pub const fn new() -> Self {
        Self { all_pieces: None }
    }
    pub fn set_all_pieces(&mut self, pieces: BitBoard) -> BitBoard {
        self.all_pieces = Some(pieces);
        pieces
    }
}

#[derive(Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}
impl Piece {
    fn to_string(&self) -> &'static str {
        match self {
            Self::King => "♚",
            Self::Queen => "♛",
            Self::Rook => "♜",
            Self::Bishop => "♝",
            Self::Knight => "♞",
            Self::Pawn => "♟",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PieceBoard {
    piece: Piece,
    board: BitBoard,
}
impl PieceBoard {
    const fn new(piece: Piece, board: BitBoard) -> Self {
        Self { piece, board }
    }
}
impl Occupied for PieceBoard {
    fn occupied(&self, square: &Square) -> bool {
        self.board.intersects(&BitBoard::from_square(&square))
    }
}
impl Movable for PieceBoard {
    fn apply_move(&self, mv: &Move) -> Self {
        if self.board.intersects(&mv.from.board) {
            Self::new(
                self.piece,
                self.board
                    // remove any pice in the from square
                    .intersection(&mv.from.board.complement())
                    // and place the piece in the to square
                    .union(&mv.to.board),
            )
        } else if self.board.intersects(&mv.to.board) {
            Self::new(
                self.piece,
                // Remove any piece in the to square
                self.board.intersection(&mv.to.board.complement()),
            )
        } else {
            *self
        }
    }
}
impl fmt::Display for PieceBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.piece, self.board)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PiecesIter<'a> {
    idx: u8,
    pieces: &'a Pieces,
}
impl<'a> PiecesIter<'a> {
    pub fn new(pieces: &'a Pieces) -> Self {
        Self { idx: 0, pieces }
    }
}
impl<'a> Iterator for PiecesIter<'a> {
    type Item = &'a PieceBoard;
    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        match self.idx {
            1 => Some(&self.pieces.king),
            2 => Some(&self.pieces.queens),
            3 => Some(&self.pieces.rooks),
            4 => Some(&self.pieces.bishops),
            5 => Some(&self.pieces.knights),
            6 => Some(&self.pieces.pawns),
            _ => None,
        }
    }
}

/// A representation of a side's pieces
#[derive(Debug)]
pub struct Pieces {
    king: PieceBoard,
    queens: PieceBoard,
    rooks: PieceBoard,
    bishops: PieceBoard,
    knights: PieceBoard,
    pawns: PieceBoard,
    all: BitBoard,
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
            king: PieceBoard::new(Piece::King, king),
            queens: PieceBoard::new(Piece::Queen, queens),
            rooks: PieceBoard::new(Piece::Rook, rooks),
            bishops: PieceBoard::new(Piece::Bishop, bishops),
            knights: PieceBoard::new(Piece::Knight, knights),
            pawns: PieceBoard::new(Piece::Pawn, pawns),
            all: king
                .union(&queens)
                .union(&rooks)
                .union(&bishops)
                .union(&knights)
                .union(&pawns),
        }
    }
    /// Return all of the boards for all of the pieces
    pub fn iter_pieces(&self) -> PiecesIter {
        PiecesIter::new(self)
    }
}
impl Occupied for Pieces {
    /// Return whether the given square is occupied by this piece set.
    fn occupied(&self, square: &Square) -> bool {
        self.all.occupied(square)
    }
}
impl Movable for Pieces {
    /// Apply a move to the pice set and return a new one
    fn apply_move(&self, mv: &Move) -> Self {
        let king = self.king.apply_move(mv);
        let queens = self.queens.apply_move(mv);
        let rooks = self.rooks.apply_move(mv);
        let bishops = self.bishops.apply_move(mv);
        let knights = self.knights.apply_move(mv);
        let pawns = self.pawns.apply_move(mv);
        Self {
            king,
            queens,
            rooks,
            bishops,
            knights,
            pawns,
            all: king
                .board
                .union(&queens.board)
                .union(&rooks.board)
                .union(&bishops.board)
                .union(&knights.board)
                .union(&pawns.board),
        }
    }
}
impl fmt::Display for Pieces {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let piece_strings = self
            .iter_pieces()
            .map(|p| format!("{}", p))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "\n{}\n", piece_strings)
    }
}

#[derive(Debug)]
pub struct BoardIter<'a> {
    idx: u8,
    board: &'a Board,
}
impl<'a> BoardIter<'a> {
    pub fn new(board: &'a Board) -> Self {
        Self { idx: 0, board }
    }
}
impl<'a> Iterator for BoardIter<'a> {
    type Item = &'a PieceBoard;
    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        match self.idx {
            1 => Some(&self.board.white.king),
            2 => Some(&self.board.white.queens),
            3 => Some(&self.board.white.rooks),
            4 => Some(&self.board.white.bishops),
            5 => Some(&self.board.white.knights),
            6 => Some(&self.board.white.pawns),
            7 => Some(&self.board.black.king),
            8 => Some(&self.board.black.queens),
            9 => Some(&self.board.black.rooks),
            10 => Some(&self.board.black.bishops),
            11 => Some(&self.board.black.knights),
            12 => Some(&self.board.black.pawns),
            _ => None,
        }
    }
}

/// A representation of the game board
#[derive(Debug)]
pub struct Board {
    white: Pieces,
    black: Pieces,
}
impl Board {
    /// Construct a new side from a set of pieces
    pub const fn new(white: Pieces, black: Pieces) -> Self {
        Self { white, black }
    }
    /// Construct a new board with typical starting positions
    pub const fn fresh_game() -> Self {
        Self::new(
            Pieces::new(
                STARTING_WHITE_KING,
                STARTING_WHITE_QUEEN,
                STARTING_WHITE_ROOKS,
                STARTING_WHITE_BISHOPS,
                STARTING_WHITE_KNIGHTS,
                STARTING_WHITE_PAWNS,
            ),
            Pieces::new(
                STARTING_BLACK_KING,
                STARTING_BLACK_QUEEN,
                STARTING_BLACK_ROOKS,
                STARTING_BLACK_BISHOPS,
                STARTING_BLACK_KNIGHTS,
                STARTING_BLACK_PAWNS,
            ),
        )
    }
    /// Return all of the bitboards comprising the board
    pub fn iter_pieces(&self) -> BoardIter {
        BoardIter::new(self)
    }
    pub fn occupant(&self, square: &Square) -> Option<(Piece, Color)> {
        let white_iter = self
            .white
            .iter_pieces()
            .filter(|pieces| pieces.occupied(square))
            .map(|pieces| (pieces.piece, Color::White));
        let black_iter = self
            .black
            .iter_pieces()
            .filter(|pieces| pieces.occupied(square))
            .map(|pieces| (pieces.piece, Color::Black));
        white_iter.chain(black_iter).next()
    }
}
impl Occupied for Board {
    /// Check whether a square is occupied
    fn occupied(&self, square: &Square) -> bool {
        self.white.all.occupied(square) || self.black.all.occupied(square)
    }
}
impl Movable for Board {
    /// Return a new board with a move applied.
    ///
    /// Move validity should be checked at the game level. No checking
    /// is done here.
    fn apply_move(&self, mv: &Move) -> Self {
        Self::new(self.white.apply_move(mv), self.black.apply_move(mv))
    }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let white_piece_strings = self
            .white
            .iter_pieces()
            .map(|p| format!("  {}", p))
            .collect::<Vec<String>>()
            .join("\n");
        let black_piece_strings = self
            .black
            .iter_pieces()
            .map(|p| format!("  {}", p))
            .collect::<Vec<String>>()
            .join("\n");
        write!(
            f,
            "\nWhite:\n{}Black:\n{}\n",
            white_piece_strings, black_piece_strings
        )
    }
}
impl Board {
    // hacky little console board output to help visualize things
    fn board_string(&self) -> String {
        // todo: measure capacity
        let mut board_str = String::with_capacity(4096);
        SQUARES
            .iter()
            .map(|square| {
                let empty_spaces = "    ";
                let empty_line = match square.color() {
                    SquareColor::White => empty_spaces.on_green(),
                    SquareColor::Black => empty_spaces.on_blue(),
                };
                let middle_line;
                if self.occupied(square) {
                    let occupant = self.occupant(square).unwrap();
                    middle_line = match occupant.1 {
                        Color::White => format!(" {}  ", occupant.0.to_string().white()),
                        Color::Black => format!(" {}  ", occupant.0.to_string().black()),
                    };
                } else {
                    middle_line = empty_spaces.into();
                }
                match square.color() {
                    SquareColor::White => {
                        [empty_line.to_string(), middle_line.on_green().to_string()]
                    }
                    SquareColor::Black => {
                        [empty_line.to_string(), middle_line.on_blue().to_string()]
                    }
                }
            })
            .collect::<Vec<[String; 2]>>()
            .chunks_exact(8)
            .rev()
            .for_each(|square_blocks: &[[String; 2]]| {
                (0..2).for_each(|idx| {
                    square_blocks
                        .iter()
                        .for_each(|block| board_str.push_str(&(block.get(idx)).unwrap()));
                    board_str.push('\n');
                })
            });
        println!("{}", board_str.capacity());
        board_str
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
        assert!(STARTING_WHITE_BISHOPS == BitBoard::from_squares(&vec![Square::C1, Square::F1]));
    }

    #[test]
    fn test_starting_black_bishops() {
        assert!(STARTING_BLACK_BISHOPS == BitBoard::from_squares(&vec![Square::C8, Square::F8]));
    }

    #[test]
    fn test_starting_white_knights() {
        assert!(STARTING_WHITE_KNIGHTS == BitBoard::from_squares(&vec![Square::B1, Square::G1]));
    }

    #[test]
    fn test_starting_black_knights() {
        assert!(STARTING_BLACK_KNIGHTS == BitBoard::from_squares(&vec![Square::B8, Square::G8]));
    }

    // *****************************************************************
    // Test board
    // *****************************************************************

    #[test]
    fn test_fresh_game_all_pieces() {
        assert!(
            BitBoard::from_boards(
                &Board::fresh_game()
                    .iter_pieces()
                    .map(|pb| pb.board)
                    .collect()
            ) == bitboard::RANK_1
                .union(&bitboard::RANK_1.shift_north())
                .union(&bitboard::RANK_8)
                .union(&bitboard::RANK_8.shift_south())
        );
    }

    #[test]
    fn test_fresh_game_white_pieces() {
        assert!(
            BitBoard::from_boards(
                &Board::fresh_game()
                    .white
                    .iter_pieces()
                    .map(|pb| pb.board)
                    .collect()
            ) == bitboard::RANK_1.union(&bitboard::RANK_1.shift_north())
        );
    }

    #[test]
    fn test_fresh_game_black_pieces() {
        assert!(
            BitBoard::from_boards(
                &Board::fresh_game()
                    .black
                    .iter_pieces()
                    .map(|pb| pb.board)
                    .collect()
            ) == bitboard::RANK_8.union(&bitboard::RANK_8.shift_south())
        );
    }

    #[test]
    fn test_occupied_true() {
        assert!(Board::fresh_game().occupied(&Square::A1) == true);
        assert!(Board::fresh_game().occupied(&Square::A8) == true);
    }

    #[test]
    fn test_occupied_false() {
        assert!(Board::fresh_game().occupied(&Square::A3) == false);
        assert!(Board::fresh_game().occupied(&Square::A6) == false);
    }

    /// It moves the piece
    #[test]
    fn test_apply_move_from_occupied_to_empty() {
        let board = Board::fresh_game();
        assert!(board.white.pawns.occupied(&Square::A2));
        assert!(!board.occupied(&Square::A3));
        let new_board = board.apply_move(&Move::new(&Square::A2, &Square::A3));
        assert!(new_board.white.pawns.occupied(&Square::A3));
        assert!(!new_board.occupied(&Square::A2));
    }

    /// It moves the piece, replacing the piece in the target location
    #[test]
    fn test_apply_move_from_occupied_to_occupied() {
        let board = Board::fresh_game();
        assert!(board.white.rooks.occupied(&Square::A1));
        assert!(board.black.pawns.occupied(&Square::A7));
        let new_board = board.apply_move(&Move::new(&Square::A1, &Square::A7));
        assert!(!new_board.white.rooks.occupied(&Square::A1));
        assert!(!new_board.black.pawns.occupied(&Square::A7));
        assert!(new_board.white.rooks.occupied(&Square::A7));
    }

    /// It does nothing
    #[test]
    fn test_apply_move_from_empty_to_empty() {
        let board = Board::fresh_game();
        assert!(!board.occupied(&Square::A3));
        assert!(!board.occupied(&Square::A4));
        let new_board = board.apply_move(&Move::new(&Square::A3, &Square::A4));
        assert!(!new_board.occupied(&Square::A3));
        assert!(!new_board.occupied(&Square::A4));
    }

    /// It removes the piece in the target location
    #[test]
    fn test_apply_move_from_empty_to_occupied() {
        let board = Board::fresh_game();
        assert!(!board.occupied(&Square::A3));
        assert!(board.white.pawns.occupied(&Square::A2));
        let new_board = board.apply_move(&Move::new(&Square::A3, &Square::A2));
        assert!(!new_board.occupied(&Square::A3));
        assert!(!new_board.occupied(&Square::A2));
        assert!(!new_board.white.pawns.occupied(&Square::A2));
    }
}
