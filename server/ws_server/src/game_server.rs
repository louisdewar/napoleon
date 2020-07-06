use crate::{Event, Room};
use actix::prelude::*;
use std::collections::HashMap;

pub struct GameServer {
    connected_sessions: HashMap<usize, Recipient<Event>>,
}
