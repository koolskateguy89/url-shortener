use common::types::StatsResponse;
use log::{debug, error};
use yew::prelude::*;

use crate::api;
use crate::hooks::use_error_redirector;

#[derive(PartialEq, Properties)]
pub struct StatsPageProps {
    pub id: AttrValue,
}

enum Status {
    Loading,
    Success(StatsResponse),
}

#[function_component(StatsPage)]
pub fn stats_page(props: &StatsPageProps) -> Html {
    let StatsPageProps { id } = props;

    let error_redirector = use_error_redirector().unwrap();

    let status = use_state(|| Status::Loading);

    {
        let id = id.clone();
        let status = status.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    debug!("id = {id}");

                    match api::url::get_stats(&id).await {
                        Ok(stats) => {
                            debug!("stats = {stats:?}");

                            status.set(Status::Success(stats));
                        }
                        Err(err) => {
                            error!("(stats) err = {err:?}");
                            let _ = error_redirector.redirect(id.to_string(), err.into());
                        }
                    }
                });
            },
            (),
        );
    }

    // don't need to actually handle all cases here because we're redirecting on error
    // and idle is impossible
    let content = move || -> Html {
        match *status {
            Status::Success(StatsResponse {
                ref url,
                ref username,
                hits: _,
                num_hits,
            }) => html! {
                <>
                    <p>{ "URL: " }{url}</p>
                    <p>{ "Num hits: " }{num_hits}</p>
                    <p>{ format!("Username: {}", username.as_deref().unwrap_or("null")) }</p>
                </>
            },
            _ => html!(<p>{ "Loading..." }</p>),
        }
    };

    html! {
        <main>
            <h1>{ "Stats" }</h1>
            { content() }
        </main>
    }
}
