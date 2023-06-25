use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::error::SearchParams;
use crate::routes::Route;

pub struct ErrorRedirector {
    navigator: Navigator,
}

impl ErrorRedirector {
    pub fn redirect(
        &self,
        id: impl Into<String>,
        cause: impl Into<String>,
    ) -> NavigationResult<()> {
        let query = SearchParams::new(id.into(), Some(cause.into()));
        self.navigator.push_with_query(&Route::Error, &query)
    }
}

#[hook]
pub fn use_error_redirector() -> Option<ErrorRedirector> {
    let navigator = use_navigator()?;
    Some(ErrorRedirector { navigator })
}
