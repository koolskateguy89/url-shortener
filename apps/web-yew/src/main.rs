use gloo_console::log;
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;

mod api;

use api::{shorten, ApiShortenResponse};

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| {
            let value = *counter + 1;
            counter.set(value);
        })
    };

    // TODO: error handle, will likely need to use Option<Result<Reponse, ...>>
    let shorten_result = use_state(|| None::<ApiShortenResponse>);
    let loading = use_state(|| false);

    let onsubmit = {
        let shorten_result = shorten_result.clone();
        let loading = loading.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            // getting formdata from submit event: https://github.com/yewstack/yew/issues/474#issuecomment-1445382035
            let target = event.target().expect("Event has no target");
            let form: HtmlFormElement = target.dyn_into().expect("Event target is not a form");

            let form_data =
                FormData::new_with_form(&form).expect("Form data could not be instantiated");
            let url = form_data
                .get("url")
                .as_string()
                .expect("Could not get url from form");

            log!("url =", &url);
            // TODO: check url is a valid URL before req

            let shorten_result = shorten_result.clone();
            let loading = loading.clone();
            wasm_bindgen_futures::spawn_local(async move {
                shorten_result.set(None);
                loading.set(true);

                let result = shorten(url).await;
                log!("result =", format!("{result:?}"));

                shorten_result.set(result.ok());
                loading.set(false);
            });
        })
    };

    // TODO: use local_url as href once routing redirect implemented
    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
            <p>
                { if *loading { "Loading..." } else { "Not loading" } }
            </p>
            <p>
                { if let Some(result) = &*shorten_result {
                    html! {
                        <>
                            <a href={ result.remote_url.clone() }>
                                { &result.local_url }
                            </a>
                            <br />
                            { &result.remote_url }
                        </>
                    }
                } else {
                    html!()
                } }
            </p>
            <form {onsubmit}>
                <input type="text" name="url" />
                <button type="submit">{ "Shorten" }</button>
            </form>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
