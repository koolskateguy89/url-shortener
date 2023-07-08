mod api;
mod app;
mod components;
mod hooks;
mod routes;

use app::App;

// TODO: switch to `wasm-logger` instead of `gloo-console`
// https://yew.rs/docs/more/debugging#wasm-logger

fn main() {
    yew::Renderer::<App>::new().render();
}
