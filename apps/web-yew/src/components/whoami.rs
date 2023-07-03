use gloo_console::log;
use yew::prelude::*;

use crate::api::{auth::whoami, NetResult, RequestStatus};
use crate::hooks::{use_query, QueryRefetcher};

impl From<NetResult<String>> for RequestStatus<String, gloo_net::Error> {
    fn from(result: NetResult<String>) -> Self {
        match result {
            Ok(me) => Self::Success(me),
            Err(err) => Self::Error(err),
        }
    }
}

#[function_component(WhoAmI)]
pub fn who_am_i() -> Html {
    let whoami_query = use_query((), |_| async move {
        {
            // this is just here to test `use_query`
            use std::time::Duration;
            use yew::platform::time::sleep;
            sleep(Duration::from_secs(1)).await;
        }

        let me = whoami().await;
        log!(format!("i am: {me:?}"));
        me
    });

    let handle_refetch = {
        let whoami_query = whoami_query.clone();

        Callback::from(move |_| {
            whoami_query.refetch(());
        })
    };

    let me = whoami_query
        .data()
        .map(|me| format!("\"{me}\""))
        .unwrap_or_default();

    // this is just here to test `use_query`
    let initial_loading = whoami_query.is_initial_loading();
    let fetching = whoami_query.is_fetching();

    html! {
        <>
            <pre>
                { "me = " }
                if whoami_query.is_fetching() {
                    <img class="h-4 w-4 animate-spin mr-2 inline" src="assets/loader-2.svg" alt="loading" />
                }
                <code>{ me }</code>
                <br />
                {"initial_loading = "}{initial_loading}
                <br />
                {"fetching = "}{fetching}
            </pre>

            <button onclick={handle_refetch} class="button-destructive">
                { "Refetch" }
            </button>
        </>
    }
}
