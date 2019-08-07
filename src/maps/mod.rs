pub mod map_controller;
pub mod map_service;

use crate::cache::maps::update_cache;
use crate::context::{generate_context, Ctx};
use crate::maps::map_controller::{create_map, delete_map, get_map, get_maps, update_map};
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
    subroute.get("/", middleware![Ctx => get_maps]);
    subroute.post("/", middleware![Ctx => create_map]);
    subroute.get("/:id", middleware![Ctx => get_map]);
    subroute.put("/:id", middleware![Ctx => update_map]);
    subroute.delete("/:id", middleware![Ctx => delete_map]);

    subroute
}
