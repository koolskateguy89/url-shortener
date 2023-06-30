pub mod auth;
pub mod url;

pub type NetResult<T> = Result<T, gloo_net::Error>;

/// Generic request status enum, not completely suitable for refetching
#[derive(Debug, Clone)]
pub enum RequestStatus<T, F> {
    Idle,
    Loading,
    Success(T),
    Error(F),
}
