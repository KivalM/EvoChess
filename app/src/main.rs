use pages::home::Home;

pub mod components;
pub mod consts;
pub mod pages;
pub mod utils;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Home>();
}
