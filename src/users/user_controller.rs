use crate::context::{ Ctx };
use thruster::{MiddlewareChain, MiddlewareReturnValue};

use crate::users::user_service;
use crate::models::users::{ NewUser, User };
use futures::future;
use std::boxed::Box;
use uuid::Uuid;

pub fn create_user(mut context: Ctx, _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx>  + Send + Sync) -> MiddlewareReturnValue<Ctx> {
  match serde_json::from_str::<NewUser>(&context.request.body()) {
    Ok(new_user) => {
      match user_service::create_user(new_user) {
        Ok(user) => {
          context.content_type("application/json");
          context.body(&serde_json::to_string(&user).unwrap());
        },
        Err(e) => {
          eprintln!("Database error: {:#?}", e);

          context.status(400);
          context.content_type("application/json");
          context.body(&serde_json::json!({"code": 2, "message": "Database error"}).to_string());
        }
      };
    },
    Err(e) => {
      context.status(400);
      context.content_type("application/json");
      context.body(&serde_json::json!({"code": 1, "message": "Invalid body"}).to_string());
    }
  };

  Box::new(future::ok(context))
}

pub fn get_user(mut context: Ctx, _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx>  + Send + Sync) -> MiddlewareReturnValue<Ctx> {
  fn error(mut context: Ctx) -> MiddlewareReturnValue<Ctx> {
    context.status(400);
    context.body("Could not get User");
    Box::new(future::ok(context))
  }

  let id = match context.params.get("id") {
    Some(_id) => _id,
    None => return error(context)
  };

  let id = match Uuid::parse_str(&id) {
    Ok(_id_as_uuid) => _id_as_uuid,
    Err(_) => return error(context)
  };

  let fetched_result = match user_service::get_user(id) {
    Ok(_fetched_result) => _fetched_result,
    Err(_) => return error(context)
  };

  match serde_json::to_string(&fetched_result) {
    Ok(result) => context.body(&result),
    Err(_) => return error(context)
  };

  Box::new(future::ok(context))
}

pub fn update_user(mut context: Ctx, _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx>  + Send + Sync) -> MiddlewareReturnValue<Ctx> {
  Box::new(future::ok(context))
}

pub fn delete_user(mut context:Ctx, _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx>  + Send + Sync) -> MiddlewareReturnValue<Ctx> {
  Box::new(future::ok(context))
}
