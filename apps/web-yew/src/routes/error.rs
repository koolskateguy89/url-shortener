use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_router::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchParams {
    id: String,
    #[allow(dead_code)]
    cause: Option<String>,
}

impl SearchParams {
    pub fn new(id: String, cause: Option<String>) -> Self {
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
