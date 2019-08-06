use crate::context::Ctx;
use thruster::{MiddlewareChain, MiddlewareReturnValue};

use crate::maps::map_service;
use crate::models::maps::{Map, NewMap};
use crate::util::error::HttpError;
use futures::future;
use std::boxed::Box;
use uuid::Uuid;

pub fn get_maps(
    mut context: Ctx,
    _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx> + Send + Sync,
) -> MiddlewareReturnValue<Ctx> {
    fn error(mut context: Ctx) -> MiddlewareReturnValue<Ctx> {
        HttpError::internal_server_error("An error occurred while loading maps")
            .set_context(&mut context);
        Box::new(future::ok(context))
    }

    context.content_type("application/json");

    let limit: i64 = context
        .query_params
        .get("limit")
        .unwrap_or(&String::from("100"))
        .parse()
        .unwrap_or(100);

    let hash = context.query_params.get("hash");

    let filters = map_service::Filters { hash };

    let fetched_result = match map_service::get_maps(limit, filters) {
        Ok(_fetched_result) => _fetched_result,
        Err(_) => return error(context),
    };

    match serde_json::to_string(&fetched_result) {
        Ok(result) => context.body(&result),
        Err(_) => return error(context),
    };

    Box::new(future::ok(context))
}

pub fn create_map(
    mut context: Ctx,
    _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx> + Send + Sync,
) -> MiddlewareReturnValue<Ctx> {
    match serde_json::from_str::<NewMap>(&context.request.body()) {
        Ok(new_map) => {
            match map_service::create_map(new_map) {
                Ok(map) => {
                    context.content_type("application/json");
                    context.body(&serde_json::to_string(&map).unwrap());
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

pub fn get_map(
    mut context: Ctx,
    _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx> + Send + Sync,
) -> MiddlewareReturnValue<Ctx> {
    fn error(mut context: Ctx) -> MiddlewareReturnValue<Ctx> {
        HttpError::bad_request("The specified map doesn't exist").set_context(&mut context);
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

    let fetched_result = match map_service::get_map(id) {
        Ok(_fetched_result) => _fetched_result,
        Err(_) => return error(context),
    };

    match serde_json::to_string(&fetched_result) {
        Ok(result) => context.body(&result),
        Err(_) => return error(context),
    };

    Box::new(future::ok(context))
}

pub fn update_map(
    mut context: Ctx,
    _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx> + Send + Sync,
) -> MiddlewareReturnValue<Ctx> {
    Box::new(future::ok(context))
}

pub fn delete_map(
    mut context: Ctx,
    _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx> + Send + Sync,
) -> MiddlewareReturnValue<Ctx> {
    Box::new(future::ok(context))
}
