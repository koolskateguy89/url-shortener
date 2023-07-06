use wasm_bindgen::UnwrapThrowExt;
use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;
use yew_query::{use_mutation, MutationDispatcher};
use yew_router::prelude::*;

use crate::api::auth::register;
use crate::routes::Route;

#[function_component(RegisterPage)]
pub fn register_page() -> Html {
    let navigator = use_navigator().expect("cannot access navigator");

    let register_mut = use_mutation(move |(username, password): (String, String)| async move {
        register(username, password).await
    });

    // on success, redirect to login page
    if register_mut.is_success() {
        navigator.push(&Route::Login);
    }

    let onsubmit = {
        let register_mut = register_mut.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            // getting formdata from submit event: https://github.com/yewstack/yew/issues/474#issuecomment-1445382035
            let form: HtmlFormElement = event
                .target_dyn_into()
                .expect_throw("Event target is not a form");

            let form_data =
                FormData::new_with_form(&form).expect_throw("Form data could not be instantiated");

            // TODO: min pw length - maybe just handle it on server
            let username = form_data
                .get("username")
                .as_string()
                .expect_throw("Could not get username from form");
            let password = form_data
                .get("password")
                .as_string()
                .expect_throw("Could not get password from form");

            register_mut.mutate((username, password));
        })
    };

    let loading = register_mut.is_loading();

    html! {
      <main class="flex h-screen flex-col items-center justify-center">
        <form {onsubmit} class="flex flex-col items-center gap-y-2">
          <input
            name="username"
            placeholder="Username"
            autocomplete="username"
            class="input"
            required=true
            disabled={loading}
          />

          <input
            type="password"
            name="password"
            placeholder="Password"
            autocomplete="current-password"
            class="input"
            required=true
            disabled={loading}
          />

          <div>
            <Link<Route> to={Route::Login} classes="button-link">
              { "Login" }
            </Link<Route>>
            <button
              type="submit"
              class="button"
              disabled={loading}
            >
              if loading {
                <img class="h-4 w-4 animate-spin mr-2 inline" src="assets/loader-2.svg" alt="loading" />
              }
              { "Register" }
            </button>
          </div>
        </form>
      </main>
    }
}
