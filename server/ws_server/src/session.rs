use crate::{GameServer, Room, RoomEvent};

use actix::prelude::*;
use actix_web_actors::ws;

use slog::{error, info, o, warn, Logger};

#[derive(Message)]
#[rtype("()")]
pub struct WebsocketMessage {
    pub content: String,
    pub session_id: usize,
    pub logger: Logger,
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
    logger: slog::Logger,
}

impl Session {
    pub fn new(game_server: Addr<GameServer>, logger: slog::Logger) -> Session {
        Session {
            id: 0,
            game_server,
            room: None,
            logger,
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
                        let logger = act.logger.new(o!("session_id" => session_id));
                        act.logger = logger;
                        act.id = session_id;

                        info!(act.logger, "Session assigned ID");
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
        if self.id == 0 {
            error!(self.logger, "ID was 0 but received message {:?}", msg);
            return;
        }

        match msg {
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
            }
            // We don't send pings so we should receive these
            Ok(ws::Message::Pong(_msg)) => {}
            Ok(ws::Message::Text(text)) => {
                let message = WebsocketMessage {
                    content: text,
                    session_id: self.id,
                    logger: self.logger.clone(),
                };

                if let Some(room) = &self.room {
                    room.do_send(message);
                } else {
                    self.game_server.do_send(message);
                }
            }
            Ok(ws::Message::Binary(_)) => warn!(self.logger, "Receieved binary message"),
            Ok(ws::Message::Close(reason)) => {
                // TODO: Tell room + game server to disconnect
                // ^^ this should be handled as a lifecycle method so that all the ways that this
                // actor might stop will all trigger a message to the server to delete the user.
                info!(self.logger, "Session closed");
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => warn!(self.logger, "Received continuation message"),
            Ok(ws::Message::Nop) => {}
            Err(_) => {
                warn!(self.logger, "Got error instead of message stopping actor");
                ctx.stop();
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
            E::JoinedRoom {
                key,
                host,
                address,
                players,
            } => {
                self.room = Some(address);
                let mut s = format!("e{},{}", key, host);
                for (username, session_id) in players {
                    s.push_str(&format!(",{},{}", username, session_id));
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
            E::NoBids => format!("nb"),
            E::BiddingOver { bid, napoleon_id } => format!("bo{},{}", bid, napoleon_id),
            E::AlliesChosen { allies, trump_suit } => {
                let mut output = String::from("ac");
                output.push(trump_suit.to_char());
                for ally in allies {
                    output.push(',');
                    output.push_str(&format!("{},{}", ally.number, ally.suit.to_char()));
                }

                output
            }
            E::BecomeAlly => format!("ab"),
            E::NextPlayer {
                player_id,
                required_suit,
            } => {
                if let Some(suit) = required_suit {
                    format!("n{},{}", player_id, suit.to_char())
                } else {
                    format!("n{}", player_id)
                }
            }
            E::GameStarted {
                player_order,
                game_settings,
            } => format!(
                "s{}\n{}",
                player_order
                    .into_iter()
                    .map(|id| format!("{}", id))
                    .collect::<Vec<_>>()
                    .join(","),
                serde_json::to_string(&game_settings).expect("Serialization failed"),
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
            E::RoundOver { winner } => format!("r{}", winner),
            E::GameOver {
                allies,
                napoleon_score_delta,
                player_score_delta,
                napoleon_bet,
                combined_napoleon_score,
            } => {
                let mut s = format!(
                    "g{},{},{},{}",
                    napoleon_score_delta, player_score_delta, napoleon_bet, combined_napoleon_score
                );
                for ally in allies {
                    s.push_str(&format!(",{}", ally));
                }
                s
            }
        };

        ctx.text(message);
    }
}
