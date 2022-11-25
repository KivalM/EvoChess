use chess::Square;
use evo_chess::error::EvoChessResult;
use web_sys::console;

use super::{pieces::x_y_to_ab, ChessUI};

impl ChessUI {
    pub fn click(&mut self, x: usize, y: usize) -> EvoChessResult<()> {
        let square_size = self.size / 8;
        let i = x as usize / square_size;
        let j = y as usize / square_size;

        // invert
        let j = 7 - j;

        if self.selected.is_none() {
            let sq = x_y_to_ab(x, y).parse::<Square>()?;
            if self.chess.board.piece_on(sq).is_some() {
                self.selected = Some((i, j));
            }
        } else {
            self.r#move(
                (self.selected.unwrap().0, self.selected.unwrap().1),
                (i, j),
                None,
            );
            self.selected = None;
        }
        Ok(())
        // self.selected = Some((i, j));
    }

    pub fn r#move(&mut self, from: (usize, usize), to: (usize, usize), promotion: Option<&str>) {
        let from = x_y_to_ab(from.0, from.1);
        let to = x_y_to_ab(to.0, to.1);

        log::warn!("{} -> {}", from, to);
        match self.chess.r#move(&from, &to, promotion) {
            Ok(_) => {
                log::warn!("Move successful");
                self.board = self.chess.board.to_string();
            }
            Err(_) => {
                log::warn!("Invalid move");
            }
        }
    }
}
