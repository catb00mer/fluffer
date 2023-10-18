use crate::{async_trait, Context, GemBytes};
use std::{
    future::Future,
    marker::{Send, Sync},
};

#[async_trait]
pub trait GemCall {
    /// Get the bytes from the route function.
    async fn gem_call(&self, context: Context) -> Vec<u8>;
}

/// Implementation of GemCall for async functions.
#[async_trait]
impl<G, GF, FN> GemCall for FN
where
    // Generic gembytes return
    G: GemBytes + Send + Sync,

    // G as a future
    GF: Future<Output = G> + Send + 'static,

    // The function body
    FN: Fn(Context) -> GF + Send + Sync,
{
    async fn gem_call(&self, context: Context) -> Vec<u8> {
        (self)(context).await.gem_bytes().await
    }
}
