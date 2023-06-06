use gloo_console::log;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;

use crate::api::shorten;
use crate::components::StatusDisplay;
use common::types::ShortenResponse;

#[derive(Debug, Clone)]
pub enum ShortenStatus {
    Idle,
    Loading,
    Success(AttrValue),
    Error(AttrValue),
}

impl From<Result<ShortenResponse, gloo_net::Error>> for ShortenStatus {
    fn from(result: Result<ShortenResponse, gloo_net::Error>) -> ShortenStatus {
        match result {
            Ok(result) => ShortenStatus::Success(result.id.into()),
            Err(err) => ShortenStatus::Error(format!("{err:?}").into()),
        }
    }
}

#[function_component]
pub fn HomePage() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| {
            let value = *counter + 1;
            counter.set(value);
        })
    };

    let status = use_state(|| ShortenStatus::Idle);
    let onsubmit = {
        let status = status.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            // getting formdata from submit event: https://github.com/yewstack/yew/issues/474#issuecomment-1445382035
            let form: HtmlFormElement = event
                .target_dyn_into()
                .expect_throw("Event target is not a form");

            let form_data =
                FormData::new_with_form(&form).expect_throw("Form data could not be instantiated");
            let url = form_data
                .get("url")
                .as_string()
                .expect_throw("Could not get url from form");

            log!("url =", &url);
            // TODO: check url is a valid URL before req

            let status = status.clone();
            wasm_bindgen_futures::spawn_local(async move {
                status.set(ShortenStatus::Loading);

                let result = shorten(url).await;
                log!("result =", format!("{result:?}"));

                status.set(result.into());
            });
        })
    };

    let status = (*status).clone();

    html! {
        <main class="flex h-screen flex-col items-center justify-center space-y-4">
            <button {onclick} class="button" >{ "+1" }</button>
            <p class="font-semibold bg-green-700 px-8 py-4 text-white">{ *counter }</p>

            <StatusDisplay {status} />

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
