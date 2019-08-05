use crate::context::Ctx;
use thruster::{MiddlewareChain, MiddlewareReturnValue};

use crate::models::users::{NewUser, User};
use crate::users::user_service;
use crate::util::error::HttpError;
use futures::future;
use std::boxed::Box;
use uuid::Uuid;

pub fn get_users(
    mut context: Ctx,
    _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx> + Send + Sync,
) -> MiddlewareReturnValue<Ctx> {
    fn error(mut context: Ctx) -> MiddlewareReturnValue<Ctx> {
        HttpError::internal_server_error("An error occurred while loading users")
            .set_context(&mut context);
        Box::new(future::ok(context))
    }

    context.content_type("application/json");

    println!("{:#?}", &context.query_params);

    let limit: i64 = context
        .query_params
        .get("limit")
        .unwrap_or(&String::from("100"))
        .parse()
        .unwrap_or(100);

    let fetched_result = match user_service::get_users(limit) {
        Ok(_fetched_result) => _fetched_result,
        Err(_) => return error(context),
    };

    match serde_json::to_string(&fetched_result) {
        Ok(result) => context.body(&result),
        Err(_) => return error(context),
    };

    Box::new(future::ok(context))
}

pub fn create_user(
    mut context: Ctx,
    _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx> + Send + Sync,
) -> MiddlewareReturnValue<Ctx> {
    match serde_json::from_str::<NewUser>(&context.request.body()) {
        Ok(new_user) => {
            match user_service::create_user(new_user) {
                Ok(user) => {
                    context.content_type("application/json");
                    context.body(&serde_json::to_string(&user).unwrap());
                }
                Err(e) => {
                    eprintln!("Database error: {:#?}", e);
                    HttpError::internal_server_error("A database error occurred")
                        .set_context(&mut context);
                }
            };
        }
        Err(e) => {
            HttpError::bad_request("The provided body is invalid").set_context(&mut context);
        }
    };

    Box::new(future::ok(context))
}

pub fn get_user(
    mut context: Ctx,
    _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx> + Send + Sync,
) -> MiddlewareReturnValue<Ctx> {
    fn error(mut context: Ctx) -> MiddlewareReturnValue<Ctx> {
        HttpError::bad_request("The specified user doesn't exist").set_context(&mut context);
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

    let fetched_result = match user_service::get_user(id) {
        Ok(_fetched_result) => _fetched_result,
        Err(_) => return error(context),
    };

    match serde_json::to_string(&fetched_result) {
        Ok(result) => context.body(&result),
        Err(_) => return error(context),
    };

    Box::new(future::ok(context))
}

pub fn update_user(
    mut context: Ctx,
    _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx> + Send + Sync,
) -> MiddlewareReturnValue<Ctx> {
    Box::new(future::ok(context))
}

pub fn delete_user(
    mut context: Ctx,
    _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx> + Send + Sync,
) -> MiddlewareReturnValue<Ctx> {
    Box::new(future::ok(context))
}
