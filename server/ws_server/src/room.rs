use actix::prelude::*;
use game::*;
use std::collections::HashMap;

#[derive(Message, Clone)]
#[rtype("()")]
pub enum RoomEvent {
    PlayerJoined {
        player_id: usize,
        username: String,
    },
    GameStarted {
        player_order: Vec<usize>,
        game_settings: GameSettings,
    },
    PlayerHand {
        hand: Deck,
    },
    PlayerBid {
        player_id: usize,
        bid: Option<usize>,
    },
    NextBidder {
        player_id: usize,
    },
    BiddingOver {
        napoleon: Napoleon,
    },
    NoBids,
    BecomeAlly,
    AlliesChosen {
        allies: Vec<Card>,
    },
    CardPlayed {
        player_id: usize,
        card: Card,
    },
    NextPlayer {
        player_id: usize,
    },
    RoundOver {
        winner: usize,
    },
    GameOver {
        allies: Vec<usize>,
        // TODO: decide scoring system
        napoleon_score_delta: i32,
        player_score_delta: i32,
    },
}

pub enum RoomState {
    Lobby {},
    /// id_map: Index of the vector is the player id from the game, and the value is the session_id
    /// of the player
    InGame {
        game: Game,
        id_map: Vec<usize>,
    },
}

struct Occupant {
    recipient: Recipient<RoomEvent>,
    username: String,
}

pub struct Room {
    players: HashMap<usize, Occupant>,
    state: RoomState,
    host: usize,
}

impl Room {
    pub fn new(session_id: usize, username: String, session: Recipient<RoomEvent>) -> Room {
        let mut players = HashMap::new();
        players.insert(
            session_id,
            Occupant {
                recipient: session,
                username,
            },
        );

        Room {
            players,
            state: RoomState::Lobby {},
            host: session_id,
        }
    }

    fn send_recipient_event(recipient: &Recipient<RoomEvent>, event: RoomEvent) {
        let _ = recipient.do_send(event);
    }

    fn send_event(&self, session_id: &usize, event: RoomEvent) {
        if let Some(Occupant { recipient, .. }) = self.players.get(session_id) {
            Self::send_recipient_event(recipient, event);
        } else {
            // TODO: log error
        }
    }

    fn broadcast(&self, event: RoomEvent) {
        for occupant in self.players.values() {
            Self::send_recipient_event(&occupant.recipient, event.clone());
        }
    }

    fn join(&mut self, session_id: usize, username: String, session: Recipient<RoomEvent>) {
        self.players.insert(
            session_id,
            Occupant {
                username,
                recipient: session,
            },
        );
    }

    fn start_game(&mut self, session_id: usize, settings: GameSettings) {
        if let RoomState::Lobby {} = &self.state {
            assert_eq!(session_id, self.host, "Non-host tried to start game");
            let id_map = self.players.keys().cloned().collect();

            self.new_game(settings, id_map);
        } else {
            panic!(
                "Session {} tried to start game when the room state wasn't lobby",
                session_id
            );
        }
    }

    fn new_game(&mut self, game_settings: GameSettings, id_map: Vec<usize>) {
        let game = Game::new(id_map.len(), game_settings.clone());
        self.broadcast(RoomEvent::GameStarted {
            player_order: id_map.clone(),
            game_settings,
        });

        for (hand, session_id) in game.get_hands().iter().zip(id_map.iter()) {
            self.send_event(session_id, RoomEvent::PlayerHand { hand: hand.clone() });
        }

        self.broadcast(RoomEvent::NextBidder { player_id: 0 });

        self.state = RoomState::InGame { game, id_map };
    }

    fn bid(&mut self, session_id: usize, bid: Option<usize>) {
        use BiddingError::*;
        use BiddingEvent::*;

        if let RoomState::InGame {
            ref mut game,
            ref id_map,
        } = self.state
        {
            if let Some(player_id) = id_map.iter().position(|id| session_id == *id) {
                match game.bid(player_id, bid) {
                    Ok(event) => {
                        match event {
                            NextBidder {
                                player_id: next_bidder,
                            } => {
                                // Send a message saying this is the next bidder
                            }
                            BiddingFinished { napoleon: Napoleon } => {}
                        }
                    }
                    Err(error) => match error {
                        InvalidGameState => {
                            panic!(
                                "Session {} tried to bet when game state wasn't bidding",
                                session_id
                            );
                        }
                        InvalidRange => todo!("error"),
                        NoBids => {
                            let settings = game.get_settings().clone();
                            let id_map = id_map.clone();
                            self.broadcast(RoomEvent::NoBids);
                            self.new_game(settings, id_map);
                        }
                        NotCurrentPlayer => todo!("error"),
                    },
                }
            } else {
                todo!("non player tried to bet (spectator)");
            }
        } else {
            panic!(
                "Session {} tried to bid when the room state wasn't in game",
                session_id
            );
        }
    }
}

impl Actor for Room {
    type Context = Context<Self>;
}
