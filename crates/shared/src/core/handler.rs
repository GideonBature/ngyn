use std::{future::Future, pin::Pin};

use crate::server::{NgynContext, NgynResponse, ToBytes};

/// Represents a handler function that takes in a mutable reference to `NgynContext` and `NgynResponse`.
pub type Handler = dyn Fn(&mut NgynContext, &mut NgynResponse) + Send + Sync + 'static;

pub type AsyncHandler = Box<
    dyn for<'a, 'b> Fn(
            &'a mut NgynContext,
            &'b mut NgynResponse,
        ) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
        + Send
        + Sync,
>;

pub enum RouteHandler {
    Sync(Box<Handler>),
    Async(AsyncHandler),
}

impl<F: Fn(&mut NgynContext, &mut NgynResponse) + Send + Sync + 'static> From<F> for RouteHandler {
    fn from(f: F) -> Self {
        RouteHandler::Sync(Box::new(f))
    }
}

impl From<AsyncHandler> for RouteHandler {
    fn from(f: AsyncHandler) -> Self {
        RouteHandler::Async(Box::new(f))
    }
}

/// Creates a `Handler` trait object from a function that takes in a mutable reference to `NgynContext` and returns a type that implements `ToBytes`.
///
/// This function is useful for creating a `Handler` trait object from a function that returns any valid type that implements `ToBytes`.
///
/// ### Example
/// ```rust ignore
/// use ngyn::server::{handler, NgynContext, ToBytes};
///
/// app.get("/hello", handler(|ctx: &mut NgynContext| {
///    "Hello, World!"
/// }));
/// ```
pub fn handler<S: ToBytes + 'static>(
    f: impl Fn(&mut NgynContext) -> S + Send + Sync + 'static,
) -> Box<Handler> {
    Box::new(move |ctx: &mut NgynContext, res: &mut NgynResponse| {
        let body = f(ctx).to_bytes();
        *res.body_mut() = body.into();
    })
}

/// Creates a `AsyncHandler` trait object from an async function that takes in a mutable reference to `NgynContext` and returns a future with output that implements `ToBytes`.
///
/// ### Example
/// ```rust ignore
/// use ngyn::server::{async_handler, NgynContext, ToBytes};
///
/// app.get("/hello", async_handler(async |ctx: &mut NgynContext| {
///    "Hello, World!"
/// }));
/// ```
pub fn async_handler<S: ToBytes + 'static, Fut: Future<Output = S> + Send + 'static>(
    f: impl Fn(&mut NgynContext) -> Fut + Send + Sync + 'static,
) -> AsyncHandler {
    Box::new(move |ctx: &mut NgynContext, res: &mut NgynResponse| {
        let fut = f(ctx);
        Box::pin(async move {
            let body = fut.await.to_bytes();
            *res.body_mut() = body.into();
        })
    })
}
