use std::time::Duration;

use wasm_bindgen_test::*;
use yew::platform::time::sleep;
use yew::prelude::*;

use url_shortener_web_yew::hooks::{use_query, QueryDispatcher};

mod common;

use crate::common::obtain_result;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

// TODO?: bascially proxy request responses, if can - that's more for testing /api

#[wasm_bindgen_test]
async fn use_query_works() {
    static MESSAGE: &str = "hi";

    #[function_component(UseComponent)]
    fn use_query_comp() -> Html {
        let query = use_query(MESSAGE, |message| async move { Ok::<_, ()>(message) });

        html! {
            <div>
                {"Test Output: "}
                <div id="result">
                    if let Some(message) = query.data() {
                        { message }
                    }
                </div>
                {"\n"}
            </div>
        }
    }

    yew::Renderer::<UseComponent>::with_root(
        gloo::utils::document().get_element_by_id("output").unwrap(),
    )
    .render();
    sleep(Duration::ZERO).await;

    let result = obtain_result();
    assert_eq!(result.as_str(), MESSAGE);
}

#[wasm_bindgen_test]
async fn use_query_with_delay() {
    static DELAY: Duration = Duration::from_millis(100);
    static MESSAGE: &str = "delayed";

    #[function_component(UseComponent)]
    fn use_query_comp() -> Html {
        let query = use_query(MESSAGE, |message| async move {
            sleep(DELAY).await;
            Ok::<_, ()>(message)
        });

        html! {
            <div>
                {"Test Output: "}
                <div id="result">
                    if let Some(message) = query.data() {
                        { message }
                    } else {
                        { "awaiting delay" }
                    }
                </div>
                {"\n"}
            </div>
        }
    }

    yew::Renderer::<UseComponent>::with_root(
        gloo::utils::document().get_element_by_id("output").unwrap(),
    )
    .render();
    sleep(Duration::ZERO).await;

    let result = obtain_result();
    assert_eq!(result.as_str(), "awaiting delay");

    // add some padding to ensure the delay has passed
    sleep(DELAY + Duration::from_millis(50)).await;

    let delayed_result = obtain_result();
    assert_eq!(delayed_result.as_str(), MESSAGE);
}

#[wasm_bindgen_test]
async fn has_old_data_while_refetching() {
    // TODO
}
