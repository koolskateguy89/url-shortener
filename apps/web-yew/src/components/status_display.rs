use yew::prelude::*;

use crate::routes::home::ShortenStatus;

impl PartialEq for ShortenStatus {
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

#[derive(PartialEq, Properties)]
pub struct StatusDisplayProps {
    pub status: ShortenStatus,
}

#[function_component(StatusDisplay)]
pub fn status_display(props: &StatusDisplayProps) -> Html {
    let StatusDisplayProps { status } = props;

    let content = move || -> Html {
        match status {
            ShortenStatus::Loading => html!("Loading..."),
            ShortenStatus::Success(result) => html! {
                <>
                    <a href={ result.remote_url.clone() } class="underline">
                        { &result.local_url }
                    </a>
                    <br />
                    { &result.remote_url }
                </>
            },
            ShortenStatus::Error(err) => html!(err),
            _ => html!(),
        }
    };

    html! {
        <p>
            { content() }
        </p>
    }
}
