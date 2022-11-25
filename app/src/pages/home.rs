use yew::prelude::*;

use crate::components::chess::Chess;
use crate::components::header::Header;

pub struct Home;

pub enum Msg {}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Header/>
                <Chess/>
            </>
        }
    }
}
