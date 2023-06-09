use yew::prelude::*;
use yew_router::prelude::*;

pub mod home;
pub mod not_found;
pub mod redirect;

use home::HomePage;
use not_found::NotFoundPage;
use redirect::RedirectPage;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    // TODO: gonna need a separate router for ID
    #[at("/:id")]
    Redirect { id: String },
    #[at("/404")]
    #[not_found]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::Redirect { id } => html! { <RedirectPage {id} /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}
