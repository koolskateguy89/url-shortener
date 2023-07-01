use gloo_console::log;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;

use crate::api::{url::shorten, RequestStatus};
use crate::components::StatusDisplay;
use common::{error::Error, types::ShortenResponse};

pub type ShortenStatus = RequestStatus<AttrValue, Error>;

impl From<Result<ShortenResponse, Error>> for ShortenStatus {
    fn from(result: Result<ShortenResponse, Error>) -> ShortenStatus {
        match result {
            Ok(result) => ShortenStatus::Success(result.id.into()),
            Err(err) => ShortenStatus::Error(err),
        }
    }
}

#[function_component(HomePage)]
pub fn home_page() -> Html {
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

            // URL validation is done server side

            let status = status.clone();
            wasm_bindgen_futures::spawn_local(async move {
                status.set(ShortenStatus::Loading);

                let result = shorten(url).await;
                log!("result =", format!("{result:?}"));

                status.set(result.into());
            });
        })
    };

    let disabled = matches!(*status, ShortenStatus::Loading);

    let status = (*status).clone();

    html! {
        <main class="flex h-screen flex-col items-center justify-center space-y-4">
            <StatusDisplay {status} />

            <form {onsubmit} class="flex flex-col items-center space-y-2">
                <input
                    type="url"
                    name="url"
                    placeholder="Url"
                    class="input"
                    required=true
                    {disabled}
                    />
                <button
                    type="submit"
                    class="button"
                    {disabled}
                >
                    { "Shorten" }
                </button>
            </form>
        </main>
    }
}
