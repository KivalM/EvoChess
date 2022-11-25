use super::Chess;

impl Chess {
    pub fn ascii(&self) -> String {
        // get FEN notation
        let board = self.board.to_string();

        let mut ascii = String::new();
        ascii.push_str(" +-------------------------------+\n");

        for piece in board.chars() {
            // run until we find a space
            if piece == ' ' {
                break;
            }

            if piece == '/' {
                ascii.push_str(" |\n");
                ascii.push_str(" +-------------------------------+\n");
            } else if piece.is_numeric() {
                for _ in 0..piece.to_digit(10).unwrap() {
                    ascii.push_str(" |  ");
                }
            } else {
                ascii.push_str(" | ");
                ascii.push(piece);
            }
        }
        ascii.push_str(" |\n +-------------------------------+\n");
        ascii
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let chess = Chess::new();
        println!("{}", chess.ascii());

        // TODO: find a way to test this
        assert_ne!(chess.ascii().len(), 0);
    }
}
