use gloo_console::log;
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;

mod api;

use api::{shorten, ApiShortenResponse};

enum ShortenResult {
    Idle,
    Loading,
    Success(ApiShortenResponse),
    Error(gloo_net::Error),
}

impl Into<ShortenResult> for Result<ApiShortenResponse, gloo_net::Error> {
    fn into(self) -> ShortenResult {
        match self {
            Ok(result) => ShortenResult::Success(result),
            Err(err) => ShortenResult::Error(err),
        }
    }
}

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

    let shorten_result = use_state(|| ShortenResult::Idle);
    let onsubmit = {
        let shorten_result = shorten_result.clone();

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
            wasm_bindgen_futures::spawn_local(async move {
                shorten_result.set(ShortenResult::Loading);

                let result = shorten(url).await;
                log!("result =", format!("{result:?}"));

                shorten_result.set(result.into());
            });
        })
    };

    // TODO: use local_url as href once routing redirect implemented
    html! {
        <main class="flex h-screen flex-col items-center justify-center space-y-4">
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
            <p>
                { match &*shorten_result {
                    ShortenResult::Loading => html! {
                        { "Loading..." }
                    },
                    ShortenResult::Success(result) => html! {
                        <>
                            <a href={ result.remote_url.clone() } class="underline">
                                { &result.local_url }
                            </a>
                            <br />
                            { &result.remote_url }
                        </>
                    },
                    ShortenResult::Error(err) => html! {
                        { err.to_string() }
                    },
                    _ => html!(),
                } }
            </p>
            <form {onsubmit} class="flex flex-col items-center space-y-2">
                <input type="text" name="url" class="input" />
                <button
                    type="submit"
                    class="button"
                >
                    { "Shorten" }
                </button>
            </form>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
