use super::Room;
use crate::{game_server::PlayerJoined, WebsocketMessage};
use actix::prelude::*;

impl Handler<WebsocketMessage> for Room {
    type Result = ();

    fn handle(&mut self, ws_message: WebsocketMessage, _context: &mut Self::Context) {
        let session_id = ws_message.session_id;
        let content = ws_message.content;
        let mut message = content.chars().peekable();

        let first_char = if let Some(first_char) = message.next() {
            first_char
        } else {
            todo!("log error and return");
        };

        match first_char {
            's' => self.start_game(session_id, game::GameSettings { ally_count: 1 }),
            'b' => {
                let bid: Option<usize> = if content.len() > 1 {
                    if let Ok(num) = content[1..].parse() {
                        Some(num)
                    } else {
                        todo!("Handle integer parse error");
                    }
                } else {
                    None
                };

                self.bid(session_id, bid);
            }
            _ => {}
        }
    }
}

impl Handler<PlayerJoined> for Room {
    type Result = ();

    fn handle(&mut self, msg: PlayerJoined, _context: &mut Self::Context) {
        self.join(msg.session_id, msg.username, msg.recipient.clone());

        msg.recipient.do_send(super::RoomEvent::JoinedRoom {
            address: msg.room_addr,
            key: msg.room_key,
        });
    }
}
