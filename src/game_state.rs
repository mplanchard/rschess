//! Tracking state snapshots of a game

use crate::board::{Board, Color};

struct StateMeta {}

struct GameState<'a> {
    previous: Option<&'a GameState<'a>>,
    board: Board,
    turn: Color,
    meta: StateMeta,
}
