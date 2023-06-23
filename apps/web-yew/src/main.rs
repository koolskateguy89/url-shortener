use yew::prelude::*;
use yew_router::prelude::*;

mod api;
mod components;
mod hooks;
mod routes;

use routes::{switch, Route};

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
