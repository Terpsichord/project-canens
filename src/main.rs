mod app;

mod error;
mod fretmap;
mod home;
mod hooktheory;
mod search_bar;
mod search_results;
mod song;
mod song_card;
mod song_info;
mod song_search;
mod spotify;

use app::App;
use wasm_logger::Config;

fn main() {
    wasm_logger::init(Config::default());
    yew::Renderer::<App>::new().render();
}
