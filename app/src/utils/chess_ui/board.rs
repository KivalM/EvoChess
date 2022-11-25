use super::ChessUI;
use wasm_bindgen::JsValue;

impl ChessUI {
    pub fn draw_chessboard(&self) {
        let square_size = self.size / 8;
        let mut color = true;

        let ctx = self.canvas_ctx.as_ref().unwrap();

        for i in 0..8 {
            for j in 0..8 {
                if color {
                    ctx.set_fill_style(&JsValue::from_str("pink"));
                } else {
                    ctx.set_fill_style(&JsValue::from_str("gray"));
                }

                ctx.fill_rect(
                    (i * square_size) as f64,
                    (j * square_size) as f64,
                    square_size as f64,
                    square_size as f64,
                );

                color = !color;
            }

            color = !color;
        }
    }
}
