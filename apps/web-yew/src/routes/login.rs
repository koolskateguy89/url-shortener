use gloo_console::log;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;

use crate::api::{
    auth::{login, logout},
    NetResult, RequestStatus,
};
use crate::components::Whoami;

type LoginStatus = RequestStatus<bool, gloo_net::Error>;

impl From<NetResult<bool>> for LoginStatus {
    fn from(result: NetResult<bool>) -> LoginStatus {
        match result {
            Ok(logged_in) => LoginStatus::Success(logged_in),
            Err(err) => LoginStatus::Error(err),
        }
    }
}

#[function_component]
pub fn LoginPage() -> Html {
    let status = use_state(|| LoginStatus::Idle);
    let onsubmit = {
        let status = status.clone();

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

            let status = status.clone();
            wasm_bindgen_futures::spawn_local(async move {
                status.set(LoginStatus::Loading);

                log!(format!("username = {username}"));
                log!(format!("password = {password}"));

                let login_result = login(username, password).await;
                log!(format!("login_result = {login_result:?}"));

                status.set(login_result.into());
            });
        })
    };

    let loading = matches!(*status, LoginStatus::Loading);

    let handle_logout = {
        Callback::from(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                // TODO: handle properly
                match logout().await {
                    Ok(logout_successful) => log!("logout_successful =", logout_successful),
                    Err(err) => log!(format!("err = {err:?}")),
                }
            });
        })
    };

    html! {
      <main class="flex h-screen flex-col items-center justify-center space-y-4">
        <div class="mb-12 flex flex-col gap-y-4">
          <Whoami />
          <button onclick={handle_logout} class="button-destructive">
            { "LOG out" }
          </button>
        </div>

        <form {onsubmit} class="flex flex-col items-center space-y-2">
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
        </form>
      </main>
    }
}
