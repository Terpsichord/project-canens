mod app;

mod error;
mod fretmap;
mod home;
mod hooktheory;
mod search;
mod song;
mod spotify;

use app::App;
use wasm_logger::Config;

fn main() {
    wasm_logger::init(Config::default());
    yew::Renderer::<App>::new().render();
}
