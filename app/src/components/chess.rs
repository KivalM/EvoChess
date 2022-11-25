use std::collections::HashMap;

use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};
use yew::prelude::*;

use crate::utils::chess_ui::{preload::preload_pieces, ChessUI};

/// This struct represents a Chess UI component.

pub struct Chess {
    chess_ui: ChessUI,
    size: usize,
}

pub enum Msg {
    Preload,
    SavePrelodedPieces(HashMap<String, HtmlImageElement>),
    Draw,
    Clicked(usize, usize),
}

impl Component for Chess {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let size = 800;
        Self {
            chess_ui: ChessUI::new(None),
            size,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let canvas_on_click = ctx.link().callback(|e: MouseEvent| {
            let x = e.offset_x() as usize;
            let y = e.offset_y() as usize;
            Msg::Clicked(x, y)
        });
        html! {
            <div class="flex justify-center items-center">
                <canvas
                    id="chessboard"
                    onclick={canvas_on_click}
                    width={self.size.to_string()}
                    height={self.size.to_string()}
                />
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let document = web_sys::window().unwrap().document().unwrap();
            let canvas = document
                .get_element_by_id("chessboard")
                .unwrap()
                .dyn_into::<HtmlCanvasElement>()
                .unwrap();

            let canvas_ctx = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            // let width = document.body().unwrap().client_width() as usize;
            // let height = document.body().unwrap().client_height() as usize;
            // let size = if width < height { width } else { height };

            self.chess_ui.set_size(self.size);
            self.chess_ui.set_canvas_ctx(canvas_ctx);

            ctx.link().send_message(Msg::Preload);
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Draw => {
                self.chess_ui.draw_chessboard();
                self.chess_ui.draw_pieces();
                self.chess_ui.draw_outlines();
            }
            Msg::Preload => {
                let theme = "default";
                ctx.link().send_future(async move {
                    let pieces = preload_pieces(theme).await.unwrap();
                    Msg::SavePrelodedPieces(pieces)
                });
            }
            Msg::SavePrelodedPieces(pieces) => {
                self.chess_ui.set_pieces(pieces);
                ctx.link().send_message(Msg::Draw);
            }
            Msg::Clicked(x, y) => {
                self.chess_ui.click(x, y);
                ctx.link().send_message(Msg::Draw);
            }
        }

        true
    }
}
