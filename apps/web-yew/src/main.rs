mod api;
mod app;
mod components;
mod hooks;
mod routes;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
