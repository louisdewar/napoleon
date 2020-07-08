use crate::{GameServer, Room, RoomEvent};

use actix::prelude::*;
use actix_web_actors::ws;

#[derive(Message)]
#[rtype("()")]
pub struct WebsocketMessage {
    pub content: String,
    pub session_id: usize,
}

#[derive(Message)]
#[rtype("usize")]
pub struct SessionStarted {
    pub recipient: Recipient<RoomEvent>,
}

pub struct Session {
    id: usize,
    room: Option<Addr<Room>>,
    game_server: Addr<GameServer>,
}

impl Session {
    pub fn new(game_server: Addr<GameServer>) -> Session {
        Session {
            id: 0,
            game_server,
            room: None,
        }
    }
}

impl Actor for Session {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let session_addr = ctx.address();

        self.game_server
            .send(SessionStarted {
                recipient: session_addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(session_id) => {
                        act.id = session_id;
                        ctx.text(format!("c{}", session_id));
                    }
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        println!("WS: {:?}", msg);

        assert_ne!(self.id, 0);

        match msg {
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
            }
            Ok(ws::Message::Text(text)) => {
                let message = WebsocketMessage {
                    content: text,
                    session_id: self.id,
                };

                if let Some(room) = &self.room {
                    room.do_send(message);
                } else {
                    self.game_server.do_send(message);
                }
            }
            Ok(ws::Message::Binary(bin)) => {}
            Ok(ws::Message::Close(reason)) => {
                // TODO: Tell room + game server to disconnect
                ctx.close(reason);
                ctx.stop();
            }
            _ => {
                dbg!("stop?");
                return;
            }
        }
    }
}

impl Handler<RoomEvent> for Session {
    type Result = ();

    fn handle(&mut self, event: RoomEvent, ctx: &mut Self::Context) {
        use RoomEvent as E;
        let message = match event {
            E::JoinedRoom { key, address } => {
                self.room = Some(address);
                format!("j{}", key)
            }
            E::NextBidder { player_id } => format!("n{}", player_id),
            E::PlayerBid { bid, player_id } => {
                if let Some(bid) = bid {
                    format!("b{},{}", player_id, bid)
                } else {
                    format!("b{}", player_id)
                }
            }
            _ => {
                println!("todo");

                return;
            }
        };

        ctx.text(message);
    }
}
