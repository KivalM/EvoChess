use std::collections::HashMap;

use evo_chess::chess::Chess;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

pub mod board;
pub mod moves;
pub mod pieces;
pub mod preload;

pub struct ChessUI {
    // This is the chess game
    chess: Chess,

    /// The board position in fen notation
    board: String,

    /// the canvas context that we will draw on
    canvas_ctx: Option<CanvasRenderingContext2d>,

    /// the size of the canvas in px
    size: usize,

    /// the color in use for black squares as a hex string
    black_color: String,

    /// the color in use for white squares as a hex string
    white_color: String,

    /// A map of the pieces to their image urls
    /// The key is the piece name in fen notation
    /// The value is the Image Element
    pieces: HashMap<String, HtmlImageElement>,

    /// the image theme to use
    theme: String,

    /// Selected square
    selected: Option<(usize, usize)>,
}

impl ChessUI {
    pub fn new(ctx: Option<CanvasRenderingContext2d>) -> Self {
        Self {
            chess: Chess::new(),
            board: String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"),
            canvas_ctx: ctx,
            size: 400,
            black_color: String::from("#000000"),
            white_color: String::from("#FFFFFF"),
            pieces: HashMap::new(),
            theme: "default".to_string(),
            selected: None,
        }
    }

    pub fn set_size(&mut self, size: usize) {
        self.size = size;
    }

    pub fn set_board(&mut self, board: String) {
        self.board = board;
    }

    pub fn set_black_color(&mut self, color: String) {
        self.black_color = color;
    }

    pub fn set_white_color(&mut self, color: String) {
        self.white_color = color;
    }

    pub fn set_canvas_ctx(&mut self, ctx: CanvasRenderingContext2d) {
        self.canvas_ctx = Some(ctx);
    }

    pub fn set_theme(&mut self, theme: String) {
        self.theme = theme;
    }

    pub fn set_pieces(&mut self, pieces: HashMap<String, HtmlImageElement>) {
        self.pieces = pieces;
    }
}
