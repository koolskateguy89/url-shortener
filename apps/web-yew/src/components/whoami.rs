use yew::prelude::*;

use crate::api::{auth::whoami, NetResult, RequestStatus};
use crate::hooks::{use_query, QueryDispatcher};

impl From<NetResult<String>> for RequestStatus<String, gloo_net::Error> {
    fn from(result: NetResult<String>) -> Self {
        match result {
            Ok(me) => Self::Success(me),
            Err(err) => Self::Error(err),
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

    let loading = matches!(whoami_query.status, RequestStatus::Loading);

    let me = if let RequestStatus::Success(me) = &whoami_query.status {
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
            </pre>

            <button onclick={handle_refetch} class="button-destructive">
                { "Refetch" }
            </button>
        </div>
    }
}
