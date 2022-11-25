// This module is a library to play chess
// we will handle the game logic here

use std::fmt::Display;

use chess::{Board, Game};

pub mod display;
pub mod moves;

/// This struct represents a chess game
pub struct Chess {
    /// The board of the game
    pub board: Board,
    /// The game handler
    pub game: Game,
}

pub enum State {
    /// The game is still going on
    Ongoing,
    /// The game is a draw
    Draw,
    /// The game is a checkmate
    Checkmate,
}

impl Chess {
    /// Creates a new chess game
    pub fn new() -> Self {
        Self {
            board: Board::default(),
            game: Game::new(),
        }
    }
}

impl Display for Chess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ascii())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_e2_e4() {
        let mut chess = Chess::new();
        chess.r#move("e2", "e4", None).unwrap();
        println!("{}", chess);
        chess.r#move("f7", "f5", None).unwrap();
        println!("{}", chess);
        chess.r#move("e4", "f5", None).unwrap();
        println!("{}", chess);
    }

    #[test]
    fn basic_b1_c3() {
        let mut chess = Chess::new();
        chess.r#move("b1", "c3", None).unwrap();
        println!("{}", chess);
    }
}
