use chess::MoveGen;
use wasm_bindgen::JsValue;

use super::ChessUI;

impl ChessUI {
    pub fn draw_pieces(&self) {
        let mut x = 0;
        let mut y = 0;

        for char in self.board.chars() {
            // end on space
            if char == ' ' {
                break;
            } else if char == '/' {
                x = 0;
                y += 1;
            } else if char.is_alphabetic() {
                let piece = self.pieces.get(&char.to_string()).unwrap();

                let ctx = self.canvas_ctx.as_ref().unwrap();
                ctx.draw_image_with_html_image_element_and_dw_and_dh(
                    piece,
                    ((x * self.size) / 8) as f64,
                    ((y * self.size) / 8) as f64,
                    (self.size / 8) as f64,
                    (self.size / 8) as f64,
                )
                .unwrap();
                x += 1;
            } else {
                x += char.to_digit(10).unwrap() as usize;
            }
        }
    }

    pub fn draw_outlines(&self) {
        if self.selected.is_none() {
            return;
        }

        let (x, y) = self.selected.unwrap();
        let from = x_y_to_ab(x, y);

        let ctx = self.canvas_ctx.as_ref().unwrap();
        ctx.set_stroke_style(&JsValue::from_str("green"));
        ctx.set_line_width(2.0);

        //invert y-axis
        let y = 7 - y;
        ctx.stroke_rect(
            ((x * self.size) / 8) as f64,
            ((y * self.size) / 8) as f64,
            (self.size / 8) as f64,
            (self.size / 8) as f64,
        );

        // generate a set of all legal moves
        let valid_moves = MoveGen::new_legal(&self.chess.board);

        for i in valid_moves {
            // check if the move is from the selected square
            if i.get_source().to_string() == from {
                let (x, y) = ab_to_x_y(&i.get_dest().to_string());

                // draw a red outline around the square
                ctx.set_stroke_style(&JsValue::from_str("red"));
                ctx.stroke_rect(
                    ((x * self.size) / 8) as f64,
                    ((y * self.size) / 8) as f64,
                    (self.size / 8) as f64,
                    (self.size / 8) as f64,
                );
            }
        }
    }
}

pub fn x_y_to_ab(x: usize, y: usize) -> String {
    let mut a = x as u32 + 97;
    let mut b = y as u32 + 49;

    if a > 104 {
        a = 104;
    }

    if b > 56 {
        b = 56;
    }

    let mut ab = String::new();
    ab.push(char::from_u32(a).unwrap());
    ab.push(char::from_u32(b).unwrap());

    ab
}

pub fn ab_to_x_y(s: &str) -> (usize, usize) {
    let mut x = s.chars().nth(0).unwrap() as usize - 97;
    let mut y = s.chars().nth(1).unwrap() as usize - 49;

    if x > 7 {
        x = 7;
    }

    if y > 7 {
        y = 7;
    }

    // invert
    y = 7 - y;

    (x, y)
}
