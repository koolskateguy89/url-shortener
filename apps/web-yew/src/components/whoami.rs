use gloo_console::log;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api::{auth::whoami, NetResult, RequestStatus};

type WhoamiStatus = RequestStatus<String, gloo_net::Error>;

impl From<NetResult<String>> for WhoamiStatus {
    fn from(result: NetResult<String>) -> WhoamiStatus {
        match result {
            Ok(me) => WhoamiStatus::Success(me),
            Err(err) => WhoamiStatus::Error(err),
        }
    }
}

impl PartialEq for WhoamiStatus {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Idle, Self::Idle)
                | (Self::Loading, Self::Loading)
                | (Self::Success(_), Self::Success(_))
                | (Self::Error(_), Self::Error(_))
        )
    }
}

// have a button to refetch? instead of manually refetching after
// login/logout
#[function_component]
pub fn Whoami() -> Html {
    let status = use_state(|| WhoamiStatus::Idle);

    let me_state = use_state(|| "idk".to_string());

    {
        use_effect_with_deps(
            move |_| {
                // TODO: instantly fetch who i am
                log!("useEffect first");
            },
            (),
        );
    }

    {
        let status = status.clone();
        let me_state = me_state.clone();
        use_effect_with_deps(
            move |(status,)| {
                log!(format!("useEffect on status change, st = {:?}", status));
                if let WhoamiStatus::Success(ref me) = **status {
                    me_state.set(me.clone());
                }
            },
            (status,),
        );
    }

    // TODO: display status

    // TODO: button to refetch

    let loading = matches!(*status, WhoamiStatus::Loading);

    let me = me_state.as_str();

    let handle_refetch = Callback::from(move |_| {
        // wasm_bindgen_futures::spawn_local(async move {
        //     // TODO: handle properly
        //     match logout().await {
        //         Ok(logout_successful) => log!("logout_successful =", logout_successful),
        //         Err(err) => log!(format!("err = {err:?}")),
        //     }
        // });
    });

    html! {
        <div>
            <pre>
                { "me = " }
                if loading {
                    <svg class="h-4 w-4 animate-spin" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-loader-2"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>
                }
                <code>{ me }</code>
                <img src="yew.svg" alt="yew logo hopefully" />
            </pre>

            <button onclick={handle_refetch} class="button-destructive">
                { "Refetch" }
            </button>
        </div>
    }
}
