use log::{debug, error};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;
use yew_query::{use_mutation, MutationDispatcher};
use yew_router::prelude::*;

use crate::api::auth::{login, logout};
use crate::components::WhoAmI;
use crate::routes::Route;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let login_mut = use_mutation(move |(username, password): (String, String)| async move {
        login(username, password).await
    });

    let onsubmit = {
        let login_mut = login_mut.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            // getting formdata from submit event: https://github.com/yewstack/yew/issues/474#issuecomment-1445382035
            let form: HtmlFormElement = event
                .target_dyn_into()
                .expect_throw("Event target is not a form");

            let form_data =
                FormData::new_with_form(&form).expect_throw("Form data could not be instantiated");

            let username = form_data
                .get("username")
                .as_string()
                .expect_throw("Could not get username from form");
            let password = form_data
                .get("password")
                .as_string()
                .expect_throw("Could not get password from form");

            login_mut.mutate((username, password));
        })
    };

    let handle_logout = Callback::from(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            // TODO: handle properly (alert?)
            match logout().await {
                Ok(logout_successful) => {
                    // debug
                    debug!("logout_successful = {logout_successful}");
                }
                Err(err) => {
                    error!("err = {err:?}");
                }
            }
        });
    });

    let loading = login_mut.is_loading();

    html! {
      <main class="flex h-screen flex-col items-center justify-center">
        <div class="mb-12 flex flex-col gap-y-4">
          <WhoAmI />

          <button onclick={handle_logout} class="button-destructive">
            { "LOG out" }
          </button>
        </div>

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
            <button
              type="submit"
              class="button"
              disabled={loading}
            >
              if loading {
                <img class="h-4 w-4 animate-spin mr-2 inline" src="assets/loader-2.svg" alt="loading" />
              }
              { "Login" }
            </button>
            <Link<Route> to={Route::Register} classes="button-link">
              { "Register" }
            </Link<Route>>
          </div>
        </form>
      </main>
    }
}
