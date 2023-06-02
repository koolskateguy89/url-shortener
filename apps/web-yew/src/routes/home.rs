use gloo_console::log;
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;

use crate::api::{shorten, ApiShortenResponse};
use crate::components::StatusDisplay;

#[derive(Debug, Clone)]
pub enum ShortenStatus {
    Idle,
    Loading,
    Success(ApiShortenResponseAttr),
    Error(AttrValue),
}

impl From<Result<ApiShortenResponse, gloo_net::Error>> for ShortenStatus {
    fn from(result: Result<ApiShortenResponse, gloo_net::Error>) -> ShortenStatus {
        match result {
            Ok(result) => ShortenStatus::Success(result.into()),
            Err(err) => ShortenStatus::Error(format!("{err:?}").into()),
        }
    }
}

/// Variant of ApiShortenResponse with AttrValue instead of String
#[derive(Debug, Clone)]
pub struct ApiShortenResponseAttr {
    pub local_url: AttrValue,
    pub remote_url: AttrValue,
    pub _id: AttrValue,
}

impl From<ApiShortenResponse> for ApiShortenResponseAttr {
    fn from(value: ApiShortenResponse) -> ApiShortenResponseAttr {
        Self {
            local_url: value.local_url.into(),
            remote_url: value.remote_url.into(),
            _id: value.id.into(),
        }
    }
}

#[function_component(HomePage)]
pub fn home() -> Html {
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

    // TODO: use local_url as href once routing redirect implemented
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
