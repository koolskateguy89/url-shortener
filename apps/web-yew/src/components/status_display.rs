use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::home::ShortenStatus;
use crate::routes::UrlRoute;
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

#[function_component(StatusDisplay)]
pub fn status_display(props: &StatusDisplayProps) -> Html {
    let StatusDisplayProps { status } = props;

    match status {
        ShortenStatus::Idle => html!(),
        ShortenStatus::Loading => html!(<p>{ "Loading..." }</p>),
        ShortenStatus::Success(id) => html! {
            <p>
                <Link<UrlRoute> classes="underline" to={UrlRoute::Redirect { id: id.to_string() }}>
                    { format!("BASE_URL/{}", id) }
                </Link<UrlRoute>>
            </p>
        },
        ShortenStatus::Error(Error::Other(err)) => html!(<p>{ err }</p>),
        ShortenStatus::Error(err) => html!(<p>{ format!("{err:?}") }</p>),
    }
}
