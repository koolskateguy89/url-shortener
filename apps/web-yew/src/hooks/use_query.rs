use std::fmt::Debug;
use std::{future::Future, rc::Rc};
use yew::prelude::*;

use crate::api::RequestStatus;

pub type QueryStatus<T, F> = RequestStatus<T, F>;

pub struct Query<F, Succ, Err> {
    func: Rc<F>,
    pub status: RequestStatus<Succ, Err>,
}

/// Generic hook for querying data. Tried to copy react-query
#[hook]
pub fn use_query<F, Fut, Succ, Err>(func: F) -> UseStateHandle<Query<F, Succ, Err>>
where
    F: Fn() -> Fut + 'static,
    Fut: Future<Output = Result<Succ, Err>>,
    Succ: Debug + PartialEq + 'static,
    Err: Debug + 'static,
{
    let query = use_state(|| Query {
        func: Rc::new(func),
        status: QueryStatus::Idle,
    });

    query
}

pub trait QueryDispatcher {
    fn fetch(&self);
}

impl<F, Fut, Succ, Err> QueryDispatcher for UseStateHandle<Query<F, Succ, Err>>
where
    F: Fn() -> Fut + 'static,
    Fut: Future<Output = Result<Succ, Err>>,
    Succ: Debug + PartialEq + 'static,
    Err: Debug + 'static,
{
    // TODO?: accept input for func
    fn fetch(&self) {
        if matches!(self.status, QueryStatus::Loading) {
            return;
        }

        let handle = self.clone();
        let func = self.func.clone();

        wasm_bindgen_futures::spawn_local(async move {
            handle.set(Query {
                func: func.clone(),
                status: RequestStatus::Loading,
            });

            let result = func().await;

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
