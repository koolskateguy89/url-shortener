use std::time::Duration;
use wasm_bindgen_test::*;
use yew::platform::time::sleep;
use yew::prelude::*;
use yew_query::{use_mutation, MutationDispatcher};

mod common;

use common::obtain_result;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn use_mutation_works() {
    static MESSAGE: &str = "hi";

    #[function_component(UseComponent)]
    fn use_mutation_comp() -> Html {
        let mutation = use_mutation(|message: &str| async move { Ok::<_, ()>(message) });

        {
            let mutation = mutation.clone();

            use_effect_with_deps(
                move |_| {
                    mutation.mutate(MESSAGE);
                },
                (),
            );
        }

        html! {
            <div>
                {"Test Output: "}
                <div id="result">
                    if let Some(message) = mutation.data() {
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

// TODO: more tests
