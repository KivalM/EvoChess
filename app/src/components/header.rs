use yew::prelude::*;

use crate::consts::NAME;

pub struct Header {
    dark_mode: bool,
}

pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { dark_mode: false }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <nav class="bg-white border-gray-200 px-2 sm:px-4 py-2.5 rounded dark:bg-gray-900">
                    <div class="container flex flex-wrap items-center justify-center mx-auto ">
                        <div class="flex items-center">
                            <img src="/assets/icons8-pawn-96.png" class="h-6 mr-3 sm:h-9" alt="EvoChess Logo" />
                            <span class="self-center text-xl font-semibold whitespace-nowrap dark:text-white montserrat">{NAME}</span>
                        </div>
                    </div>
                </nav>
            </div>
        }
    }
}
