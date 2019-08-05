pub mod user_controller;
pub mod user_service;

use crate::context::{generate_context, Ctx};
use crate::users::user_controller::{get_users, create_user, delete_user, get_user, update_user};
use thruster::{middleware, App, MiddlewareChain, MiddlewareReturnValue, Request};

pub fn init() -> App<Request, Ctx> {
    let mut subroute = App::<Request, Ctx>::create(generate_context);

    subroute.get("/", middleware![Ctx => get_users]);
    subroute.post("/", middleware![Ctx => create_user]);
    subroute.get("/:id", middleware![Ctx => get_user]);
    subroute.put("/:id", middleware![Ctx => update_user]);
    subroute.delete("/:id", middleware![Ctx => delete_user]);

    subroute
}
