use std::fmt::Debug;
use std::{future::Future, rc::Rc};
use yew::prelude::*;

use crate::api::RequestStatus;

pub struct Mutation<F, Succ, Err> {
    func: Rc<F>,
    pub status: RequestStatus<Succ, Err>,
}

/// Generic hook for querying data. Tried to copy react-query
#[hook]
pub fn use_mutation<F, In, Fut, Succ, Err>(func: F) -> UseStateHandle<Mutation<F, Succ, Err>>
where
    F: Fn(In) -> Fut + 'static,
    In: 'static,
    Fut: Future<Output = Result<Succ, Err>>,
    Succ: Debug + PartialEq + 'static,
    Err: Debug + 'static,
{
    let mutation = use_state(|| Mutation {
        func: Rc::new(func),
        status: RequestStatus::Idle,
    });

    mutation
}

pub trait MutationDispatcher<In> {
    fn mutate(&self, input: In);
}

impl<F, In, Fut, Succ, Err> MutationDispatcher<In> for UseStateHandle<Mutation<F, Succ, Err>>
where
    F: Fn(In) -> Fut + 'static,
    In: 'static,
    Fut: Future<Output = Result<Succ, Err>>,
    Succ: Debug + PartialEq + 'static,
    Err: Debug + 'static,
{
    fn mutate(&self, input: In) {
        if matches!(self.status, RequestStatus::Loading) {
            return;
        }

        let handle = self.clone();
        let func = self.func.clone();

        wasm_bindgen_futures::spawn_local(async move {
            handle.set(Mutation {
                func: func.clone(),
                status: RequestStatus::Loading,
            });

            let result = func(input).await;

            match result {
                Ok(data) => {
                    handle.set(Mutation {
                        func,
                        status: RequestStatus::Success(data),
                    });
                }
                Err(err) => {
                    handle.set(Mutation {
                        func,
                        status: RequestStatus::Error(err),
                    });
                }
            }
        });
    }
}
