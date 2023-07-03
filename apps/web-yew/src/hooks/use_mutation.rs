use yew::prelude::*;

use std::{future::Future, rc::Rc};

use crate::api::RequestStatus;

pub struct Mutation<F, SUCC, ERR> {
    func: Rc<F>,
    pub status: RequestStatus<SUCC, ERR>,
}

impl<F, SUCC, ERR> Mutation<F, SUCC, ERR> {
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
pub fn use_mutation<F, In, Fut, SUCC, ERR>(func: F) -> UseStateHandle<Mutation<F, SUCC, ERR>>
where
    F: Fn(In) -> Fut + 'static,
    In: 'static,
    Fut: Future<Output = Result<SUCC, ERR>>,
    SUCC: 'static,
    ERR: 'static,
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

impl<F, In, Fut, SUCC, ERR> MutationDispatcher<In> for UseStateHandle<Mutation<F, SUCC, ERR>>
where
    F: Fn(In) -> Fut + 'static,
    In: 'static,
    Fut: Future<Output = Result<SUCC, ERR>>,
    SUCC: 'static,
    ERR: 'static,
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
