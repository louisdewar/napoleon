mod game_server;
mod room;
mod session;

pub use game_server::GameServer;
pub use room::{Room, RoomEvent};
pub use session::{Session, WebsocketMessage};

use actix::prelude::*;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

use slog::Logger;
use sloggers::Build;
use sloggers::terminal::{TerminalLoggerBuilder, Destination};
use sloggers::types::Severity;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut builder = TerminalLoggerBuilder::new();
    builder.level(Severity::Debug);
    builder.destination(Destination::Stderr);

    let logger = builder.build().unwrap();

    let game_server = GameServer::new(logger.clone()).start();
    HttpServer::new(move || {
        App::new()
            .data((logger.clone(), game_server.clone()))
            .service(web::resource("/ws/").to(socket_route))
    })
    .bind(("0.0.0.0", 3001))?
    .run()
    .await
}

async fn socket_route(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<(Logger, Addr<GameServer>)>,
) -> Result<HttpResponse, Error> {
    let (logger, addr) = data.get_ref().clone();
    ws::start(
        Session::new(addr, logger),
        &req,
        stream,
    )
}
