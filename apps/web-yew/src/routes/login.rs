use gloo_console::log;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;

use crate::api::login;
use common::error::Error;

// TODO: extract fetch status enum to a common module (in web-yew),
// with generic types for success and error
#[derive(Debug, Clone)]
pub enum LoginStatus {
    Idle,
    Loading,
    Success,
    // TODO: will probably change inner to login error type
    Error(Error),
}

// TODO: probably change error type to login error type
impl From<Result<(), Error>> for LoginStatus {
    fn from(result: Result<(), Error>) -> LoginStatus {
        match result {
            Ok(_) => LoginStatus::Success,
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
            // TODO
            log!("submit");
        })
    };

    let loading = matches!(*status, LoginStatus::Loading);
    let loading = true;

    let handle_logout = {
        Callback::from(move |_| {
            // TODO
            log!("logout");
        })
    };

    html! {
      <main class="flex h-screen flex-col items-center justify-center space-y-4">
        <div class="mb-20 flex flex-col gap-y-4">
          <pre>
            { "me = " }
            <code>
              if loading {
                <svg class="h-4 w-4 animate-spin mr-2 inline" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-loader-2"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>
              }
              { "TODO: fetch whoami" }
            </code>
          </pre>
          <button onclick={handle_logout} class="button-destructive">
            { "LOG out" }
          </button>
        </div>

        <form {onsubmit} class="flex flex-col items-center space-y-2">
          <input
            name="username"
            placeholder="Username"
            auto-complete="username"
            class="input"
            required=true
            disabled={loading}
          />
          <input
            type="password"
            name="password"
            placeholder="Password"
            auto-complete="current-password"
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
              <svg class="h-4 w-4 animate-spin mr-2 inline" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-loader-2"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>
            }
            { "Login" }
          </button>
        </form>
      </main>
    }
}
