use crate::{session::SessionStarted, Room, RoomEvent, WebsocketMessage};
use actix::prelude::*;
use std::collections::HashMap;

pub struct GameServer {
    connected_sessions: HashMap<usize, Recipient<RoomEvent>>,
    rooms: HashMap<String, Addr<Room>>,
}

impl GameServer {
    pub fn new() -> GameServer {
        GameServer {
            connected_sessions: HashMap::new(),
            rooms: HashMap::new(),
        }
    }

    fn create_room(&mut self, session_id: usize, username: String) {
        // TODO: Make random
        if let Some(recipient) = self.connected_sessions.get(&session_id) {
            let address = Room::new(session_id, username, recipient.clone()).start();
            let key = "room_key".to_string();
            self.rooms.insert(key.clone(), address.clone());

            // Ignore error
            let _ = recipient.do_send(RoomEvent::JoinedRoom { key, address });
        } else {
            todo!("log internal server error");
        }
    }

    fn join_room(&mut self, session_id: usize, username: String, room_key: &String) {
        if let Some(recipient) = self.connected_sessions.get(&session_id) {
            if let Some(room_address) = self.rooms.get(room_key) {
                room_address.do_send(PlayerJoined {
                    username: username.clone(),
                    session_id,
                    room_addr: room_address.clone(),
                    room_key: room_key.clone(),
                    recipient: recipient.clone(),
                });
            }
        } else {
            todo!("log internal server error");
        }
    }

    fn connect(&mut self, recipient: Recipient<RoomEvent>) -> usize {
        let id = self.connected_sessions.len() + 1;
        self.connected_sessions.insert(id, recipient);
        id
    }
}

impl Actor for GameServer {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype("()")]
pub struct PlayerJoined {
    pub username: String,
    pub session_id: usize,
    pub room_addr: Addr<Room>,
    pub room_key: String,
    pub recipient: Recipient<RoomEvent>,
}

impl Handler<WebsocketMessage> for GameServer {
    type Result = ();

    fn handle(&mut self, ws_message: WebsocketMessage, _: &mut Self::Context) {
        let session_id = ws_message.session_id;
        let content = ws_message.content;
        let mut message = content.chars();

        let first_char = if let Some(first_char) = message.next() {
            first_char
        } else {
            todo!("log error and return");
        };

        match first_char {
            'c' => {
                let username = message.collect();
                self.create_room(session_id, username);
            }
            'j' => {
                if let Some(split_index) = content.find(',') {
                    let username = content[1..split_index].to_string();
                    let room_key = content[split_index + 1..].to_string();

                    self.join_room(session_id, username, &room_key);
                } else {
                    todo!("handle error")
                }
            }
            _ => {}
        }
    }
}

impl Handler<SessionStarted> for GameServer {
    type Result = usize;

    fn handle(&mut self, msg: SessionStarted, _: &mut Self::Context) -> usize {
        self.connect(msg.recipient)
    }
}
