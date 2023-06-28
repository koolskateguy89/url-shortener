use yew::prelude::*;
use yew_router::prelude::*;

pub mod error;
pub mod home;
pub mod login;
pub mod not_found;
pub mod redirect;
pub mod stats;

use error::ErrorPage;
use home::HomePage;
use login::LoginPage;
use not_found::NotFoundPage;
use redirect::RedirectPage;
use stats::StatsPage;

/// Main app router
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/:id")]
    UrlRoot,
    #[at("/:id/:path")]
    Url,
    #[at("/error")]
    Error,
    #[at("/login")]
    Login,
    #[at("/404")]
    #[not_found]
    NotFound,
}

/// Router for `/:id/*`
#[derive(Clone, Routable, PartialEq)]
pub enum UrlRoute {
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
        Route::UrlRoot | Route::Url => html! {
            <Switch<UrlRoute> render={switch_url} />
        },
        Route::Error => html! { <ErrorPage /> },
        Route::Login => html! { <LoginPage /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}

fn switch_url(route: UrlRoute) -> Html {
    match route {
        UrlRoute::Redirect { id } => html! { <RedirectPage {id} /> },
        UrlRoute::Stats { id } => html! { <StatsPage {id} /> },
        UrlRoute::NotFound => html! { <NotFoundPage /> },
    }
}
