use yew::prelude::*;
use yew_router::prelude::*;

mod api;
mod components;
mod hooks;
mod routes;

use routes::{switch, Route};

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
