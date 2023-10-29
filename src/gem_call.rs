use crate::{async_trait, Client, GemBytes};
use std::{
    future::Future,
    marker::{Send, Sync},
};

#[async_trait]
pub trait GemCall<S> {
    /// Get the bytes from the route function.
    async fn gem_call(&self, client: Client<S>) -> Vec<u8>;
}

/// Implementation of GemCall for async functions.
#[async_trait]
impl<G, GF, S, FN> GemCall<S> for FN
where
    // Generic gembytes return
    G: GemBytes + Send + Sync,

    // G as a future
    GF: Future<Output = G> + Send + 'static,

    // State
    S: Send + Sync + Clone + 'static,

    // The function body
    FN: Fn(Client<S>) -> GF + Send + Sync,
{
    async fn gem_call(&self, client: Client<S>) -> Vec<u8> {
        (self)(client).await.gem_bytes().await
    }
}
