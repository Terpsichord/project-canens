#![warn(clippy::unwrap_used)]

mod app;

mod backend;
mod error;
mod external_button;
mod fretmap;
mod home;
mod hooktheory;
mod navbar;
mod search;
mod song;
#[cfg(debug_secrets)]
mod spotify;
mod tabs;

use app::App;
use wasm_logger::Config;

fn main() {
    wasm_logger::init(Config::default());
    yew::Renderer::<App>::new().render();
}
