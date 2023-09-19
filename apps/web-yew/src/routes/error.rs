use serde::{Deserialize, Serialize};
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ErrorCause {
    NotFound,
    Other(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchParams {
    id: String,
    cause: Option<ErrorCause>,
}

impl SearchParams {
    pub fn new(id: String, cause: Option<ErrorCause>) -> Self {
        Self { id, cause }
    }
}

#[function_component(ErrorPage)]
pub fn error_page() -> Html {
    let location = use_location().expect_throw("location isn't set ???");
    let search_params: SearchParams = location.query().expect_throw("query couldn't be parsed");

    html! {
        <main>
            <h1>{ "Error" }</h1>
            <pre>
                { format!("ID = {}", search_params.id) }
            </pre>
            <pre>
                { format!("search_params = {search_params:#?}") }
            </pre>
        </main>
    }
}
