mod app;
mod search_bar;
mod search_results;
mod song_search;
mod spotify;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
