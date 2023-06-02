use yew::prelude::*;

#[function_component]
pub fn NotFoundPage() -> Html {
    html! {
      <main class="h-screen flex items-center justify-center">
        { "404" }
      </main>
    }
}
