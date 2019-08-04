use crate::context::Ctx;
use thruster::{MiddlewareChain, MiddlewareReturnValue};

use crate::models::scores::{NewScore, Score};
use crate::scores::score_service;
use crate::util::error::Error;
use futures::future;
use std::boxed::Box;
use uuid::Uuid;

pub fn create_score(
    mut context: Ctx,
    _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx> + Send + Sync,
) -> MiddlewareReturnValue<Ctx> {
    match serde_json::from_str::<NewScore>(&context.request.body()) {
        Ok(new_score) => {
            match score_service::create_score(new_score) {
                Ok(score) => {
                    context.content_type("application/json");
                    context.body(&serde_json::to_string(&score).unwrap());
                }
                Err(e) => {
                    eprintln!("Database error: {:#?}", e);
                    Error::internal_error().set_context(&mut context);
                }
            };
        }
        Err(e) => {
            Error::request_error().set_context(&mut context);
        }
    };

    Box::new(future::ok(context))
}

pub fn get_score(
    mut context: Ctx,
    _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx> + Send + Sync,
) -> MiddlewareReturnValue<Ctx> {
    fn error(mut context: Ctx) -> MiddlewareReturnValue<Ctx> {
        context.status(400);
        context.body("Could not get Score");
        Box::new(future::ok(context))
    }

    let id = match context.params.get("id") {
        Some(_id) => _id,
        None => return error(context),
    };

    let id = match Uuid::parse_str(&id) {
        Ok(_id_as_uuid) => _id_as_uuid,
        Err(_) => return error(context),
    };

    let fetched_result = match score_service::get_score(id) {
        Ok(_fetched_result) => _fetched_result,
        Err(_) => return error(context),
    };

    match serde_json::to_string(&fetched_result) {
        Ok(result) => context.body(&result),
        Err(_) => return error(context),
    };

    Box::new(future::ok(context))
}

pub fn update_score(
    mut context: Ctx,
    _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx> + Send + Sync,
) -> MiddlewareReturnValue<Ctx> {
    Box::new(future::ok(context))
}

pub fn delete_score(
    mut context: Ctx,
    _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx> + Send + Sync,
) -> MiddlewareReturnValue<Ctx> {
    Box::new(future::ok(context))
}
