use chess::Square;

use crate::error::{EvoChessError, EvoChessResult};

use super::Chess;

impl Chess {
    /// This function is used to make a move
    pub fn r#move(&mut self, from: &str, to: &str, promotion: Option<&str>) -> EvoChessResult<()> {
        let from = from.parse::<Square>()?;
        let to = to.parse::<Square>()?;

        // parse the promotion
        let mut promotion = match promotion {
            Some(promotion) => match promotion {
                "q" => Some(chess::Piece::Queen),
                "r" => Some(chess::Piece::Rook),
                "b" => Some(chess::Piece::Bishop),
                "n" => Some(chess::Piece::Knight),
                _ => return Err(EvoChessError::new("Invalid promotion".to_string())),
            },
            None => Some(chess::Piece::Queen),
        };

        // check that a promotion is needed
        let piece = match self.board.piece_on(from) {
            Some(piece) => piece,
            None => return Err(EvoChessError::new("No piece on from square".to_string())),
        };

        if piece == chess::Piece::Pawn {
            let turn = self.board.side_to_move();
            if turn == chess::Color::White {
                if to.get_rank() != chess::Rank::Eighth {
                    promotion = None;
                }
            } else {
                if to.get_rank() != chess::Rank::First {
                    promotion = None;
                }
            }
        } else {
            promotion = None;
        }

        let chess_move = chess::ChessMove::new(from, to, promotion);
        println!("{:?}", chess_move);
        let moved = self.game.make_move(chess_move);

        if moved {
            // propogate changes to the board
            let board = self.board.clone();
            board.make_move(chess_move, &mut self.board);
        } else {
            return Err(EvoChessError::new(format!(
                "Cannot move piece on {} to {} because that is not a valid move",
                from.to_string(),
                to.to_string()
            )));
        }

        Ok(())
    }
}
