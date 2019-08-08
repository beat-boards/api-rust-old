use crate::context::Ctx;
use thruster::MiddlewareReturnValue;

use crate::models::users::NewUser;
use crate::users::user_service;
use crate::util::error::HttpError;
use crate::util::query_string;
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

    let (offset, limit) = query_string::get_offset_and_limit(&context.query_params);

    let rank = match context.query_params.get("rank") {
        Some(_) => true,
        None => false,
    };

    let steam_id = context.query_params.get("steamId");
    let steam_id: Option<i64> = match steam_id {
        Some(id) => match id.parse::<i64>() {
            Ok(id) => Some(id),
            Err(_) => None,
        },
        None => None,
    };
    let oculus_id = context.query_params.get("oculusId");

    let filters = user_service::Filters {
        steam_id,
        oculus_id,
    };

    if offset + limit <= 1000 && offset % 50 == 0 && limit % 50 == 0 {
        if let user_service::Filters {
            steam_id: None,
            oculus_id: None,
        } = filters
        {
            context.body(&user_service::get_cached_users(
                offset, limit, rank, filters,
            ));
            return Box::new(future::ok(context));
        }
    }

    let fetched_result = match user_service::get_users(offset, limit, filters) {
        Ok(_fetched_result) => _fetched_result,
        Err(_) => return error(context),
    };

    if rank == true {
        match serde_json::to_string(&user_service::rank::rank_users(fetched_result, offset)) {
            Ok(result) => context.body(&result),
            Err(_) => return error(context),
        };
    } else {
        match serde_json::to_string(&fetched_result) {
            Ok(result) => context.body(&result),
            Err(_) => return error(context),
        };
    }

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
        Err(_) => {
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
