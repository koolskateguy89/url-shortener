use yew::prelude::*;

use std::{future::Future, rc::Rc};

use crate::api::RequestStatus;

pub struct Query<F, Succ, Err> {
    func: Rc<F>,
    pub status: RequestStatus<Succ, Err>,
}

impl<F, Succ, Err> Query<F, Succ, Err> {
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
pub fn use_query<F, In, Fut, Succ, Err>(input: In, func: F) -> UseStateHandle<Query<F, Succ, Err>>
where
    F: Fn(In) -> Fut + 'static,
    In: 'static,
    Fut: Future<Output = Result<Succ, Err>>,
    Succ: 'static,
    Err: 'static,
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

impl<F, In, Fut, Succ, Err> QueryDispatcher<In> for UseStateHandle<Query<F, Succ, Err>>
where
    F: Fn(In) -> Fut + 'static,
    In: 'static,
    Fut: Future<Output = Result<Succ, Err>>,
    Succ: 'static,
    Err: 'static,
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

// TODO: write tests - but how to test hooks?
