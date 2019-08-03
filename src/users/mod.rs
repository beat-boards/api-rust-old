pub mod user_controller;
pub mod user_service;

use thruster::{App, middleware, MiddlewareChain, MiddlewareReturnValue, Request};
use crate::context::{generate_context, Ctx};
use crate::users::user_controller::{
  create_user,
  get_user,
  update_user,
  delete_user
};

pub fn init() -> App<Request, Ctx> {
  let mut subroute = App::<Request, Ctx>::create(generate_context);

  subroute.post("/", middleware![Ctx => create_user]);
  subroute.get("/:id", middleware![Ctx => get_user]);
  subroute.put("/:id", middleware![Ctx => update_user]);
  subroute.delete("/:id", middleware![Ctx => delete_user]);

  subroute
}
