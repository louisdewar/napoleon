mod game_server;
mod room;
mod session;

pub use game_server::GameServer;
pub use room::Room;
pub use session::Session;

use actix::prelude::*;

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub enum Event {}

fn main() {
    println!("Hello, world!");
}
