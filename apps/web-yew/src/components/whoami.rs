use gloo_console::log;
use yew::prelude::*;
use yew_query::{use_query, QueryRefetcher};

use crate::api::{auth::whoami, NetResult, RequestStatus};

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

    html! {
        <>
            <pre>
                { "me = " }
                if whoami_query.is_fetching() {
                    <img class="h-4 w-4 animate-spin mr-2 inline" src="assets/loader-2.svg" alt="loading" />
                }
                <code>{ me }</code>
            </pre>

            <button onclick={handle_refetch} class="button-destructive">
                { "Refetch" }
            </button>
        </>
    }
}
