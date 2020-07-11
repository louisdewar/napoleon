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
            Ok(ws::Message::Binary(_)) => {}
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
            E::JoinedRoom { key, address, players } => {
                self.room = Some(address);
                let mut s = format!("e{}", key);
                for (username, session_id) in players {
                    s.push_str(format!(",{},{}", username, session_id));
                }
                s
            }
            E::NextBidder { player_id } => format!("bn{}", player_id),
            E::PlayerBid { bid, player_id } => {
                if let Some(bid) = bid {
                    format!("bp{},{}", player_id, bid)
                } else {
                    format!("bp{}", player_id)
                }
            }
            E::PlayerJoined {
                player_id,
                username,
            } => format!("j{},{}", username, player_id),
            E::NoBids => format!("bn"),
            E::BiddingOver { bid, napoleon_id } => format!("bo{},{}", bid, napoleon_id),
            E::AlliesChosen { allies, trump_suit } => {
                let mut output = String::from("ac");
                output.push(trump_suit.to_char());
                for ally in allies {
                    output.push(',');
                    output.push_str(&format!("{}", ally));
                }

                output
            }
            E::BecomeAlly => format!("ab"),
            E::NextPlayer { player_id } => format!("n{}", player_id),
            E::GameStarted { player_order, .. } => format!(
                "s{}",
                player_order
                    .into_iter()
                    .map(|id| format!("{}", id))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            E::PlayerHand { hand } => format!(
                "h{}",
                hand.into_iter()
                    .map(|card| format!("{}{}", card.number, card.suit.to_char()))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            E::CardPlayed { player_id, card } => {
                format!("p{},{}{}", player_id, card.number, card.suit.to_char())
            }
            E::RoundOver { winner } => {
                format!("r{}", winner)
            }
            E::GameOver { allies, napoleon_score_delta, player_score_delta, napoleon_bet, combined_napoleon_score } => {
                let mut s = format!("g{},{},{},{}", napoleon_score_delta, player_score_delta, napoleon_bet, combined_napoleon_score);
                for ally in allies {
                    s.push_str(format!(",{}", ally));
                }
                s
            }
        };

        ctx.text(message);
    }
}
