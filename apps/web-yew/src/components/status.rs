use yew::prelude::*;

use crate::ShortenStatus;

#[derive(PartialEq, Properties)]
pub struct StatusDisplayProps {
    pub status: ShortenStatus,
}

#[function_component]
pub fn StatusDisplay(props: &StatusDisplayProps) -> Html {
    let StatusDisplayProps { status } = props;

    let content = move || -> Html {
        match status {
            ShortenStatus::Loading => html! {
                { "Loading..." }
            },
            ShortenStatus::Success(result) => html! {
                <>
                    <a href={ result.remote_url.clone() } class="underline">
                        { &result.local_url }
                    </a>
                    <br />
                    { &result.remote_url }
                </>
            },
            ShortenStatus::Error(err) => html! {
                { err }
            },
            _ => html!(),
        }
    };

    html! {
        <p>
            { content() }
        </p>
    }
}
