use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn NotFoundPage() -> Html {
    let location = use_location().expect_throw("location isn't set ???");
    let path = location.path();

    html! {
      <main class="h-screen flex flex-col items-center justify-center">
        <h1>
            { "404" }
        </h1>
        <p>
            { "path = " }
            { path }
        </p>
      </main>
    }
}
