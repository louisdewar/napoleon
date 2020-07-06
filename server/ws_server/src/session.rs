use crate::{GameServer, Room};

use actix::prelude::*;

pub struct Session {
    user_id: usize,
    room: Option<Addr<Room>>,
    game_server: Addr<GameServer>,
}
