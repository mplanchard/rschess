//! Common traits

use crate::square::Square;

/// Something that can indicate whether it's occupied
pub trait Occupied {
    fn occupied(&self, square: Square) -> bool;
}
