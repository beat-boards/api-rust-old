#[macro_use]
extern crate thruster;
extern crate futures;

use futures::future;
use std::boxed::Box;

use thruster::builtins::server::Server;
use thruster::server::ThrusterServer;
use thruster::{App, BasicContext as Ctx, MiddlewareChain, MiddlewareReturnValue, Request};

fn plaintext(
    mut context: Ctx,
    _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx> + Send + Sync,
) -> MiddlewareReturnValue<Ctx> {
    let val = "Pong!";
    context.body(val);

    Box::new(future::ok(context))
}

fn main() {
    println!("Starting server...");

    let mut app = App::<Request, Ctx>::new_basic();

    app.get("/ping", middleware![Ctx => plaintext]);

    let server = Server::new(app);
    server.start("0.0.0.0", 4321);
}
