use std::time::Duration;
use wasm_bindgen_test::*;
use yew::platform::time::sleep;
use yew::prelude::*;
use yew_query::use_query;

mod common;

use common::obtain_result;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

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

// tbh this tests .data()
// TODO: directly check the status with if let QueryStatus::Success
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
    // TODO: test impl functions (data/error) with refetching
}

#[wasm_bindgen_test]
async fn is_xxx_while_refetching() {
    // TODO: test impl functions (is_xxx) with refetching

    // {
    //     // this is just here to test `use_query`
    //     use std::time::Duration;
    //     use yew::platform::time::sleep;
    //     sleep(Duration::from_secs(1)).await;
    // }

    // // this is just here to test `use_query`
    // let initial_loading = whoami_query.is_initial_loading();
    // let fetching = whoami_query.is_fetching();

    // <br />
    // {"initial_loading = "}{initial_loading}
    // <br />
    // {"fetching = "}{fetching}
}
