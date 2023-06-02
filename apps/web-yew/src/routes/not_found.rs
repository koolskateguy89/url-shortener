use yew::prelude::*;

#[function_component(NotFoundPage)]
pub fn not_found() -> Html {
    html! {
      <main>
        { "404" }
      </main>
    }
}
