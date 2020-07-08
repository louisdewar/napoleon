mod game_server;
mod room;
mod session;

pub use game_server::GameServer;
pub use room::{Room, RoomEvent};
pub use session::{Session, WebsocketMessage};

use actix::prelude::*;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let game_server = GameServer::new().start();
    
    HttpServer::new(move || {
        App::new()
            .data(game_server.clone())
            .service(web::resource("/ws/").to(socket_route))
        
    })
    .bind(("0.0.0.0", 3001))?
    .run()
    .await
}

async fn socket_route(
    req: HttpRequest,
    stream: web::Payload,
    game_server: web::Data<Addr<GameServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        Session::new(game_server.get_ref().clone()),
        &req,
        stream,
    )
}
