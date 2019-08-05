pub mod map_controller;
pub mod map_service;

use crate::context::{generate_context, Ctx};
use crate::maps::map_controller::{create_map, delete_map, get_map, get_maps, update_map};
use thruster::thruster_middleware::query_params::query_params;
use thruster::{middleware, App, MiddlewareChain, MiddlewareReturnValue, Request};

pub fn init() -> App<Request, Ctx> {
    let mut subroute = App::<Request, Ctx>::create(generate_context);

    subroute.use_middleware("/", middleware![Ctx => query_params]);
    subroute.get("/", middleware![Ctx => get_maps]);
    subroute.post("/", middleware![Ctx => create_map]);
    subroute.get("/:id", middleware![Ctx => get_map]);
    subroute.put("/:id", middleware![Ctx => update_map]);
    subroute.delete("/:id", middleware![Ctx => delete_map]);

    subroute
}
