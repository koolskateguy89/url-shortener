use common::error::UrlError;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api::ApiError;
use crate::routes::error::{ErrorCause, SearchParams};
use crate::routes::Route;

pub struct ErrorRedirector {
    navigator: Navigator,
}

impl From<UrlError> for ErrorCause {
    fn from(err: UrlError) -> Self {
        match err {
            UrlError::NotFound => Self::NotFound,
            UrlError::InvalidUrl => Self::Other("Invalid URL".to_string()),
        }
    }
}

impl From<ApiError<UrlError>> for ErrorCause {
    fn from(err: ApiError<UrlError>) -> Self {
        match err {
            ApiError::Error(e) => e.into(),
            ApiError::Other(s) => Self::Other(s),
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
