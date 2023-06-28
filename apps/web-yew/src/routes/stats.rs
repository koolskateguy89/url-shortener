use gloo_console::{error, log};
use yew::prelude::*;

use crate::api;
use crate::hooks::use_error_redirector;
use common::types::StatsResponse;

#[derive(PartialEq, Properties)]
pub struct StatsPageProps {
    pub id: AttrValue,
}

enum Status {
    Loading,
    Success(StatsResponse),
}

#[function_component]
pub fn StatsPage(props: &StatsPageProps) -> Html {
    let StatsPageProps { id } = props;

    let error_redirector = use_error_redirector().unwrap();

    let status = use_state(|| Status::Loading);

    {
        let id = id.clone();
        let status = status.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    log!(format!("id = {id:?}"));

                    match api::url::get_stats(&id).await {
                        Ok(stats) => {
                            log!(format!("stats = {stats:?}"));

                            status.set(Status::Success(stats));
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

    let content = move || -> Html {
        match *status {
            Status::Loading => html!(<p>{ "Loading..." }</p>),
            Status::Success(StatsResponse {
                ref url,
                hits: _,
                num_hits,
            }) => html! {
                <>
                    <p>{ "URL: " }{url}</p>
                    <p>{ "Num hits: " }{num_hits}</p>
                </>
            },
        }
    };

    html! {
        <main>
            <h1>{ "Stats" }</h1>
            { content() }
        </main>
    }
}
