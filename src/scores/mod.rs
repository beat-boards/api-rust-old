pub mod score_controller;
pub mod score_service;

use crate::context::{generate_context, Ctx};
use crate::scores::score_controller::{create_score, delete_score, get_score, update_score};
use thruster::{middleware, App, MiddlewareChain, MiddlewareReturnValue, Request};

pub fn init() -> App<Request, Ctx> {
    let mut subroute = App::<Request, Ctx>::create(generate_context);

    subroute.post("/", middleware![Ctx => create_score]);
    subroute.get("/:id", middleware![Ctx => get_score]);
    subroute.put("/:id", middleware![Ctx => update_score]);
    subroute.delete("/:id", middleware![Ctx => delete_score]);

    subroute
}
