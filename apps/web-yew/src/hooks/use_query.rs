use yew::prelude::*;

use std::{future::Future, rc::Rc};

use crate::api::RequestStatus;

// TODO: use instead of RequestStatus to support refetching
// pub enum QueryStatus<SUCC, ERR> {
//     Idle, // - remove? because of fetch on mount
//     Loading,
//     Success(Succ),
//     Error(Err),
//     Refetching(Option<Succ>),
// }

// TODO?: extract this into it's own crate
// idts cos it lacks basically all the non-trivial react query features
// e.g. query keys, invalidating by query key, global query client
// might still do it within this project

pub struct Query<F, SUCC, ERR> {
    func: Rc<F>,
    pub status: RequestStatus<SUCC, ERR>,
}

impl<F, SUCC, ERR> Query<F, SUCC, ERR> {
    pub fn data(&self) -> Option<&SUCC> {
        match &self.status {
            RequestStatus::Success(data) => Some(data),
            _ => None,
        }
    }

    pub fn error(&self) -> Option<&ERR> {
        match &self.status {
            RequestStatus::Error(err) => Some(err),
            _ => None,
        }
    }

    pub fn is_idle(&self) -> bool {
        matches!(self.status, RequestStatus::Idle)
    }

    pub fn is_loading(&self) -> bool {
        matches!(self.status, RequestStatus::Loading)
    }

    pub fn is_success(&self) -> bool {
        matches!(self.status, RequestStatus::Success(_))
    }

    pub fn is_error(&self) -> bool {
        matches!(self.status, RequestStatus::Error(_))
    }
}

/// Generic hook for querying data. Tried to copy react-query
#[hook]
pub fn use_query<F, In, Fut, SUCC, ERR>(input: In, func: F) -> UseStateHandle<Query<F, SUCC, ERR>>
where
    F: Fn(In) -> Fut + 'static,
    In: 'static,
    Fut: Future<Output = Result<SUCC, ERR>>,
    SUCC: 'static,
    ERR: 'static,
{
    let query = use_state(|| Query {
        func: Rc::new(func),
        status: RequestStatus::Idle,
    });

    // fetch on mount
    {
        let query = query.clone();

        use_effect_with_deps(
            move |_| {
                query.fetch(input);
            },
            (),
        );
    }

    query
}

pub trait QueryDispatcher<Input> {
    fn fetch(&self, input: Input);
}

impl<F, In, Fut, SUCC, ERR> QueryDispatcher<In> for UseStateHandle<Query<F, SUCC, ERR>>
where
    F: Fn(In) -> Fut + 'static,
    In: 'static,
    Fut: Future<Output = Result<SUCC, ERR>>,
    SUCC: 'static,
    ERR: 'static,
{
    fn fetch(&self, input: In) {
        if matches!(self.status, RequestStatus::Loading) {
            return;
        }

        let handle = self.clone();
        let func = self.func.clone();

        wasm_bindgen_futures::spawn_local(async move {
            handle.set(Query {
                func: func.clone(),
                status: RequestStatus::Loading,
            });

            let result = func(input).await;

            match result {
                Ok(data) => {
                    handle.set(Query {
                        func,
                        status: RequestStatus::Success(data),
                    });
                }
                Err(err) => {
                    handle.set(Query {
                        func,
                        status: RequestStatus::Error(err),
                    });
                }
            }
        });
    }
}
