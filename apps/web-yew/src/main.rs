use gloo_console::log;
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;

mod api;
mod components;

use api::{shorten, ApiShortenResponse};
use components::StatusDisplay;

#[derive(Debug, Clone)]
pub enum ShortenStatus {
    Idle,
    Loading,
    Success(ApiShortenResponseAttr),
    Error(AttrValue),
}

impl PartialEq for ShortenStatus {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Idle, Self::Idle) => true,
            (Self::Loading, Self::Loading) => true,
            (Self::Success(_), Self::Success(_)) => true,
            (Self::Error(_), Self::Error(_)) => true,
            _ => false,
        }
    }
}

impl Into<ShortenStatus> for Result<ApiShortenResponse, gloo_net::Error> {
    fn into(self) -> ShortenStatus {
        match self {
            Ok(result) => ShortenStatus::Success(result.into()),
            Err(err) => ShortenStatus::Error(format!("{err:?}").into()),
        }
    }
}

/// Variant of ApiShortenResponse with AttrValue instead of String
#[derive(Debug, Clone)]
pub struct ApiShortenResponseAttr {
    local_url: AttrValue,
    remote_url: AttrValue,
    _id: AttrValue,
}

impl Into<ApiShortenResponseAttr> for ApiShortenResponse {
    fn into(self) -> ApiShortenResponseAttr {
        ApiShortenResponseAttr {
            local_url: self.local_url.into(),
            remote_url: self.remote_url.into(),
            _id: self.id.into(),
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
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>

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

fn main() {
    yew::Renderer::<App>::new().render();
}
