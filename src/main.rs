extern crate dotenv;
extern crate env_logger;
extern crate futures;
extern crate serde;
extern crate serde_json;
extern crate thruster;
extern crate tokio;
extern crate tokio_proto;
extern crate tokio_service;
extern crate uuid;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_repr;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;

pub mod models;
pub mod schema;

mod context;
mod util;

mod maps;
mod scores;
mod users;

use dotenv::dotenv;
use futures::{future, Future};
use std::boxed::Box;
use std::env;

use std::time::Instant;
use thruster::server::Server;
use thruster::ThrusterServer;
use thruster::{middleware, App, MiddlewareChain, MiddlewareReturnValue};

use crate::maps::init as map_routes;
use crate::scores::init as score_routes;
use crate::users::init as user_routes;

use crate::context::{generate_context, Ctx};

use crate::util::error::HttpError;

fn profiling(
    context: Ctx,
    next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx> + Send + Sync,
) -> MiddlewareReturnValue<Ctx> {
    let start_time = Instant::now();

    let ctx_future = next(context).and_then(move |ctx| {
        let elapsed_time = start_time.elapsed();
        println!(
            "[{}μs] {} -- {}",
            elapsed_time.as_micros(),
            ctx.request.method(),
            ctx.request.path()
        );

        future::ok(ctx)
    });

    Box::new(ctx_future)
}

fn ping(
    mut context: Ctx,
    _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx> + Send + Sync,
) -> MiddlewareReturnValue<Ctx> {
    context.body("Pong!");

    Box::new(future::ok(context))
}

pub fn not_found(
    mut context: Ctx,
    _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx> + Send + Sync,
) -> MiddlewareReturnValue<Ctx> {
    HttpError::not_found("The specified route doesn't exist on this server")
        .set_context(&mut context);
    Box::new(future::ok(context))
}

fn main() {
    dotenv().ok();

    let mut app = App::create(generate_context);

    app.use_middleware("/", middleware![Ctx => profiling]);
    app.get("/ping", middleware![Ctx => ping]);

    app.use_sub_app("/maps", map_routes());
    app.use_sub_app("/scores", score_routes());
    app.use_sub_app("/users", user_routes());

    app.set404(middleware![Ctx => not_found]);

    let host = env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or("4321".to_string());

    println!("Running on {}:{}", &host, &port);
    let server = Server::new(app);
    server.start(&host, port.parse::<u16>().unwrap());
}
