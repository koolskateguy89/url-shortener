use gloo_console::{error, log};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::window;
use yew::prelude::*;

use crate::api;
use common::types::LengthenResponse;

#[derive(PartialEq, Properties)]
pub struct RedirectPageProps {
    pub id: AttrValue,
}

/// Loading icon: https://lucide.dev/icons/loader-2
#[function_component]
pub fn RedirectPage(props: &RedirectPageProps) -> Html {
    let RedirectPageProps { id } = props;

    let is_error = use_state(|| false);

    {
        let id = id.clone();
        let is_error = is_error.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match api::lengthen(id.to_string()).await {
                        Ok(LengthenResponse { url }) => {
                            log!(format!("url = {url:?}"));

                            let window = window().expect_throw("window is undefined");
                            let location = window.location();

                            location.set_href(&url).expect_throw("Could not redirect");
                        }
                        Err(err) => {
                            error!(format!("err = {err:?}"));
                            is_error.set(true);
                        }
                    }
                });
            },
            (),
        );
    }

    html! {
      <main class="h-[100dvh] flex flex-col items-center justify-center">
        if *is_error {
            <p class="bg-red-600 text-white p-8">
                { "Error redirecting" }
            </p>
        } else {
            { "Redirecting for " }
            { id }
            <svg class="h-4 w-4 animate-spin" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-loader-2"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>
        }
      </main>
    }
}
