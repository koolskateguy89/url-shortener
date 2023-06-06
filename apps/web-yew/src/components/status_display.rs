use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::home::ShortenStatus;
use crate::routes::Route;
use common::error::Error;

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

#[function_component]
pub fn StatusDisplay(props: &StatusDisplayProps) -> Html {
    let StatusDisplayProps { status } = props;

    let content = move || -> Html {
        match status {
            ShortenStatus::Loading => html!("Loading..."),
            ShortenStatus::Success(id) => html! {
                <Link<Route> to={Route::Redirect { id: id.to_string() }}>
                    <span class="underline">
                        { format!("BASE_URL/{}", id) }
                    </span>
                </Link<Route>>
            },
            ShortenStatus::Error(Error::Other(err)) => html!(err),
            ShortenStatus::Error(err) => html!(format!("{err:?}")),
            _ => html!(),
        }
    };

    html! {
        <p>
            { content() }
        </p>
    }
}
