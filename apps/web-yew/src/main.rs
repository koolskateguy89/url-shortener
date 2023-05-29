use yew::prelude::*;

use common::add;

// TODO: use gloo to fetch data from server
// https://yew.rs/docs/tutorial#fetching-data-using-external-rest-api

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
            <p>{ add(2, 3) }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
