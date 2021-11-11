use thruster::{async_middleware, middleware_fn};
use thruster::{App, BasicContext as Ctx, Request, Server, ThrusterServer};
use thruster::{MiddlewareNext, MiddlewareResult};

#[middleware_fn]
async fn get(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let val = "Hello, World!";
    context.body(val);
    Ok(context)
}

#[middleware_fn]
async fn set(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let val = "Hello, World!";
    context.body(val);
    Ok(context)
}

pub fn new_server(hostname: &str, port: u16){
    let mut app =  App::<Request, Ctx, ()>::new_basic();
    app.get("/get", async_middleware!(Ctx, [get]));
    app.get("/set", async_middleware!(Ctx, [set]));
    let server = Server::new(app);
    tokio::spawn(server.build(&hostname, port));
}