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

pub struct MoveComponent {
    square: Square,
    board: BitBoard,
}
impl MoveComponent {
    pub const fn new(square: Square) -> Self {
        Self {
            square,
            board: BitBoard::from_square(square),
        }
    }
}

pub struct Move {
    from: MoveComponent,
    to: MoveComponent,
}
impl Move {
    pub const fn new(from: Square, to: Square) -> Self {
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

#[derive(Debug, Clone, Copy)]
enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
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
    const fn occupied(&self, square: Square) -> bool {
        self.board.intersects(&BitBoard::from_square(square))
    }
    pub fn apply_move(&self, mv: Move) -> Self {
        if self.board.intersects(&mv.from.board) {
            Self::new(
                self.piece,
                self.board
                    .intersection(&mv.from.board.complement())
                    .union(&mv.to.board),
            )
        } else {
            *self
        }
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
        }
    }
    /// Return all of the boards for all of the pieces
    pub fn all_pieces(&self) -> PiecesIter {
        PiecesIter::new(self)
    }
    pub fn occupied(&self, square: Square) -> bool {
        self.all_pieces().any(|pb| pb.board.occupied(square))
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
    pub fn all_pieces(&self) -> BoardIter {
        BoardIter::new(self)
    }
    /// Check whether a square is occupied
    pub fn occupied(&self, square: Square) -> bool {
        self.all_pieces().any(|pb| pb.board.occupied(square))
    }
    // /// Return a new board with a move applied.
    // ///
    // /// Move validity should be checked at the game level. No checking
    // /// is done here.
    // pub fn apply_move(&self, mv: Move) -> Board {

    // }
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
            BitBoard::from_boards(
                &Board::fresh_game()
                    .all_pieces()
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
                    .all_pieces()
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
                    .all_pieces()
                    .map(|pb| pb.board)
                    .collect()
            ) == bitboard::RANK_8.union(&bitboard::RANK_8.shift_south())
        );
    }

    #[test]
    fn test_occupied_true() {
        assert!(Board::fresh_game().occupied(Square::A1) == true);
        assert!(Board::fresh_game().occupied(Square::A8) == true);
    }

    #[test]
    fn test_occupied_false() {
        assert!(Board::fresh_game().occupied(Square::A3) == false);
        assert!(Board::fresh_game().occupied(Square::A6) == false);
    }
}
