use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::error::{ErrorCause, SearchParams};
use crate::routes::Route;
use common::error::Error as CommonError;

pub struct ErrorRedirector {
    navigator: Navigator,
}

impl From<CommonError> for ErrorCause {
    fn from(err: CommonError) -> Self {
        match err {
            CommonError::InvalidUrl => Self::Other("Invalid URL".to_string()),
            CommonError::NotFound => Self::NotFound,
            CommonError::Other(err) => Self::Other(err),
        }
    }
}

impl ErrorRedirector {
    pub fn redirect(&self, id: impl Into<String>, cause: ErrorCause) -> NavigationResult<()> {
        let query = SearchParams::new(id.into(), Some(cause));
        self.navigator.push_with_query(&Route::Error, &query)
    }
}

#[hook]
pub fn use_error_redirector() -> Option<ErrorRedirector> {
    let navigator = use_navigator()?;
    Some(ErrorRedirector { navigator })
}
