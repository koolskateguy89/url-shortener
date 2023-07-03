use std::{future::Future, rc::Rc};

use yew::prelude::*;

// TODO?: extract this into it's own crate
// idts cos it lacks basically all the non-trivial react query features
// e.g. query keys, invalidating by query key, global query client
// might still do it within this project

pub enum QueryStatus<SUCC, ERR> {
    Idle, // - remove? because of fetch on mount
    /// Initial loading state
    Loading,
    Success(Rc<SUCC>),
    Error(Rc<ERR>),
    Refetching(Option<Result<Rc<SUCC>, Rc<ERR>>>),
}

pub struct Query<F, SUCC, ERR> {
    func: Rc<F>,
    pub status: QueryStatus<SUCC, ERR>,
}

impl<F, SUCC, ERR> Query<F, SUCC, ERR> {
    pub fn data(&self) -> Option<&SUCC> {
        match &self.status {
            QueryStatus::Success(data) => Some(data),
            QueryStatus::Refetching(Some(Ok(data))) => Some(data),
            _ => None,
        }
    }

    pub fn error(&self) -> Option<&ERR> {
        match &self.status {
            QueryStatus::Error(err) => Some(err),
            QueryStatus::Refetching(Some(Err(err))) => Some(err),
            _ => None,
        }
    }

    pub fn is_idle(&self) -> bool {
        matches!(self.status, QueryStatus::Idle)
    }

    pub fn is_initial_loading(&self) -> bool {
        matches!(self.status, QueryStatus::Loading)
    }

    pub fn is_success(&self) -> bool {
        matches!(self.status, QueryStatus::Success(_))
    }

    pub fn is_error(&self) -> bool {
        matches!(self.status, QueryStatus::Error(_))
    }

    pub fn is_refetching(&self) -> bool {
        matches!(self.status, QueryStatus::Refetching(_))
    }

    /// Is true when initial loading or refetching
    pub fn is_fetching(&self) -> bool {
        matches!(
            self.status,
            QueryStatus::Loading | QueryStatus::Refetching(_)
        )
    }
}

/// Generic hook for querying data. Copied basic idea from react-query.
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
        status: QueryStatus::Idle,
    });

    {
        // fetch on mount
        let query = query.clone();

        use_effect_with_deps(
            move |_| {
                initial_fetch(&query, input);
            },
            (),
        );
    }

    query
}

fn initial_fetch<F, In, Fut, SUCC, ERR>(handle: &UseStateHandle<Query<F, SUCC, ERR>>, input: In)
where
    F: Fn(In) -> Fut + 'static,
    In: 'static,
    Fut: Future<Output = Result<SUCC, ERR>>,
    SUCC: 'static,
    ERR: 'static,
{
    let handle = handle.clone();
    let func = handle.func.clone();

    wasm_bindgen_futures::spawn_local(async move {
        handle.set(Query {
            func: func.clone(),
            status: QueryStatus::Loading,
        });

        let result = func(input).await;

        match result {
            Ok(data) => {
                handle.set(Query {
                    func,
                    status: QueryStatus::Success(data.into()),
                });
            }
            Err(err) => {
                handle.set(Query {
                    func,
                    status: QueryStatus::Error(err.into()),
                });
            }
        }
    });
}

pub trait QueryRefetcher<Input> {
    fn refetch(&self, input: Input);
}

impl<F, In, Fut, SUCC, ERR> QueryRefetcher<In> for UseStateHandle<Query<F, SUCC, ERR>>
where
    F: Fn(In) -> Fut + 'static,
    In: 'static,
    Fut: Future<Output = Result<SUCC, ERR>>,
    SUCC: 'static,
    ERR: 'static,
{
    fn refetch(&self, input: In) {
        let refetching_status = QueryStatus::Refetching(match &self.status {
            QueryStatus::Idle => None,
            QueryStatus::Loading => None,
            QueryStatus::Success(succ) => Some(Ok(succ.clone())),
            QueryStatus::Error(err) => Some(Err(err.clone())),
            // If already refetching, don't do anything
            QueryStatus::Refetching(_) => return,
        });

        let handle = self.clone();
        let func = self.func.clone();

        wasm_bindgen_futures::spawn_local(async move {
            handle.set(Query {
                func: func.clone(),
                status: refetching_status,
            });

            let result = func(input).await;

            match result {
                Ok(data) => {
                    handle.set(Query {
                        func,
                        status: QueryStatus::Success(data.into()),
                    });
                }
                Err(err) => {
                    handle.set(Query {
                        func,
                        status: QueryStatus::Error(err.into()),
                    });
                }
            }
        });
    }
}
