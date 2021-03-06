use crate::context::Ctx;
use thruster::MiddlewareReturnValue;

use crate::models::scores::NewScore;
use crate::scores::score_service;
use crate::util::error::HttpError;
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
                    HttpError::internal_server_error("A database error occurred")
                        .set_context(&mut context);
                }
            };
        }
        Err(_) => {
            HttpError::bad_request("The provided body is invalid").set_context(&mut context);
        }
    };

    Box::new(future::ok(context))
}

pub fn get_score(
    mut context: Ctx,
    _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx> + Send + Sync,
) -> MiddlewareReturnValue<Ctx> {
    fn error(mut context: Ctx) -> MiddlewareReturnValue<Ctx> {
        HttpError::bad_request("The specified score doesn't exist").set_context(&mut context);
        Box::new(future::ok(context))
    }

    context.content_type("application/json");

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
