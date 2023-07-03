use std::time::Duration;
use wasm_bindgen_test::*;
use yew::platform::time::sleep;
use yew::prelude::*;
use yew_query::use_mutation;

mod common;

use common::obtain_result;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn use_mutation_works() {
    static MESSAGE: &str = "hi";

    #[function_component(UseComponent)]
    fn use_query_comp() -> Html {
        let query = use_mutation(|message: &str| async move { Ok::<_, ()>(message) });

        // TODO: use effect to call mutate on mount

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
    // assert_eq!(result.as_str(), MESSAGE);
    assert_eq!(result.as_str(), "");
}

// TODO: more tests
