use gloo_console::{error, log};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::window;
use yew::prelude::*;

use crate::api;
use crate::hooks::use_error_redirector;
use common::types::LengthenResponse;

#[derive(PartialEq, Properties)]
pub struct RedirectPageProps {
    pub id: AttrValue,
}

/// Loading icon: https://lucide.dev/icons/loader-2
#[function_component]
pub fn RedirectPage(props: &RedirectPageProps) -> Html {
    let RedirectPageProps { id } = props;

    let error_redirector = use_error_redirector().unwrap();

    {
        let id = id.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match api::url::lengthen(&id).await {
                        Ok(LengthenResponse { url }) => {
                            log!(format!("url = {url:?}"));

                            let window = window().expect_throw("window is undefined");
                            let location = window.location();

                            location.set_href(&url).expect_throw("Could not redirect");
                        }
                        Err(err) => {
                            error!(format!("err = {err:?}"));
                            let _ = error_redirector.redirect(id.to_string(), format!("{err:?}"));
                        }
                    }
                });
            },
            (),
        );
    }

    html! {
        <main class="h-[100dvh] flex flex-col items-center justify-center">
            { "Redirecting for " }
            { id }
            <svg class="h-4 w-4 animate-spin" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-loader-2"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>
        </main>
    }
}
