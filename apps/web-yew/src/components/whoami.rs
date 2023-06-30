use yew::prelude::*;

use crate::api::{auth::whoami, NetResult, RequestStatus};
use crate::hooks::{use_query, QueryDispatcher, QueryStatus};

type WhoamiStatus = RequestStatus<String, gloo_net::Error>;

impl From<NetResult<String>> for WhoamiStatus {
    fn from(result: NetResult<String>) -> WhoamiStatus {
        match result {
            Ok(me) => WhoamiStatus::Success(me),
            Err(err) => WhoamiStatus::Error(err),
        }
    }
}

#[function_component]
pub fn Whoami() -> Html {
    let whoami_query = use_query(whoami);

    {
        // fetch whoami on mount
        let whoami_query = whoami_query.clone();

        use_effect_with_deps(
            move |_| {
                whoami_query.fetch();
            },
            (),
        );
    }

    let handle_refetch = {
        let whoami_query = whoami_query.clone();

        Callback::from(move |_| {
            whoami_query.fetch();
        })
    };

    let loading = matches!(whoami_query.status, WhoamiStatus::Loading);

    let me = if let QueryStatus::Success(me) = &whoami_query.status {
        format!("\"{me}\"")
    } else {
        "".to_string()
    };

    html! {
        <div>
            <pre>
                { "me = " }
                if loading {
                    <img class="h-4 w-4 animate-spin mr-2 inline" src="assets/loader-2.svg" alt="loading" />
                }
                <code>{ me }</code>
                <img class="h-4 w-4 animate-spin mr-2" src="assets/yew.svg" alt="yew logo hopefully (nope)" />
            </pre>

            <button onclick={handle_refetch} class="button-destructive">
                { "Refetch" }
            </button>
        </div>
    }
}
