use common::types::LengthenResponse;
use gloo_utils::window;
use log::{debug, error};
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

use crate::api;
use crate::hooks::use_error_redirector;

#[derive(PartialEq, Properties)]
pub struct RedirectPageProps {
    pub id: AttrValue,
}

#[function_component(RedirectPage)]
pub fn redirect_page(props: &RedirectPageProps) -> Html {
    let RedirectPageProps { id } = props;

    let error_redirector = use_error_redirector().unwrap();

    {
        let id = id.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match api::url::lengthen(&id).await {
                        Ok(LengthenResponse { url }) => {
                            debug!("url = {url}");

                            window()
                                .location()
                                .set_href(&url)
                                .expect_throw("Could not redirect");
                        }
                        Err(err) => {
                            error!("(redirect) err = {err:?}");
                            let _ = error_redirector.redirect(id.to_string(), err.into());
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
            <img class="h-4 w-4 animate-spin" src="assets/loader-2.svg" alt="loading" />
        </main>
    }
}
