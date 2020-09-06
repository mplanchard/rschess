//! Common traits

use crate::board::Move;
use crate::square::Square;

/// Something that can indicate whether it's occupied
pub trait Occupied {
    fn occupied(&self, square: &Square) -> bool;
}
pub trait Movable {
    fn apply_move(&self, mv: &Move) -> Self;
}
