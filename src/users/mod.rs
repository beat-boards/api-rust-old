pub mod user_controller;
pub mod user_service;

use crate::cache::users::update_cache;
use crate::context::{generate_context, Ctx};
use crate::users::user_controller::{create_user, delete_user, get_user, get_users, update_user};
use futures::future::lazy;
use futures::{future, Future};
use std::boxed::Box;
use thruster::thruster_middleware::query_params::query_params;
use thruster::{middleware, App, MiddlewareChain, MiddlewareReturnValue, Request};

fn cache(
    context: Ctx,
    next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx> + Send + Sync,
) -> MiddlewareReturnValue<Ctx> {
    let ctx_future = next(context).and_then(move |ctx| {
        if ctx.request.method() != "GET" {
            tokio::spawn(lazy(|| {
                update_cache();
                Ok(())
            }));
        }

        future::ok(ctx)
    });

    Box::new(ctx_future)
}

pub fn init() -> App<Request, Ctx> {
    let mut subroute = App::<Request, Ctx>::create(generate_context);

    subroute.use_middleware("/", middleware![Ctx => cache, Ctx => query_params]);
    subroute.get("/", middleware![Ctx => get_users]);
    subroute.post("/", middleware![Ctx => create_user]);
    subroute.get("/:id", middleware![Ctx => get_user]);
    subroute.put("/:id", middleware![Ctx => update_user]);
    subroute.delete("/:id", middleware![Ctx => delete_user]);

    subroute
}
