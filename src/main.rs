mod app;

mod search_bar;
mod search_results;
mod song_card;
mod song_search;
mod spotify;

use wasm_logger::Config;
use app::App;

fn main() {
    wasm_logger::init(Config::default());
    yew::Renderer::<App>::new().render();
}
