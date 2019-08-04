use crate::context::Ctx;
use thruster::{MiddlewareChain, MiddlewareReturnValue};

use crate::maps::map_service;
use crate::models::maps::{Map, NewMap};
use futures::future;
use std::boxed::Box;
use uuid::Uuid;

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

                    context.status(400);
                    context.content_type("application/json");
                    context.body(
                        &serde_json::json!({"code": 2, "message": "Database error"}).to_string(),
                    );
                }
            };
        }
        Err(e) => {
            context.status(400);
            context.content_type("application/json");
            context.body(&serde_json::json!({"code": 1, "message": "Invalid body"}).to_string());
        }
    };

    Box::new(future::ok(context))
}

pub fn get_map(
    mut context: Ctx,
    _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx> + Send + Sync,
) -> MiddlewareReturnValue<Ctx> {
    fn error(mut context: Ctx) -> MiddlewareReturnValue<Ctx> {
        context.status(400);
        context.body("Could not get Map");
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
