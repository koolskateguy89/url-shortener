use yew::prelude::*;
use yew_router::prelude::*;

pub mod home;
pub mod not_found;
pub mod redirect;
pub mod stats;

use home::HomePage;
use not_found::NotFoundPage;
use redirect::RedirectPage;
use stats::StatsPage;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    // TODO: probably gonna want a separate router for `:id`
    #[at("/:id")]
    Redirect { id: String },
    #[at("/:id/stats")]
    Stats { id: String },
    #[at("/404")]
    #[not_found]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::Redirect { id } => html! { <RedirectPage {id} /> },
        Route::Stats { id } => html! { <StatsPage {id} /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}
