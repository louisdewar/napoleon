use crate::Event;
use actix::prelude::*;
use game::*;
use std::collections::HashMap;

pub enum RoomState {
    Lobby { host: usize },
    InGame { game: Game },
}

pub struct Room {
    players: HashMap<usize, Recipient<Event>>,
    state: RoomState,
}

impl Room {
    pub fn new(session_id: usize, session: Recipient<Event>) -> Room {
        let mut players = HashMap::new();
        players.insert(session_id, session);

        Room {
            players,
            state: RoomState::Lobby { host: session_id },
        }
    }
}
