use crate::{session::SessionStarted, Room, RoomEvent, WebsocketMessage};
use actix::prelude::*;
use std::collections::HashMap;

use slog::{error, warn, Logger};

pub struct GameServer {
    connected_sessions: HashMap<usize, Recipient<RoomEvent>>,
    rooms: HashMap<String, Addr<Room>>,
    logger: Logger,
}

impl GameServer {
    pub fn new(logger: Logger) -> GameServer {
        GameServer {
            connected_sessions: HashMap::new(),
            rooms: HashMap::new(),
            logger,
        }
    }

    fn create_room(&mut self, session_id: usize, username: String) {
        use rand::{distributions::Alphanumeric, Rng};
        use std::collections::hash_map::Entry;

        if let Some(recipient) = self.connected_sessions.get(&session_id) {
            let mut rng = rand::thread_rng();

            loop {
                let key: String = std::iter::repeat(())
                    .map(|()| rng.sample(Alphanumeric))
                    .take(5)
                    .collect();

                if let Entry::Vacant(entry) = self.rooms.entry(key.clone()) {
                    let address = Room::new(
                        session_id,
                        username.clone(),
                        recipient.clone(),
                        self.logger.new(slog::o!("room_key" => key.clone())),
                    )
                    .start();
                    entry.insert(address.clone());

                    let _ = recipient.do_send(RoomEvent::JoinedRoom {
                        key,
                        host: session_id,
                        address,
                        players: vec![(username, session_id)],
                    });
                    return;
                }
            }
        } else {
            error!(
                self.logger,
                "Session didn't exist when creating room";
                "session_id" => session_id
            );
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
            error!(
                self.logger,
                "Session didn't exist when creating room";
                "session_id" => session_id
            );
        }
    }

    fn connect(&mut self, recipient: Recipient<RoomEvent>) -> usize {
        use std::collections::hash_map::Entry;
        loop {
            let id = rand::random();

            if id == 0 {
                continue;
            }

            if let Entry::Vacant(entry) = self.connected_sessions.entry(id) {
                entry.insert(recipient);

                return id;
            }
        }
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
            warn!(ws_message.logger, "Empty message sent");
            return;
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
                    warn!(
                        ws_message.logger,
                        "Join room message was improperly formatted"
                    );
                    return;
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
